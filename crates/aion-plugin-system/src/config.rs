use crate::ExecutionLimits;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Plugin system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSystemConfig {
    /// Plugin directories to scan
    pub plugin_directories: Vec<PathBuf>,
    /// Default execution limits
    pub default_limits: ExecutionLimits,
    /// Hot reload configuration
    pub hot_reload: HotReloadConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Marketplace configuration
    pub marketplace: MarketplaceConfig,
    /// Runtime configuration
    pub runtime: RuntimeConfig,
    /// Loader configuration
    pub loader: LoaderConfig,
}

impl Default for PluginSystemConfig {
    fn default() -> Self {
        Self {
            plugin_directories: vec![PathBuf::from("./plugins")],
            default_limits: ExecutionLimits::default(),
            hot_reload: HotReloadConfig::default(),
            security: SecurityConfig::default(),
            marketplace: MarketplaceConfig::default(),
            runtime: RuntimeConfig::default(),
            loader: LoaderConfig::default(),
        }
    }
}

/// Hot reload configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotReloadConfig {
    /// Enable hot reload
    pub enabled: bool,
    /// Debounce delay in milliseconds
    pub debounce_ms: u64,
}

impl Default for HotReloadConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            debounce_ms: 100,
        }
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable signature verification
    pub verify_signatures: bool,
    /// Trusted public keys
    pub trusted_keys: Vec<String>,
    /// Enable sandbox
    pub enable_sandbox: bool,
    /// Maximum plugin size in bytes
    pub max_plugin_size: usize,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            verify_signatures: false,
            trusted_keys: Vec::new(),
            enable_sandbox: true,
            max_plugin_size: 100 * 1024 * 1024, // 100 MB
        }
    }
}

/// Marketplace configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceConfig {
    /// Marketplace API URL
    pub api_url: String,
    /// API authentication token
    pub api_token: Option<String>,
    /// Enable marketplace
    pub enabled: bool,
    /// Cache directory
    pub cache_dir: PathBuf,
}

impl Default for MarketplaceConfig {
    fn default() -> Self {
        Self {
            api_url: "https://marketplace.aion.dev/api".to_string(),
            api_token: None,
            enabled: false,
            cache_dir: PathBuf::from("./cache/marketplace"),
        }
    }
}

/// Runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Enable native plugins
    pub enable_native: bool,
    /// Enable WASM plugins
    pub enable_wasm: bool,
    /// WASM memory limit in pages
    pub wasm_memory_pages: u32,
    /// WASM stack size
    pub wasm_stack_size: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            enable_native: true,
            enable_wasm: true,
            wasm_memory_pages: 1024, // 64 MB
            wasm_stack_size: 1024 * 1024, // 1 MB
        }
    }
}

/// Loader configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoaderConfig {
    /// Verify checksums
    pub verify_checksums: bool,
    /// Load plugin metadata only
    pub metadata_only: bool,
}

impl Default for LoaderConfig {
    fn default() -> Self {
        Self {
            verify_checksums: true,
            metadata_only: false,
        }
    }
}
