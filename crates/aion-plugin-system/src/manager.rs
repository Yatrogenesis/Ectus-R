use crate::{
    plugin::*,
    loader::*,
    runtime::*,
    marketplace::*,
    security::*,
    events::*,
    config::*,
    errors::*,
    PluginContext,
    PluginExecutionResult,
    ExecutionLimits,
    PluginPermissions,
};
use dashmap::DashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Central plugin manager for loading, executing, and managing plugins
pub struct PluginManager {
    /// Loaded plugins registry
    plugins: Arc<DashMap<Uuid, Arc<LoadedPlugin>>>,
    /// Plugin runtime factory
    runtime_factory: Arc<RuntimeFactory>,
    /// Plugin loader
    loader: Arc<PluginLoader>,
    /// Security manager
    security: Arc<SecurityManager>,
    /// Event bus
    event_bus: Arc<EventBus>,
    /// Marketplace client
    marketplace: Arc<MarketplaceClient>,
    /// Configuration
    config: Arc<PluginSystemConfig>,
    /// Plugin watchers for hot-reload
    watchers: Arc<RwLock<DashMap<PathBuf, notify::RecommendedWatcher>>>,
}

impl PluginManager {
    /// Create a new plugin manager with default configuration
    pub async fn new() -> Result<Self> {
        let config = PluginSystemConfig::default();
        Self::new_with_config(config).await
    }

    /// Create a new plugin manager with custom configuration
    pub async fn new_with_config(config: PluginSystemConfig) -> Result<Self> {
        let config = Arc::new(config);

        let security = Arc::new(SecurityManager::new(config.security.clone()).await?);
        let event_bus = Arc::new(EventBus::new());
        let marketplace = Arc::new(MarketplaceClient::new(config.marketplace.clone()).await?);
        let runtime_factory = Arc::new(RuntimeFactory::new(config.runtime.clone()).await?);
        let loader = Arc::new(PluginLoader::new(config.loader.clone()).await?);

        let manager = Self {
            plugins: Arc::new(DashMap::new()),
            runtime_factory,
            loader,
            security,
            event_bus,
            marketplace,
            config,
            watchers: Arc::new(RwLock::new(DashMap::new())),
        };

        // Load plugins from configured directories
        manager.load_plugins_from_directories().await?;

        Ok(manager)
    }

    /// Load a plugin from a file path
    pub async fn load_plugin<P: AsRef<Path>>(&self, path: P) -> Result<Uuid> {
        let path = path.as_ref();
        tracing::info!("Loading plugin from: {}", path.display());

        // Security check
        self.security.validate_plugin_path(path).await?;

        // Load plugin metadata and binary
        let plugin_info = self.loader.load_plugin(path).await?;

        // Validate plugin
        self.security.validate_plugin(&plugin_info).await?;

        // Create runtime
        let runtime = self.runtime_factory.create_runtime(&plugin_info).await?;

        // Create loaded plugin
        let loaded_plugin = LoadedPlugin {
            id: plugin_info.metadata.id,
            info: plugin_info,
            runtime,
            loaded_at: chrono::Utc::now(),
            execution_count: std::sync::atomic::AtomicU64::new(0),
            last_executed: Arc::new(RwLock::new(None)),
        };

        let plugin_id = loaded_plugin.id;
        self.plugins.insert(plugin_id, Arc::new(loaded_plugin));

        // Emit plugin loaded event
        self.event_bus.emit(PluginEvent::Loaded {
            plugin_id,
            timestamp: chrono::Utc::now(),
        }).await;

        // Setup hot-reload watching if enabled
        if self.config.hot_reload.enabled {
            self.setup_hot_reload(path, plugin_id).await?;
        }

        tracing::info!("Plugin loaded successfully: {}", plugin_id);
        Ok(plugin_id)
    }

