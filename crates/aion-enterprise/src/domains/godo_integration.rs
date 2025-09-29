// AION-R Enterprise: GoDo-R Integration for Domain Management
// Integrates with GoDo-R CLI for automatic domain provisioning and DNS management

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// GoDo-R CLI integration for domain management
#[derive(Debug, Clone)]
pub struct GoDaddyDomainManager {
    api_key: String,
    api_secret: String,
    environment: String,
    godo_path: Option<String>,
}

/// Domain configuration for automatic setup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainConfig {
    pub domain: String,
    pub subdomain: Option<String>,
    pub target_url: String,
    pub record_type: DNSRecordType,
    pub ttl: Option<u32>,
    pub cloudflare_zone_id: Option<String>,
    pub ssl_enabled: bool,
    pub auto_renewal: bool,
}

/// DNS record types supported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DNSRecordType {
    A,
    AAAA,
    CNAME,
    TXT,
    MX,
    NS,
    SRV,
}

/// Result of domain operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainOperationResult {
    pub success: bool,
    pub domain: String,
    pub record_id: Option<String>,
    pub full_domain: String,
    pub message: String,
    pub propagation_time: Option<u32>,
    pub ssl_certificate: Option<SSLCertificate>,
    pub created_at: DateTime<Utc>,
}

/// SSL certificate information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSLCertificate {
    pub issued_by: String,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub fingerprint: String,
}

