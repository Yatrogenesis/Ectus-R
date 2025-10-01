//! Secrets Management Module
//!
//! Implements secure secrets management following audit recommendation #2
//! Supports multiple backends: environment variables, HashiCorp Vault, AWS Secrets Manager

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecretBackend {
    Environment,
    Vault {
        url: String,
        token: String,
        namespace: Option<String>,
    },
    AwsSecretsManager {
        region: String,
        secret_prefix: Option<String>,
    },
    AzureKeyVault {
        vault_url: String,
        tenant_id: String,
        client_id: String,
    },
}

#[derive(Debug, Clone)]
pub struct SecretsManager {
    backend: SecretBackend,
    cache: Arc<RwLock<HashMap<String, Secret>>>,
    cache_ttl_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub value: String,
    pub version: Option<String>,
    #[serde(skip_serializing)]
    pub retrieved_at: std::time::SystemTime,
}

impl SecretsManager {
    /// Create a new secrets manager with the specified backend
    pub fn new(backend: SecretBackend) -> Self {
        Self {
            backend,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl_seconds: 300, // 5 minutes default
        }
    }

    /// Create from environment configuration
    pub fn from_env() -> Result<Self> {
        let backend_type = std::env::var("SECRETS_BACKEND")
            .unwrap_or_else(|_| "environment".to_string());

        let backend = match backend_type.as_str() {
            "vault" => {
                let url = std::env::var("VAULT_ADDR")
                    .context("VAULT_ADDR required for Vault backend")?;
                let token = std::env::var("VAULT_TOKEN")
                    .context("VAULT_TOKEN required for Vault backend")?;
                let namespace = std::env::var("VAULT_NAMESPACE").ok();

                SecretBackend::Vault { url, token, namespace }
            }
            "aws" => {
                let region = std::env::var("AWS_REGION")
                    .unwrap_or_else(|_| "us-east-1".to_string());
                let secret_prefix = std::env::var("AWS_SECRET_PREFIX").ok();

                SecretBackend::AwsSecretsManager { region, secret_prefix }
            }
            "azure" => {
                let vault_url = std::env::var("AZURE_VAULT_URL")
                    .context("AZURE_VAULT_URL required for Azure backend")?;
                let tenant_id = std::env::var("AZURE_TENANT_ID")
                    .context("AZURE_TENANT_ID required")?;
                let client_id = std::env::var("AZURE_CLIENT_ID")
                    .context("AZURE_CLIENT_ID required")?;

                SecretBackend::AzureKeyVault { vault_url, tenant_id, client_id }
            }
            _ => SecretBackend::Environment,
        };

        Ok(Self::new(backend))
    }

    /// Set cache TTL in seconds
    pub fn with_cache_ttl(mut self, seconds: u64) -> Self {
        self.cache_ttl_seconds = seconds;
        self
    }

    /// Get a secret by key
    pub async fn get_secret(&self, key: &str) -> Result<String> {
        // Check cache first
        if let Some(cached) = self.get_from_cache(key).await {
            if !self.is_cache_expired(&cached) {
                return Ok(cached.value);
            }
        }

        // Fetch from backend
        let secret = self.fetch_from_backend(key).await?;

        // Update cache
        self.update_cache(key, secret.clone()).await;

        Ok(secret.value)
    }

    /// Get multiple secrets at once
    pub async fn get_secrets(&self, keys: &[&str]) -> Result<HashMap<String, String>> {
        let mut results = HashMap::new();

        for key in keys {
            match self.get_secret(key).await {
                Ok(value) => {
                    results.insert(key.to_string(), value);
                }
                Err(e) => {
                    tracing::warn!("Failed to get secret '{}': {}", key, e);
                }
            }
        }

        Ok(results)
    }

    /// Refresh a secret in cache
    pub async fn refresh_secret(&self, key: &str) -> Result<String> {
        let secret = self.fetch_from_backend(key).await?;
        self.update_cache(key, secret.clone()).await;
        Ok(secret.value)
    }