    /// Unload a plugin
    pub async fn unload_plugin(&self, plugin_id: &Uuid) -> Result<()> {
        tracing::info!("Unloading plugin: {}", plugin_id);

        if let Some((_, loaded_plugin)) = self.plugins.remove(plugin_id) {
            // Stop runtime
            loaded_plugin.runtime.shutdown().await?;

            // Remove hot-reload watcher
            let mut watchers = self.watchers.write().await;
            if let Some(path) = self.find_plugin_path(plugin_id) {
                watchers.remove(&path);
            }

            // Emit plugin unloaded event
            self.event_bus.emit(PluginEvent::Unloaded {
                plugin_id: *plugin_id,
                timestamp: chrono::Utc::now(),
            }).await;

            tracing::info!("Plugin unloaded successfully: {}", plugin_id);
            Ok(())
        } else {
            Err(PluginError::PluginNotFound(*plugin_id))
        }
    }

    /// Execute a plugin function
    pub async fn execute_plugin(
        &self,
        plugin_id: &Uuid,
        function: &str,
        input: serde_json::Value,
    ) -> Result<PluginExecutionResult> {
        self.execute_plugin_with_context(plugin_id, function, input, None).await
    }

    /// Execute a plugin function with custom context
    pub async fn execute_plugin_with_context(
        &self,
        plugin_id: &Uuid,
        function: &str,
        input: serde_json::Value,
        limits: Option<ExecutionLimits>,
    ) -> Result<PluginExecutionResult> {
        let execution_id = Uuid::new_v4();
        let start_time = std::time::Instant::now();

        tracing::info!(
            "Executing plugin function: {} {} {}",
            plugin_id,
            function,
            execution_id
        );

        // Get plugin
        let plugin = self.plugins.get(plugin_id)
            .ok_or(PluginError::PluginNotFound(*plugin_id))?;

        // Create execution context
        let context = PluginContext {
            execution_id,
            plugin_id: *plugin_id,
            timestamp: chrono::Utc::now(),
            input: input.clone(),
            environment: self.create_environment().await,
            limits: limits.unwrap_or_else(|| self.config.default_limits.clone()),
            permissions: self.get_plugin_permissions(plugin_id).await?,
        };

        // Security validation
        self.security.validate_execution(&context).await?;

        // Emit execution started event
        self.event_bus.emit(PluginEvent::ExecutionStarted {
            plugin_id: *plugin_id,
            execution_id,
            function: function.to_string(),
            timestamp: chrono::Utc::now(),
        }).await;

        // Execute plugin
        let result = match plugin.runtime.execute(function, &context).await {
            Ok(result) => {
                plugin.execution_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                *plugin.last_executed.write().await = Some(chrono::Utc::now());

                PluginExecutionResult {
                    execution_id,
                    plugin_id: *plugin_id,
                    success: true,
                    result: Some(result.output),
                    error: None,
                    duration_ms: start_time.elapsed().as_millis() as u64,
                    memory_usage: result.memory_usage,
                    generated_files: result.generated_files,
                    logs: result.logs,
                }
            }
            Err(error) => {
                PluginExecutionResult {
                    execution_id,
                    plugin_id: *plugin_id,
                    success: false,
                    result: None,
                    error: Some(error.to_string()),
                    duration_ms: start_time.elapsed().as_millis() as u64,
                    memory_usage: crate::MemoryUsage {
                        peak_bytes: 0,
                        final_bytes: 0,
                        allocations: 0,
                    },
                    generated_files: Vec::new(),
                    logs: Vec::new(),
                }
            }
        };

        // Emit execution completed event
        self.event_bus.emit(PluginEvent::ExecutionCompleted {
            plugin_id: *plugin_id,
            execution_id,
            success: result.success,
            duration_ms: result.duration_ms,
            timestamp: chrono::Utc::now(),
        }).await;

        tracing::info!(
            "Plugin execution completed: {} {} {} ({}ms)",
            plugin_id,
            function,
            execution_id,
            result.duration_ms
        );

        Ok(result)
    }

    /// List all loaded plugins
    pub async fn list_plugins(&self) -> Vec<PluginInfo> {
        self.plugins.iter()
            .map(|entry| (*entry.key(), entry.value().info.clone()))
            .collect()
    }

