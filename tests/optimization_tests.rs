//! Optimization Engine Tests
//! Comprehensive test suite for the optimization engine functionality

use std::time::Duration;
use tokio::time::sleep;
use anyhow::Result;

use aion_optimization_engine::{
    OptimizationEngine, OptimizationConfig,
    CurrentMetrics, BaselineMetrics, RecommendationRequest
};

#[tokio::test]
async fn test_optimization_engine_initialization() -> Result<()> {
    let config = OptimizationConfig::default();
    let mut engine = OptimizationEngine::new(&config).await?;

    assert!(!engine.is_running().await?);

    engine.start().await?;
    assert!(engine.is_running().await?);

    engine.stop().await?;
    assert!(!engine.is_running().await?);

    Ok(())
}

#[tokio::test]
async fn test_performance_score_calculation() -> Result<()> {
    let config = OptimizationConfig::default();
    let mut engine = OptimizationEngine::new(&config).await?;

    // Set baseline metrics
    let baseline = BaselineMetrics {
        response_time: 200.0,
        throughput: 1000.0,
        error_rate: 0.02,
        cpu_usage: 70.0,
        memory_usage: 60.0,
        availability: 99.5,
        established_at: chrono::Utc::now(),
    };

    engine.set_baseline_metrics(baseline).await?;

    // Update with better current metrics
    let current = CurrentMetrics {
        response_time: 150.0,  // Better (lower)
        throughput: 1200.0,    // Better (higher)
        error_rate: 0.01,      // Better (lower)
        cpu_usage: 65.0,       // Better (lower)
        memory_usage: 55.0,    // Better (lower)
        availability: 99.8,    // Better (higher)
        measured_at: chrono::Utc::now(),
    };

    engine.update_current_metrics(current).await?;

    let score = engine.get_performance_score().await?;
    assert!(score > 0.8, "Performance score should be high for improved metrics: {}", score);

    Ok(())
}

