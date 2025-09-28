use crate::{PackageType, PackageVisibility, PackageLicense};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Main package model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    /// Unique package identifier
    pub id: Uuid,
    /// Package name (unique)
    pub name: String,
    /// Package description
    pub description: String,
    /// Type of package
    pub package_type: PackageType,
    /// Visibility level
    pub visibility: PackageVisibility,
    /// License information
    pub license: PackageLicense,
    /// Publisher user ID
    pub publisher_id: Uuid,
    /// Current/latest version
    pub current_version: semver::Version,
    /// Total download count
    pub total_downloads: u64,
    /// Average rating (0.0 - 5.0)
    pub rating: f32,
    /// Number of reviews
    pub review_count: u32,
    /// Package tags for discovery
    pub tags: Vec<String>,
    /// Package categories
    pub categories: Vec<String>,
    /// README content (markdown)
    pub readme: Option<String>,
    /// Documentation URL
    pub documentation_url: Option<String>,
    /// Source repository URL
    pub repository_url: Option<String>,
    /// Project homepage URL
    pub homepage_url: Option<String>,
    /// Package creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Additional metadata
    pub metadata: serde_json::Value,
    /// Pricing information for commercial packages
    pub pricing: Option<PackagePricing>,
}

/// Package version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersion {
    /// Version ID
    pub id: Uuid,
    /// Parent package ID
    pub package_id: Uuid,
    /// Semantic version
    pub version: semver::Version,
    /// File storage URL
    pub file_url: String,
    /// File size in bytes
    pub file_size: i64,
    /// File hash for integrity
    pub file_hash: String,
    /// Version changelog
    pub changelog: Option<String>,
    /// Runtime dependencies
    pub dependencies: HashMap<String, String>,
    /// Development dependencies
    pub dev_dependencies: HashMap<String, String>,
    /// Peer dependencies
    pub peer_dependencies: HashMap<String, String>,
    /// Compatibility information
    pub compatibility: Option<CompatibilityInfo>,
    /// Publication timestamp
    pub published_at: chrono::DateTime<chrono::Utc>,
    /// Whether version is yanked
    pub yanked: bool,
    /// Reason for yanking
    pub yank_reason: Option<String>,
}

/// Package compatibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityInfo {
    /// Minimum AION version
    pub min_aion_version: Option<semver::Version>,
    /// Maximum AION version
    pub max_aion_version: Option<semver::Version>,
    /// Supported operating systems
    pub supported_os: Vec<String>,
    /// Supported architectures
    pub supported_arch: Vec<String>,
    /// Required features
    pub required_features: Vec<String>,
}

/// User model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: Uuid,
    /// Unique username
    pub username: String,
    /// User email
    pub email: String,
    /// Display name
    pub display_name: Option<String>,
    /// Avatar image URL
    pub avatar_url: Option<String>,
    /// User bio/description
    pub bio: Option<String>,
    /// Website URL
    pub website_url: Option<String>,
    /// Company/organization
    pub company: Option<String>,
    /// Location
    pub location: Option<String>,
    /// Verified developer status
    pub verified: bool,
    /// User role
    pub role: UserRole,
    /// Account creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last login timestamp
    pub last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    /// User settings
    pub settings: UserSettings,
    /// Profile statistics
    pub stats: UserStats,
}

/// User roles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserRole {
    /// Regular user
    User,
    /// Package publisher
    Publisher,
    /// Marketplace moderator
    Moderator,
    /// Administrator
    Admin,
}

/// User settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    /// Email notifications enabled
    pub email_notifications: bool,
    /// Marketing emails enabled
    pub marketing_emails: bool,
    /// Profile visibility
    pub profile_public: bool,
    /// Preferred language
    pub language: String,
    /// Timezone
    pub timezone: String,
}

/// User statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    /// Number of published packages
    pub packages_published: u32,
    /// Total downloads across all packages
    pub total_downloads: u64,
    /// Number of reviews written
    pub reviews_written: u32,
    /// Number of packages installed
    pub packages_installed: u32,
    /// Follower count
    pub followers: u32,
    /// Following count
    pub following: u32,
}