    /// Get plugin information
    pub async fn get_plugin_info(&self, plugin_id: &Uuid) -> Result<PluginInfo> {
        self.plugins.get(plugin_id)
            .map(|plugin| plugin.info.clone())
            .ok_or(PluginError::PluginNotFound(*plugin_id))
    }

    /// Get plugin statistics
    pub async fn get_plugin_stats(&self, plugin_id: &Uuid) -> Result<PluginStats> {
        let plugin = self.plugins.get(plugin_id)
            .ok_or(PluginError::PluginNotFound(*plugin_id))?;

        Ok(PluginStats {
            plugin_id: *plugin_id,
            loaded_at: plugin.loaded_at,
            execution_count: plugin.execution_count.load(std::sync::atomic::Ordering::Relaxed),
            last_executed: *plugin.last_executed.read().await,
            memory_usage: plugin.runtime.get_memory_usage().await?,
            status: plugin.runtime.get_status().await?,
        })
    }

    /// Install plugin from marketplace
    pub async fn install_plugin(&self, plugin_name: &str, version: Option<&str>) -> Result<Uuid> {
        tracing::info!("Installing plugin from marketplace: {} {:?}", plugin_name, version);

        // Download plugin from marketplace
        let plugin_package = self.marketplace.download_plugin(plugin_name, version).await?;

        // Verify plugin signature
        self.security.verify_plugin_signature(&plugin_package).await?;

        // Extract plugin to plugins directory
        let plugin_path = self.extract_plugin_package(&plugin_package).await?;

        // Load plugin
        let plugin_id = self.load_plugin(&plugin_path).await?;

        // Mark as installed
        self.marketplace.mark_plugin_installed(plugin_name, &plugin_id).await?;

        tracing::info!("Plugin installed successfully: {} -> {}", plugin_name, plugin_id);
        Ok(plugin_id)
    }

    /// Uninstall plugin
    pub async fn uninstall_plugin(&self, plugin_id: &Uuid) -> Result<()> {
        // Unload plugin
        self.unload_plugin(plugin_id).await?;

        // Remove plugin files
        if let Some(plugin_path) = self.find_plugin_path(plugin_id) {
            if plugin_path.exists() {
                if plugin_path.is_file() {
                    tokio::fs::remove_file(&plugin_path).await?;
                } else {
                    tokio::fs::remove_dir_all(&plugin_path).await?;
                }
            }
        }

        // Mark as uninstalled in marketplace
        self.marketplace.mark_plugin_uninstalled(plugin_id).await?;

        tracing::info!("Plugin uninstalled successfully: {}", plugin_id);
        Ok(())
    }

    /// Reload a plugin (hot-reload)
    pub async fn reload_plugin(&self, plugin_id: &Uuid) -> Result<()> {
        tracing::info!("Reloading plugin: {}", plugin_id);

        // Find plugin path
        let plugin_path = self.find_plugin_path(plugin_id)
            .ok_or(PluginError::PluginNotFound(*plugin_id))?;

        // Unload current plugin
        self.unload_plugin(plugin_id).await?;

        // Load plugin again
        let new_plugin_id = self.load_plugin(&plugin_path).await?;

        if new_plugin_id != *plugin_id {
            tracing::warn!(
                "Plugin ID changed after reload: {} -> {}",
                plugin_id,
                new_plugin_id
            );
        }

        tracing::info!("Plugin reloaded successfully: {}", plugin_id);
        Ok(())
    }

    /// Subscribe to plugin events
    pub async fn subscribe_events(&self) -> tokio::sync::broadcast::Receiver<PluginEvent> {
        self.event_bus.subscribe().await
    }

    /// Shutdown plugin manager
    pub async fn shutdown(&self) -> Result<()> {
        tracing::info!("Shutting down plugin manager");

        // Unload all plugins
        let plugin_ids: Vec<Uuid> = self.plugins.iter().map(|entry| *entry.key()).collect();
        for plugin_id in plugin_ids {
            if let Err(e) = self.unload_plugin(&plugin_id).await {
                tracing::error!("Failed to unload plugin {}: {}", plugin_id, e);
            }
        }

        // Stop file watchers
        self.watchers.write().await.clear();

        tracing::info!("Plugin manager shutdown complete");
        Ok(())
    }

