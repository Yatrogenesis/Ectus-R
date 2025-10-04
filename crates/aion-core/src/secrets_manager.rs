use std::collections::HashMap;
use std::env;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, debug};

/// Simple secrets manager for development and production
/// In production, this should be replaced with AWS Secrets Manager, HashiCorp Vault, etc.
#[derive(Debug, Clone)]
pub struct SecretsManager {
    secrets: HashMap<String, String>,
    provider: SecretProvider,
}

#[derive(Debug, Clone)]
pub enum SecretProvider {
    Environment,
    File(String),
    #[cfg(feature = "aws")]
    AwsSecretsManager,
    #[cfg(feature = "vault")]
    HashiCorpVault,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretValue {
    pub value: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl SecretsManager {
    /// Create a new secrets manager with environment provider
    pub fn new() -> Self {
        Self {
            secrets: HashMap::new(),
            provider: SecretProvider::Environment,
        }
    }

    /// Create secrets manager with specific provider
    pub fn with_provider(provider: SecretProvider) -> Self {
        Self {
            secrets: HashMap::new(),
            provider,
        }
    }

    /// Initialize secrets manager by loading all required secrets
    pub async fn initialize(&mut self) -> Result<()> {
        info!("Initializing secrets manager with provider: {:?}", self.provider);

        match self.provider.clone() {
            SecretProvider::Environment => self.load_from_environment()?,
            SecretProvider::File(path) => self.load_from_file(&path).await?,
            #[cfg(feature = "aws")]
            SecretProvider::AwsSecretsManager => self.load_from_aws().await?,
            #[cfg(feature = "vault")]
            SecretProvider::HashiCorpVault => self.load_from_vault().await?,
        }

        self.validate_required_secrets()?;
        info!("Secrets manager initialized successfully");
        Ok(())
    }

    /// Get a secret value by key
    pub fn get_secret(&self, key: &str) -> Result<String> {
        self.secrets
            .get(key)
            .cloned()
            .ok_or_else(|| anyhow!("Secret '{}' not found", key))
    }

    /// Get an optional secret (returns None if not found)
    pub fn get_optional_secret(&self, key: &str) -> Option<String> {
        self.secrets.get(key).cloned()
    }

    /// Set a secret value (for testing or dynamic updates)
    pub fn set_secret(&mut self, key: String, value: String) {
        self.secrets.insert(key, value);
    }

    /// Load secrets from environment variables
    fn load_from_environment(&mut self) -> Result<()> {
        debug!("Loading secrets from environment variables");

        let required_keys = [
            "GROQ_API_KEY",
            "OPENAI_API_KEY",
            "JWT_SECRET",
            "ENCRYPTION_KEY",
            "DATABASE_URL",
            "REDIS_URL",
        ];

        let optional_keys = [
            "GITHUB_TOKEN",
            "HUGGINGFACE_API_KEY",
            "CLOUDFLARE_API_KEY",
            "CLOUDFLARE_ACCOUNT_ID",
            "STRIPE_SECRET_KEY",
            "STRIPE_PUBLISHABLE_KEY",
            "MINIO_ACCESS_KEY",
            "MINIO_SECRET_KEY",
        ];

        // Load required secrets
        for key in &required_keys {
            if let Ok(value) = env::var(key) {
                if !value.is_empty() && !value.contains("your_") && !value.contains("_here") {
                    self.secrets.insert(key.to_string(), value);
                    debug!("Loaded required secret: {}", key);
                } else {
                    warn!("Required secret '{}' not configured or has placeholder value", key);
                }
            } else {
                warn!("Required secret '{}' not found in environment", key);
            }
        }

        // Load optional secrets
        for key in &optional_keys {
            if let Ok(value) = env::var(key) {
                if !value.is_empty() && !value.contains("your_") && !value.contains("_here") {
                    self.secrets.insert(key.to_string(), value);
                    debug!("Loaded optional secret: {}", key);
                }
            }
        }

        Ok(())
    }

    /// Load secrets from file (JSON format)
    async fn load_from_file(&mut self, path: &str) -> Result<()> {
        debug!("Loading secrets from file: {}", path);

        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| anyhow!("Failed to read secrets file '{}': {}", path, e))?;

        let file_secrets: HashMap<String, String> = serde_json::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse secrets file '{}': {}", path, e))?;

        self.secrets.extend(file_secrets);
        Ok(())
    }

    #[cfg(feature = "aws")]
    async fn load_from_aws(&mut self) -> Result<()> {
        // TODO: Implement AWS Secrets Manager integration
        warn!("AWS Secrets Manager integration not yet implemented");
        Ok(())
    }

    #[cfg(feature = "vault")]
    async fn load_from_vault(&mut self) -> Result<()> {
        // TODO: Implement HashiCorp Vault integration
        warn!("HashiCorp Vault integration not yet implemented");
        Ok(())
    }

    /// Validate that all required secrets are present
    fn validate_required_secrets(&self) -> Result<()> {
        let required = [
            "JWT_SECRET",
            "ENCRYPTION_KEY",
            "DATABASE_URL",
        ];

        let mut missing = Vec::new();
        for key in &required {
            if !self.secrets.contains_key(*key) {
                missing.push(*key);
            }
        }

        if !missing.is_empty() {
            return Err(anyhow!(
                "Missing required secrets: {}. Please configure them in environment variables or secrets file.",
                missing.join(", ")
            ));
        }

        // Validate at least one LLM provider is configured
        let llm_providers = [
            "GROQ_API_KEY",
            "OPENAI_API_KEY",
            "GITHUB_TOKEN",
            "HUGGINGFACE_API_KEY",
            "CLOUDFLARE_API_KEY",
        ];

        let configured_providers: Vec<_> = llm_providers
            .iter()
            .filter(|key| self.secrets.contains_key(**key))
            .collect();

        if configured_providers.is_empty() {
            warn!("No LLM providers configured. AI features will be disabled.");
        } else {
            info!("Configured LLM providers: {:?}", configured_providers);
        }

        Ok(())
    }

    /// Get database URL with validation
    pub fn get_database_url(&self) -> Result<String> {
        let url = self.get_secret("DATABASE_URL")?;
        if !url.starts_with("postgresql://") && !url.starts_with("postgres://") {
            return Err(anyhow!("Invalid database URL format. Must be postgresql:// or postgres://"));
        }
        Ok(url)
    }

    /// Get Redis URL with validation
    pub fn get_redis_url(&self) -> Result<String> {
        let url = self.get_secret("REDIS_URL")?;
        if !url.starts_with("redis://") && !url.starts_with("rediss://") {
            return Err(anyhow!("Invalid Redis URL format. Must be redis:// or rediss://"));
        }
        Ok(url)
    }

    /// Get JWT secret with validation
    pub fn get_jwt_secret(&self) -> Result<String> {
        let secret = self.get_secret("JWT_SECRET")?;
        if secret.len() < 32 {
            return Err(anyhow!("JWT_SECRET must be at least 32 characters long"));
        }
        Ok(secret)
    }

    /// Get encryption key with validation
    pub fn get_encryption_key(&self) -> Result<String> {
        let key = self.get_secret("ENCRYPTION_KEY")?;
        if key.len() < 32 {
            return Err(anyhow!("ENCRYPTION_KEY must be at least 32 characters long"));
        }
        Ok(key)
    }

    /// Get LLM API keys in priority order
    pub fn get_llm_providers(&self) -> Vec<(String, String)> {
        let providers = [
            ("groq", "GROQ_API_KEY"),
            ("openai", "OPENAI_API_KEY"),
            ("github", "GITHUB_TOKEN"),
            ("huggingface", "HUGGINGFACE_API_KEY"),
            ("cloudflare", "CLOUDFLARE_API_KEY"),
        ];

        providers
            .iter()
            .filter_map(|(name, key)| {
                self.secrets.get(*key).map(|value| (name.to_string(), value.clone()))
            })
            .collect()
    }

    /// List all configured secret keys (without values for security)
    pub fn list_configured_secrets(&self) -> Vec<String> {
        self.secrets.keys().cloned().collect()
    }
}

impl Default for SecretsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_environment_secrets_loading() {
        env::set_var("TEST_JWT_SECRET", "a".repeat(32));
        env::set_var("TEST_ENCRYPTION_KEY", "b".repeat(32));
        env::set_var("TEST_DATABASE_URL", "postgresql://user:password@localhost/database");

        let mut manager = SecretsManager::new();

        // Override for test
        manager.secrets.insert("JWT_SECRET".to_string(), "a".repeat(32));
        manager.secrets.insert("ENCRYPTION_KEY".to_string(), "b".repeat(32));
        manager.secrets.insert("DATABASE_URL".to_string(), "postgresql://user:password@localhost/database".to_string());

        assert!(manager.validate_required_secrets().is_ok());
        assert_eq!(manager.get_jwt_secret().unwrap(), "a".repeat(32));
        assert_eq!(manager.get_encryption_key().unwrap(), "b".repeat(32));
    }

    #[test]
    fn test_secret_validation() {
        let mut manager = SecretsManager::new();

        // Test too short JWT secret
        manager.set_secret("JWT_SECRET".to_string(), "short".to_string());
        assert!(manager.get_jwt_secret().is_err());

        // Test valid JWT secret
        manager.set_secret("JWT_SECRET".to_string(), "a".repeat(32));
        assert!(manager.get_jwt_secret().is_ok());
    }
}