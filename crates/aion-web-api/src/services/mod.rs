//! Service layer for business logic

pub mod monitoring;
pub mod ai;
pub mod deployment;
pub mod auth;
pub mod email_marketing;

// Re-export services
pub use monitoring::MonitoringService;
pub use ai::AIService;
pub use deployment::DeploymentService;
pub use auth::AuthService;
pub use email_marketing::EmailMarketingService;