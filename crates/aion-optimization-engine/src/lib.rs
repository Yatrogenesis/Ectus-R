//! Advanced Optimization Engine with Machine Learning
//!
//! This module provides sophisticated optimization capabilities using machine learning
//! to predict and optimize system performance in real-time.

pub mod ml_optimizer;
pub mod predictive_analyzer;
pub mod auto_tuner;
pub mod telemetry;
pub mod recommendation_engine;
pub mod models;
pub mod metrics_collector;

pub use ml_optimizer::*;
pub use predictive_analyzer::*;
pub use auto_tuner::*;
pub use telemetry::*;
pub use recommendation_engine::*;
pub use models::*;
pub use metrics_collector::*;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Configuration for the optimization engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Enable ML-based optimization
    pub enable_ml_optimization: bool,

    /// Enable predictive analysis
    pub enable_predictive_analysis: bool,

    /// Enable auto-tuning
    pub enable_auto_tuning: bool,

    /// Collection interval for metrics (seconds)
    pub metrics_collection_interval: u64,

    /// Minimum data points required for ML training
    pub min_training_data_points: usize,

    /// Model retraining interval (hours)
    pub model_retrain_interval: u64,

    /// Performance threshold for optimization triggers
    pub performance_threshold: f64,

    /// Auto-tuning sensitivity (0.0 to 1.0)
    pub auto_tuning_sensitivity: f64,

    /// Maximum concurrent optimization tasks
    pub max_concurrent_optimizations: usize,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            enable_ml_optimization: true,
            enable_predictive_analysis: true,
            enable_auto_tuning: true,
            metrics_collection_interval: 30,
            min_training_data_points: 1000,
            model_retrain_interval: 24,
            performance_threshold: 0.8,
            auto_tuning_sensitivity: 0.7,
            max_concurrent_optimizations: 5,
        }
    }
}

/// Main optimization engine that coordinates all optimization subsystems
#[derive(Debug)]
pub struct OptimizationEngine {
    config: OptimizationConfig,
    ml_optimizer: MLOptimizer,
    predictive_analyzer: PredictiveAnalyzer,
    auto_tuner: AutoTuner,
    telemetry: TelemetryCollector,
    recommendation_engine: RecommendationEngine,
    metrics_collector: MetricsCollector,
}

impl OptimizationEngine {
    /// Create a new optimization engine instance
    pub async fn new(config: OptimizationConfig) -> Result<Self> {
        let ml_optimizer = MLOptimizer::new(&config).await?;
        let predictive_analyzer = PredictiveAnalyzer::new(&config).await?;
        let auto_tuner = AutoTuner::new(&config).await?;
        let telemetry = TelemetryCollector::new(&config).await?;
        let recommendation_engine = RecommendationEngine::new(&config).await?;
        let metrics_collector = MetricsCollector::new(&config).await?;

        Ok(Self {
            config,
            ml_optimizer,
            predictive_analyzer,
            auto_tuner,
            telemetry,
            recommendation_engine,
            metrics_collector,
        })
    }

    /// Start the optimization engine
    pub async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting optimization engine");

        // Start telemetry collection
        self.telemetry.start().await?;

        // Start metrics collection
        self.metrics_collector.start().await?;

        // Start predictive analysis if enabled
        if self.config.enable_predictive_analysis {
            self.predictive_analyzer.start().await?;
        }

        // Start auto-tuning if enabled
        if self.config.enable_auto_tuning {
            self.auto_tuner.start().await?;
        }

        // Start ML optimization if enabled
        if self.config.enable_ml_optimization {
            self.ml_optimizer.start().await?;
        }

