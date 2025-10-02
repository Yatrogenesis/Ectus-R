// AION-R Server Configuration
// Configuration management for the server

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main application configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub auth: AuthConfig,
    pub ai_engine: AIEngineConfig, // Stub implementation
    pub storage: StorageConfig,
    pub logging: LoggingConfig,
    pub monitoring: MonitoringConfig,
    pub features: FeatureConfig,
}

/// AI Engine configuration (stub)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AIEngineConfig {
    pub model_path: String,
    pub max_concurrent_requests: u32,
}

impl Default for AIEngineConfig {
    fn default() -> Self {
        Self {
            model_path: "./models".to_string(),
            max_concurrent_requests: 10,
        }
    }
}

/// Server configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub max_connections: Option<usize>,
    pub request_timeout_seconds: Option<u64>,
    pub max_request_size: Option<usize>,
}

/// Database configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connect_timeout_seconds: Option<u64>,
    pub idle_timeout_seconds: Option<u64>,
}

/// Redis configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: Option<u32>,
    pub connection_timeout_seconds: Option<u64>,
}

/// Authentication configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiry_seconds: Option<u64>,
    pub refresh_token_expiry_seconds: Option<u64>,
    pub max_failed_attempts: Option<u32>,
    pub lockout_duration_seconds: Option<u64>,
    pub enable_mfa: Option<bool>,
}

/// Storage configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageConfig {
    pub storage_type: String,
    pub base_path: Option<String>,
    pub s3_bucket: Option<String>,
    pub s3_region: Option<String>,
    pub max_file_size: Option<usize>,
}

/// Logging configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
    pub file_path: Option<String>,
    pub max_file_size: Option<String>,
    pub max_files: Option<u32>,
}

/// Monitoring configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitoringConfig {
    pub enable_metrics: bool,
    pub metrics_port: Option<u16>,
    pub enable_tracing: Option<bool>,
    pub jaeger_endpoint: Option<String>,
}

/// Feature flags configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeatureConfig {
    pub enable_code_generation: bool,
    pub enable_requirements_analysis: bool,
    pub enable_text_processing: bool,
    pub enable_image_processing: bool,
    pub enable_audio_processing: bool,
    pub enable_admin_panel: bool,
    pub enable_swagger_ui: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: None,
                max_connections: Some(1000),
                request_timeout_seconds: Some(30),
                max_request_size: Some(50 * 1024 * 1024), // 50MB
            },
            database: DatabaseConfig {
                url: "postgresql://aion_user:password@localhost:5432/aion".to_string(),
                max_connections: Some(20),
                min_connections: Some(5),
                connect_timeout_seconds: Some(10),
                idle_timeout_seconds: Some(600),
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                max_connections: Some(20),
                connection_timeout_seconds: Some(5),
            },
            auth: AuthConfig {
                jwt_secret: "your-secret-key-here".to_string(),
                jwt_expiry_seconds: Some(3600),
                refresh_token_expiry_seconds: Some(7 * 24 * 3600),
                max_failed_attempts: Some(5),
                lockout_duration_seconds: Some(900),
                enable_mfa: Some(false),
            },
            ai_engine: AIEngineConfig::default(),
            storage: StorageConfig {
                storage_type: "local".to_string(),
                base_path: Some("./storage".to_string()),
                s3_bucket: None,
                s3_region: None,
                max_file_size: Some(100 * 1024 * 1024), // 100MB
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
                output: "stdout".to_string(),
                file_path: None,
                max_file_size: Some("10MB".to_string()),
                max_files: Some(10),
            },
            monitoring: MonitoringConfig {
                enable_metrics: true,
                metrics_port: Some(9090),
                enable_tracing: Some(true),
                jaeger_endpoint: None,
            },
            features: FeatureConfig {
                enable_code_generation: true,
                enable_requirements_analysis: true,
                enable_text_processing: true,
                enable_image_processing: true,
                enable_audio_processing: true,
                enable_admin_panel: true,
                enable_swagger_ui: true,
            },
        }
    }
}

impl AppConfig {
    /// Load configuration from environment and files
    pub fn load() -> anyhow::Result<Self> {
        let mut config = Self::default();

        // Load from environment variables
        config.load_from_env();

        // Load from config file if exists
        if let Ok(config_file) = std::env::var("AION_CONFIG_FILE") {
            config.load_from_file(&config_file)?;
        } else {
            // Try default config files
            for path in &["./config/default.toml", "./aion-config.toml", "/etc/aion/config.toml"] {
                if std::path::Path::new(path).exists() {
                    config.load_from_file(path)?;
                    break;
                }
            }
        }

        // Validate configuration
        config.validate()?;

        Ok(config)
    }

    /// Load configuration from environment variables
    fn load_from_env(&mut self) {
        // Server configuration
        if let Ok(host) = std::env::var("AION_HOST") {
            self.server.host = host;
        }
        if let Ok(port) = std::env::var("AION_PORT") {
            if let Ok(port) = port.parse() {
                self.server.port = port;
            }
        }

        // Database configuration
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            self.database.url = db_url;
        }

        // Redis configuration
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            self.redis.url = redis_url;
        }

        // Authentication configuration
        if let Ok(jwt_secret) = std::env::var("JWT_SECRET") {
            self.auth.jwt_secret = jwt_secret;
        }

        // Logging configuration
        if let Ok(log_level) = std::env::var("LOG_LEVEL") {
            self.logging.level = log_level;
        }
    }

    /// Load configuration from TOML file
    fn load_from_file(&mut self, path: &str) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(path)?;
        let file_config: AppConfig = toml::from_str(&content)?;

        // Merge with current configuration
        *self = file_config;

        Ok(())
    }

    /// Validate configuration
    fn validate(&self) -> anyhow::Result<()> {
        // Validate JWT secret
        if self.auth.jwt_secret.len() < 32 {
            anyhow::bail!("JWT secret must be at least 32 characters long");
        }

        // Validate database URL
        if !self.database.url.starts_with("postgresql://") {
            anyhow::bail!("Invalid database URL format");
        }

        // Validate Redis URL
        if !self.redis.url.starts_with("redis://") {
            anyhow::bail!("Invalid Redis URL format");
        }

        // Validate server configuration
        if self.server.port == 0 {
            anyhow::bail!("Server port must be greater than 0");
        }

        Ok(())
    }

    /// Get configuration as environment variables for containers
    pub fn to_env_vars(&self) -> Vec<(String, String)> {
        vec![
            ("AION_HOST".to_string(), self.server.host.clone()),
            ("AION_PORT".to_string(), self.server.port.to_string()),
            ("DATABASE_URL".to_string(), self.database.url.clone()),
            ("REDIS_URL".to_string(), self.redis.url.clone()),
            ("JWT_SECRET".to_string(), self.auth.jwt_secret.clone()),
            ("LOG_LEVEL".to_string(), self.logging.level.clone()),
        ]
    }
}