use crate::{
    models::*,
    database::*,
    storage::*,
    security::*,
    analytics::*,
    payments::*,
    notifications::*,
    search::*,
    validation::*,
    errors::*,
    config::*,
    PackageType,
    PackageVisibility,
    MarketplaceStats,
    PackageSummary,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Core marketplace engine managing all marketplace operations
pub struct Marketplace {
    /// Database connection pool
    database: Arc<DatabaseManager>,
    /// File storage backend
    storage: Arc<StorageManager>,
    /// Security and authentication
    security: Arc<SecurityManager>,
    /// Analytics and metrics collection
    analytics: Arc<AnalyticsEngine>,
    /// Payment processing
    payments: Arc<PaymentProcessor>,
    /// Notification system
    notifications: Arc<NotificationManager>,
    /// Search engine
    search: Arc<SearchEngine>,
    /// Package validation
    validator: Arc<PackageValidator>,
    /// Configuration
    config: Arc<MarketplaceConfig>,
    /// In-memory caches
    package_cache: Arc<RwLock<dashmap::DashMap<Uuid, Package>>>,
    user_cache: Arc<RwLock<dashmap::DashMap<Uuid, User>>>,
}

impl Marketplace {
    /// Create a new marketplace instance
    pub async fn new(config: MarketplaceConfig) -> Result<Self> {
        let config = Arc::new(config);

        // Initialize core components
        let database = Arc::new(DatabaseManager::new(config.database.clone()).await?);
        let storage = Arc::new(StorageManager::new(config.storage.clone()).await?);
        let security = Arc::new(SecurityManager::new(config.security.clone()).await?);
        let analytics = Arc::new(AnalyticsEngine::new(config.analytics.clone()).await?);
        let payments = Arc::new(PaymentProcessor::new(config.payments.clone()).await?);
        let notifications = Arc::new(NotificationManager::new(config.notifications.clone()).await?);
        let search = Arc::new(SearchEngine::new(config.search.clone()).await?);
        let validator = Arc::new(PackageValidator::new(config.validation.clone()).await?);

        let marketplace = Self {
            database,
            storage,
            security,
            analytics,
            payments,
            notifications,
            search,
            validator,
            config,
            package_cache: Arc::new(RwLock::new(dashmap::DashMap::new())),
            user_cache: Arc::new(RwLock::new(dashmap::DashMap::new())),
        };

        // Initialize database schema
        marketplace.database.initialize_schema().await?;

        // Populate search index
        marketplace.rebuild_search_index().await?;

        Ok(marketplace)
    }

    /// Publish a new package to the marketplace
    pub async fn publish_package(
        &self,
        publisher_id: Uuid,
        package_data: CreatePackageRequest,
        package_files: Vec<u8>,
    ) -> Result<Package> {
        tracing::info!("Publishing package: {} by {}", package_data.name, publisher_id);

        // Validate publisher permissions
        let publisher = self.get_user(publisher_id).await?;
        self.security.validate_publish_permission(&publisher, &package_data).await?;

        // Validate package content
        let validation_result = self.validator.validate_package(&package_files, &package_data).await?;
        if !validation_result.is_valid {
            return Err(MarketplaceError::ValidationFailed(validation_result.errors));
        }

        // Security scan
        let security_scan = self.security.scan_package_content(&package_files).await?;
        if security_scan.has_threats {
            return Err(MarketplaceError::SecurityThreatDetected(security_scan.threats));
        }

        // Create package record
        let package_id = Uuid::new_v4();
        let package_version = semver::Version::parse(&package_data.version)
            .map_err(|e| MarketplaceError::InvalidVersion(e.to_string()))?;

        let package = Package {
            id: package_id,
            name: package_data.name.clone(),
            description: package_data.description.clone(),
            package_type: package_data.package_type,
            visibility: package_data.visibility.unwrap_or(PackageVisibility::Public),
            license: package_data.license,
            publisher_id,
            current_version: package_version.clone(),
            total_downloads: 0,
            rating: 0.0,
            review_count: 0,
            tags: package_data.tags.clone(),
            categories: package_data.categories.clone(),
            readme: package_data.readme.clone(),
            documentation_url: package_data.documentation_url.clone(),
            repository_url: package_data.repository_url.clone(),
            homepage_url: package_data.homepage_url.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            metadata: package_data.metadata.unwrap_or_default(),
        };

        // Store package files
        let storage_path = format!("packages/{}/{}/{}.tar.gz",
            package.name, package_version, package.name);
        let file_url = self.storage.store_package_files(&storage_path, &package_files).await?;

        // Create package version record
        let version_record = PackageVersion {
            id: Uuid::new_v4(),
            package_id,
            version: package_version,
            file_url,
            file_size: package_files.len() as i64,
            file_hash: self.calculate_file_hash(&package_files),
            changelog: package_data.changelog,
            dependencies: package_data.dependencies.unwrap_or_default(),
            dev_dependencies: package_data.dev_dependencies.unwrap_or_default(),
            peer_dependencies: package_data.peer_dependencies.unwrap_or_default(),
            compatibility: package_data.compatibility,
            published_at: chrono::Utc::now(),
            yanked: false,
            yank_reason: None,
        };

        // Store in database
        let mut tx = self.database.begin_transaction().await?;

        self.database.create_package(&mut tx, &package).await?;
        self.database.create_package_version(&mut tx, &version_record).await?;

        // Update publisher statistics
        self.database.increment_publisher_package_count(&mut tx, publisher_id).await?;

        tx.commit().await?;

        // Update caches
        self.package_cache.write().await.insert(package_id, package.clone());

        // Index in search engine
        self.search.index_package(&package).await?;

        // Send notifications
        self.notifications.notify_package_published(&package, &publisher).await?;

        // Record analytics
        self.analytics.record_package_published(&package).await?;

        tracing::info!("Package published successfully: {} ({})", package.name, package_id);
        Ok(package)
    }

    /// Search for packages
    pub async fn search_packages(
        &self,
        query: &str,
        filters: Option<SearchFilters>,
        pagination: Option<PaginationParams>,
    ) -> Result<SearchResults<PackageSummary>> {
        tracing::debug!("Searching packages: query='{}' filters={:?}", query, filters);

        let search_params = SearchParams {
            query: query.to_string(),
            filters: filters.unwrap_or_default(),
            pagination: pagination.unwrap_or_default(),
        };

        let results = self.search.search_packages(&search_params).await?;

        // Convert to summaries
        let mut summaries = Vec::new();
        for package_id in &results.items {
            if let Ok(package) = self.get_package(*package_id).await {
                if let Ok(author) = self.get_user(package.publisher_id).await {
                    summaries.push(PackageSummary {
                        id: package.id,
                        name: package.name,
                        description: package.description,
                        package_type: package.package_type,
                        version: package.current_version,
                        downloads: package.total_downloads,
                        rating: package.rating,
                        review_count: package.review_count,
                        author: UserSummary {
                            id: author.id,
                            username: author.username,
                            display_name: author.display_name,
                            avatar_url: author.avatar_url,
                            verified: author.verified,
                        },
                        updated_at: package.updated_at,
                        tags: package.tags,
                    });
                }
            }
        }

        Ok(SearchResults {
            items: summaries,
            total_count: results.total_count,
            page: results.page,
            per_page: results.per_page,
            total_pages: results.total_pages,
        })
    }

    /// Download a package
    pub async fn download_package(
        &self,
        package_id: Uuid,
        version: Option<&str>,
        downloader_id: Option<Uuid>,
    ) -> Result<PackageDownload> {
        tracing::info!("Downloading package: {} version {:?}", package_id, version);

        let package = self.get_package(package_id).await?;

        // Check visibility permissions
        if package.visibility == PackageVisibility::Private {
            if let Some(downloader_id) = downloader_id {
                self.security.validate_package_access(&package, downloader_id).await?;
            } else {
                return Err(MarketplaceError::AccessDenied);
            }
        }

        // Get specific version or latest
        let version_record = if let Some(version_str) = version {
            let version = semver::Version::parse(version_str)
                .map_err(|e| MarketplaceError::InvalidVersion(e.to_string()))?;
            self.database.get_package_version(package_id, &version).await?
        } else {
            self.database.get_latest_package_version(package_id).await?
        };

        // Check if version is yanked
        if version_record.yanked {
            return Err(MarketplaceError::VersionYanked(version_record.yank_reason));
        }

        // Generate download URL with expiration
        let download_url = self.storage.generate_download_url(&version_record.file_url).await?;

        // Record download analytics
        let download_record = PackageDownload {
            id: Uuid::new_v4(),
            package_id,
            version: version_record.version.clone(),
            downloader_id,
            download_url: download_url.clone(),
            downloaded_at: chrono::Utc::now(),
            file_size: version_record.file_size,
            user_agent: None, // Could be filled from request headers
            ip_address: None, // Could be filled from request
        };

        // Update download count (async)
        let database = Arc::clone(&self.database);
        let analytics = Arc::clone(&self.analytics);
        let package_clone = package.clone();
        tokio::spawn(async move {
            if let Err(e) = database.increment_download_count(package_id).await {
                tracing::error!("Failed to increment download count: {}", e);
            }
            if let Err(e) = analytics.record_package_download(&package_clone, &download_record).await {
                tracing::error!("Failed to record download analytics: {}", e);
            }
        });

        Ok(download_record)
    }

    /// Install a package (for plugin system integration)
    pub async fn install_package(
        &self,
        package_name: &str,
        version: Option<&str>,
        installer_id: Uuid,
    ) -> Result<InstallationResult> {
        tracing::info!("Installing package: {} version {:?} for user {}",
            package_name, version, installer_id);

        // Find package by name
        let package = self.database.get_package_by_name(package_name).await?;

        // Download package
        let download = self.download_package(package.id, version, Some(installer_id)).await?;

        // Download actual files
        let package_files = self.storage.download_package_files(&download.download_url).await?;

        // Verify file integrity
        let calculated_hash = self.calculate_file_hash(&package_files);
        let version_record = self.database.get_package_version(package.id, &download.version).await?;

        if calculated_hash != version_record.file_hash {
            return Err(MarketplaceError::FileIntegrityCheckFailed);
        }

        // Extract and prepare for installation
        let extracted_path = self.extract_package_files(&package_files).await?;

        // Create installation record
        let installation = PackageInstallation {
            id: Uuid::new_v4(),
            package_id: package.id,
            version: download.version,
            installer_id,
            installation_path: extracted_path.clone(),
            installed_at: chrono::Utc::now(),
            status: InstallationStatus::Completed,
            metadata: serde_json::Value::Null,
        };

        // Record installation
        self.database.create_installation_record(&installation).await?;

        // Update user's installed packages
        self.database.add_user_installation(installer_id, &installation).await?;

        // Record analytics
        self.analytics.record_package_installation(&package, &installation).await?;

        Ok(InstallationResult {
            installation,
            extracted_path,
            package_files,
        })
    }

    /// Get package details
    pub async fn get_package(&self, package_id: Uuid) -> Result<Package> {
        // Check cache first
        if let Some(package) = self.package_cache.read().await.get(&package_id) {
            return Ok(package.clone());
        }

        // Load from database
        let package = self.database.get_package(package_id).await?;

        // Update cache
        self.package_cache.write().await.insert(package_id, package.clone());

        Ok(package)
    }

    /// Get user details
    pub async fn get_user(&self, user_id: Uuid) -> Result<User> {
        // Check cache first
        if let Some(user) = self.user_cache.read().await.get(&user_id) {
            return Ok(user.clone());
        }

        // Load from database
        let user = self.database.get_user(user_id).await?;

        // Update cache
        self.user_cache.write().await.insert(user_id, user.clone());

        Ok(user)
    }

    /// Get marketplace statistics
    pub async fn get_stats(&self) -> Result<MarketplaceStats> {
        let stats = self.analytics.get_marketplace_stats().await?;
        Ok(stats)
    }

    /// Submit a package review
    pub async fn submit_review(
        &self,
        package_id: Uuid,
        reviewer_id: Uuid,
        review_data: CreateReviewRequest,
    ) -> Result<PackageReview> {
        tracing::info!("Submitting review for package {} by user {}", package_id, reviewer_id);

        // Validate package exists
        let package = self.get_package(package_id).await?;

        // Check if user has already reviewed this package
        if self.database.has_user_reviewed_package(reviewer_id, package_id).await? {
            return Err(MarketplaceError::AlreadyReviewed);
        }

        // Validate review content
        self.validator.validate_review_content(&review_data).await?;

        // Create review
        let review = PackageReview {
            id: Uuid::new_v4(),
            package_id,
            reviewer_id,
            rating: review_data.rating,
            title: review_data.title,
            content: review_data.content,
            helpful_votes: 0,
            total_votes: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            verified_purchase: self.database.has_user_downloaded_package(reviewer_id, package_id).await?,
        };

        // Store review
        self.database.create_review(&review).await?;

        // Update package rating
        self.update_package_rating(package_id).await?;

        // Send notification to package owner
        let reviewer = self.get_user(reviewer_id).await?;
        let package_owner = self.get_user(package.publisher_id).await?;
        self.notifications.notify_review_submitted(&package, &review, &reviewer, &package_owner).await?;

        // Record analytics
        self.analytics.record_review_submitted(&package, &review).await?;

        Ok(review)
    }

    /// Process payment for premium package
    pub async fn process_payment(
        &self,
        package_id: Uuid,
        buyer_id: Uuid,
        payment_method: PaymentMethod,
    ) -> Result<PaymentResult> {
        let package = self.get_package(package_id).await?;

        // Check if package requires payment
        if let Some(pricing) = &package.pricing {
            let payment_result = self.payments.process_payment(
                &package,
                pricing,
                buyer_id,
                payment_method,
            ).await?;

            // Grant access on successful payment
            if payment_result.success {
                self.database.grant_package_access(buyer_id, package_id).await?;
            }

            Ok(payment_result)
        } else {
            Err(MarketplaceError::PackageNotPaid)
        }
    }

    /// Rebuild search index
    async fn rebuild_search_index(&self) -> Result<()> {
        tracing::info!("Rebuilding search index");

        let packages = self.database.get_all_packages().await?;

        for package in packages {
            self.search.index_package(&package).await?;
        }

        tracing::info!("Search index rebuilt successfully");
        Ok(())
    }

    /// Update package rating based on reviews
    async fn update_package_rating(&self, package_id: Uuid) -> Result<()> {
        let reviews = self.database.get_package_reviews(package_id).await?;

        if reviews.is_empty() {
            return Ok(());
        }

        let total_rating: f32 = reviews.iter().map(|r| r.rating as f32).sum();
        let average_rating = total_rating / reviews.len() as f32;
        let review_count = reviews.len() as u32;

        self.database.update_package_rating(package_id, average_rating, review_count).await?;

        // Update cache
        if let Some(mut package) = self.package_cache.write().await.get_mut(&package_id) {
            package.rating = average_rating;
            package.review_count = review_count;
        }

        Ok(())
    }

    /// Calculate file hash for integrity verification
    fn calculate_file_hash(&self, data: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Extract package files to temporary location
    async fn extract_package_files(&self, package_data: &[u8]) -> Result<std::path::PathBuf> {
        let temp_dir = tempfile::tempdir()?;
        let extract_path = temp_dir.path().to_path_buf();

        // Extract tar.gz
        let cursor = std::io::Cursor::new(package_data);
        let gz_decoder = flate2::read::GzDecoder::new(cursor);
        let mut archive = tar::Archive::new(gz_decoder);
        archive.unpack(&extract_path)?;

        Ok(extract_path)
    }
}