        tracing::info!("Optimization engine started successfully");
        Ok(())
    }

    /// Stop the optimization engine
    pub async fn stop(&mut self) -> Result<()> {
        tracing::info!("Stopping optimization engine");

        self.ml_optimizer.stop().await?;
        self.auto_tuner.stop().await?;
        self.predictive_analyzer.stop().await?;
        self.metrics_collector.stop().await?;
        self.telemetry.stop().await?;

        tracing::info!("Optimization engine stopped");
        Ok(())
    }

    /// Get current optimization recommendations
    pub async fn get_recommendations(&self) -> Result<Vec<OptimizationRecommendation>> {
        self.recommendation_engine.get_recommendations().await
    }

    /// Get system performance predictions
    pub async fn get_performance_predictions(&self, horizon_minutes: u32) -> Result<PerformancePrediction> {
        self.predictive_analyzer.predict_performance(horizon_minutes).await
    }

    /// Apply optimization recommendations
    pub async fn apply_optimizations(&mut self, recommendations: &[OptimizationRecommendation]) -> Result<OptimizationResult> {
        self.auto_tuner.apply_optimizations(recommendations).await
    }

    /// Get optimization engine status
    pub async fn get_status(&self) -> Result<OptimizationStatus> {
        Ok(OptimizationStatus {
            ml_optimizer_status: self.ml_optimizer.get_status().await?,
            predictive_analyzer_status: self.predictive_analyzer.get_status().await?,
            auto_tuner_status: self.auto_tuner.get_status().await?,
            telemetry_status: self.telemetry.get_status().await?,
            total_optimizations_applied: self.auto_tuner.get_total_optimizations().await?,
            current_performance_score: self.metrics_collector.get_performance_score().await?,
        })
    }
}

/// Optimization engine status
#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationStatus {
    pub ml_optimizer_status: MLOptimizerStatus,
    pub predictive_analyzer_status: PredictiveAnalyzerStatus,
    pub auto_tuner_status: AutoTunerStatus,
    pub telemetry_status: TelemetryStatus,
    pub total_optimizations_applied: u64,
    pub current_performance_score: f64,
}

/// Optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    pub id: Uuid,
    pub category: OptimizationCategory,
    pub description: String,
    pub priority: OptimizationPriority,
    pub expected_impact: f64,
    pub confidence: f64,
    pub parameters: HashMap<String, OptimizationParameter>,
    pub created_at: DateTime<Utc>,
    pub estimated_implementation_time: u64, // seconds
}

/// Optimization categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationCategory {
    Database,
    Memory,
    CPU,
    Network,
    Cache,
    Application,
    Infrastructure,
}

/// Optimization priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
pub enum OptimizationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Optimization parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationParameter {
    pub name: String,
    pub current_value: String,
    pub recommended_value: String,
    pub parameter_type: ParameterType,
}

/// Parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    Integer,
    Float,
    String,
    Boolean,
    Duration,
    Size,
}

/// Optimization result
#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub recommendations_applied: Vec<Uuid>,
    pub recommendations_failed: Vec<(Uuid, String)>,
    pub performance_improvement: f64,
    pub application_time: DateTime<Utc>,
}

/// Performance prediction
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformancePrediction {
    pub predicted_response_time: f64,
    pub predicted_throughput: f64,
    pub predicted_error_rate: f64,
    pub predicted_resource_usage: ResourceUsagePrediction,
    pub confidence_interval: ConfidenceInterval,
    pub prediction_time: DateTime<Utc>,
    pub horizon_minutes: u32,
}

/// Resource usage prediction
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceUsagePrediction {
    pub cpu_percentage: f64,
    pub memory_percentage: f64,
    pub disk_io_rate: f64,
    pub network_io_rate: f64,
}

/// Confidence interval for predictions
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_optimization_engine_creation() {
        let config = OptimizationConfig::default();
        let result = OptimizationEngine::new(config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_optimization_config_default() {
        let config = OptimizationConfig::default();
        assert!(config.enable_ml_optimization);
        assert!(config.enable_predictive_analysis);
        assert!(config.enable_auto_tuning);
        assert_eq!(config.metrics_collection_interval, 30);
    }
}