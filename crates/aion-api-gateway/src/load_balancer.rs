use crate::gateway::UpstreamService;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    Random,
}

pub struct LoadBalancer {
    services: Arc<RwLock<HashMap<String, Vec<UpstreamInstance>>>>,
    algorithm: LoadBalancingAlgorithm,
    round_robin_counters: Arc<RwLock<HashMap<String, AtomicUsize>>>,
}

#[derive(Debug, Clone)]
pub struct UpstreamInstance {
    pub url: String,
    pub weight: u32,
    pub healthy: bool,
    pub active_connections: AtomicUsize,
}

impl LoadBalancer {
    pub async fn new(upstream_services: Vec<UpstreamService>) -> Result<Self> {
        let mut services = HashMap::new();
        let mut counters = HashMap::new();

        for service in upstream_services {
            let instance = UpstreamInstance {
                url: service.base_url,
                weight: service.weight,
                healthy: true,
                active_connections: AtomicUsize::new(0),
            };

            services.insert(service.name.clone(), vec![instance]);
            counters.insert(service.name, AtomicUsize::new(0));
        }

        Ok(Self {
            services: Arc::new(RwLock::new(services)),
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            round_robin_counters: Arc::new(RwLock::new(counters)),
        })
    }

    pub async fn get_upstream(&self, service_name: &str) -> Result<String> {
        let services = self.services.read().await;
        let instances = services
            .get(service_name)
            .ok_or_else(|| anyhow::anyhow!("Service not found: {}", service_name))?;

        let healthy_instances: Vec<&UpstreamInstance> = instances
            .iter()
            .filter(|instance| instance.healthy)
            .collect();

        if healthy_instances.is_empty() {
            return Err(anyhow::anyhow!("No healthy instances for service: {}", service_name));
        }

        let selected_instance = match self.algorithm {
            LoadBalancingAlgorithm::RoundRobin => {
                self.round_robin_select(service_name, &healthy_instances).await
            }
            LoadBalancingAlgorithm::WeightedRoundRobin => {
                self.weighted_round_robin_select(&healthy_instances)
            }
            LoadBalancingAlgorithm::LeastConnections => {
                self.least_connections_select(&healthy_instances)
            }
            LoadBalancingAlgorithm::Random => {
                self.random_select(&healthy_instances)
            }
        };

        Ok(selected_instance.url.clone())
    }

    async fn round_robin_select(&self, service_name: &str, instances: &[&UpstreamInstance]) -> &UpstreamInstance {
        let counters = self.round_robin_counters.read().await;
        if let Some(counter) = counters.get(service_name) {
            let index = counter.fetch_add(1, Ordering::Relaxed) % instances.len();
            instances[index]
        } else {
            instances[0]
        }
    }

    fn weighted_round_robin_select(&self, instances: &[&UpstreamInstance]) -> &UpstreamInstance {
        // Simplified weighted selection - in production, use proper weighted round-robin
        let total_weight: u32 = instances.iter().map(|i| i.weight).sum();
        let random_weight = fastrand::u32(1..=total_weight);

        let mut cumulative_weight = 0;
        for instance in instances {
            cumulative_weight += instance.weight;
            if random_weight <= cumulative_weight {
                return instance;
            }
        }

        instances[0]
    }

    fn least_connections_select(&self, instances: &[&UpstreamInstance]) -> &UpstreamInstance {
        instances
            .iter()
            .min_by_key(|instance| instance.active_connections.load(Ordering::Relaxed))
            .copied()
            .unwrap_or(instances[0])
    }

    fn random_select(&self, instances: &[&UpstreamInstance]) -> &UpstreamInstance {
        let index = fastrand::usize(0..instances.len());
        instances[index]
    }

    pub async fn mark_instance_unhealthy(&self, service_name: &str, instance_url: &str) {
        let mut services = self.services.write().await;
        if let Some(instances) = services.get_mut(service_name) {
            for instance in instances.iter_mut() {
                if instance.url == instance_url {
                    instance.healthy = false;
                    tracing::warn!("Marked instance unhealthy: {} -> {}", service_name, instance_url);
                    break;
                }
            }
        }
    }

    pub async fn mark_instance_healthy(&self, service_name: &str, instance_url: &str) {
        let mut services = self.services.write().await;
        if let Some(instances) = services.get_mut(service_name) {
            for instance in instances.iter_mut() {
                if instance.url == instance_url {
                    instance.healthy = true;
                    tracing::info!("Marked instance healthy: {} -> {}", service_name, instance_url);
                    break;
                }
            }
        }
    }
}