use crate::config::LoaderConfig;
use crate::errors::{PluginError, Result};
use crate::plugin::{PluginInfo, PluginMetadata, PluginFunction};
use std::path::Path;
use uuid::Uuid;

/// Plugin loader for loading plugin files and metadata
pub struct PluginLoader {
    config: LoaderConfig,
}

impl PluginLoader {
    /// Create a new plugin loader
    pub async fn new(config: LoaderConfig) -> Result<Self> {
        Ok(Self { config })
    }

    /// Load a plugin from a file
    pub async fn load_plugin(&self, path: &Path) -> Result<PluginInfo> {
        tracing::info!("Loading plugin file: {}", path.display());

        // Read plugin file
        let binary_data = tokio::fs::read(path).await?;

        // Verify checksum if configured
        if self.config.verify_checksums {
            // In a real implementation, would verify against known checksums
            tracing::debug!("Checksum verification enabled");
        }

        // Load metadata
        let metadata = self.load_metadata(path, &binary_data).await?;

        // Verify plugin API compatibility
        self.verify_api_compatibility(&metadata)?;

        Ok(PluginInfo {
            metadata,
            source_path: path.to_path_buf(),
            binary_data: if self.config.metadata_only {
                Vec::new()
            } else {
                binary_data
            },
        })
    }

    /// Load plugin metadata
    async fn load_metadata(
        &self,
        path: &Path,
        _binary_data: &[u8],
    ) -> Result<PluginMetadata> {
        // Look for metadata file next to plugin
        let metadata_path = path.with_extension("json");

        if metadata_path.exists() {
            // Load from metadata file
            let metadata_content = tokio::fs::read_to_string(&metadata_path).await?;
            let metadata: PluginMetadata = serde_json::from_str(&metadata_content)?;
            Ok(metadata)
        } else {
            // Generate default metadata
            tracing::warn!(
                "No metadata file found for plugin: {}, generating default",
                path.display()
            );
            Ok(self.generate_default_metadata(path))
        }
    }

    /// Generate default metadata for a plugin
    fn generate_default_metadata(&self, path: &Path) -> PluginMetadata {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        PluginMetadata {
            id: Uuid::new_v4(),
            name: name.clone(),
            version: semver::Version::new(0, 1, 0),
            description: Some(format!("Plugin: {}", name)),
            author: None,
            license: None,
            homepage: None,
            repository: None,
            keywords: Vec::new(),
            categories: Vec::new(),
            dependencies: Vec::new(),
            functions: vec![PluginFunction {
                name: "execute".to_string(),
                description: Some("Default execute function".to_string()),
                input_schema: None,
                output_schema: None,
            }],
            api_version: semver::VersionReq::parse("^1.0.0").unwrap(),
            checksum: None,
        }
    }

    /// Verify plugin API compatibility
    fn verify_api_compatibility(&self, metadata: &PluginMetadata) -> Result<()> {
        let current_api_version = semver::Version::parse(crate::PLUGIN_API_VERSION)
            .map_err(|e| PluginError::ValidationFailed(e.to_string()))?;

        if !metadata.api_version.matches(&current_api_version) {
            return Err(PluginError::ValidationFailed(format!(
                "Plugin API version mismatch: required {} but current is {}",
                metadata.api_version,
                current_api_version
            )));
        }

        Ok(())
    }
}
