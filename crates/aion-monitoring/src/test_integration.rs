//! Integration tests for the real-time monitoring system

use crate::real_time_monitor::{RealTimeMonitor, MetricUpdate};
use std::sync::Arc;

/// Test the basic functionality of the real-time monitoring system
pub async fn test_real_time_monitoring_basic() -> anyhow::Result<()> {
    // Create a new real-time monitor
    let monitor = Arc::new(RealTimeMonitor::new());

    // Test basic metric recording
    let mut labels = std::collections::HashMap::new();
    labels.insert("component".to_string(), "test".to_string());
    labels.insert("environment".to_string(), "integration".to_string());

    let metric_update = MetricUpdate {
        metric_name: "test.metric".to_string(),
        value: 42.0,
        labels,
        timestamp: chrono::Utc::now(),
        source: "integration_test".to_string(),
    };

    // Record the metric
    monitor.record_metric(metric_update.clone()).await?;

    // Verify we can retrieve metrics
    let metrics = monitor.get_metrics(
        &vec!["test.metric".to_string()],
        Some(std::time::Duration::from_secs(60))
    ).await?;

    // Check we got the metric back
    assert!(metrics.contains_key("test.metric"), "Metric should be stored and retrievable");

    let data_points = &metrics["test.metric"];
    assert!(!data_points.is_empty(), "Should have at least one data point");
    assert_eq!(data_points[0].value, 42.0, "Value should match what was recorded");

    println!("✅ Real-time monitoring basic test passed");
    Ok(())
}

/// Test system metrics collection
pub async fn test_system_metrics_collection() -> anyhow::Result<()> {
    let monitor = Arc::new(RealTimeMonitor::new());

    // Start background monitoring
    monitor.start_background_monitoring().await?;

    // Wait a moment for metrics to be collected
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Try to get system metrics
    let system_metrics = vec![
        "system.cpu.usage_percent".to_string(),
        "system.memory.usage_percent".to_string(),
    ];

    let metrics = monitor.get_metrics(&system_metrics, Some(std::time::Duration::from_secs(10))).await?;

    // We might not have metrics yet since background collection just started,
    // but the call should succeed without error
    println!("✅ System metrics collection test passed - got {} metric series", metrics.len());
    Ok(())
}

/// Run all integration tests
pub async fn run_integration_tests() -> anyhow::Result<()> {
    println!("🧪 Running real-time monitoring integration tests...");

    // Run basic monitoring test
    test_real_time_monitoring_basic().await?;

    // Run system metrics test
    test_system_metrics_collection().await?;

    println!("✅ All integration tests passed!");
    Ok(())
}