//! API Request Handlers
//! Contains all HTTP request handlers for the Ectus-R Web API

pub mod system;
pub mod ai;
pub mod deployments;
pub mod projects;
pub mod auth;
pub mod admin;
pub mod dashboard;
pub mod optimization;
pub mod payments;
pub mod analytics;

// Re-export handler functions
pub use system::*;
pub use ai::*;
pub use deployments::*;
pub use projects::*;
pub use auth::*;
pub use admin::*;
pub use dashboard::*;
pub use optimization::*;
pub use payments::*;
pub use analytics::*;