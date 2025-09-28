//! # AION API Client SDK
//!
//! Rust SDK for the AION autonomous software engineering platform.
//!
//! ## Features
//! - Full API coverage for project management
//! - Real-time progress tracking via WebSocket
//! - Template and scaffolding management
//! - QA automation and testing
//! - Performance monitoring
//! - Async/await support with Tokio
//!
//! ## Example
//! ```rust
//! use aion_api_client::{AionClient, ProjectRequest};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = AionClient::new("https://api.aion.dev", "your-api-key")?;
//!
//!     let project = client.projects().create(ProjectRequest {
//!         name: "my-app".to_string(),
//!         description: Some("A sample application".to_string()),
//!         tech_stack: vec!["rust".to_string(), "actix-web".to_string()],
//!         ..Default::default()
//!     }).await?;
//!
//!     println!("Created project: {}", project.id);
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod types;
pub mod projects;
pub mod templates;
pub mod qa;
pub mod progress;
pub mod websocket;
pub mod error;

pub use client::*;
pub use types::*;
pub use projects::*;
pub use templates::*;
pub use qa::*;
pub use progress::*;
pub use websocket::*;
pub use error::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_initialization() {
        let client = AionClient::new("https://api.example.com", "test-key");
        assert!(client.is_ok());
    }
}