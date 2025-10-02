use thiserror::Error;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, PluginError>;

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    PluginNotFound(Uuid),

    #[error("Invalid plugin format: {0}")]
    InvalidFormat(String),

    #[error("Plugin validation failed: {0}")]
    ValidationFailed(String),

    #[error("Plugin execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Plugin load error: {0}")]
    LoadError(String),

    #[error("Security violation: {0}")]
    SecurityViolation(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Marketplace error: {0}")]
    MarketplaceError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("File watcher error: {0}")]
    Notify(#[from] notify::Error),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}
