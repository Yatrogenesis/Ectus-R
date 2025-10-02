use crate::{MemoryUsage, GeneratedFile, LogEntry};
use crate::runtime::PluginRuntime;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Plugin information loaded from plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Plugin metadata
    pub metadata: PluginMetadata,
    /// Source file path
    pub source_path: PathBuf,
    /// Plugin binary data
    #[serde(skip)]
    pub binary_data: Vec<u8>,
}

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Unique plugin ID
    pub id: Uuid,
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: semver::Version,
    /// Plugin description
    pub description: Option<String>,
    /// Plugin author
    pub author: Option<String>,
    /// Plugin license
    pub license: Option<String>,
    /// Plugin homepage
    pub homepage: Option<String>,
    /// Plugin repository
    pub repository: Option<String>,
    /// Plugin keywords
    pub keywords: Vec<String>,
    /// Plugin categories
    pub categories: Vec<String>,
    /// Plugin dependencies
    pub dependencies: Vec<PluginDependency>,
    /// Exported functions
    pub functions: Vec<PluginFunction>,
    /// Required AION API version
    pub api_version: semver::VersionReq,
    /// Plugin checksum (SHA256)
    pub checksum: Option<String>,
}

/// Plugin dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// Dependency plugin name
    pub name: String,
    /// Required version
    pub version: semver::VersionReq,
    /// Is dependency optional
    pub optional: bool,
}

/// Plugin function declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginFunction {
    /// Function name
    pub name: String,
    /// Function description
    pub description: Option<String>,
    /// Input schema (JSON Schema)
    pub input_schema: Option<serde_json::Value>,
    /// Output schema (JSON Schema)
    pub output_schema: Option<serde_json::Value>,
}

/// Loaded plugin instance
pub struct LoadedPlugin {
    /// Plugin ID
    pub id: Uuid,
    /// Plugin information
    pub info: PluginInfo,
    /// Plugin runtime instance
    pub runtime: Arc<dyn PluginRuntime>,
    /// When the plugin was loaded
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    /// Total execution count
    pub execution_count: AtomicU64,
    /// Last execution timestamp
    pub last_executed: Arc<RwLock<Option<chrono::DateTime<chrono::Utc>>>>,
}

/// Plugin statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginStats {
    /// Plugin ID
    pub plugin_id: Uuid,
    /// When loaded
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    /// Total executions
    pub execution_count: u64,
    /// Last execution
    pub last_executed: Option<chrono::DateTime<chrono::Utc>>,
    /// Current memory usage
    pub memory_usage: usize,
    /// Plugin status
    pub status: PluginStatus,
}

/// Plugin status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PluginStatus {
    /// Plugin is ready to execute
    Ready,
    /// Plugin is currently executing
    Executing,
    /// Plugin has error
    Error,
    /// Plugin is shutting down
    ShuttingDown,
}

/// Plugin permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginPermissions {
    /// Can read files
    pub read_files: bool,
    /// Can write files
    pub write_files: bool,
    /// Can make network requests
    pub network_access: bool,
    /// Can execute system commands
    pub execute_commands: bool,
    /// Can access environment variables
    pub env_access: bool,
    /// Allowed file paths (globs)
    pub allowed_paths: Vec<String>,
    /// Blocked file paths (globs)
    pub blocked_paths: Vec<String>,
    /// Allowed network domains
    pub allowed_domains: Vec<String>,
}

impl Default for PluginPermissions {
    fn default() -> Self {
        Self {
            read_files: true,
            write_files: false,
            network_access: false,
            execute_commands: false,
            env_access: false,
            allowed_paths: vec!["**/*".to_string()],
            blocked_paths: Vec::new(),
            allowed_domains: Vec::new(),
        }
    }
}

/// Plugin runtime execution result
#[derive(Debug, Clone)]
pub struct RuntimeExecutionResult {
    /// Output data
    pub output: serde_json::Value,
    /// Memory usage stats
    pub memory_usage: MemoryUsage,
    /// Generated files
    pub generated_files: Vec<GeneratedFile>,
    /// Log entries
    pub logs: Vec<LogEntry>,
}

/// Plugin package for installation
#[derive(Debug, Clone)]
pub struct PluginPackage {
    /// Package name
    pub name: String,
    /// Package version
    pub version: semver::Version,
    /// Main plugin file
    pub main_file: String,
    /// Package data (tarball)
    pub data: Vec<u8>,
    /// Package signature
    pub signature: Option<Vec<u8>>,
}