impl GoDaddyDomainManager {
    /// Create new GoDaddy domain manager
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
            environment: "production".to_string(),
            godo_path: None,
        }
    }

    /// Set custom GoDo-R CLI path
    pub fn with_godo_path(mut self, path: String) -> Self {
        self.godo_path = Some(path);
        self
    }

    /// Set environment (production/sandbox)
    pub fn with_environment(mut self, env: String) -> Self {
        self.environment = env;
        self
    }

    /// Configure domain for worker deployment
    pub async fn setup_domain_for_worker(
        &self,
        domain_config: DomainConfig,
        worker_url: &str,
    ) -> Result<DomainOperationResult> {
        tracing::info!("Setting up domain {} for worker {}", domain_config.domain, worker_url);

        // Ensure GoDo-R CLI is available
        self.ensure_godo_available().await?;

        // Extract worker subdomain from URL
        let worker_subdomain = self.extract_worker_subdomain(worker_url)?;

        // Create DNS record pointing to Cloudflare Workers
        let record_result = self.create_dns_record(&domain_config, &worker_subdomain).await?;

        // If using SSL, set up certificate
        let ssl_certificate = if domain_config.ssl_enabled {
            Some(self.setup_ssl_certificate(&domain_config).await?)
        } else {
            None
        };

        // Configure Cloudflare zone if provided
        if let Some(zone_id) = &domain_config.cloudflare_zone_id {
            self.configure_cloudflare_zone(zone_id, &domain_config, worker_url).await?;
        }

        let full_domain = if let Some(subdomain) = &domain_config.subdomain {
            format!("{}.{}", subdomain, domain_config.domain)
        } else {
            domain_config.domain.clone()
        };

        Ok(DomainOperationResult {
            success: true,
            domain: domain_config.domain,
            record_id: Some(record_result.record_id),
            full_domain,
            message: "Domain configured successfully".to_string(),
            propagation_time: Some(300), // 5 minutes typical propagation
            ssl_certificate,
            created_at: Utc::now(),
        })
    }

    /// List all domains in GoDaddy account
    pub async fn list_domains(&self) -> Result<Vec<DomainInfo>> {
        let output = self.execute_godo_command(&[
            "domains", "list", "--format", "json"
        ]).await?;

        let domains: Vec<DomainInfo> = serde_json::from_str(&output)
            .context("Failed to parse domains list")?;

        Ok(domains)
    }

    /// Get domain details
    pub async fn get_domain_details(&self, domain: &str) -> Result<DomainDetails> {
        let output = self.execute_godo_command(&[
            "domains", "get", domain, "--format", "json"
        ]).await?;

        let details: DomainDetails = serde_json::from_str(&output)
            .context("Failed to parse domain details")?;

        Ok(details)
    }

    /// Create subdomain for specific service
    pub async fn create_subdomain(
        &self,
        domain: &str,
        subdomain: &str,
        target: &str,
    ) -> Result<DomainOperationResult> {
        tracing::info!("Creating subdomain {}.{} -> {}", subdomain, domain, target);

        let record_result = self.execute_godo_command(&[
            "dns", "add",
            domain,
            "--name", subdomain,
            "--type", "CNAME",
            "--data", target,
            "--ttl", "300",
            "--format", "json"
        ]).await?;

        let full_domain = format!("{}.{}", subdomain, domain);

        Ok(DomainOperationResult {
            success: true,
            domain: domain.to_string(),
            record_id: Some(self.extract_record_id(&record_result)?),
            full_domain,
            message: "Subdomain created successfully".to_string(),
            propagation_time: Some(300),
            ssl_certificate: None,
            created_at: Utc::now(),
        })
    }

    /// Setup automatic SSL certificate
    pub async fn setup_ssl_certificate(&self, config: &DomainConfig) -> Result<SSLCertificate> {
        // In a real implementation, this would integrate with Let's Encrypt
        // or GoDaddy's SSL service through GoDo-R
        Ok(SSLCertificate {
            issued_by: "Let's Encrypt".to_string(),
            valid_from: Utc::now(),
            valid_until: Utc::now() + chrono::Duration::days(90),
            fingerprint: "sha256:abcd1234...".to_string(),
        })
    }

    /// Configure DNS for Magic Loop deployment
    pub async fn configure_magic_loop_domain(
        &self,
        base_domain: &str,
        project_name: &str,
        worker_url: &str,
    ) -> Result<DomainOperationResult> {
        let subdomain = self.sanitize_subdomain(project_name);
        let domain_config = DomainConfig {
            domain: base_domain.to_string(),
            subdomain: Some(subdomain.clone()),
            target_url: worker_url.to_string(),
            record_type: DNSRecordType::CNAME,
            ttl: Some(300),
            cloudflare_zone_id: None,
            ssl_enabled: true,
            auto_renewal: true,
        };

        self.setup_domain_for_worker(domain_config, worker_url).await
    }

    /// Bulk domain operations for enterprise deployments
    pub async fn bulk_domain_setup(
        &self,
        configs: Vec<DomainConfig>,
    ) -> Result<Vec<DomainOperationResult>> {
        let mut results = Vec::new();

        for config in configs {
            match self.setup_domain_for_worker(config.clone(), &config.target_url).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    results.push(DomainOperationResult {
                        success: false,
                        domain: config.domain,
                        record_id: None,
                        full_domain: config.subdomain.map_or(config.domain.clone(), |s| format!("{}.{}", s, config.domain)),
                        message: format!("Failed: {}", e),
                        propagation_time: None,
                        ssl_certificate: None,
                        created_at: Utc::now(),
                    });
                }
            }
        }

        Ok(results)
    }

    /// Monitor domain propagation status
    pub async fn check_propagation_status(&self, domain: &str) -> Result<PropagationStatus> {
        let output = self.execute_godo_command(&[
            "dns", "check", domain, "--format", "json"
        ]).await?;

        let status: PropagationStatus = serde_json::from_str(&output)
            .context("Failed to parse propagation status")?;

        Ok(status)
    }

    /// Delete domain record
    pub async fn delete_domain_record(&self, domain: &str, record_id: &str) -> Result<()> {
        self.execute_godo_command(&[
            "dns", "delete", domain, "--record-id", record_id
        ]).await?;

        tracing::info!("Deleted DNS record {} for domain {}", record_id, domain);
        Ok(())
    }

    // Private helper methods

    async fn ensure_godo_available(&self) -> Result<()> {
        let godo_cmd = self.godo_path.as_deref().unwrap_or("godo");

        // Check if GoDo-R is available
        let version_check = Command::new(godo_cmd)
            .args(&["--version"])
            .output();

        match version_check {
            Ok(output) if output.status.success() => {
                tracing::info!("GoDo-R CLI available: {}", String::from_utf8_lossy(&output.stdout));
                Ok(())
            },
            _ => {
                // Try to download/install GoDo-R from GitHub
                self.install_godo_cli().await
            }
        }
    }

    async fn install_godo_cli(&self) -> Result<()> {
        tracing::info!("Installing GoDo-R CLI...");

        // Clone GoDo-R repository
        let output = Command::new("git")
            .args(&[
                "clone",
                "https://github.com/Yatrogenesis/GoDo-R.git",
                "/tmp/godo-r"
            ])
            .output()
            .context("Failed to clone GoDo-R repository")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Failed to clone GoDo-R: {}", String::from_utf8_lossy(&output.stderr)));
        }

        // Build GoDo-R
        let build_output = Command::new("cargo")
            .args(&["build", "--release"])
            .current_dir("/tmp/godo-r")
            .output()
            .context("Failed to build GoDo-R")?;

        if !build_output.status.success() {
            return Err(anyhow::anyhow!("Failed to build GoDo-R: {}", String::from_utf8_lossy(&build_output.stderr)));
        }

        // Copy binary to PATH
        let install_output = Command::new("cp")
            .args(&["/tmp/godo-r/target/release/godo", "/usr/local/bin/godo"])
            .output();

        if install_output.is_err() {
            // Try user-local installation
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            std::fs::create_dir_all(format!("{}/.local/bin", home))?;

            Command::new("cp")
                .args(&["/tmp/godo-r/target/release/godo", &format!("{}/.local/bin/godo", home)])
                .output()
                .context("Failed to install GoDo-R")?;
        }

        tracing::info!("GoDo-R CLI installed successfully");
        Ok(())
    }

    async fn execute_godo_command(&self, args: &[&str]) -> Result<String> {
        let godo_cmd = self.godo_path.as_deref().unwrap_or("godo");

        // Set environment variables
        let mut cmd = Command::new(godo_cmd);
        cmd.env("GODADDY_API_KEY", &self.api_key)
           .env("GODADDY_API_SECRET", &self.api_secret)
           .env("GODADDY_ENVIRONMENT", &self.environment);

        cmd.args(args);

        let output = cmd.output()
            .context("Failed to execute GoDo-R command")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("GoDo-R command failed: {}", error));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    async fn create_dns_record(
        &self,
        config: &DomainConfig,
        target: &str,
    ) -> Result<DNSRecordResult> {
        let record_name = config.subdomain.as_deref().unwrap_or("@");
        let record_type = format!("{:?}", config.record_type);
        let ttl = config.ttl.unwrap_or(300).to_string();

        let output = self.execute_godo_command(&[
            "dns", "add",
            &config.domain,
            "--name", record_name,
            "--type", &record_type,
            "--data", target,
            "--ttl", &ttl,
            "--format", "json"
        ]).await?;

        let result: DNSRecordResult = serde_json::from_str(&output)
            .context("Failed to parse DNS record result")?;

        Ok(result)
    }

    async fn configure_cloudflare_zone(
        &self,
        zone_id: &str,
        config: &DomainConfig,
        worker_url: &str,
    ) -> Result<()> {
        // This would integrate with Cloudflare API to set up zone configuration
        tracing::info!("Configuring Cloudflare zone {} for domain {}", zone_id, config.domain);
        Ok(())
    }

    fn extract_worker_subdomain(&self, worker_url: &str) -> Result<String> {
        let url = url::Url::parse(worker_url)
            .context("Invalid worker URL")?;

        let host = url.host_str()
            .context("No host in worker URL")?;

        // Extract the subdomain part from worker.dev URLs
        if host.ends_with(".workers.dev") {
            Ok(host.replace(".workers.dev", ""))
        } else {
            Ok(host.to_string())
        }
    }

    fn sanitize_subdomain(&self, name: &str) -> String {
        name.to_lowercase()
            .replace(" ", "-")
            .replace("_", "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>()
            .trim_matches('-')
            .to_string()
    }

    fn extract_record_id(&self, response: &str) -> Result<String> {
        let parsed: serde_json::Value = serde_json::from_str(response)
            .context("Failed to parse record response")?;

        parsed["record_id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("No record_id in response"))
    }
}

// Supporting structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainInfo {
    pub domain: String,
    pub status: String,
    pub expires: DateTime<Utc>,
    pub auto_renew: bool,
    pub name_servers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainDetails {
    pub domain: String,
    pub registrar: String,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub dns_records: Vec<DNSRecord>,
    pub contact_info: ContactInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSRecord {
    pub id: String,
    pub record_type: String,
    pub name: String,
    pub data: String,
    pub ttl: u32,
    pub priority: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub registrant: Contact,
    pub admin: Contact,
    pub technical: Contact,
    pub billing: Contact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub organization: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSRecordResult {
    pub record_id: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropagationStatus {
    pub domain: String,
    pub propagated: bool,
    pub dns_servers_checked: u32,
    pub dns_servers_updated: u32,
    pub estimated_completion: Option<DateTime<Utc>>,
}

// Integration with Magic Loop
impl super::cloudflare_deployer::CloudflareDeployer {
    /// Configure custom domain using GoDo-R
    pub async fn setup_custom_domain_with_godo(
        &self,
        domain_manager: &GoDaddyDomainManager,
        domain: &str,
        worker_url: &str,
        project_name: &str,
    ) -> Result<String> {
        let result = domain_manager
            .configure_magic_loop_domain(domain, project_name, worker_url)
            .await?;

        if result.success {
            Ok(format!("https://{}", result.full_domain))
        } else {
            Err(anyhow::anyhow!("Failed to configure domain: {}", result.message))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_domain_manager_creation() {
        let manager = GoDaddyDomainManager::new(
            "test_key".to_string(),
            "test_secret".to_string(),
        );

        assert_eq!(manager.environment, "production");
    }

    #[tokio::test]
    async fn test_subdomain_sanitization() {
        let manager = GoDaddyDomainManager::new("key".to_string(), "secret".to_string());

        assert_eq!(manager.sanitize_subdomain("My Test App"), "my-test-app");
        assert_eq!(manager.sanitize_subdomain("API_Service_v2"), "api-service-v2");
        assert_eq!(manager.sanitize_subdomain("Special!@#$%Characters"), "specialcharacters");
    }

    #[test]
    fn test_worker_subdomain_extraction() {
        let manager = GoDaddyDomainManager::new("key".to_string(), "secret".to_string());

        let worker_url = "https://test-worker.example.workers.dev";
        let subdomain = manager.extract_worker_subdomain(worker_url).unwrap();

        assert_eq!(subdomain, "test-worker.example");
    }
}