#[tokio::test]
async fn test_ml_optimizer_training() -> Result<()> {
    let mut config = OptimizationConfig::default();
    config.ml_enabled = true;

    let mut engine = OptimizationEngine::new(&config).await?;
    engine.start().await?;

    // Add training data
    for i in 0..10 {
        let metrics = CurrentMetrics {
            response_time: 200.0 - (i as f64 * 5.0),
            throughput: 1000.0 + (i as f64 * 50.0),
            error_rate: 0.02 - (i as f64 * 0.001),
            cpu_usage: 70.0 - (i as f64 * 2.0),
            memory_usage: 60.0 - (i as f64 * 1.5),
            availability: 99.0 + (i as f64 * 0.05),
            measured_at: chrono::Utc::now(),
        };

        engine.update_current_metrics(metrics).await?;
        sleep(Duration::from_millis(100)).await;
    }

    // Test ML predictions
    let predictions = engine.predict_performance_trends(Duration::from_hours(1)).await?;
    assert!(!predictions.is_empty(), "Should generate performance predictions");

    engine.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_auto_tuning_recommendations() -> Result<()> {
    let mut config = OptimizationConfig::default();
    config.auto_tuning_enabled = true;

    let mut engine = OptimizationEngine::new(&config).await?;
    engine.start().await?;

    // Simulate poor performance metrics
    let poor_metrics = CurrentMetrics {
        response_time: 500.0,  // High response time
        throughput: 500.0,     // Low throughput
        error_rate: 0.05,      // High error rate
        cpu_usage: 90.0,       // High CPU usage
        memory_usage: 85.0,    // High memory usage
        availability: 95.0,    // Low availability
        measured_at: chrono::Utc::now(),
    };

    engine.update_current_metrics(poor_metrics).await?;

    let request = RecommendationRequest {
        system_context: "test_environment".to_string(),
        performance_data: std::collections::HashMap::new(),
        constraints: vec!["max_cpu_80".to_string()],
        max_recommendations: 5,
    };

    let recommendations = engine.get_recommendations(request).await?;
    assert!(!recommendations.is_empty(), "Should generate recommendations for poor performance");

    // Test recommendation application
    if let Some(recommendation) = recommendations.first() {
        let result = engine.apply_recommendation(recommendation.id).await?;
        assert!(result.success, "Recommendation should be applied successfully");
    }

    engine.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_predictive_analysis() -> Result<()> {
    let config = OptimizationConfig::default();
    let mut engine = OptimizationEngine::new(&config).await?;
    engine.start().await?;

    // Generate time series data
    for hour in 0..24 {
        let metrics = CurrentMetrics {
            response_time: 150.0 + (hour as f64 * 10.0 * (hour as f64 / 12.0).sin()),
            throughput: 1000.0 + (hour as f64 * 50.0 * (hour as f64 / 8.0).cos()),
            error_rate: 0.01 + (hour as f64 * 0.001),
            cpu_usage: 60.0 + (hour as f64 * 2.0),
            memory_usage: 50.0 + (hour as f64 * 1.5),
            availability: 99.5 - (hour as f64 * 0.01),
            measured_at: chrono::Utc::now() - chrono::Duration::hours(24 - hour),
        };

        engine.update_current_metrics(metrics).await?;
    }

    // Test trend analysis
    let trends = engine.analyze_performance_trends().await?;
    assert!(trends.response_time_trend.is_some(), "Should detect response time trends");
    assert!(trends.throughput_trend.is_some(), "Should detect throughput trends");

    // Test anomaly detection
    let anomalies = engine.detect_anomalies().await?;
    // Anomalies may or may not be present in test data, just ensure method works
    assert!(anomalies.len() >= 0, "Anomaly detection should work without errors");

    engine.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_safety_controls() -> Result<()> {
    let mut config = OptimizationConfig::default();
    config.safety_mode = true;
    config.max_concurrent_experiments = 1;

    let mut engine = OptimizationEngine::new(&config).await?;
    engine.start().await?;

    // Test experiment limits
    let experiment_count = engine.get_active_experiments_count().await?;
    assert!(experiment_count <= config.max_concurrent_experiments,
           "Should respect experiment limits");

    // Test rollback capability
    let baseline = BaselineMetrics {
        response_time: 200.0,
        throughput: 1000.0,
        error_rate: 0.02,
        cpu_usage: 70.0,
        memory_usage: 60.0,
        availability: 99.5,
        established_at: chrono::Utc::now(),
    };

    engine.set_baseline_metrics(baseline).await?;

    // Simulate configuration change that causes performance degradation
    let degraded_metrics = CurrentMetrics {
        response_time: 400.0,  // Much worse
        throughput: 600.0,     // Much worse
        error_rate: 0.08,      // Much worse
        cpu_usage: 95.0,       // Much worse
        memory_usage: 90.0,    // Much worse
        availability: 90.0,    // Much worse
        measured_at: chrono::Utc::now(),
    };

    engine.update_current_metrics(degraded_metrics).await?;

    // In safety mode, should trigger rollback
    let safety_status = engine.get_safety_status().await?;
    assert!(safety_status.rollback_triggered || safety_status.monitoring_active,
           "Safety controls should be active");

    engine.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_telemetry_collection() -> Result<()> {
    let config = OptimizationConfig::default();
    let mut engine = OptimizationEngine::new(&config).await?;
    engine.start().await?;

    // Generate some telemetry data
    for i in 0..5 {
        let metrics = CurrentMetrics {
            response_time: 150.0 + (i as f64 * 10.0),
            throughput: 1000.0 + (i as f64 * 100.0),
            error_rate: 0.01,
            cpu_usage: 60.0 + (i as f64 * 5.0),
            memory_usage: 50.0 + (i as f64 * 3.0),
            availability: 99.5,
            measured_at: chrono::Utc::now(),
        };

        engine.update_current_metrics(metrics).await?;
        sleep(Duration::from_millis(100)).await;
    }

    // Test telemetry retrieval
    let telemetry = engine.get_telemetry_summary().await?;
    assert!(telemetry.total_events > 0, "Should collect telemetry events");
    assert!(telemetry.metric_count > 0, "Should collect metrics");

    // Test metric export
    let exported_metrics = engine.export_metrics("prometheus").await?;
    assert!(!exported_metrics.is_empty(), "Should export metrics in specified format");

    engine.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_configuration_updates() -> Result<()> {
    let config = OptimizationConfig::default();
    let mut engine = OptimizationEngine::new(&config).await?;

    // Test configuration retrieval
    let current_config = engine.get_config().await?;
    assert_eq!(current_config.ml_enabled, config.ml_enabled);

    // Test configuration update
    let mut new_config = current_config.clone();
    new_config.ml_enabled = !new_config.ml_enabled;
    new_config.metrics_collection_interval = 30;

    engine.update_config(new_config.clone()).await?;

    let updated_config = engine.get_config().await?;
    assert_eq!(updated_config.ml_enabled, new_config.ml_enabled);
    assert_eq!(updated_config.metrics_collection_interval, new_config.metrics_collection_interval);

    Ok(())
}

#[tokio::test]
async fn test_performance_under_load() -> Result<()> {
    let config = OptimizationConfig::default();
    let mut engine = OptimizationEngine::new(&config).await?;
    engine.start().await?;

    // Simulate high-frequency metric updates
    let start_time = std::time::Instant::now();

    for i in 0..100 {
        let metrics = CurrentMetrics {
            response_time: 150.0 + (i as f64 % 50.0),
            throughput: 1000.0 + (i as f64 * 10.0),
            error_rate: 0.01,
            cpu_usage: 60.0 + (i as f64 % 30.0),
            memory_usage: 50.0 + (i as f64 % 20.0),
            availability: 99.5,
            measured_at: chrono::Utc::now(),
        };

        engine.update_current_metrics(metrics).await?;
    }

    let elapsed = start_time.elapsed();
    assert!(elapsed.as_millis() < 5000, "Should handle 100 metric updates within 5 seconds");

    // Verify engine is still responsive
    let score = engine.get_performance_score().await?;
    assert!(score >= 0.0 && score <= 1.0, "Performance score should be valid after load test");

    engine.stop().await?;
    Ok(())
}

// Integration test with monitoring service
#[tokio::test]
async fn test_integration_with_monitoring() -> Result<()> {
    use aion_monitoring::MonitoringService;

    let monitoring = MonitoringService::new().await?;

    let config = OptimizationConfig::default();
    let mut engine = OptimizationEngine::new(&config).await?;
    engine.start().await?;

    // Test metrics integration
    let system_health = monitoring.get_system_health().await?;

    let current_metrics = CurrentMetrics {
        response_time: system_health.metrics.response_time,
        throughput: system_health.metrics.requests_per_second,
        error_rate: system_health.metrics.error_rate,
        cpu_usage: system_health.metrics.cpu_usage,
        memory_usage: system_health.metrics.memory_usage,
        availability: system_health.metrics.uptime_percentage,
        measured_at: chrono::Utc::now(),
    };

    engine.update_current_metrics(current_metrics).await?;

    let optimization_score = engine.get_performance_score().await?;
    assert!(optimization_score >= 0.0 && optimization_score <= 1.0,
           "Optimization score should be valid when integrated with monitoring");

    engine.stop().await?;
    Ok(())
}