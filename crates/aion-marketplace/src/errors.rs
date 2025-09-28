use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum MarketplaceError {
    #[error("Database connection failed: {0}")]
    DatabaseConnection(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Package not found: {0}")]
    PackageNotFound(Uuid),

    #[error("User not found: {0}")]
    UserNotFound(Uuid),

    #[error("Version not found: {0}")]
    VersionNotFound(semver::Version),

    #[error("Package validation failed: {0:?}")]
    ValidationFailed(Vec<String>),

    #[error("Security threat detected: {0:?}")]
    SecurityThreatDetected(Vec<String>),

    #[error("Invalid version format: {0}")]
    InvalidVersion(String),

    #[error("Access denied")]
    AccessDenied,

    #[error("Version yanked: {0:?}")]
    VersionYanked(Option<String>),

    #[error("File integrity check failed")]
    FileIntegrityCheckFailed,

    #[error("Already reviewed this package")]
    AlreadyReviewed,

    #[error("Package is not paid")]
    PackageNotPaid,

    #[error("Payment processing failed: {0}")]
    PaymentFailed(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Search engine error: {0}")]
    SearchError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),
}

pub type Result<T> = std::result::Result<T, MarketplaceError>;