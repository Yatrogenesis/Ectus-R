// CLI Configuration Management

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    pub api_url: String,
    pub auth: AuthConfig,
    pub output: OutputConfig,
    pub generation: GenerationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: String,
    pub no_color: bool,
    pub verbose: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub default_language: String,
    pub default_architecture: String,
    pub default_optimization: String,
    pub auto_download: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:8080".to_string(),
            auth: AuthConfig {
                token: None,
                refresh_token: None,
                expires_at: None,
            },
            output: OutputConfig {
                format: "table".to_string(),
                no_color: false,
                verbose: false,
            },
            generation: GenerationConfig {
                default_language: "rust".to_string(),
                default_architecture: "layered".to_string(),
                default_optimization: "balanced".to_string(),
                auto_download: false,
            },
        }
    }
}

impl CliConfig {
    /// Load configuration from file or create default
    pub fn load(config_path: Option<&Path>) -> Result<Self> {
        let config_path = match config_path {
            Some(path) => path.to_path_buf(),
            None => Self::default_config_path()?,
        };

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: CliConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Create default config
            let config = Self::default();
            config.save(&config_path)?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get default configuration file path
    pub fn default_config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
        Ok(home.join(".config").join("aion").join("config.toml"))
    }

    /// Get authentication token
    pub fn get_auth_token(&self) -> Option<String> {
        // Check if token is expired
        if let Some(expires_at) = self.auth.expires_at {
            let now = chrono::Utc::now().timestamp();
            if now >= expires_at {
                return None; // Token expired
            }
        }
        self.auth.token.clone()
    }

    /// Set authentication tokens
    pub fn set_auth_tokens(&mut self, token: String, refresh_token: String, expires_in: u64) {
        let expires_at = chrono::Utc::now().timestamp() + expires_in as i64;
        self.auth.token = Some(token);
        self.auth.refresh_token = Some(refresh_token);
        self.auth.expires_at = Some(expires_at);
    }

    /// Clear authentication
    pub fn clear_auth(&mut self) {
        self.auth.token = None;
        self.auth.refresh_token = None;
        self.auth.expires_at = None;
    }

    /// Check if authenticated
    pub fn is_authenticated(&self) -> bool {
        self.get_auth_token().is_some()
    }
}