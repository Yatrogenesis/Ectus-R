//! Metrics Collection and Aggregation
//!
//! High-performance metrics collection system with real-time aggregation

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use tracing::{info, warn, error, debug};

use crate::OptimizationConfig;

/// High-performance metrics collector
#[derive(Debug)]
pub struct MetricsCollector {
    config: OptimizationConfig,
    performance_score_calculator: Arc<RwLock<PerformanceScoreCalculator>>,
    is_running: Arc<RwLock<bool>>,
}

/// Performance score calculation engine
#[derive(Debug, Default)]
pub struct PerformanceScoreCalculator {
    pub weights: ScoreWeights,
    pub baseline_metrics: Option<BaselineMetrics>,
    pub current_metrics: Option<CurrentMetrics>,
    pub score_history: Vec<ScoreHistoryPoint>,
}

/// Weights for performance score calculation
#[derive(Debug)]
pub struct ScoreWeights {
    pub response_time_weight: f64,
    pub throughput_weight: f64,
    pub error_rate_weight: f64,
    pub resource_utilization_weight: f64,
    pub availability_weight: f64,
}

impl Default for ScoreWeights {
    fn default() -> Self {
        Self {
            response_time_weight: 0.25,
            throughput_weight: 0.25,
            error_rate_weight: 0.20,
            resource_utilization_weight: 0.20,
            availability_weight: 0.10,
        }
    }
}

/// Baseline metrics for comparison
#[derive(Debug, Clone)]
pub struct BaselineMetrics {
    pub response_time: f64,
    pub throughput: f64,
    pub error_rate: f64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub availability: f64,
    pub established_at: DateTime<Utc>,
}

/// Current system metrics
#[derive(Debug, Clone)]
pub struct CurrentMetrics {
    pub response_time: f64,
    pub throughput: f64,
    pub error_rate: f64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub availability: f64,
    pub measured_at: DateTime<Utc>,
}

/// Score history point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreHistoryPoint {
    pub timestamp: DateTime<Utc>,
    pub overall_score: f64,
    pub component_scores: ComponentScores,
}

