//! # AION Plugin System
//!
//! Extensible plugin architecture for the AION autonomous software engineering platform.
//!
//! ## Features
//! - Native dynamic library plugins (.dll/.so/.dylib)
//! - WebAssembly (WASM) plugins with WASI support
//! - Plugin lifecycle management (load, unload, hot-reload)
//! - Secure sandbox execution for plugins
//! - Plugin dependency resolution and versioning
//! - Event-driven plugin communication
//! - Plugin marketplace integration
//! - Performance monitoring and resource limits
//! - Plugin configuration and permissions
//!
//! ## Plugin Types
//! - **Code Generators**: Custom project templates and scaffolding
//! - **Quality Analyzers**: Custom testing and analysis tools
//! - **Build Tools**: Custom build pipelines and optimization
//! - **Language Processors**: Support for new programming languages
//! - **Deployment Providers**: Custom deployment targets and strategies
//! - **Monitoring Tools**: Custom metrics and observability
//! - **Integration Connectors**: Third-party service integrations
//!
//! ## Example
//! ```rust
//! use aion_plugin_system::{PluginManager, PluginConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut manager = PluginManager::new().await?;
//!
//!     // Load a plugin
//!     let plugin_id = manager.load_plugin("./plugins/my-generator.wasm").await?;
//!
//!     // Execute plugin
//!     let result = manager.execute_plugin(&plugin_id, "generate", serde_json::json!({
//!         "template": "react-app",
//!         "name": "my-project"
//!     })).await?;
//!
//!     println!("Generated: {:?}", result);
//!     Ok(())
//! }
//! ```

pub mod manager;
pub mod plugin;
pub mod loader;
pub mod runtime;
pub mod marketplace;
pub mod security;
pub mod events;
pub mod config;
pub mod errors;

pub use manager::*;
pub use plugin::*;
pub use loader::*;
pub use runtime::*;
pub use marketplace::*;
pub use security::*;
pub use events::*;
pub use config::*;
pub use errors::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Plugin system version
pub const PLUGIN_SYSTEM_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Plugin API version compatibility
pub const PLUGIN_API_VERSION: &str = "1.0.0";

/// Maximum plugin execution timeout in seconds
pub const MAX_PLUGIN_EXECUTION_TIMEOUT: u64 = 300;

/// Maximum plugin memory usage in bytes (256 MB)
pub const MAX_PLUGIN_MEMORY: usize = 256 * 1024 * 1024;

/// Plugin execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContext {
    /// Unique execution ID
    pub execution_id: Uuid,
    /// Plugin ID
    pub plugin_id: Uuid,
    /// Execution timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Input parameters
    pub input: serde_json::Value,
    /// Environment variables
    pub environment: HashMap<String, String>,
    /// Execution limits
    pub limits: ExecutionLimits,
    /// User permissions
    pub permissions: PluginPermissions,
}

/// Plugin execution limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionLimits {
    /// Maximum execution time in seconds
    pub timeout_seconds: u64,
    /// Maximum memory usage in bytes
    pub max_memory_bytes: usize,
    /// Maximum file system operations
    pub max_fs_operations: Option<usize>,
    /// Maximum network requests
    pub max_network_requests: Option<usize>,
    /// CPU usage percentage limit
    pub cpu_limit_percent: Option<f32>,
}

impl Default for ExecutionLimits {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_memory_bytes: 64 * 1024 * 1024, // 64 MB
            max_fs_operations: Some(1000),
            max_network_requests: Some(50),
            cpu_limit_percent: Some(80.0),
        }
    }
}

/// Plugin execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginExecutionResult {
    /// Execution ID
    pub execution_id: Uuid,
    /// Plugin ID
    pub plugin_id: Uuid,
    /// Success status
    pub success: bool,
    /// Result data
    pub result: Option<serde_json::Value>,
    /// Error message if failed
    pub error: Option<String>,
    /// Execution duration in milliseconds
    pub duration_ms: u64,
    /// Memory usage statistics
    pub memory_usage: MemoryUsage,
    /// Generated files
    pub generated_files: Vec<GeneratedFile>,
    /// Log messages
    pub logs: Vec<LogEntry>,
}

/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsage {
    /// Peak memory usage in bytes
    pub peak_bytes: usize,
    /// Final memory usage in bytes
    pub final_bytes: usize,
    /// Number of allocations
    pub allocations: usize,
}

/// Generated file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    /// File path relative to project root
    pub path: String,
    /// File content or None if binary
    pub content: Option<String>,
    /// Whether file is binary
    pub is_binary: bool,
    /// File permissions (Unix style)
    pub permissions: Option<u32>,
    /// File size in bytes
    pub size_bytes: usize,
}

/// Plugin log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Log level
    pub level: LogLevel,
    /// Log message
    pub message: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Additional context
    pub context: Option<serde_json::Value>,
}

/// Log levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Initialize the plugin system
pub async fn initialize_plugin_system(config: PluginSystemConfig) -> anyhow::Result<PluginManager> {
    tracing::info!("Initializing AION Plugin System v{}", PLUGIN_SYSTEM_VERSION);

    let manager = PluginManager::new_with_config(config).await?;

    tracing::info!("Plugin system initialized successfully");
    Ok(manager)
}