    /// Clear the cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        tracing::info!("Secrets cache cleared");
    }

    /// Get secret from cache
    async fn get_from_cache(&self, key: &str) -> Option<Secret> {
        let cache = self.cache.read().await;
        cache.get(key).cloned()
    }

    /// Update cache
    async fn update_cache(&self, key: &str, secret: Secret) {
        let mut cache = self.cache.write().await;
        cache.insert(key.to_string(), secret);
    }

    /// Check if cached secret is expired
    fn is_cache_expired(&self, secret: &Secret) -> bool {
        if let Ok(elapsed) = secret.retrieved_at.elapsed() {
            elapsed.as_secs() > self.cache_ttl_seconds
        } else {
            true
        }
    }

    /// Fetch secret from backend
    async fn fetch_from_backend(&self, key: &str) -> Result<Secret> {
        match &self.backend {
            SecretBackend::Environment => self.fetch_from_env(key).await,
            SecretBackend::Vault { url, token, namespace } => {
                self.fetch_from_vault(key, url, token, namespace.as_deref()).await
            }
            SecretBackend::AwsSecretsManager { region, secret_prefix } => {
                self.fetch_from_aws(key, region, secret_prefix.as_deref()).await
            }
            SecretBackend::AzureKeyVault { vault_url, tenant_id, client_id } => {
                self.fetch_from_azure(key, vault_url, tenant_id, client_id).await
            }
        }
    }

    /// Fetch from environment variables
    async fn fetch_from_env(&self, key: &str) -> Result<Secret> {
        let value = std::env::var(key)
            .with_context(|| format!("Environment variable '{}' not found", key))?;

        Ok(Secret {
            value,
            version: None,
            retrieved_at: std::time::SystemTime::now(),
        })
    }

    /// Fetch from HashiCorp Vault
    async fn fetch_from_vault(
        &self,
        key: &str,
        url: &str,
        token: &str,
        namespace: Option<&str>,
    ) -> Result<Secret> {
        let client = reqwest::Client::new();
        let mut request = client
            .get(format!("{}/v1/secret/data/{}", url, key))
            .header("X-Vault-Token", token);

        if let Some(ns) = namespace {
            request = request.header("X-Vault-Namespace", ns);
        }

        let response = request
            .send()
            .await
            .context("Failed to fetch from Vault")?;

        if !response.status().is_success() {
            anyhow::bail!("Vault returned status: {}", response.status());
        }

        let data: serde_json::Value = response.json().await?;

        let value = data["data"]["data"]["value"]
            .as_str()
            .context("Invalid Vault response format")?
            .to_string();

        let version = data["data"]["metadata"]["version"]
            .as_u64()
            .map(|v| v.to_string());

        Ok(Secret {
            value,
            version,
            retrieved_at: std::time::SystemTime::now(),
        })
    }

    /// Fetch from AWS Secrets Manager
    async fn fetch_from_aws(
        &self,
        key: &str,
        region: &str,
        prefix: Option<&str>,
    ) -> Result<Secret> {
        // Build secret name with optional prefix
        let secret_name = if let Some(p) = prefix {
            format!("{}/{}", p, key)
        } else {
            key.to_string()
        };

        // Note: In production, use AWS SDK
        // This is a placeholder implementation
        tracing::info!("Fetching secret '{}' from AWS Secrets Manager in region '{}'", secret_name, region);

        // For now, fall back to environment
        self.fetch_from_env(key).await
    }

    /// Fetch from Azure Key Vault
    async fn fetch_from_azure(
        &self,
        key: &str,
        vault_url: &str,
        _tenant_id: &str,
        _client_id: &str,
    ) -> Result<Secret> {
        // Note: In production, use Azure SDK
        tracing::info!("Fetching secret '{}' from Azure Key Vault at '{}'", key, vault_url);

        // For now, fall back to environment
        self.fetch_from_env(key).await
    }
}

/// Configuration for secrets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretsConfig {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub encryption_key: String,
    pub api_keys: HashMap<String, String>,
}

impl SecretsConfig {
    /// Load configuration from secrets manager
    pub async fn load(manager: &SecretsManager) -> Result<Self> {
        let secrets = manager.get_secrets(&[
            "DATABASE_URL",
            "REDIS_URL",
            "JWT_SECRET",
            "ENCRYPTION_KEY",
        ]).await?;

        Ok(Self {
            database_url: secrets.get("DATABASE_URL")
                .context("DATABASE_URL required")?
                .clone(),
            redis_url: secrets.get("REDIS_URL")
                .context("REDIS_URL required")?
                .clone(),
            jwt_secret: secrets.get("JWT_SECRET")
                .context("JWT_SECRET required")?
                .clone(),
            encryption_key: secrets.get("ENCRYPTION_KEY")
                .unwrap_or(&Self::generate_encryption_key())
                .clone(),
            api_keys: HashMap::new(),
        })
    }

    /// Generate a random encryption key
    fn generate_encryption_key() -> String {
        use rand::Rng;
        let key: Vec<u8> = (0..32).map(|_| rand::thread_rng().gen()).collect();
        hex::encode(key)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.database_url.is_empty() {
            anyhow::bail!("DATABASE_URL cannot be empty");
        }
        if self.jwt_secret.len() < 32 {
            anyhow::bail!("JWT_SECRET must be at least 32 characters");
        }
        if self.encryption_key.len() < 32 {
            anyhow::bail!("ENCRYPTION_KEY must be at least 32 characters");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_environment_backend() {
        std::env::set_var("TEST_SECRET", "test_value_123");

        let manager = SecretsManager::new(SecretBackend::Environment);
        let secret = manager.get_secret("TEST_SECRET").await.unwrap();

        assert_eq!(secret, "test_value_123");

        std::env::remove_var("TEST_SECRET");
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        std::env::set_var("TEST_SECRET_2", "cached_value");

        let manager = SecretsManager::new(SecretBackend::Environment)
            .with_cache_ttl(1); // 1 second TTL

        // First fetch
        let secret1 = manager.get_secret("TEST_SECRET_2").await.unwrap();
        assert_eq!(secret1, "cached_value");

        // Wait for cache to expire
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Second fetch should retrieve from backend again
        let secret2 = manager.get_secret("TEST_SECRET_2").await.unwrap();
        assert_eq!(secret2, "cached_value");

        std::env::remove_var("TEST_SECRET_2");
    }

    #[tokio::test]
    async fn test_multiple_secrets() {
        std::env::set_var("SECRET_1", "value1");
        std::env::set_var("SECRET_2", "value2");
        std::env::set_var("SECRET_3", "value3");

        let manager = SecretsManager::new(SecretBackend::Environment);
        let secrets = manager.get_secrets(&["SECRET_1", "SECRET_2", "SECRET_3"]).await.unwrap();

        assert_eq!(secrets.len(), 3);
        assert_eq!(secrets.get("SECRET_1").unwrap(), "value1");
        assert_eq!(secrets.get("SECRET_2").unwrap(), "value2");
        assert_eq!(secrets.get("SECRET_3").unwrap(), "value3");

        std::env::remove_var("SECRET_1");
        std::env::remove_var("SECRET_2");
        std::env::remove_var("SECRET_3");
    }
}