    /// Load plugins from configured directories
    async fn load_plugins_from_directories(&self) -> Result<()> {
        for plugin_dir in &self.config.plugin_directories {
            if !plugin_dir.exists() {
                tracing::warn!("Plugin directory does not exist: {}", plugin_dir.display());
                continue;
            }

            self.load_plugins_from_directory(plugin_dir).await?;
        }

        Ok(())
    }

    /// Load plugins from a directory
    async fn load_plugins_from_directory(&self, dir: &Path) -> Result<()> {
        let mut entries = tokio::fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.is_file() {
                let extension = path.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("");

                if self.is_plugin_file(extension) {
                    if let Err(e) = self.load_plugin(&path).await {
                        tracing::error!("Failed to load plugin {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok(())
    }

    /// Check if file extension indicates a plugin
    fn is_plugin_file(&self, extension: &str) -> bool {
        matches!(extension, "wasm" | "dll" | "so" | "dylib")
    }

    /// Setup hot-reload watching for a plugin
    async fn setup_hot_reload(&self, path: &Path, plugin_id: Uuid) -> Result<()> {
        use notify::{Watcher, RecursiveMode, Event, EventKind};

        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                if matches!(event.kind, EventKind::Modify(_)) {
                    let _ = tx.try_send(event);
                }
            }
        })?;

        watcher.watch(path, RecursiveMode::NonRecursive)?;

        let manager = Arc::downgrade(&Arc::new(self.clone()));
        let plugin_path = path.to_path_buf();

        tokio::spawn(async move {
            while let Some(_event) = rx.recv().await {
                if let Some(manager) = manager.upgrade() {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                    if let Err(e) = manager.reload_plugin(&plugin_id).await {
                        tracing::error!("Hot-reload failed for plugin {}: {}", plugin_id, e);
                    }
                }
            }
        });

        self.watchers.write().await.insert(plugin_path, watcher);
        Ok(())
    }

    /// Create execution environment variables
    async fn create_environment(&self) -> std::collections::HashMap<String, String> {
        let mut env = std::collections::HashMap::new();
        env.insert("AION_VERSION".to_string(), crate::PLUGIN_SYSTEM_VERSION.to_string());
        env.insert("AION_API_VERSION".to_string(), crate::PLUGIN_API_VERSION.to_string());
        env.insert("AION_TIMESTAMP".to_string(), chrono::Utc::now().to_rfc3339());
        env
    }

    /// Get plugin permissions
    async fn get_plugin_permissions(&self, plugin_id: &Uuid) -> Result<PluginPermissions> {
        // Default permissions - in production this would come from configuration/database
        Ok(PluginPermissions::default())
    }

    /// Find plugin file path by ID
    fn find_plugin_path(&self, plugin_id: &Uuid) -> Option<PathBuf> {
        self.plugins.get(plugin_id)
            .map(|plugin| plugin.info.source_path.clone())
    }

    /// Extract plugin package
    async fn extract_plugin_package(&self, package: &PluginPackage) -> Result<PathBuf> {
        let extract_dir = self.config.plugin_directories[0].join(&package.name);
        tokio::fs::create_dir_all(&extract_dir).await?;

        // Extract tarball
        let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(package.data.as_slice()));
        archive.unpack(&extract_dir)?;

        Ok(extract_dir.join(&package.main_file))
    }
}

// Implement Clone for PluginManager to enable use in hot-reload
impl Clone for PluginManager {
    fn clone(&self) -> Self {
        Self {
            plugins: Arc::clone(&self.plugins),
            runtime_factory: Arc::clone(&self.runtime_factory),
            loader: Arc::clone(&self.loader),
            security: Arc::clone(&self.security),
            event_bus: Arc::clone(&self.event_bus),
            marketplace: Arc::clone(&self.marketplace),
            config: Arc::clone(&self.config),
            watchers: Arc::clone(&self.watchers),
        }
    }
}