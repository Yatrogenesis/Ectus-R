//! # Performance Monitoring
//!
//! Performance metrics and monitoring for the AI engine.

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Performance metrics for the AI engine
#[derive(Debug)]
pub struct PerformanceMetrics {
    /// Total number of inferences
    total_inferences: AtomicU64,
    /// Total successful inferences
    successful_inferences: AtomicU64,
    /// Total failed inferences
    failed_inferences: AtomicU64,
    /// Total inference time in milliseconds
    total_inference_time_ms: AtomicU64,
    /// Total memory used
    total_memory_used: AtomicUsize,
    /// Per-model metrics
    model_metrics: Arc<DashMap<String, ModelMetrics>>,
    /// Start time for uptime calculation
    start_time: Instant,
}

#[derive(Debug, Default)]
struct ModelMetrics {
    total_inferences: AtomicU64,
    successful_inferences: AtomicU64,
    failed_inferences: AtomicU64,
    total_time_ms: AtomicU64,
    total_memory: AtomicUsize,
    min_time_ms: AtomicU64,
    max_time_ms: AtomicU64,
}

/// Performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    /// Total number of inferences
    pub total_inferences: u64,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Average inference time in milliseconds
    pub avg_inference_time_ms: f64,
    /// Average memory usage per inference
    pub avg_memory_usage: usize,
    /// Inferences per second (throughput)
    pub inferences_per_second: f64,
    /// Total uptime in seconds
    pub uptime_seconds: u64,
    /// Per-model statistics
    pub model_stats: std::collections::HashMap<String, ModelStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelStats {
    /// Number of inferences for this model
    pub total_inferences: u64,
    /// Success rate for this model
    pub success_rate: f64,
    /// Average inference time for this model
    pub avg_time_ms: f64,
    /// Minimum inference time
    pub min_time_ms: u64,
    /// Maximum inference time
    pub max_time_ms: u64,
    /// Average memory usage for this model
    pub avg_memory_usage: usize,
}

impl PerformanceMetrics {
    /// Create new performance metrics tracker
    pub fn new() -> Self {
        Self {
            total_inferences: AtomicU64::new(0),
            successful_inferences: AtomicU64::new(0),
            failed_inferences: AtomicU64::new(0),
            total_inference_time_ms: AtomicU64::new(0),
            total_memory_used: AtomicUsize::new(0),
            model_metrics: Arc::new(DashMap::new()),
            start_time: Instant::now(),
        }
    }

    /// Record an inference
    pub fn record_inference(
        &self,
        model: &str,
        duration: Duration,
        memory_usage: usize,
        success: bool,
    ) {
        let duration_ms = duration.as_millis() as u64;

        // Update global metrics
        self.total_inferences.fetch_add(1, Ordering::Relaxed);
        self.total_inference_time_ms
            .fetch_add(duration_ms, Ordering::Relaxed);
        self.total_memory_used
            .fetch_add(memory_usage, Ordering::Relaxed);

        if success {
            self.successful_inferences.fetch_add(1, Ordering::Relaxed);
        } else {
            self.failed_inferences.fetch_add(1, Ordering::Relaxed);
        }

        // Update model-specific metrics
        let model_metrics = self
            .model_metrics
            .entry(model.to_string())
            .or_insert_with(ModelMetrics::default);

        model_metrics
            .total_inferences
            .fetch_add(1, Ordering::Relaxed);
        model_metrics
            .total_time_ms
            .fetch_add(duration_ms, Ordering::Relaxed);
        model_metrics
            .total_memory
            .fetch_add(memory_usage, Ordering::Relaxed);

        if success {
            model_metrics
                .successful_inferences
                .fetch_add(1, Ordering::Relaxed);
        } else {
            model_metrics
                .failed_inferences
                .fetch_add(1, Ordering::Relaxed);
        }

        // Update min/max times
        loop {
            let current_min = model_metrics.min_time_ms.load(Ordering::Relaxed);
            if current_min == 0 || duration_ms < current_min {
                if model_metrics
                    .min_time_ms
                    .compare_exchange_weak(current_min, duration_ms, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    break;
                }
            } else {
                break;
            }
        }

        loop {
            let current_max = model_metrics.max_time_ms.load(Ordering::Relaxed);
            if duration_ms > current_max {
                if model_metrics
                    .max_time_ms
                    .compare_exchange_weak(current_max, duration_ms, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    break;
                }
            } else {
                break;
            }
        }
    }

    /// Get current performance statistics
    pub fn get_stats(&self) -> PerformanceStats {
        let total_inferences = self.total_inferences.load(Ordering::Relaxed);
        let successful_inferences = self.successful_inferences.load(Ordering::Relaxed);
        let total_time_ms = self.total_inference_time_ms.load(Ordering::Relaxed);
        let total_memory = self.total_memory_used.load(Ordering::Relaxed);
        let uptime = self.start_time.elapsed();

        let success_rate = if total_inferences > 0 {
            successful_inferences as f64 / total_inferences as f64
        } else {
            0.0
        };

        let avg_inference_time_ms = if total_inferences > 0 {
            total_time_ms as f64 / total_inferences as f64
        } else {
            0.0
        };

        let avg_memory_usage = if total_inferences > 0 {
            total_memory / total_inferences as usize
        } else {
            0
        };

        let inferences_per_second = if uptime.as_secs() > 0 {
            total_inferences as f64 / uptime.as_secs() as f64
        } else {
            0.0
        };

        let mut model_stats = std::collections::HashMap::new();
        for entry in self.model_metrics.iter() {
            let model_name = entry.key().clone();
            let metrics = entry.value();

            let model_total = metrics.total_inferences.load(Ordering::Relaxed);
            let model_successful = metrics.successful_inferences.load(Ordering::Relaxed);
            let model_time = metrics.total_time_ms.load(Ordering::Relaxed);
            let model_memory = metrics.total_memory.load(Ordering::Relaxed);

            model_stats.insert(
                model_name,
                ModelStats {
                    total_inferences: model_total,
                    success_rate: if model_total > 0 {
                        model_successful as f64 / model_total as f64
                    } else {
                        0.0
                    },
                    avg_time_ms: if model_total > 0 {
                        model_time as f64 / model_total as f64
                    } else {
                        0.0
                    },
                    min_time_ms: metrics.min_time_ms.load(Ordering::Relaxed),
                    max_time_ms: metrics.max_time_ms.load(Ordering::Relaxed),
                    avg_memory_usage: if model_total > 0 {
                        model_memory / model_total as usize
                    } else {
                        0
                    },
                },
            );
        }

        PerformanceStats {
            total_inferences,
            success_rate,
            avg_inference_time_ms,
            avg_memory_usage,
            inferences_per_second,
            uptime_seconds: uptime.as_secs(),
            model_stats,
        }
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.total_inferences.store(0, Ordering::Relaxed);
        self.successful_inferences.store(0, Ordering::Relaxed);
        self.failed_inferences.store(0, Ordering::Relaxed);
        self.total_inference_time_ms.store(0, Ordering::Relaxed);
        self.total_memory_used.store(0, Ordering::Relaxed);
        self.model_metrics.clear();
    }

    /// Get throughput metrics for the last N seconds
    pub fn get_recent_throughput(&self, seconds: u64) -> f64 {
        // This is a simplified implementation
        // In a real system, you'd want to track time-windowed metrics
        let uptime = self.start_time.elapsed();
        let recent_window = std::cmp::min(uptime.as_secs(), seconds);

        if recent_window > 0 {
            let total_inferences = self.total_inferences.load(Ordering::Relaxed);
            total_inferences as f64 / recent_window as f64
        } else {
            0.0
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}