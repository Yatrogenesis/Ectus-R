//! AION Real-Time Monitoring System
//!
//! This crate provides real-time monitoring and metrics collection capabilities
//! for the AION platform including Prometheus metrics and OpenTelemetry tracing.

use anyhow::Result;
use std::sync::Arc;

// Core monitoring modules
pub mod real_time_monitor;
pub mod websocket_service;
pub mod test_integration;

// Prometheus exporter - ROADMAP Task 1.1
pub mod prometheus_exporter_v2;

// Distributed tracing - ROADMAP Task 1.5
pub mod tracing;

// Re-export the main types
pub use real_time_monitor::{RealTimeMonitor, MetricUpdate, DataPoint, DashboardUpdate, AlertEvent};
pub use websocket_service::{WebSocketService, WSMessage, ClientType};
pub use test_integration::*;
pub use prometheus_exporter_v2::{PrometheusExporter, MetricsRegistry};
pub use tracing::{TracingConfig, TracingGuard, init_tracing, create_db_span, create_http_span, create_ai_span, create_external_api_span, add_span_event, set_span_error};

/// Main entry point for the monitoring system
pub struct AionMonitoring {
    pub real_time_monitor: Arc<RealTimeMonitor>,
    pub websocket_service: Arc<WebSocketService>,
    pub prometheus_exporter: Option<PrometheusExporter>,
    pub metrics_registry: Arc<MetricsRegistry>,
}

impl AionMonitoring {
    /// Create a new AION monitoring system without Prometheus
    pub fn new() -> Self {
        let real_time_monitor = Arc::new(RealTimeMonitor::new());
        let websocket_service = Arc::new(WebSocketService::new(
            Arc::clone(&real_time_monitor)
        ));
        let metrics_registry = Arc::new(MetricsRegistry::new());

        Self {
            real_time_monitor,
            websocket_service,
            prometheus_exporter: None,
            metrics_registry,
        }
    }

    /// Create a new AION monitoring system with Prometheus exporter
    ///
    /// # Arguments
    /// * `prometheus_addr` - Address to bind Prometheus metrics server (e.g., "0.0.0.0:9090")
    ///
    /// # Returns
    /// * `Result<Self>` - New instance with Prometheus exporter or error
    pub fn with_prometheus(prometheus_addr: std::net::SocketAddr) -> Result<Self> {
        let real_time_monitor = Arc::new(RealTimeMonitor::new());
        let websocket_service = Arc::new(WebSocketService::new(
            Arc::clone(&real_time_monitor)
        ));
        let metrics_registry = Arc::new(MetricsRegistry::new());
        let prometheus_exporter = Some(PrometheusExporter::new(prometheus_addr)?);

        Ok(Self {
            real_time_monitor,
            websocket_service,
            prometheus_exporter,
            metrics_registry,
        })
    }

    /// Start the monitoring system
    ///
    /// This starts all monitoring services including Prometheus exporter if configured.
    pub async fn start(&mut self) -> Result<()> {
        // Start background monitoring
        self.real_time_monitor.start_background_monitoring().await?;

        // Start Prometheus exporter if configured
        if let Some(ref mut exporter) = self.prometheus_exporter {
            exporter.start().await?;
            tracing::info!("Prometheus metrics exporter started");
        }

        tracing::info!("AION monitoring system started successfully");
        Ok(())
    }

    /// Record a metric (legacy interface)
    pub async fn record_metric(&self, metric: MetricUpdate) -> Result<()> {
        self.real_time_monitor.record_metric(metric).await
    }

    /// Get metrics (legacy interface)
    pub async fn get_metrics(
        &self,
        metric_names: &[String],
        time_range: Option<std::time::Duration>
    ) -> Result<std::collections::HashMap<String, Vec<DataPoint>>> {
        self.real_time_monitor.get_metrics(metric_names, time_range).await
    }

    /// Get the metrics registry for recording Prometheus metrics
    pub fn metrics(&self) -> Arc<MetricsRegistry> {
        Arc::clone(&self.metrics_registry)
    }

    /// Get the Prometheus metrics output (for testing)
    pub fn get_prometheus_metrics(&self) -> Option<String> {
        self.prometheus_exporter.as_ref().map(|e| e.render_metrics())
    }
}

impl Default for AionMonitoring {
    fn default() -> Self {
        Self::new()
    }
}