/// Package review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageReview {
    /// Review ID
    pub id: Uuid,
    /// Package being reviewed
    pub package_id: Uuid,
    /// Reviewer user ID
    pub reviewer_id: Uuid,
    /// Rating (1-5)
    pub rating: u8,
    /// Review title
    pub title: Option<String>,
    /// Review content
    pub content: String,
    /// Helpful votes received
    pub helpful_votes: u32,
    /// Total votes received
    pub total_votes: u32,
    /// Review creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Review update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Whether reviewer purchased package
    pub verified_purchase: bool,
}

/// Package download record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDownload {
    /// Download ID
    pub id: Uuid,
    /// Package downloaded
    pub package_id: Uuid,
    /// Version downloaded
    pub version: semver::Version,
    /// User who downloaded (optional for anonymous)
    pub downloader_id: Option<Uuid>,
    /// Download URL (with expiration)
    pub download_url: String,
    /// Download timestamp
    pub downloaded_at: chrono::DateTime<chrono::Utc>,
    /// File size downloaded
    pub file_size: i64,
    /// User agent string
    pub user_agent: Option<String>,
    /// IP address
    pub ip_address: Option<String>,
}

/// Package installation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInstallation {
    /// Installation ID
    pub id: Uuid,
    /// Package installed
    pub package_id: Uuid,
    /// Version installed
    pub version: semver::Version,
    /// User who installed
    pub installer_id: Uuid,
    /// Installation path
    pub installation_path: std::path::PathBuf,
    /// Installation timestamp
    pub installed_at: chrono::DateTime<chrono::Utc>,
    /// Installation status
    pub status: InstallationStatus,
    /// Additional metadata
    pub metadata: serde_json::Value,
}

/// Installation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstallationStatus {
    /// Installation in progress
    Installing,
    /// Installation completed
    Completed,
    /// Installation failed
    Failed,
    /// Installation cancelled
    Cancelled,
    /// Package uninstalled
    Uninstalled,
}

/// Package pricing for commercial packages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagePricing {
    /// Base price in cents (USD)
    pub price_cents: u64,
    /// Currency code
    pub currency: String,
    /// Pricing model
    pub model: PricingModel,
    /// Free trial period in days
    pub trial_days: Option<u32>,
    /// Discount information
    pub discounts: Vec<PricingDiscount>,
}

/// Pricing models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PricingModel {
    /// One-time purchase
    OneTime,
    /// Monthly subscription
    Monthly,
    /// Annual subscription
    Annual,
    /// Usage-based pricing
    UsageBased { per_unit_cents: u64 },
    /// Tiered pricing
    Tiered { tiers: Vec<PricingTier> },
}

/// Pricing tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingTier {
    /// Tier name
    pub name: String,
    /// Price in cents
    pub price_cents: u64,
    /// Features included
    pub features: Vec<String>,
    /// Usage limits
    pub limits: HashMap<String, u64>,
}

/// Pricing discount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingDiscount {
    /// Discount code
    pub code: String,
    /// Discount type
    pub discount_type: DiscountType,
    /// Discount value
    pub value: u64,
    /// Valid from
    pub valid_from: chrono::DateTime<chrono::Utc>,
    /// Valid until
    pub valid_until: chrono::DateTime<chrono::Utc>,
    /// Usage limit
    pub usage_limit: Option<u32>,
    /// Current usage count
    pub usage_count: u32,
}

/// Discount types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscountType {
    /// Percentage discount
    Percentage,
    /// Fixed amount discount
    FixedAmount,
    /// Free trial extension
    TrialExtension,
}

