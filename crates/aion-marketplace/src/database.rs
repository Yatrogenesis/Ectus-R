use crate::{models::*, errors::*, config::DatabaseConfig};
use sqlx::{PgPool, Postgres, Transaction, Row};
use uuid::Uuid;
use std::collections::HashMap;

/// Database manager for marketplace operations
pub struct DatabaseManager {
    pool: PgPool,
}

impl DatabaseManager {
    /// Create a new database manager
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        let pool = PgPool::connect(&config.url).await
            .map_err(|e| MarketplaceError::DatabaseConnection(e.to_string()))?;

        Ok(Self { pool })
    }

    /// Initialize database schema
    pub async fn initialize_schema(&self) -> Result<()> {
        tracing::info!("Initializing database schema");

        // Create extensions
        sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"")
            .execute(&self.pool).await?;
        sqlx::query("CREATE EXTENSION IF NOT EXISTS \"pg_trgm\"")
            .execute(&self.pool).await?;

        // Create tables
        self.create_users_table().await?;
        self.create_packages_table().await?;
        self.create_package_versions_table().await?;
        self.create_package_reviews_table().await?;
        self.create_package_downloads_table().await?;
        self.create_package_installations_table().await?;
        self.create_user_package_access_table().await?;
        self.create_payment_records_table().await?;

        // Create indexes
        self.create_indexes().await?;

        tracing::info!("Database schema initialized successfully");
        Ok(())
    }

    /// Begin a database transaction
    pub async fn begin_transaction(&self) -> Result<Transaction<'_, Postgres>> {
        self.pool.begin().await
            .map_err(|e| MarketplaceError::DatabaseError(e.to_string()))
    }

    /// Create a new package
    pub async fn create_package(&self, tx: &mut Transaction<'_, Postgres>, package: &Package) -> Result<()> {
        let visibility_str = match package.visibility {
            crate::PackageVisibility::Public => "public",
            crate::PackageVisibility::Private => "private",
            crate::PackageVisibility::Unlisted => "unlisted",
        };

        let package_type_str = match package.package_type {
            crate::PackageType::Template => "template",
            crate::PackageType::Plugin => "plugin",
            crate::PackageType::Library => "library",
            crate::PackageType::Tool => "tool",
            crate::PackageType::Deployment => "deployment",
            crate::PackageType::Component => "component",
        };

        sqlx::query!(
            r#"
            INSERT INTO packages (
                id, name, description, package_type, visibility, publisher_id,
                current_version, total_downloads, rating, review_count,
                tags, categories, readme, documentation_url, repository_url,
                homepage_url, created_at, updated_at, metadata
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19
            )
            "#,
            package.id,
            package.name,
            package.description,
            package_type_str,
            visibility_str,
            package.publisher_id,
            package.current_version.to_string(),
            package.total_downloads as i64,
            package.rating,
            package.review_count as i32,
            &package.tags,
            &package.categories,
            package.readme,
            package.documentation_url,
            package.repository_url,
            package.homepage_url,
            package.created_at,
            package.updated_at,
            package.metadata
        ).execute(tx).await?;

        Ok(())
    }

    /// Create a package version
    pub async fn create_package_version(&self, tx: &mut Transaction<'_, Postgres>, version: &PackageVersion) -> Result<()> {
        let dependencies_json = serde_json::to_value(&version.dependencies)?;
        let dev_dependencies_json = serde_json::to_value(&version.dev_dependencies)?;
        let peer_dependencies_json = serde_json::to_value(&version.peer_dependencies)?;
        let compatibility_json = serde_json::to_value(&version.compatibility)?;

        sqlx::query!(
            r#"
            INSERT INTO package_versions (
                id, package_id, version, file_url, file_size, file_hash,
                changelog, dependencies, dev_dependencies, peer_dependencies,
                compatibility, published_at, yanked, yank_reason
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
            )
            "#,
            version.id,
            version.package_id,
            version.version.to_string(),
            version.file_url,
            version.file_size,
            version.file_hash,
            version.changelog,
            dependencies_json,
            dev_dependencies_json,
            peer_dependencies_json,
            compatibility_json,
            version.published_at,
            version.yanked,
            version.yank_reason
        ).execute(tx).await?;

        Ok(())
    }

    /// Get package by ID
    pub async fn get_package(&self, package_id: Uuid) -> Result<Package> {
        let row = sqlx::query!(
            r#"
            SELECT
                id, name, description, package_type, visibility, publisher_id,
                current_version, total_downloads, rating, review_count,
                tags, categories, readme, documentation_url, repository_url,
                homepage_url, created_at, updated_at, metadata
            FROM packages WHERE id = $1
            "#,
            package_id
        ).fetch_one(&self.pool).await
        .map_err(|_| MarketplaceError::PackageNotFound(package_id))?;

        Ok(Package {
            id: row.id,
            name: row.name,
            description: row.description,
            package_type: self.parse_package_type(&row.package_type),
            visibility: self.parse_visibility(&row.visibility),
            license: crate::PackageLicense::OpenSource("MIT".to_string()), // Default for now
            publisher_id: row.publisher_id,
            current_version: semver::Version::parse(&row.current_version).unwrap(),
            total_downloads: row.total_downloads as u64,
            rating: row.rating,
            review_count: row.review_count as u32,
            tags: row.tags,
            categories: row.categories,
            readme: row.readme,
            documentation_url: row.documentation_url,
            repository_url: row.repository_url,
            homepage_url: row.homepage_url,
            created_at: row.created_at,
            updated_at: row.updated_at,
            metadata: row.metadata,
            pricing: None, // TODO: Implement pricing table
        })
    }

    /// Get package by name
    pub async fn get_package_by_name(&self, name: &str) -> Result<Package> {
        let row = sqlx::query!(
            r#"
            SELECT
                id, name, description, package_type, visibility, publisher_id,
                current_version, total_downloads, rating, review_count,
                tags, categories, readme, documentation_url, repository_url,
                homepage_url, created_at, updated_at, metadata
            FROM packages WHERE name = $1
            "#,
            name
        ).fetch_one(&self.pool).await
        .map_err(|_| MarketplaceError::PackageNotFound(Uuid::nil()))?;

        Ok(Package {
            id: row.id,
            name: row.name,
            description: row.description,
            package_type: self.parse_package_type(&row.package_type),
            visibility: self.parse_visibility(&row.visibility),
            license: crate::PackageLicense::OpenSource("MIT".to_string()),
            publisher_id: row.publisher_id,
            current_version: semver::Version::parse(&row.current_version).unwrap(),
            total_downloads: row.total_downloads as u64,
            rating: row.rating,
            review_count: row.review_count as u32,
            tags: row.tags,
            categories: row.categories,
            readme: row.readme,
            documentation_url: row.documentation_url,
            repository_url: row.repository_url,
            homepage_url: row.homepage_url,
            created_at: row.created_at,
            updated_at: row.updated_at,
            metadata: row.metadata,
            pricing: None,
        })
    }

    /// Get all packages (for search index rebuilding)
    pub async fn get_all_packages(&self) -> Result<Vec<Package>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                id, name, description, package_type, visibility, publisher_id,
                current_version, total_downloads, rating, review_count,
                tags, categories, readme, documentation_url, repository_url,
                homepage_url, created_at, updated_at, metadata
            FROM packages
            ORDER BY created_at DESC
            "#
        ).fetch_all(&self.pool).await?;

        let mut packages = Vec::new();
        for row in rows {
            packages.push(Package {
                id: row.id,
                name: row.name,
                description: row.description,
                package_type: self.parse_package_type(&row.package_type),
                visibility: self.parse_visibility(&row.visibility),
                license: crate::PackageLicense::OpenSource("MIT".to_string()),
                publisher_id: row.publisher_id,
                current_version: semver::Version::parse(&row.current_version).unwrap(),
                total_downloads: row.total_downloads as u64,
                rating: row.rating,
                review_count: row.review_count as u32,
                tags: row.tags,
                categories: row.categories,
                readme: row.readme,
                documentation_url: row.documentation_url,
                repository_url: row.repository_url,
                homepage_url: row.homepage_url,
                created_at: row.created_at,
                updated_at: row.updated_at,
                metadata: row.metadata,
                pricing: None,
            });
        }

        Ok(packages)
    }

    /// Get package version
    pub async fn get_package_version(&self, package_id: Uuid, version: &semver::Version) -> Result<PackageVersion> {
        let row = sqlx::query!(
            r#"
            SELECT
                id, package_id, version, file_url, file_size, file_hash,
                changelog, dependencies, dev_dependencies, peer_dependencies,
                compatibility, published_at, yanked, yank_reason
            FROM package_versions
            WHERE package_id = $1 AND version = $2
            "#,
            package_id,
            version.to_string()
        ).fetch_one(&self.pool).await
        .map_err(|_| MarketplaceError::VersionNotFound(version.clone()))?;

        Ok(PackageVersion {
            id: row.id,
            package_id: row.package_id,
            version: semver::Version::parse(&row.version).unwrap(),
            file_url: row.file_url,
            file_size: row.file_size,
            file_hash: row.file_hash,
            changelog: row.changelog,
            dependencies: serde_json::from_value(row.dependencies).unwrap_or_default(),
            dev_dependencies: serde_json::from_value(row.dev_dependencies).unwrap_or_default(),
            peer_dependencies: serde_json::from_value(row.peer_dependencies).unwrap_or_default(),
            compatibility: serde_json::from_value(row.compatibility).unwrap_or_default(),
            published_at: row.published_at,
            yanked: row.yanked,
            yank_reason: row.yank_reason,
        })
    }

    /// Get latest package version
    pub async fn get_latest_package_version(&self, package_id: Uuid) -> Result<PackageVersion> {
        let row = sqlx::query!(
            r#"
            SELECT
                id, package_id, version, file_url, file_size, file_hash,
                changelog, dependencies, dev_dependencies, peer_dependencies,
                compatibility, published_at, yanked, yank_reason
            FROM package_versions
            WHERE package_id = $1 AND NOT yanked
            ORDER BY published_at DESC
            LIMIT 1
            "#,
            package_id
        ).fetch_one(&self.pool).await
        .map_err(|_| MarketplaceError::PackageNotFound(package_id))?;

        Ok(PackageVersion {
            id: row.id,
            package_id: row.package_id,
            version: semver::Version::parse(&row.version).unwrap(),
            file_url: row.file_url,
            file_size: row.file_size,
            file_hash: row.file_hash,
            changelog: row.changelog,
            dependencies: serde_json::from_value(row.dependencies).unwrap_or_default(),
            dev_dependencies: serde_json::from_value(row.dev_dependencies).unwrap_or_default(),
            peer_dependencies: serde_json::from_value(row.peer_dependencies).unwrap_or_default(),
            compatibility: serde_json::from_value(row.compatibility).unwrap_or_default(),
            published_at: row.published_at,
            yanked: row.yanked,
            yank_reason: row.yank_reason,
        })
    }

    /// Get user by ID
    pub async fn get_user(&self, user_id: Uuid) -> Result<User> {
        let row = sqlx::query!(
            r#"
            SELECT
                id, username, email, display_name, avatar_url, bio,
                website_url, company, location, verified, role,
                created_at, last_login_at, settings, stats
            FROM users WHERE id = $1
            "#,
            user_id
        ).fetch_one(&self.pool).await
        .map_err(|_| MarketplaceError::UserNotFound(user_id))?;

        Ok(User {
            id: row.id,
            username: row.username,
            email: row.email,
            display_name: row.display_name,
            avatar_url: row.avatar_url,
            bio: row.bio,
            website_url: row.website_url,
            company: row.company,
            location: row.location,
            verified: row.verified,
            role: self.parse_user_role(&row.role),
            created_at: row.created_at,
            last_login_at: row.last_login_at,
            settings: serde_json::from_value(row.settings).unwrap_or_default(),
            stats: serde_json::from_value(row.stats).unwrap_or_default(),
        })
    }

    /// Create a review
    pub async fn create_review(&self, review: &PackageReview) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO package_reviews (
                id, package_id, reviewer_id, rating, title, content,
                helpful_votes, total_votes, created_at, updated_at, verified_purchase
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            )
            "#,
            review.id,
            review.package_id,
            review.reviewer_id,
            review.rating as i16,
            review.title,
            review.content,
            review.helpful_votes as i32,
            review.total_votes as i32,
            review.created_at,
            review.updated_at,
            review.verified_purchase
        ).execute(&self.pool).await?;

        Ok(())
    }

    /// Get package reviews
    pub async fn get_package_reviews(&self, package_id: Uuid) -> Result<Vec<PackageReview>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                id, package_id, reviewer_id, rating, title, content,
                helpful_votes, total_votes, created_at, updated_at, verified_purchase
            FROM package_reviews
            WHERE package_id = $1
            ORDER BY created_at DESC
            "#,
            package_id
        ).fetch_all(&self.pool).await?;

        let mut reviews = Vec::new();
        for row in rows {
            reviews.push(PackageReview {
                id: row.id,
                package_id: row.package_id,
                reviewer_id: row.reviewer_id,
                rating: row.rating as u8,
                title: row.title,
                content: row.content,
                helpful_votes: row.helpful_votes as u32,
                total_votes: row.total_votes as u32,
                created_at: row.created_at,
                updated_at: row.updated_at,
                verified_purchase: row.verified_purchase,
            });
        }

        Ok(reviews)
    }

    /// Check if user has reviewed package
    pub async fn has_user_reviewed_package(&self, user_id: Uuid, package_id: Uuid) -> Result<bool> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM package_reviews WHERE reviewer_id = $1 AND package_id = $2",
            user_id,
            package_id
        ).fetch_one(&self.pool).await?;

        Ok(count.count.unwrap_or(0) > 0)
    }

    /// Check if user has downloaded package
    pub async fn has_user_downloaded_package(&self, user_id: Uuid, package_id: Uuid) -> Result<bool> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM package_downloads WHERE downloader_id = $1 AND package_id = $2",
            user_id,
            package_id
        ).fetch_one(&self.pool).await?;

        Ok(count.count.unwrap_or(0) > 0)
    }

    /// Update package rating
    pub async fn update_package_rating(&self, package_id: Uuid, rating: f32, review_count: u32) -> Result<()> {
        sqlx::query!(
            "UPDATE packages SET rating = $1, review_count = $2, updated_at = NOW() WHERE id = $3",
            rating,
            review_count as i32,
            package_id
        ).execute(&self.pool).await?;

        Ok(())
    }

    /// Increment download count
    pub async fn increment_download_count(&self, package_id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE packages SET total_downloads = total_downloads + 1, updated_at = NOW() WHERE id = $1",
            package_id
        ).execute(&self.pool).await?;

        Ok(())
    }

    /// Increment publisher package count
    pub async fn increment_publisher_package_count(&self, tx: &mut Transaction<'_, Postgres>, publisher_id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET stats = jsonb_set(
                stats,
                '{packages_published}',
                (COALESCE((stats->>'packages_published')::int, 0) + 1)::text::jsonb
            )
            WHERE id = $1
            "#,
            publisher_id
        ).execute(tx).await?;

        Ok(())
    }

    /// Create installation record
    pub async fn create_installation_record(&self, installation: &PackageInstallation) -> Result<()> {
        let status_str = match installation.status {
            InstallationStatus::Installing => "installing",
            InstallationStatus::Completed => "completed",
            InstallationStatus::Failed => "failed",
            InstallationStatus::Cancelled => "cancelled",
            InstallationStatus::Uninstalled => "uninstalled",
        };

        sqlx::query!(
            r#"
            INSERT INTO package_installations (
                id, package_id, version, installer_id, installation_path,
                installed_at, status, metadata
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8
            )
            "#,
            installation.id,
            installation.package_id,
            installation.version.to_string(),
            installation.installer_id,
            installation.installation_path.to_string_lossy().to_string(),
            installation.installed_at,
            status_str,
            installation.metadata
        ).execute(&self.pool).await?;

        Ok(())
    }

    /// Add user installation
    pub async fn add_user_installation(&self, user_id: Uuid, installation: &PackageInstallation) -> Result<()> {
        // Update user stats
        sqlx::query!(
            r#"
            UPDATE users
            SET stats = jsonb_set(
                stats,
                '{packages_installed}',
                (COALESCE((stats->>'packages_installed')::int, 0) + 1)::text::jsonb
            )
            WHERE id = $1
            "#,
            user_id
        ).execute(&self.pool).await?;

        Ok(())
    }

    /// Grant package access (for paid packages)
    pub async fn grant_package_access(&self, user_id: Uuid, package_id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_package_access (id, user_id, package_id, granted_at, expires_at)
            VALUES ($1, $2, $3, NOW(), NULL)
            ON CONFLICT (user_id, package_id) DO NOTHING
            "#,
            Uuid::new_v4(),
            user_id,
            package_id
        ).execute(&self.pool).await?;

        Ok(())
    }

    // Private helper methods

    fn parse_package_type(&self, type_str: &str) -> crate::PackageType {
        match type_str {
            "template" => crate::PackageType::Template,
            "plugin" => crate::PackageType::Plugin,
            "library" => crate::PackageType::Library,
            "tool" => crate::PackageType::Tool,
            "deployment" => crate::PackageType::Deployment,
            "component" => crate::PackageType::Component,
            _ => crate::PackageType::Template,
        }
    }

    fn parse_visibility(&self, visibility_str: &str) -> crate::PackageVisibility {
        match visibility_str {
            "public" => crate::PackageVisibility::Public,
            "private" => crate::PackageVisibility::Private,
            "unlisted" => crate::PackageVisibility::Unlisted,
            _ => crate::PackageVisibility::Public,
        }
    }

    fn parse_user_role(&self, role_str: &str) -> UserRole {
        match role_str {
            "user" => UserRole::User,
            "publisher" => UserRole::Publisher,
            "moderator" => UserRole::Moderator,
            "admin" => UserRole::Admin,
            _ => UserRole::User,
        }
    }

    // Table creation methods

    async fn create_users_table(&self) -> Result<()> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                username VARCHAR(50) UNIQUE NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                display_name VARCHAR(100),
                avatar_url TEXT,
                bio TEXT,
                website_url TEXT,
                company VARCHAR(100),
                location VARCHAR(100),
                verified BOOLEAN DEFAULT FALSE,
                role VARCHAR(20) DEFAULT 'user',
                created_at TIMESTAMPTZ DEFAULT NOW(),
                last_login_at TIMESTAMPTZ,
                settings JSONB DEFAULT '{}',
                stats JSONB DEFAULT '{
                    "packages_published": 0,
                    "total_downloads": 0,
                    "reviews_written": 0,
                    "packages_installed": 0,
                    "followers": 0,
                    "following": 0
                }'
            )
            "#
        ).execute(&self.pool).await?;

        Ok(())
    }

    async fn create_packages_table(&self) -> Result<()> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS packages (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                name VARCHAR(100) UNIQUE NOT NULL,
                description TEXT NOT NULL,
                package_type VARCHAR(20) NOT NULL,
                visibility VARCHAR(20) DEFAULT 'public',
                publisher_id UUID NOT NULL REFERENCES users(id),
                current_version VARCHAR(50) NOT NULL,
                total_downloads BIGINT DEFAULT 0,
                rating REAL DEFAULT 0.0,
                review_count INTEGER DEFAULT 0,
                tags TEXT[] DEFAULT '{}',
                categories TEXT[] DEFAULT '{}',
                readme TEXT,
                documentation_url TEXT,
                repository_url TEXT,
                homepage_url TEXT,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW(),
                metadata JSONB DEFAULT '{}'
            )
            "#
        ).execute(&self.pool).await?;

        Ok(())
    }

    async fn create_package_versions_table(&self) -> Result<()> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS package_versions (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                package_id UUID NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
                version VARCHAR(50) NOT NULL,
                file_url TEXT NOT NULL,
                file_size BIGINT NOT NULL,
                file_hash VARCHAR(64) NOT NULL,
                changelog TEXT,
                dependencies JSONB DEFAULT '{}',
                dev_dependencies JSONB DEFAULT '{}',
                peer_dependencies JSONB DEFAULT '{}',
                compatibility JSONB,
                published_at TIMESTAMPTZ DEFAULT NOW(),
                yanked BOOLEAN DEFAULT FALSE,
                yank_reason TEXT,
                UNIQUE(package_id, version)
            )
            "#
        ).execute(&self.pool).await?;

        Ok(())
    }

    async fn create_package_reviews_table(&self) -> Result<()> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS package_reviews (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                package_id UUID NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
                reviewer_id UUID NOT NULL REFERENCES users(id),
                rating SMALLINT NOT NULL CHECK (rating >= 1 AND rating <= 5),
                title VARCHAR(200),
                content TEXT NOT NULL,
                helpful_votes INTEGER DEFAULT 0,
                total_votes INTEGER DEFAULT 0,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW(),
                verified_purchase BOOLEAN DEFAULT FALSE,
                UNIQUE(package_id, reviewer_id)
            )
            "#
        ).execute(&self.pool).await?;

        Ok(())
    }

    async fn create_package_downloads_table(&self) -> Result<()> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS package_downloads (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                package_id UUID NOT NULL REFERENCES packages(id),
                version VARCHAR(50) NOT NULL,
                downloader_id UUID REFERENCES users(id),
                downloaded_at TIMESTAMPTZ DEFAULT NOW(),
                file_size BIGINT,
                user_agent TEXT,
                ip_address INET
            )
            "#
        ).execute(&self.pool).await?;

        Ok(())
    }

    async fn create_package_installations_table(&self) -> Result<()> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS package_installations (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                package_id UUID NOT NULL REFERENCES packages(id),
                version VARCHAR(50) NOT NULL,
                installer_id UUID NOT NULL REFERENCES users(id),
                installation_path TEXT NOT NULL,
                installed_at TIMESTAMPTZ DEFAULT NOW(),
                status VARCHAR(20) DEFAULT 'completed',
                metadata JSONB DEFAULT '{}'
            )
            "#
        ).execute(&self.pool).await?;

        Ok(())
    }

    async fn create_user_package_access_table(&self) -> Result<()> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS user_package_access (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                user_id UUID NOT NULL REFERENCES users(id),
                package_id UUID NOT NULL REFERENCES packages(id),
                granted_at TIMESTAMPTZ DEFAULT NOW(),
                expires_at TIMESTAMPTZ,
                UNIQUE(user_id, package_id)
            )
            "#
        ).execute(&self.pool).await?;

        Ok(())
    }

    async fn create_payment_records_table(&self) -> Result<()> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS payment_records (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                payment_id VARCHAR(100) UNIQUE NOT NULL,
                package_id UUID NOT NULL REFERENCES packages(id),
                buyer_id UUID NOT NULL REFERENCES users(id),
                amount_cents BIGINT NOT NULL,
                currency VARCHAR(3) DEFAULT 'USD',
                status VARCHAR(20) NOT NULL,
                payment_method JSONB NOT NULL,
                processed_at TIMESTAMPTZ DEFAULT NOW(),
                fee_cents BIGINT DEFAULT 0,
                failure_reason TEXT
            )
            "#
        ).execute(&self.pool).await?;

        Ok(())
    }

    async fn create_indexes(&self) -> Result<()> {
        // Package search indexes
        sqlx::query!(
            "CREATE INDEX IF NOT EXISTS idx_packages_name_gin ON packages USING gin(name gin_trgm_ops)"
        ).execute(&self.pool).await?;

        sqlx::query!(
            "CREATE INDEX IF NOT EXISTS idx_packages_description_gin ON packages USING gin(description gin_trgm_ops)"
        ).execute(&self.pool).await?;

        sqlx::query!(
            "CREATE INDEX IF NOT EXISTS idx_packages_tags_gin ON packages USING gin(tags)"
        ).execute(&self.pool).await?;

        sqlx::query!(
            "CREATE INDEX IF NOT EXISTS idx_packages_categories_gin ON packages USING gin(categories)"
        ).execute(&self.pool).await?;

        // Performance indexes
        sqlx::query!(
            "CREATE INDEX IF NOT EXISTS idx_packages_type_visibility ON packages(package_type, visibility)"
        ).execute(&self.pool).await?;

        sqlx::query!(
            "CREATE INDEX IF NOT EXISTS idx_packages_publisher ON packages(publisher_id)"
        ).execute(&self.pool).await?;

        sqlx::query!(
            "CREATE INDEX IF NOT EXISTS idx_packages_downloads ON packages(total_downloads DESC)"
        ).execute(&self.pool).await?;

        sqlx::query!(
            "CREATE INDEX IF NOT EXISTS idx_packages_rating ON packages(rating DESC)"
        ).execute(&self.pool).await?;

        sqlx::query!(
            "CREATE INDEX IF NOT EXISTS idx_packages_created_at ON packages(created_at DESC)"
        ).execute(&self.pool).await?;

        sqlx::query!(
            "CREATE INDEX IF NOT EXISTS idx_package_versions_package ON package_versions(package_id)"
        ).execute(&self.pool).await?;

        sqlx::query!(
            "CREATE INDEX IF NOT EXISTS idx_package_reviews_package ON package_reviews(package_id)"
        ).execute(&self.pool).await?;

        sqlx::query!(
            "CREATE INDEX IF NOT EXISTS idx_package_downloads_package ON package_downloads(package_id)"
        ).execute(&self.pool).await?;

        Ok(())
    }
}