//! Optimization Engine API Handlers
//! Provides endpoints for managing and monitoring the optimization engine

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::AppState;
use aion_optimization_engine::{OptimizationConfig, OptimizationMetrics, RecommendationRequest, OptimizationRecommendation};

/// Optimization status response
#[derive(Debug, Serialize)]
pub struct OptimizationStatusResponse {
    pub status: String,
    pub is_running: bool,
    pub uptime: String,
    pub last_optimization: Option<DateTime<Utc>>,
    pub performance_score: f64,
    pub active_experiments: u32,
    pub total_recommendations: u32,
}

/// Optimization metrics response
#[derive(Debug, Serialize)]
pub struct OptimizationMetricsResponse {
    pub current_score: f64,
    pub score_history: Vec<ScorePoint>,
    pub metrics: MetricsData,
    pub recommendations_applied: u32,
    pub performance_improvement: f64,
}

#[derive(Debug, Serialize)]
pub struct ScorePoint {
    pub timestamp: DateTime<Utc>,
    pub score: f64,
}

#[derive(Debug, Serialize)]
pub struct MetricsData {
    pub response_time: f64,
    pub throughput: f64,
    pub error_rate: f64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub availability: f64,
}

/// Configuration update request
#[derive(Debug, Deserialize)]
pub struct OptimizationConfigRequest {
    pub ml_enabled: Option<bool>,
    pub auto_tuning_enabled: Option<bool>,
    pub metrics_collection_interval: Option<u64>,
    pub recommendation_threshold: Option<f64>,
    pub max_concurrent_experiments: Option<u32>,
    pub safety_mode: Option<bool>,
}

/// Get optimization engine status
pub async fn get_optimization_status(
    State(state): State<AppState>,
) -> Result<Json<OptimizationStatusResponse>, StatusCode> {
    let engine = state.optimization_engine.read().await;

    let status = match engine.is_running().await {
        Ok(running) => {
            if running {
                "running".to_string()
            } else {
                "stopped".to_string()
            }
        }
        Err(_) => "error".to_string(),
    };

    let performance_score = engine.get_performance_score().await.unwrap_or(0.0);
    let uptime = engine.get_uptime().await.unwrap_or_default();
    let last_optimization = engine.get_last_optimization_time().await.ok();
    let active_experiments = engine.get_active_experiments_count().await.unwrap_or(0);
    let total_recommendations = engine.get_total_recommendations().await.unwrap_or(0);

    Ok(Json(OptimizationStatusResponse {
        status,
        is_running: status == "running",
        uptime,
        last_optimization,
        performance_score,
        active_experiments,
        total_recommendations,
    }))
}

/// Get optimization configuration
pub async fn get_optimization_config(
    State(state): State<AppState>,
) -> Result<Json<OptimizationConfig>, StatusCode> {
    let engine = state.optimization_engine.read().await;

    match engine.get_config().await {
        Ok(config) => Ok(Json(config)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Update optimization configuration
pub async fn update_optimization_config(
    State(state): State<AppState>,
    Json(request): Json<OptimizationConfigRequest>,
) -> Result<Json<OptimizationConfig>, StatusCode> {
    let mut engine = state.optimization_engine.write().await;

    let mut config = engine.get_config().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Update configuration fields if provided
    if let Some(ml_enabled) = request.ml_enabled {
        config.ml_enabled = ml_enabled;
    }
    if let Some(auto_tuning_enabled) = request.auto_tuning_enabled {
        config.auto_tuning_enabled = auto_tuning_enabled;
    }
    if let Some(interval) = request.metrics_collection_interval {
        config.metrics_collection_interval = interval;
    }
    if let Some(threshold) = request.recommendation_threshold {
        config.recommendation_threshold = threshold;
    }
    if let Some(max_experiments) = request.max_concurrent_experiments {
        config.max_concurrent_experiments = max_experiments;
    }
    if let Some(safety_mode) = request.safety_mode {
        config.safety_mode = safety_mode;
    }

    // Apply the updated configuration
    match engine.update_config(config.clone()).await {
        Ok(_) => Ok(Json(config)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Get optimization metrics
pub async fn get_optimization_metrics(
    State(state): State<AppState>,
) -> Result<Json<OptimizationMetricsResponse>, StatusCode> {
    let engine = state.optimization_engine.read().await;

    let current_score = engine.get_performance_score().await.unwrap_or(0.0);
    let score_history = engine.get_score_history().await
        .unwrap_or_default()
        .into_iter()
        .map(|point| ScorePoint {
            timestamp: point.timestamp,
            score: point.overall_score,
        })
        .collect();

    let metrics = engine.get_current_metrics().await
        .map(|m| MetricsData {
            response_time: m.response_time,
            throughput: m.throughput,
            error_rate: m.error_rate,
            cpu_usage: m.cpu_usage,
            memory_usage: m.memory_usage,
            availability: m.availability,
        })
        .unwrap_or(MetricsData {
            response_time: 0.0,
            throughput: 0.0,
            error_rate: 0.0,
            cpu_usage: 0.0,
            memory_usage: 0.0,
            availability: 0.0,
        });

    let recommendations_applied = engine.get_applied_recommendations_count().await.unwrap_or(0);
    let performance_improvement = engine.get_performance_improvement().await.unwrap_or(0.0);

    Ok(Json(OptimizationMetricsResponse {
        current_score,
        score_history,
        metrics,
        recommendations_applied,
        performance_improvement,
    }))
}

/// Get optimization recommendations
pub async fn get_optimization_recommendations(
    State(state): State<AppState>,
) -> Result<Json<Vec<OptimizationRecommendation>>, StatusCode> {
    let engine = state.optimization_engine.read().await;

    let request = RecommendationRequest {
        system_context: "web_api".to_string(),
        performance_data: HashMap::new(),
        constraints: Vec::new(),
        max_recommendations: 10,
    };

    match engine.get_recommendations(request).await {
        Ok(recommendations) => Ok(Json(recommendations)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Start optimization engine
pub async fn start_optimization(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut engine = state.optimization_engine.write().await;

    match engine.start().await {
        Ok(_) => Ok(Json(serde_json::json!({
            "status": "success",
            "message": "Optimization engine started successfully",
            "timestamp": Utc::now()
        }))),
        Err(e) => {
            tracing::error!("Failed to start optimization engine: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Stop optimization engine
pub async fn stop_optimization(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut engine = state.optimization_engine.write().await;

    match engine.stop().await {
        Ok(_) => Ok(Json(serde_json::json!({
            "status": "success",
            "message": "Optimization engine stopped successfully",
            "timestamp": Utc::now()
        }))),
        Err(e) => {
            tracing::error!("Failed to stop optimization engine: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}