/// Search filters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchFilters {
    /// Filter by package type
    pub package_type: Option<PackageType>,
    /// Filter by tags
    pub tags: Vec<String>,
    /// Filter by categories
    pub categories: Vec<String>,
    /// Filter by license type
    pub license_type: Option<String>,
    /// Minimum rating filter
    pub min_rating: Option<f32>,
    /// Filter by verified publishers only
    pub verified_only: bool,
    /// Filter by free packages only
    pub free_only: bool,
    /// Date range filters
    pub created_after: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_after: Option<chrono::DateTime<chrono::Utc>>,
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-based)
    pub page: u32,
    /// Items per page
    pub per_page: u32,
    /// Sort field
    pub sort_by: SortField,
    /// Sort order
    pub sort_order: SortOrder,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
            sort_by: SortField::Relevance,
            sort_order: SortOrder::Descending,
        }
    }
}

/// Sort fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortField {
    /// Search relevance (default)
    Relevance,
    /// Download count
    Downloads,
    /// Average rating
    Rating,
    /// Creation date
    CreatedAt,
    /// Last update date
    UpdatedAt,
    /// Package name
    Name,
}

/// Sort orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    /// Ascending order
    Ascending,
    /// Descending order
    Descending,
}

/// Search results container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults<T> {
    /// Result items
    pub items: Vec<T>,
    /// Total count (for pagination)
    pub total_count: u64,
    /// Current page
    pub page: u32,
    /// Items per page
    pub per_page: u32,
    /// Total pages
    pub total_pages: u32,
}

/// Package creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePackageRequest {
    /// Package name
    pub name: String,
    /// Package description
    pub description: String,
    /// Package type
    pub package_type: PackageType,
    /// Initial version
    pub version: String,
    /// Package visibility
    pub visibility: Option<PackageVisibility>,
    /// License
    pub license: PackageLicense,
    /// Package tags
    pub tags: Vec<String>,
    /// Package categories
    pub categories: Vec<String>,
    /// README content
    pub readme: Option<String>,
    /// Documentation URL
    pub documentation_url: Option<String>,
    /// Repository URL
    pub repository_url: Option<String>,
    /// Homepage URL
    pub homepage_url: Option<String>,
    /// Version changelog
    pub changelog: Option<String>,
    /// Dependencies
    pub dependencies: Option<HashMap<String, String>>,
    /// Development dependencies
    pub dev_dependencies: Option<HashMap<String, String>>,
    /// Peer dependencies
    pub peer_dependencies: Option<HashMap<String, String>>,
    /// Compatibility info
    pub compatibility: Option<CompatibilityInfo>,
    /// Additional metadata
    pub metadata: Option<serde_json::Value>,
    /// Pricing (for commercial packages)
    pub pricing: Option<PackagePricing>,
}

/// Review creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReviewRequest {
    /// Rating (1-5)
    pub rating: u8,
    /// Review title
    pub title: Option<String>,
    /// Review content
    pub content: String,
}

/// Payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    /// Credit/debit card
    Card {
        /// Card token from payment processor
        token: String,
    },
    /// PayPal payment
    PayPal {
        /// PayPal payment ID
        payment_id: String,
    },
    /// Cryptocurrency payment
    Crypto {
        /// Cryptocurrency type
        currency: String,
        /// Wallet address
        address: String,
    },
    /// Bank transfer
    BankTransfer {
        /// Bank account details
        account_details: serde_json::Value,
    },
}

/// Payment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResult {
    /// Payment ID
    pub payment_id: String,
    /// Whether payment was successful
    pub success: bool,
    /// Amount charged in cents
    pub amount_cents: u64,
    /// Currency used
    pub currency: String,
    /// Payment timestamp
    pub processed_at: chrono::DateTime<chrono::Utc>,
    /// Transaction fee in cents
    pub fee_cents: u64,
    /// Payment method used
    pub payment_method: PaymentMethod,
    /// Failure reason if not successful
    pub failure_reason: Option<String>,
}

/// Installation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationResult {
    /// Installation record
    pub installation: PackageInstallation,
    /// Path where package was extracted
    pub extracted_path: std::path::PathBuf,
    /// Package file data
    pub package_files: Vec<u8>,
}

/// Search parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    /// Search query
    pub query: String,
    /// Search filters
    pub filters: SearchFilters,
    /// Pagination parameters
    pub pagination: PaginationParams,
}