//! # AI Engine Error Types
//!
//! Comprehensive error handling for the AI engine.

use thiserror::Error;

/// AI Engine specific errors
#[derive(Error, Debug)]
pub enum AIEngineError {
    #[error("Model not found: {model}")]
    ModelNotFound { model: String },

    #[error("Inference backend not available: {backend}")]
    BackendNotAvailable { backend: String },

    #[error("Invalid input format for model {model}: {reason}")]
    InvalidInputFormat { model: String, reason: String },

    #[error("Model loading failed: {model} - {reason}")]
    ModelLoadingFailed { model: String, reason: String },

    #[error("Inference timeout after {timeout_ms}ms")]
    InferenceTimeout { timeout_ms: u64 },

    #[error("Memory limit exceeded: requested {requested} bytes, limit {limit} bytes")]
    MemoryLimitExceeded { requested: usize, limit: usize },

    #[error("Concurrent inference limit reached: {limit}")]
    ConcurrencyLimitReached { limit: usize },

    #[error("Model cache error: {reason}")]
    CacheError { reason: String },

    #[error("Preprocessing failed: {reason}")]
    PreprocessingFailed { reason: String },

    #[error("Postprocessing failed: {reason}")]
    PostprocessingFailed { reason: String },

    #[error("Configuration error: {field} - {reason}")]
    ConfigurationError { field: String, reason: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Join error: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("Generic error: {0}")]
    Generic(#[from] anyhow::Error),
}

/// Result type for AI Engine operations
pub type AIResult<T> = Result<T, AIEngineError>;

/// Error context for better error reporting
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorContext {
    /// Model being used when error occurred
    pub model: Option<String>,
    /// Backend being used when error occurred
    pub backend: Option<String>,
    /// Request ID associated with the error
    pub request_id: Option<uuid::Uuid>,
    /// Additional context information
    pub context: std::collections::HashMap<String, String>,
    /// Timestamp when error occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ErrorContext {
    pub fn new() -> Self {
        Self {
            model: None,
            backend: None,
            request_id: None,
            context: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    pub fn with_backend(mut self, backend: String) -> Self {
        self.backend = Some(backend);
        self
    }

    pub fn with_request_id(mut self, request_id: uuid::Uuid) -> Self {
        self.request_id = Some(request_id);
        self
    }

    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Enhanced error with context
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextualError {
    pub error: String,
    pub context: ErrorContext,
}

impl ContextualError {
    pub fn new(error: impl ToString, context: ErrorContext) -> Self {
        Self {
            error: error.to_string(),
            context,
        }
    }
}

impl std::fmt::Display for ContextualError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)?;

        if let Some(model) = &self.context.model {
            write!(f, " (model: {})", model)?;
        }

        if let Some(backend) = &self.context.backend {
            write!(f, " (backend: {})", backend)?;
        }

        if let Some(request_id) = &self.context.request_id {
            write!(f, " (request: {})", request_id)?;
        }

        Ok(())
    }
}

impl std::error::Error for ContextualError {}

/// Error recovery strategies
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ErrorRecoveryStrategy {
    /// Retry with exponential backoff
    RetryWithBackoff {
        max_retries: u32,
        initial_delay_ms: u64,
        max_delay_ms: u64,
    },
    /// Fallback to different model
    FallbackModel { fallback_model: String },
    /// Fallback to different backend
    FallbackBackend { fallback_backend: String },
    /// Use cached result if available
    UseCachedResult,
    /// Return default/safe result
    ReturnDefault,
    /// Fail fast without recovery
    FailFast,
}

/// Error metrics for monitoring
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorMetrics {
    pub total_errors: u64,
    pub errors_by_type: std::collections::HashMap<String, u64>,
    pub errors_by_model: std::collections::HashMap<String, u64>,
    pub errors_by_backend: std::collections::HashMap<String, u64>,
    pub recovery_attempts: u64,
    pub successful_recoveries: u64,
}

impl Default for ErrorMetrics {
    fn default() -> Self {
        Self {
            total_errors: 0,
            errors_by_type: std::collections::HashMap::new(),
            errors_by_model: std::collections::HashMap::new(),
            errors_by_backend: std::collections::HashMap::new(),
            recovery_attempts: 0,
            successful_recoveries: 0,
        }
    }
}

/// Utility functions for error handling
pub mod utils {
    use super::*;

    /// Check if an error is recoverable
    pub fn is_recoverable_error(error: &AIEngineError) -> bool {
        match error {
            AIEngineError::InferenceTimeout { .. } => true,
            AIEngineError::MemoryLimitExceeded { .. } => false,
            AIEngineError::ConcurrencyLimitReached { .. } => true,
            AIEngineError::ModelNotFound { .. } => false,
            AIEngineError::BackendNotAvailable { .. } => true,
            AIEngineError::CacheError { .. } => true,
            AIEngineError::Io(_) => true,
            AIEngineError::Http(_) => true,
            _ => false,
        }
    }

    /// Get suggested recovery strategy for an error
    pub fn get_recovery_strategy(error: &AIEngineError) -> Option<ErrorRecoveryStrategy> {
        match error {
            AIEngineError::InferenceTimeout { .. } => Some(ErrorRecoveryStrategy::RetryWithBackoff {
                max_retries: 3,
                initial_delay_ms: 1000,
                max_delay_ms: 5000,
            }),
            AIEngineError::ConcurrencyLimitReached { .. } => {
                Some(ErrorRecoveryStrategy::RetryWithBackoff {
                    max_retries: 5,
                    initial_delay_ms: 100,
                    max_delay_ms: 1000,
                })
            }
            AIEngineError::BackendNotAvailable { .. } => {
                Some(ErrorRecoveryStrategy::FallbackBackend {
                    fallback_backend: "candle".to_string(),
                })
            }
            AIEngineError::CacheError { .. } => Some(ErrorRecoveryStrategy::RetryWithBackoff {
                max_retries: 2,
                initial_delay_ms: 500,
                max_delay_ms: 2000,
            }),
            _ => None,
        }
    }

    /// Create error context from inference request
    pub fn error_context_from_request(
        request: &crate::inference::InferenceRequest,
    ) -> ErrorContext {
        ErrorContext::new()
            .with_model(request.model.clone())
            .with_request_id(request.id)
            .with_context(
                "input_type".to_string(),
                format!("{:?}", std::mem::discriminant(&request.input)),
            )
    }
}