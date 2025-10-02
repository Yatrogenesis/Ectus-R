use crate::gateway::{GatewayConfig, UpstreamHealth};
use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

pub struct HealthChecker {
    config: Arc<GatewayConfig>,
    client: Client,
    upstream_health: Arc<RwLock<Vec<UpstreamHealth>>>,
    monitoring_active: Arc<RwLock<bool>>,
}

impl HealthChecker {
    pub async fn new(config: Arc<GatewayConfig>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Self {
            config,
            client,
            upstream_health: Arc::new(RwLock::new(Vec::new())),
            monitoring_active: Arc::new(RwLock::new(false)),
        })
    }

    pub async fn start_monitoring(&self) -> Result<()> {
        {
            let mut active = self.monitoring_active.write().await;
            *active = true;
        }

        let health_checker = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(health_checker.config.health_check_interval_seconds));

            loop {
                interval.tick().await;

                let active = *health_checker.monitoring_active.read().await;
                if !active {
                    break;
                }

                if let Err(e) = health_checker.check_all_upstreams().await {
                    tracing::error!("Health check failed: {}", e);
                }
            }
        });

        tracing::info!("Health monitoring started for {} upstream services", self.config.upstream_services.len());
        Ok(())
    }

    pub async fn stop_monitoring(&self) -> Result<()> {
        let mut active = self.monitoring_active.write().await;
        *active = false;
        tracing::info!("Health monitoring stopped");
        Ok(())
    }

    async fn check_all_upstreams(&self) -> Result<()> {
        let mut health_results = Vec::new();

        for service in &self.config.upstream_services {
            let health = self.check_upstream_health(service).await;
            health_results.push(health);
        }

        // Update health status
        let mut upstream_health = self.upstream_health.write().await;
        *upstream_health = health_results;

        Ok(())
    }

    async fn check_upstream_health(&self, service: &crate::gateway::UpstreamService) -> UpstreamHealth {
        let health_url = format!("{}{}", service.base_url.trim_end_matches('/'), &service.health_check_path);
        let start_time = Instant::now();

        match self.client.get(&health_url).send().await {
            Ok(response) => {
                let response_time = start_time.elapsed().as_millis() as u64;
                let status = if response.status().is_success() {
                    "healthy".to_string()
                } else {
                    format!("unhealthy (HTTP {})", response.status())
                };

                UpstreamHealth {
                    service_name: service.name.clone(),
                    url: service.base_url.clone(),
                    status,
                    response_time_ms: response_time,
                    last_check: Utc::now(),
                }
            }
            Err(e) => {
                let response_time = start_time.elapsed().as_millis() as u64;
                UpstreamHealth {
                    service_name: service.name.clone(),
                    url: service.base_url.clone(),
                    status: format!("unhealthy ({})", e),
                    response_time_ms: response_time,
                    last_check: Utc::now(),
                }
            }
        }
    }

    pub async fn check_gateway_health(&self) -> GatewayHealthStatus {
        let upstream_health = self.upstream_health.read().await;
        let healthy_count = upstream_health.iter().filter(|h| h.status == "healthy").count();
        let total_count = upstream_health.len();

        let overall_status = if total_count == 0 {
            "unknown".to_string()
        } else if healthy_count == total_count {
            "healthy".to_string()
        } else if healthy_count > 0 {
            "degraded".to_string()
        } else {
            "unhealthy".to_string()
        };

        GatewayHealthStatus {
            status: overall_status,
            upstream_services: upstream_health.clone(),
            healthy_upstreams: healthy_count,
            total_upstreams: total_count,
            last_check: Utc::now(),
        }
    }

    pub async fn get_upstream_health(&self) -> Vec<UpstreamHealth> {
        let upstream_health = self.upstream_health.read().await;
        upstream_health.clone()
    }
}

impl Clone for HealthChecker {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            client: self.client.clone(),
            upstream_health: self.upstream_health.clone(),
            monitoring_active: self.monitoring_active.clone(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GatewayHealthStatus {
    pub status: String,
    pub upstream_services: Vec<UpstreamHealth>,
    pub healthy_upstreams: usize,
    pub total_upstreams: usize,
    pub last_check: chrono::DateTime<chrono::Utc>,
}