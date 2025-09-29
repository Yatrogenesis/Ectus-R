//! AION Real-Time Monitoring System
//!
//! This crate provides real-time monitoring and metrics collection capabilities
//! for the AION platform.

use anyhow::Result;

// Core monitoring modules
pub mod real_time_monitor;
pub mod websocket_service;
pub mod test_integration;

// Re-export the main types
pub use real_time_monitor::{RealTimeMonitor, MetricUpdate, DataPoint, DashboardUpdate, AlertEvent};
pub use websocket_service::{WebSocketService, WSMessage, ClientType};
pub use test_integration::*;

/// Main entry point for the monitoring system
pub struct AionMonitoring {
    pub real_time_monitor: std::sync::Arc<RealTimeMonitor>,
    pub websocket_service: std::sync::Arc<WebSocketService>,
}

impl AionMonitoring {
    /// Create a new AION monitoring system
    pub fn new() -> Self {
        let real_time_monitor = std::sync::Arc::new(RealTimeMonitor::new());
        let websocket_service = std::sync::Arc::new(WebSocketService::new(
            std::sync::Arc::clone(&real_time_monitor)
        ));

        Self {
            real_time_monitor,
            websocket_service,
        }
    }

    /// Start the monitoring system
    pub async fn start(&self) -> Result<()> {
        // Start background monitoring
        self.real_time_monitor.start_background_monitoring().await?;

        tracing::info!("AION monitoring system started successfully");
        Ok(())
    }

    /// Record a metric
    pub async fn record_metric(&self, metric: MetricUpdate) -> Result<()> {
        self.real_time_monitor.record_metric(metric).await
    }

    /// Get metrics
    pub async fn get_metrics(
        &self,
        metric_names: &[String],
        time_range: Option<std::time::Duration>
    ) -> Result<std::collections::HashMap<String, Vec<DataPoint>>> {
        self.real_time_monitor.get_metrics(metric_names, time_range).await
    }
}