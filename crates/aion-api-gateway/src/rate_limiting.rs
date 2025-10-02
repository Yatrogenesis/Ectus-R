use anyhow::Result;
use axum::extract::Request;
use governor::{Quota, RateLimiter as GovernorRateLimiter};
use std::collections::HashMap;
use std::net::IpAddr;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

pub struct RateLimiter {
    limiters: Arc<RwLock<HashMap<String, GovernorRateLimiter<String, dashmap::DashMap<String, governor::state::InMemoryState>, governor::clock::DefaultClock>>>>,
    default_quota: Quota,
}

impl RateLimiter {
    pub async fn new() -> Result<Self> {
        let default_quota = Quota::per_minute(NonZeroU32::new(100).unwrap()); // 100 requests per minute

        Ok(Self {
            limiters: Arc::new(RwLock::new(HashMap::new())),
            default_quota,
        })
    }

    pub async fn check_rate_limit(&self, request: &Request) -> Result<()> {
        let client_ip = self.extract_client_ip(request);
        let key = format!("global:{}", client_ip);

        let limiters = self.limiters.read().await;
        if let Some(limiter) = limiters.get("global") {
            match limiter.check_key(&key) {
                Ok(_) => Ok(()),
                Err(_) => Err(anyhow::anyhow!("Rate limit exceeded")),
            }
        } else {
            // If no limiter configured, allow the request
            Ok(())
        }
    }

    pub async fn configure_rate_limit(&self, name: &str, requests_per_minute: u32) -> Result<()> {
        let quota = Quota::per_minute(NonZeroU32::new(requests_per_minute).unwrap());
        let limiter = GovernorRateLimiter::dashmap(quota);

        let mut limiters = self.limiters.write().await;
        limiters.insert(name.to_string(), limiter);

        tracing::info!("Configured rate limit '{}': {} requests per minute", name, requests_per_minute);
        Ok(())
    }

    fn extract_client_ip(&self, request: &Request) -> String {
        // Try to get real IP from headers
        if let Some(forwarded_for) = request.headers().get("X-Forwarded-For") {
            if let Ok(forwarded_str) = forwarded_for.to_str() {
                if let Some(first_ip) = forwarded_str.split(',').next() {
                    return first_ip.trim().to_string();
                }
            }
        }

        if let Some(real_ip) = request.headers().get("X-Real-IP") {
            if let Ok(ip_str) = real_ip.to_str() {
                return ip_str.to_string();
            }
        }

        // Fallback to connection info (not available in this context)
        "unknown".to_string()
    }
}

#[derive(Debug)]
pub struct RateLimitConfig {
    pub name: String,
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub per_user: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            requests_per_minute: 100,
            burst_size: 10,
            per_user: false,
        }
    }
}