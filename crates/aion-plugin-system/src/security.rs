use crate::config::SecurityConfig;
use crate::errors::{PluginError, Result};
use crate::plugin::{PluginInfo, PluginPackage};
use crate::PluginContext;
use std::path::Path;

/// Security manager for plugin validation and sandboxing
pub struct SecurityManager {
    config: SecurityConfig,
}

impl SecurityManager {
    /// Create a new security manager
    pub async fn new(config: SecurityConfig) -> Result<Self> {
        Ok(Self { config })
    }

    /// Validate a plugin file path
    pub async fn validate_plugin_path(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(PluginError::ValidationFailed(format!(
                "Plugin file does not exist: {}",
                path.display()
            )));
        }

        if !path.is_file() {
            return Err(PluginError::ValidationFailed(format!(
                "Plugin path is not a file: {}",
                path.display()
            )));
        }

        // Check file size
        let metadata = std::fs::metadata(path)?;
        if metadata.len() > self.config.max_plugin_size as u64 {
            return Err(PluginError::ValidationFailed(format!(
                "Plugin file exceeds maximum size: {} > {}",
                metadata.len(),
                self.config.max_plugin_size
            )));
        }

        Ok(())
    }

    /// Validate a loaded plugin
    pub async fn validate_plugin(&self, plugin: &PluginInfo) -> Result<()> {
        // Validate metadata
        if plugin.metadata.name.is_empty() {
            return Err(PluginError::ValidationFailed(
                "Plugin name is empty".to_string(),
            ));
        }

        // Verify checksum if configured
        if self.config.verify_signatures {
            if let Some(expected_checksum) = &plugin.metadata.checksum {
                let actual_checksum = self.calculate_checksum(&plugin.binary_data);
                if &actual_checksum != expected_checksum {
                    return Err(PluginError::SecurityViolation(
                        "Plugin checksum mismatch".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    /// Validate plugin execution context
    pub async fn validate_execution(&self, context: &PluginContext) -> Result<()> {
        // Validate execution limits
        if context.limits.timeout_seconds == 0 {
            return Err(PluginError::ValidationFailed(
                "Execution timeout must be greater than zero".to_string(),
            ));
        }

        if context.limits.max_memory_bytes == 0 {
            return Err(PluginError::ValidationFailed(
                "Memory limit must be greater than zero".to_string(),
            ));
        }

        Ok(())
    }

    /// Verify plugin package signature
    pub async fn verify_plugin_signature(&self, package: &PluginPackage) -> Result<()> {
        if !self.config.verify_signatures {
            return Ok(());
        }

        if package.signature.is_none() {
            return Err(PluginError::SecurityViolation(
                "Plugin package has no signature".to_string(),
            ));
        }

        // In a real implementation, this would verify the signature
        // using the trusted keys from config
        tracing::warn!("Signature verification not implemented");

        Ok(())
    }

    /// Calculate SHA-256 checksum
    fn calculate_checksum(&self, data: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }
}

/// Hex encoding helper
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes
            .as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}
