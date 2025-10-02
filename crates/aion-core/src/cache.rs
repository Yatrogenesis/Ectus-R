use crate::PlatformConfig;
use anyhow::Result;
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_capacity: u64,
    pub time_to_live: Duration,
    pub time_to_idle: Duration,
}

pub struct CacheManager {
    config: Arc<PlatformConfig>,
    cache_config: CacheConfig,
    primary_cache: Cache<String, CacheValue>,
    session_cache: Cache<String, SessionData>,
    user_cache: Cache<uuid::Uuid, UserData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheValue {
    String(String),
    Json(serde_json::Value),
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: uuid::Uuid,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub user_id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub last_login: chrono::DateTime<chrono::Utc>,
}

impl CacheManager {
    pub async fn new(config: Arc<PlatformConfig>) -> Result<Self> {
        let cache_config = CacheConfig {
            max_capacity: 10000,
            time_to_live: Duration::from_secs(3600), // 1 hour
            time_to_idle: Duration::from_secs(1800), // 30 minutes
        };

        let primary_cache = Cache::builder()
            .max_capacity(cache_config.max_capacity)
            .time_to_live(cache_config.time_to_live)
            .time_to_idle(cache_config.time_to_idle)
            .build();

        let session_cache = Cache::builder()
            .max_capacity(1000)
            .time_to_live(Duration::from_secs(86400)) // 24 hours
            .time_to_idle(Duration::from_secs(3600))  // 1 hour
            .build();

        let user_cache = Cache::builder()
            .max_capacity(5000)
            .time_to_live(Duration::from_secs(3600))  // 1 hour
            .time_to_idle(Duration::from_secs(1800))  // 30 minutes
            .build();

        Ok(Self {
            config,
            cache_config,
            primary_cache,
            session_cache,
            user_cache,
        })
    }

    // Primary cache operations
    pub async fn set(&self, key: String, value: CacheValue) {
        self.primary_cache.insert(key, value).await;
    }

    pub async fn get(&self, key: &str) -> Option<CacheValue> {
        self.primary_cache.get(key).await
    }

    pub async fn remove(&self, key: &str) {
        self.primary_cache.invalidate(key).await;
    }

    // Session cache operations
    pub async fn set_session(&self, session_id: String, session_data: SessionData) {
        self.session_cache.insert(session_id, session_data).await;
    }

    pub async fn get_session(&self, session_id: &str) -> Option<SessionData> {
        self.session_cache.get(session_id).await
    }

    pub async fn remove_session(&self, session_id: &str) {
        self.session_cache.invalidate(session_id).await;
    }

    // User cache operations
    pub async fn set_user(&self, user_id: uuid::Uuid, user_data: UserData) {
        self.user_cache.insert(user_id, user_data).await;
    }

    pub async fn get_user(&self, user_id: uuid::Uuid) -> Option<UserData> {
        self.user_cache.get(&user_id).await
    }

    pub async fn remove_user(&self, user_id: uuid::Uuid) {
        self.user_cache.invalidate(&user_id).await;
    }

    // Cache statistics
    pub async fn get_stats(&self) -> CacheStats {
        CacheStats {
            primary_cache: CacheStat {
                entry_count: self.primary_cache.entry_count(),
                weighted_size: self.primary_cache.weighted_size(),
            },
            session_cache: CacheStat {
                entry_count: self.session_cache.entry_count(),
                weighted_size: self.session_cache.weighted_size(),
            },
            user_cache: CacheStat {
                entry_count: self.user_cache.entry_count(),
                weighted_size: self.user_cache.weighted_size(),
            },
        }
    }

    // Cache management
    pub async fn clear_all(&self) {
        self.primary_cache.invalidate_all();
        self.session_cache.invalidate_all();
        self.user_cache.invalidate_all();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub primary_cache: CacheStat,
    pub session_cache: CacheStat,
    pub user_cache: CacheStat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStat {
    pub entry_count: u64,
    pub weighted_size: u64,
}