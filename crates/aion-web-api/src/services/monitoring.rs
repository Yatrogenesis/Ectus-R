//! Monitoring service - Bridge to aion-monitoring crate

use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use uuid::Uuid;
use crate::models::*;
use aion_monitoring::{
    MetricsCollector, SystemMonitor, AlertManager, PerformanceTracker,
    models::{Metric, Alert as MonitoringAlert, AlertLevel as MonitoringAlertLevel},
};

/// Service for system monitoring and metrics with real monitoring integration
pub struct MonitoringService {
    metrics_collector: Arc<MetricsCollector>,
    system_monitor: Arc<SystemMonitor>,
    alert_manager: Arc<AlertManager>,
    performance_tracker: Arc<PerformanceTracker>,
}

impl MonitoringService {
    pub async fn new() -> Result<Self> {
        println!("🔧 Initializing Monitoring Service with real monitoring...");

        // Initialize real monitoring components
        let metrics_collector = Arc::new(MetricsCollector::new().await?);
        let system_monitor = Arc::new(SystemMonitor::new().await?);
        let alert_manager = Arc::new(AlertManager::new().await?);
        let performance_tracker = Arc::new(PerformanceTracker::new().await?);

        // Start background monitoring tasks
        system_monitor.start_monitoring().await?;
        metrics_collector.start_collection().await?;

        println!("✅ Monitoring Service initialized with real system metrics");

        Ok(Self {
            metrics_collector,
            system_monitor,
            alert_manager,
            performance_tracker,
        })
    }

    /// Get overall system health
    pub async fn get_system_health(&self) -> Result<SystemHealth> {
        // Get real system metrics from monitoring components
        let system_stats = self.system_monitor.get_system_stats().await?;
        let network_stats = self.system_monitor.get_network_stats().await?;
        let active_alerts = self.get_active_alerts().await?;

        let system_metrics = SystemMetrics {
            cpu_usage: system_stats.cpu_usage_percent,
            memory_usage: system_stats.memory_usage_percent,
            disk_usage: system_stats.disk_usage_percent,
            network_io: NetworkIO {
                bytes_in: network_stats.bytes_received,
                bytes_out: network_stats.bytes_sent,
                packets_in: network_stats.packets_received,
                packets_out: network_stats.packets_sent,
            },
            active_connections: network_stats.active_connections,
            requests_per_second: self.performance_tracker.get_current_rps().await?,
        };

        // Determine overall system status
        let status = if system_metrics.cpu_usage > 90.0 || system_metrics.memory_usage > 95.0 {
            "critical"
        } else if system_metrics.cpu_usage > 70.0 || system_metrics.memory_usage > 80.0 {
            "degraded"
        } else if active_alerts.iter().any(|a| matches!(a.level, AlertLevel::Critical)) {
            "warning"
        } else {
            "operational"
        };

        Ok(SystemHealth {
            status: status.to_string(),
            metrics: system_metrics,
            active_alerts,
        })
    }

    /// Get time series metrics
    pub async fn get_metrics(
        &self,
        metric_names: &[String],
        duration: Option<chrono::Duration>
    ) -> Result<HashMap<String, MetricData>> {
        let mut metrics = HashMap::new();

        let duration = duration.unwrap_or(chrono::Duration::hours(1));
        let now = chrono::Utc::now();
        let start_time = now - duration;

        // Get real metrics from the metrics collector
        for metric_name in metric_names {
            let raw_metrics = self.metrics_collector
                .get_metric_history(metric_name, start_time, now)
                .await?;

            // Convert monitoring metrics to web API format
            let data_points: Vec<DataPoint> = raw_metrics.into_iter().map(|m| {
                DataPoint {
                    timestamp: m.timestamp,
                    value: m.value,
                    tags: m.tags,
                }
            }).collect();

            // Calculate summary statistics
            let values: Vec<f64> = data_points.iter().map(|p| p.value).collect();
            let summary = if !values.is_empty() {
                let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let avg = values.iter().sum::<f64>() / values.len() as f64;
                let current = values.last().copied().unwrap_or(0.0);

                // Calculate trend
                let trend = if values.len() >= 2 {
                    let first_half_avg = values.iter().take(values.len() / 2).sum::<f64>() / (values.len() / 2) as f64;
                    let second_half_avg = values.iter().skip(values.len() / 2).sum::<f64>() / (values.len() - values.len() / 2) as f64;

                    if second_half_avg > first_half_avg * 1.1 {
                        "increasing"
                    } else if second_half_avg < first_half_avg * 0.9 {
                        "decreasing"
                    } else {
                        "stable"
                    }
                } else {
                    "stable"
                };

                MetricSummary {
                    min,
                    max,
                    avg,
                    current,
                    trend: trend.to_string(),
                }
            } else {
                // Fallback for empty data
                MetricSummary {
                    min: 0.0,
                    max: 0.0,
                    avg: 0.0,
                    current: 0.0,
                    trend: "no_data".to_string(),
                }
            };

            let metric_data = MetricData {
                name: metric_name.clone(),
                unit: self.get_metric_unit(metric_name),
                data_points,
                summary,
            };

            metrics.insert(metric_name.clone(), metric_data);
        }

        Ok(metrics)
    }

    /// Get active alerts
    pub async fn get_active_alerts(&self) -> Result<Vec<Alert>> {
        // Get real alerts from the alert manager
        let monitoring_alerts = self.alert_manager.get_active_alerts().await?;

        // Convert monitoring alerts to web API format
        let alerts: Vec<Alert> = monitoring_alerts.into_iter().map(|alert| {
            let level = match alert.level {
                MonitoringAlertLevel::Critical => AlertLevel::Critical,
                MonitoringAlertLevel::High => AlertLevel::Error,
                MonitoringAlertLevel::Medium => AlertLevel::Warning,
                MonitoringAlertLevel::Low => AlertLevel::Info,
                MonitoringAlertLevel::Info => AlertLevel::Info,
            };

            Alert {
                id: alert.id,
                level,
                title: alert.title,
                message: alert.message,
                timestamp: alert.timestamp,
                source: alert.source,
                acknowledged: alert.acknowledged,
            }
        }).collect();

        Ok(alerts)
    }


    /// Get appropriate unit for a metric
    fn get_metric_unit(&self, metric_name: &str) -> String {
        match metric_name {
            name if name.contains("percent") => "percent".to_string(),
            name if name.contains("time_ms") => "milliseconds".to_string(),
            name if name.contains("requests_per_second") => "requests/sec".to_string(),
            name if name.contains("bytes") => "bytes".to_string(),
            name if name.contains("rate") => "percent".to_string(),
            _ => "count".to_string(),
        }
    }
}