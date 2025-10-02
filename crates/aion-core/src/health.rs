use crate::{PlatformConfig, PlatformService, ServiceHealth, HealthStatus};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;

pub struct HealthChecker {
    config: Arc<PlatformConfig>,
    services: Arc<RwLock<HashMap<String, Arc<dyn PlatformService>>>>,
    health_cache: Arc<RwLock<HashMap<String, ServiceHealth>>>,
    monitoring_active: Arc<tokio::sync::RwLock<bool>>,
}

impl HealthChecker {
    pub async fn new(config: Arc<PlatformConfig>) -> Result<Self> {
        Ok(Self {
            config,
            services: Arc::new(RwLock::new(HashMap::new())),
            health_cache: Arc::new(RwLock::new(HashMap::new())),
            monitoring_active: Arc::new(tokio::sync::RwLock::new(false)),
        })
    }

    pub async fn register_service(
        &self,
        service_name: String,
        service: Arc<dyn PlatformService>,
    ) -> Result<()> {
        let mut services = self.services.write().await;
        services.insert(service_name, service);
        Ok(())
    }

    pub async fn start_monitoring(&self) -> Result<()> {
        {
            let mut active = self.monitoring_active.write().await;
            *active = true;
        }

        let health_checker = Arc::new(self.clone());

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

            loop {
                interval.tick().await;

                let active = *health_checker.monitoring_active.read().await;
                if !active {
                    break;
                }

                if let Err(e) = health_checker.check_all_services_internal().await {
                    tracing::error!("Health check failed: {}", e);
                }
            }
        });

        tracing::info!("Health monitoring started");
        Ok(())
    }

    pub async fn stop_monitoring(&self) -> Result<()> {
        let mut active = self.monitoring_active.write().await;
        *active = false;
        tracing::info!("Health monitoring stopped");
        Ok(())
    }

    pub async fn check_all_services(&self) -> Result<Vec<ServiceHealth>> {
        let services = self.services.read().await;
        let mut health_results = Vec::new();

        for (service_name, service) in services.iter() {
            match service.health_check().await {
                Ok(health) => health_results.push(health),
                Err(e) => {
                    let failed_health = ServiceHealth {
                        service_name: service_name.clone(),
                        status: HealthStatus::Critical,
                        uptime_seconds: 0,
                        last_check: Utc::now(),
                        metrics: serde_json::json!({
                            "error": e.to_string()
                        }),
                        dependencies: vec![],
                    };
                    health_results.push(failed_health);
                }
            }
        }

        // Update cache
        let mut cache = self.health_cache.write().await;
        cache.clear();
        for health in &health_results {
            cache.insert(health.service_name.clone(), health.clone());
        }

        Ok(health_results)
    }

    async fn check_all_services_internal(&self) -> Result<()> {
        let _ = self.check_all_services().await?;
        Ok(())
    }

    pub async fn get_service_health(&self, service_name: &str) -> Option<ServiceHealth> {
        let cache = self.health_cache.read().await;
        cache.get(service_name).cloned()
    }
}

impl Clone for HealthChecker {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            services: self.services.clone(),
            health_cache: self.health_cache.clone(),
            monitoring_active: self.monitoring_active.clone(),
        }
    }
}