use thiserror::Error;

#[derive(Error, Debug)]
pub enum AionError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Rate limit exceeded. Retry after: {retry_after:?} seconds")]
    RateLimit { retry_after: Option<u64> },

    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Network timeout")]
    Timeout,

    #[error("Connection error: {0}")]
    Connection(String),
}

pub type Result<T> = std::result::Result<T, AionError>;

impl AionError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            AionError::Timeout
            | AionError::Connection(_)
            | AionError::RateLimit { .. }
            | AionError::Http(_)
        )
    }

    pub fn retry_after(&self) -> Option<u64> {
        match self {
            AionError::RateLimit { retry_after } => *retry_after,
            _ => None,
        }
    }
}