use anyhow::Result;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircuitState {
    Closed = 0,   // Normal operation
    Open = 1,     // Circuit is open, failing fast
    HalfOpen = 2, // Testing if service recovered
}

pub struct CircuitBreaker {
    state: AtomicU8,
    failure_count: AtomicU64,
    success_count: AtomicU64,
    last_failure_time: Arc<RwLock<Option<Instant>>>,
    config: CircuitBreakerConfig,
}

#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u64,
    pub success_threshold: u64,
    pub timeout: Duration,
    pub reset_timeout: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 2,
            timeout: Duration::from_secs(60),
            reset_timeout: Duration::from_secs(30),
        }
    }
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: AtomicU8::new(CircuitState::Closed as u8),
            failure_count: AtomicU64::new(0),
            success_count: AtomicU64::new(0),
            last_failure_time: Arc::new(RwLock::new(None)),
            config,
        }
    }

    pub async fn call<F, Fut, T>(&self, operation: F) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        // Check if circuit should allow the call
        if !self.can_execute().await {
            return Err(anyhow::anyhow!("Circuit breaker is open"));
        }

        // Execute the operation
        match operation().await {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(error) => {
                self.on_failure().await;
                Err(error)
            }
        }
    }

    async fn can_execute(&self) -> bool {
        let state = CircuitState::from(self.state.load(Ordering::Relaxed));

        match state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if we should transition to half-open
                if let Some(last_failure) = *self.last_failure_time.read().await {
                    if last_failure.elapsed() >= self.config.reset_timeout {
                        self.transition_to_half_open().await;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }

    async fn on_success(&self) {
        let state = CircuitState::from(self.state.load(Ordering::Relaxed));

        match state {
            CircuitState::HalfOpen => {
                let success_count = self.success_count.fetch_add(1, Ordering::Relaxed) + 1;
                if success_count >= self.config.success_threshold {
                    self.transition_to_closed().await;
                }
            }
            CircuitState::Closed => {
                // Reset failure count on success
                self.failure_count.store(0, Ordering::Relaxed);
            }
            CircuitState::Open => {
                // Should not happen in normal flow
            }
        }
    }

    async fn on_failure(&self) {
        let state = CircuitState::from(self.state.load(Ordering::Relaxed));

        match state {
            CircuitState::Closed => {
                let failure_count = self.failure_count.fetch_add(1, Ordering::Relaxed) + 1;
                if failure_count >= self.config.failure_threshold {
                    self.transition_to_open().await;
                }
            }
            CircuitState::HalfOpen => {
                self.transition_to_open().await;
            }
            CircuitState::Open => {
                // Already open, update failure time
                let mut last_failure = self.last_failure_time.write().await;
                *last_failure = Some(Instant::now());
            }
        }
    }

    async fn transition_to_closed(&self) {
        self.state.store(CircuitState::Closed as u8, Ordering::Relaxed);
        self.failure_count.store(0, Ordering::Relaxed);
        self.success_count.store(0, Ordering::Relaxed);
        tracing::info!("Circuit breaker transitioned to CLOSED");
    }

    async fn transition_to_open(&self) {
        self.state.store(CircuitState::Open as u8, Ordering::Relaxed);
        let mut last_failure = self.last_failure_time.write().await;
        *last_failure = Some(Instant::now());
        tracing::warn!("Circuit breaker transitioned to OPEN");
    }

    async fn transition_to_half_open(&self) {
        self.state.store(CircuitState::HalfOpen as u8, Ordering::Relaxed);
        self.success_count.store(0, Ordering::Relaxed);
        tracing::info!("Circuit breaker transitioned to HALF-OPEN");
    }

    pub fn get_state(&self) -> CircuitState {
        CircuitState::from(self.state.load(Ordering::Relaxed))
    }

    pub fn get_metrics(&self) -> CircuitBreakerMetrics {
        CircuitBreakerMetrics {
            state: self.get_state(),
            failure_count: self.failure_count.load(Ordering::Relaxed),
            success_count: self.success_count.load(Ordering::Relaxed),
        }
    }
}

impl From<u8> for CircuitState {
    fn from(value: u8) -> Self {
        match value {
            0 => CircuitState::Closed,
            1 => CircuitState::Open,
            2 => CircuitState::HalfOpen,
            _ => CircuitState::Closed,
        }
    }
}

#[derive(Debug)]
pub struct CircuitBreakerMetrics {
    pub state: CircuitState,
    pub failure_count: u64,
    pub success_count: u64,
}

pub struct CircuitBreakerManager {
    breakers: Arc<RwLock<HashMap<String, Arc<CircuitBreaker>>>>,
}

impl CircuitBreakerManager {
    pub fn new() -> Self {
        Self {
            breakers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_or_create(&self, name: &str, config: Option<CircuitBreakerConfig>) -> Arc<CircuitBreaker> {
        let breakers = self.breakers.read().await;
        if let Some(breaker) = breakers.get(name) {
            breaker.clone()
        } else {
            drop(breakers);

            let mut breakers = self.breakers.write().await;
            // Double-check in case another thread created it
            if let Some(breaker) = breakers.get(name) {
                breaker.clone()
            } else {
                let config = config.unwrap_or_default();
                let breaker = Arc::new(CircuitBreaker::new(config));
                breakers.insert(name.to_string(), breaker.clone());
                breaker
            }
        }
    }

    pub async fn get_all_metrics(&self) -> HashMap<String, CircuitBreakerMetrics> {
        let breakers = self.breakers.read().await;
        let mut metrics = HashMap::new();

        for (name, breaker) in breakers.iter() {
            metrics.insert(name.clone(), breaker.get_metrics());
        }

        metrics
    }
}