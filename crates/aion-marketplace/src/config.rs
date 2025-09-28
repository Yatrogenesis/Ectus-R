use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceConfig {
    pub database: DatabaseConfig,
    pub storage: StorageConfig,
    pub security: SecurityConfig,
    pub analytics: AnalyticsConfig,
    pub payments: PaymentConfig,
    pub notifications: NotificationConfig,
    pub search: SearchConfig,
    pub validation: ValidationConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: u64,
    pub idle_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub provider: StorageProvider,
    pub bucket: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub endpoint: Option<String>,
    pub cdn_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageProvider {
    S3,
    MinIO,
    Local { path: PathBuf },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub jwt_expiration: u64,
    pub rate_limit_per_minute: u32,
    pub max_file_size: usize,
    pub allowed_file_types: Vec<String>,
    pub virus_scanning: bool,
    pub content_scanning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    pub enabled: bool,
    pub retention_days: u32,
    pub batch_size: usize,
    pub flush_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentConfig {
    pub enabled: bool,
    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,
    pub commission_rate: f32,
    pub minimum_payout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub email_enabled: bool,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub webhook_enabled: bool,
    pub webhook_urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub provider: SearchProvider,
    pub elasticsearch_url: Option<String>,
    pub index_name: String,
    pub reindex_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchProvider {
    InMemory,
    Elasticsearch,
    OpenSearch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub enabled: bool,
    pub max_package_size: usize,
    pub required_files: Vec<String>,
    pub forbidden_patterns: Vec<String>,
    pub license_validation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub cors_origins: Vec<String>,
    pub tls_cert: Option<PathBuf>,
    pub tls_key: Option<PathBuf>,
}

impl Default for MarketplaceConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                url: "postgresql://localhost/aion_marketplace".to_string(),
                max_connections: 20,
                min_connections: 5,
                connection_timeout: 30,
                idle_timeout: 600,
            },
            storage: StorageConfig {
                provider: StorageProvider::Local {
                    path: PathBuf::from("./storage")
                },
                bucket: "aion-marketplace".to_string(),
                region: "us-east-1".to_string(),
                access_key: "".to_string(),
                secret_key: "".to_string(),
                endpoint: None,
                cdn_url: None,
            },
            security: SecurityConfig {
                jwt_secret: "change-me-in-production".to_string(),
                jwt_expiration: 86400,
                rate_limit_per_minute: 100,
                max_file_size: 100 * 1024 * 1024,
                allowed_file_types: vec![
                    "tar.gz".to_string(),
                    "zip".to_string(),
                    "wasm".to_string(),
                ],
                virus_scanning: false,
                content_scanning: true,
            },
            analytics: AnalyticsConfig {
                enabled: true,
                retention_days: 90,
                batch_size: 1000,
                flush_interval: 60,
            },
            payments: PaymentConfig {
                enabled: false,
                stripe_secret_key: "".to_string(),
                stripe_webhook_secret: "".to_string(),
                commission_rate: 0.10,
                minimum_payout: 5000,
            },
            notifications: NotificationConfig {
                email_enabled: false,
                smtp_host: "localhost".to_string(),
                smtp_port: 587,
                smtp_username: "".to_string(),
                smtp_password: "".to_string(),
                from_email: "noreply@aion.dev".to_string(),
                webhook_enabled: false,
                webhook_urls: vec![],
            },
            search: SearchConfig {
                provider: SearchProvider::InMemory,
                elasticsearch_url: None,
                index_name: "aion_packages".to_string(),
                reindex_interval: 3600,
            },
            validation: ValidationConfig {
                enabled: true,
                max_package_size: 100 * 1024 * 1024,
                required_files: vec!["package.json".to_string()],
                forbidden_patterns: vec![
                    "node_modules/".to_string(),
                    ".git/".to_string(),
                    "target/".to_string(),
                ],
                license_validation: true,
            },
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: 4,
                cors_origins: vec!["*".to_string()],
                tls_cert: None,
                tls_key: None,
            },
        }
    }
}

impl MarketplaceConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let mut config = Self::default();

        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            config.database.url = db_url;
        }

        if let Ok(jwt_secret) = std::env::var("JWT_SECRET") {
            config.security.jwt_secret = jwt_secret;
        }

        if let Ok(stripe_key) = std::env::var("STRIPE_SECRET_KEY") {
            config.payments.stripe_secret_key = stripe_key;
            config.payments.enabled = true;
        }

        if let Ok(port) = std::env::var("PORT") {
            config.server.port = port.parse()?;
        }

        Ok(config)
    }
}