/// Component scores breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentScores {
    pub response_time_score: f64,
    pub throughput_score: f64,
    pub error_rate_score: f64,
    pub resource_utilization_score: f64,
    pub availability_score: f64,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub async fn new(config: &OptimizationConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            performance_score_calculator: Arc::new(RwLock::new(PerformanceScoreCalculator::default())),
            is_running: Arc::new(RwLock::new(false)),
        })
    }

    /// Start the metrics collector
    pub async fn start(&mut self) -> Result<()> {
        *self.is_running.write().await = true;
        info!("Metrics Collector started");

        // Start performance score calculation
        self.start_score_calculation().await?;

        Ok(())
    }

    /// Stop the metrics collector
    pub async fn stop(&mut self) -> Result<()> {
        *self.is_running.write().await = false;
        info!("Metrics Collector stopped");
        Ok(())
    }

    /// Get current performance score
    pub async fn get_performance_score(&self) -> Result<f64> {
        let calculator = self.performance_score_calculator.read().await;

        if let Some(current) = &calculator.current_metrics {
            Ok(self.calculate_performance_score(&calculator, current).await?)
        } else {
            Ok(0.5) // Default neutral score
        }
    }

    /// Update current metrics
    pub async fn update_metrics(&self, metrics: CurrentMetrics) -> Result<()> {
        let mut calculator = self.performance_score_calculator.write().await;

        // Calculate performance score
        let score = self.calculate_performance_score(&calculator, &metrics).await?;

        // Store current metrics
        calculator.current_metrics = Some(metrics.clone());

        // Add to history
        let score_point = ScoreHistoryPoint {
            timestamp: metrics.measured_at,
            overall_score: score,
            component_scores: self.calculate_component_scores(&calculator, &metrics).await?,
        };

        calculator.score_history.push(score_point);

        // Keep limited history
        if calculator.score_history.len() > 1000 {
            calculator.score_history.remove(0);
        }

        Ok(())
    }

    /// Set baseline metrics
    pub async fn set_baseline(&self, metrics: BaselineMetrics) -> Result<()> {
        let mut calculator = self.performance_score_calculator.write().await;
        calculator.baseline_metrics = Some(metrics);
        info!("Baseline metrics established");
        Ok(())
    }

    // Private implementation methods

    async fn start_score_calculation(&self) -> Result<()> {
        let collector = self.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                tokio::time::Duration::from_secs(collector.config.metrics_collection_interval)
            );

            loop {
                interval.tick().await;
                if *collector.is_running.read().await {
                    if let Err(e) = collector.collect_and_calculate().await {
                        error!("Failed to collect and calculate metrics: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    async fn collect_and_calculate(&self) -> Result<()> {
        // Collect current system metrics (simplified)
        let current_metrics = CurrentMetrics {
            response_time: 150.0,    // Would collect from actual monitoring
            throughput: 1200.0,      // Requests per second
            error_rate: 0.01,        // 1% error rate
            cpu_usage: 65.0,         // 65% CPU usage
            memory_usage: 70.0,      // 70% memory usage
            availability: 99.9,      // 99.9% availability
            measured_at: Utc::now(),
        };

        self.update_metrics(current_metrics).await?;
        Ok(())
    }

    async fn calculate_performance_score(&self, calculator: &PerformanceScoreCalculator, current: &CurrentMetrics) -> Result<f64> {
        let baseline = match &calculator.baseline_metrics {
            Some(baseline) => baseline,
            None => {
                // If no baseline, return neutral score
                return Ok(0.7); // Assume decent performance
            }
        };

        let weights = &calculator.weights;

        // Calculate individual component scores (0.0 to 1.0, higher is better)
        let response_time_score = self.calculate_response_time_score(current.response_time, baseline.response_time);
        let throughput_score = self.calculate_throughput_score(current.throughput, baseline.throughput);
        let error_rate_score = self.calculate_error_rate_score(current.error_rate, baseline.error_rate);
        let resource_score = self.calculate_resource_score(current.cpu_usage, current.memory_usage, baseline.cpu_usage, baseline.memory_usage);
        let availability_score = self.calculate_availability_score(current.availability, baseline.availability);

        // Calculate weighted overall score
        let overall_score = response_time_score * weights.response_time_weight
            + throughput_score * weights.throughput_weight
            + error_rate_score * weights.error_rate_weight
            + resource_score * weights.resource_utilization_weight
            + availability_score * weights.availability_weight;

        Ok(overall_score.clamp(0.0, 1.0))
    }

    async fn calculate_component_scores(&self, calculator: &PerformanceScoreCalculator, current: &CurrentMetrics) -> Result<ComponentScores> {
        let baseline = match &calculator.baseline_metrics {
            Some(baseline) => baseline,
            None => {
                return Ok(ComponentScores {
                    response_time_score: 0.7,
                    throughput_score: 0.7,
                    error_rate_score: 0.7,
                    resource_utilization_score: 0.7,
                    availability_score: 0.7,
                });
            }
        };

        Ok(ComponentScores {
            response_time_score: self.calculate_response_time_score(current.response_time, baseline.response_time),
            throughput_score: self.calculate_throughput_score(current.throughput, baseline.throughput),
            error_rate_score: self.calculate_error_rate_score(current.error_rate, baseline.error_rate),
            resource_utilization_score: self.calculate_resource_score(current.cpu_usage, current.memory_usage, baseline.cpu_usage, baseline.memory_usage),
            availability_score: self.calculate_availability_score(current.availability, baseline.availability),
        })
    }

    fn calculate_response_time_score(&self, current: f64, baseline: f64) -> f64 {
        // Lower response time is better
        if current <= baseline {
            1.0 // Perfect score if current is better than or equal to baseline
        } else {
            let ratio = baseline / current;
            ratio.clamp(0.0, 1.0)
        }
    }

    fn calculate_throughput_score(&self, current: f64, baseline: f64) -> f64 {
        // Higher throughput is better
        if current >= baseline {
            1.0 // Perfect score if current is better than or equal to baseline
        } else {
            let ratio = current / baseline;
            ratio.clamp(0.0, 1.0)
        }
    }

    fn calculate_error_rate_score(&self, current: f64, baseline: f64) -> f64 {
        // Lower error rate is better
        if current <= baseline {
            1.0 // Perfect score if current is better than or equal to baseline
        } else {
            let ratio = baseline / current;
            ratio.clamp(0.0, 1.0)
        }
    }

    fn calculate_resource_score(&self, current_cpu: f64, current_memory: f64, baseline_cpu: f64, baseline_memory: f64) -> f64 {
        // Lower resource usage is better (with some considerations for utilization efficiency)
        let cpu_score = if current_cpu <= baseline_cpu {
            1.0
        } else {
            (baseline_cpu / current_cpu).clamp(0.0, 1.0)
        };

        let memory_score = if current_memory <= baseline_memory {
            1.0
        } else {
            (baseline_memory / current_memory).clamp(0.0, 1.0)
        };

        // Average of CPU and memory scores
        (cpu_score + memory_score) / 2.0
    }

    fn calculate_availability_score(&self, current: f64, baseline: f64) -> f64 {
        // Higher availability is better
        if current >= baseline {
            1.0
        } else {
            let ratio = current / baseline;
            ratio.clamp(0.0, 1.0)
        }
    }
}

// Clone implementation for background tasks
impl Clone for MetricsCollector {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            performance_score_calculator: Arc::clone(&self.performance_score_calculator),
            is_running: Arc::clone(&self.is_running),
        }
    }
}