//! # AION Marketplace
//!
//! Comprehensive marketplace foundation for AION templates, plugins, and autonomous software components.
//!
//! ## Features
//! - **Template Marketplace**: Discover and share project templates
//! - **Plugin Ecosystem**: Browse and install autonomous extensions
//! - **Component Library**: Reusable software modules and libraries
//! - **Commercial Support**: Paid templates and premium plugins
//! - **Quality Assurance**: Automated testing and validation
//! - **Security Scanning**: Malware and vulnerability detection
//! - **Version Management**: Semantic versioning and compatibility
//! - **Analytics & Metrics**: Usage statistics and performance data
//! - **Developer Monetization**: Revenue sharing and subscriptions
//! - **Enterprise Features**: Private marketplaces and custom catalogs
//!
//! ## Architecture
//! ```text
//! ┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
//! │   Web Portal    │    │   REST API       │    │   Admin Panel   │
//! │   (React SPA)   │◄──►│   (Axum Server)  │◄──►│   (Management)  │
//! └─────────────────┘    └──────────────────┘    └─────────────────┘
//!                                 │
//!                        ┌────────▼────────┐
//!                        │   Core Engine   │
//!                        │   (Rust Logic)  │
//!                        └────────┬────────┘
//!                                 │
//!         ┌───────────────────────┼───────────────────────┐
//!         │                       │                       │
//! ┌───────▼───────┐    ┌─────────▼─────────┐    ┌────────▼────────┐
//! │   Database    │    │   File Storage    │    │   Cache Layer   │
//! │  (PostgreSQL) │    │   (S3/MinIO)     │    │    (Redis)      │
//! └───────────────┘    └───────────────────┘    └─────────────────┘
//! ```
//!
//! ## Example Usage
//! ```rust
//! use aion_marketplace::{Marketplace, MarketplaceConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = MarketplaceConfig::from_env()?;
//!     let marketplace = Marketplace::new(config).await?;
//!
//!     // Search for templates
//!     let templates = marketplace.search_templates("react", None).await?;
//!     println!("Found {} React templates", templates.len());
//!
//!     // Install a plugin
//!     let plugin = marketplace.install_plugin("ai-code-reviewer", "1.2.0").await?;
//!     println!("Installed plugin: {}", plugin.id);
//!
//!     Ok(())
//! }
//! ```

pub mod core;
pub mod api;
pub mod models;
pub mod database;
pub mod storage;
pub mod security;
pub mod analytics;
pub mod payments;
pub mod notifications;
pub mod search;
pub mod validation;
pub mod errors;
pub mod config;

// Re-export dummy types for plugin system integration
pub struct PluginInfo;
pub struct PluginContext;
pub struct PluginPackage;

pub use core::*;
pub use api::*;
pub use models::*;
pub use database::*;
pub use storage::*;
pub use security::*;
pub use analytics::*;
pub use payments::*;
pub use notifications::*;
pub use search::*;
pub use validation::*;
pub use errors::*;
pub use config::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Marketplace version
pub const MARKETPLACE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Supported package types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PackageType {
    /// Project templates
    Template,
    /// Runtime plugins
    Plugin,
    /// Code libraries
    Library,
    /// Development tools
    Tool,
    /// Deployment configurations
    Deployment,
    /// Custom components
    Component,
}

/// Package visibility levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PackageVisibility {
    /// Public packages available to all users
    Public,
    /// Private packages for organization/user only
    Private,
    /// Unlisted packages accessible via direct link
    Unlisted,
}

/// Package licensing options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PackageLicense {
    /// Open source licenses
    OpenSource(String),
    /// Commercial/proprietary license
    Commercial,
    /// Custom license terms
    Custom(String),
}

/// Marketplace statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceStats {
    /// Total number of packages
    pub total_packages: u64,
    /// Total downloads across all packages
    pub total_downloads: u64,
    /// Number of active developers
    pub active_developers: u64,
    /// Packages by type
    pub packages_by_type: HashMap<PackageType, u64>,
    /// Top downloaded packages
    pub top_packages: Vec<PackageSummary>,
    /// Recent activity metrics
    pub recent_activity: ActivityMetrics,
}

/// Package summary for listings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSummary {
    /// Package ID
    pub id: Uuid,
    /// Package name
    pub name: String,
    /// Package description
    pub description: String,
    /// Package type
    pub package_type: PackageType,
    /// Current version
    pub version: semver::Version,
    /// Download count
    pub downloads: u64,
    /// Average rating
    pub rating: f32,
    /// Number of reviews
    pub review_count: u32,
    /// Author information
    pub author: UserSummary,
    /// Last updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Package tags
    pub tags: Vec<String>,
}

/// User summary for package attribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummary {
    /// User ID
    pub id: Uuid,
    /// Username
    pub username: String,
    /// Display name
    pub display_name: Option<String>,
    /// Avatar URL
    pub avatar_url: Option<String>,
    /// Verified developer status
    pub verified: bool,
}

/// Activity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityMetrics {
    /// Downloads in last 24 hours
    pub downloads_24h: u64,
    /// Downloads in last 7 days
    pub downloads_7d: u64,
    /// Downloads in last 30 days
    pub downloads_30d: u64,
    /// New packages in last 7 days
    pub new_packages_7d: u64,
    /// Package updates in last 7 days
    pub updates_7d: u64,
}

/// Initialize the marketplace system
pub async fn initialize_marketplace(config: MarketplaceConfig) -> anyhow::Result<Marketplace> {
    tracing::info!("Initializing AION Marketplace v{}", MARKETPLACE_VERSION);

    let marketplace = Marketplace::new(config).await?;

    tracing::info!("Marketplace initialized successfully");
    Ok(marketplace)
}