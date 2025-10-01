use crate::PlatformConfig;
use anyhow::Result;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub collection_interval_ms: u64,
    pub retention_hours: u32,
    pub export_prometheus: bool,
    pub export_opentelemetry: bool,
    pub custom_metrics: bool,
}

pub struct EnterpriseMetrics {
    config: Arc<PlatformConfig>,
    metrics_config: MetricsConfig,

    // Performance metrics
    request_count: AtomicU64,
    request_duration_sum: AtomicU64, // Store as microseconds
    error_count: AtomicU64,

    // System metrics
    memory_usage: AtomicU64, // Store as bytes
    cpu_usage: AtomicU64, // Store as percentage * 100
    disk_usage: AtomicU64, // Store as bytes
    network_rx_bytes: AtomicU64,
    network_tx_bytes: AtomicU64,

    // Business metrics
    active_users: AtomicU64,
    ai_operations: AtomicU64,
    data_processed_gb: AtomicU64, // Store as MB

    // Custom metrics storage
    custom_metrics: Arc<DashMap<String, MetricValue>>,

    // Time series data
    time_series: Arc<DashMap<String, Vec<TimeSeriesPoint>>>,

    // System info
    system: Arc<tokio::sync::Mutex<System>>,
    start_time: Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram {
        buckets: Vec<f64>,
        counts: Vec<u64>,
        sum: f64,
        count: u64,
    },
    Summary {
        quantiles: Vec<(f64, f64)>, // (quantile, value)
        sum: f64,
        count: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub labels: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSummary {
    pub collection_time: DateTime<Utc>,
    pub uptime_seconds: u64,

    // Performance
    pub requests_per_second: f64,
    pub average_response_time_ms: f64,
    pub error_rate_percent: f64,

    // System
    pub memory_usage_mb: f64,
    pub memory_usage_percent: f64,
    pub cpu_usage_percent: f64,
    pub disk_usage_percent: f64,

    // Network
    pub network_rx_mbps: f64,
    pub network_tx_mbps: f64,

    // Business
    pub active_users: u64,
    pub ai_operations_per_minute: f64,
    pub data_processed_gb_per_hour: f64,

    // Health indicators
    pub health_score: f64,
    pub alerts: Vec<MetricAlert>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAlert {
    pub alert_id: String,
    pub severity: AlertSeverity,
    pub metric_name: String,
    pub current_value: f64,
    pub threshold: f64,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

impl EnterpriseMetrics {
    pub async fn new(config: Arc<PlatformConfig>) -> Result<Self> {
        let metrics_config = MetricsConfig {
            collection_interval_ms: 1000, // 1 second
            retention_hours: 24,
            export_prometheus: true,
            export_opentelemetry: config.enterprise_options.audit_logging,
            custom_metrics: true,
        };

        let mut system = System::new_all();
        system.refresh_all();

        Ok(Self {
            config,
            metrics_config,
            request_count: AtomicU64::new(0),
            request_duration_sum: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            memory_usage: AtomicU64::new(0),
            cpu_usage: AtomicU64::new(0),
            disk_usage: AtomicU64::new(0),
            network_rx_bytes: AtomicU64::new(0),
            network_tx_bytes: AtomicU64::new(0),
            active_users: AtomicU64::new(0),
            ai_operations: AtomicU64::new(0),
            data_processed_gb: AtomicU64::new(0),
            custom_metrics: Arc::new(DashMap::new()),
            time_series: Arc::new(DashMap::new()),
            system: Arc::new(tokio::sync::Mutex::new(system)),
            start_time: Instant::now(),
        })
    }

    pub async fn start_collection(&self) -> Result<()> {
        let metrics = Arc::new(self.clone());
        let interval_ms = self.metrics_config.collection_interval_ms;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));

            loop {
                interval.tick().await;
                if let Err(e) = metrics.collect_system_metrics().await {
                    tracing::error!("Failed to collect system metrics: {}", e);
                }
            }
        });

        tracing::info!("Enterprise metrics collection started");
        Ok(())
    }

    pub async fn stop_collection(&self) -> Result<()> {
        // In a real implementation, you'd store the task handle and cancel it
        tracing::info!("Enterprise metrics collection stopped");
        Ok(())
    }

    async fn collect_system_metrics(&self) -> Result<()> {
        let mut system = self.system.lock().await;
        system.refresh_all();

        let pid = sysinfo::get_current_pid().unwrap_or_else(|_| sysinfo::Pid::from(0));

        if let Some(process) = system.process(pid) {
            let memory_bytes = process.memory();
            let cpu_percent_scaled = (process.cpu_usage() * 100.0) as u64;

            self.memory_usage.store(memory_bytes, Ordering::Relaxed);
            self.cpu_usage.store(cpu_percent_scaled, Ordering::Relaxed);

            // Store time series data
            let memory_mb = memory_bytes as f64 / 1024.0 / 1024.0;
            let cpu_percent = cpu_percent_scaled as f64 / 100.0;
            self.record_time_series("memory_usage_mb", memory_mb, None).await;
            self.record_time_series("cpu_usage_percent", cpu_percent, None).await;
        }

        // Collect disk usage
        let disks = sysinfo::Disks::new_with_refreshed_list();
        let total_disk = disks.iter().map(|d| d.total_space()).sum::<u64>();
        let used_disk = disks.iter().map(|d| d.total_space() - d.available_space()).sum::<u64>();
        let disk_usage_bytes = used_disk;

        self.disk_usage.store(disk_usage_bytes, Ordering::Relaxed);
        self.record_time_series("disk_usage_bytes", disk_usage_bytes as f64, None).await;

        Ok(())
    }

    async fn record_time_series(
        &self,
        metric_name: &str,
        value: f64,
        labels: Option<std::collections::HashMap<String, String>>,
    ) {
        let point = TimeSeriesPoint {
            timestamp: Utc::now(),
            value,
            labels: labels.unwrap_or_default(),
        };

        let mut series = self.time_series.entry(metric_name.to_string()).or_insert_with(Vec::new);
        series.push(point);

        // Keep only recent data (retention policy)
        let cutoff = Utc::now() - chrono::Duration::hours(self.metrics_config.retention_hours as i64);
        series.retain(|p| p.timestamp > cutoff);
    }

    // Performance metrics
    pub fn increment_request_count(&self) {
        self.request_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_request_duration(&self, duration: Duration) {
        let duration_ms = duration.as_secs_f64() * 1000.0;
        self.request_duration_sum.fetch_add(duration_ms as u64, Ordering::Relaxed);
    }

    pub fn increment_error_count(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }

    pub async fn record_service_start(&self, service_name: &str) -> Result<()> {
        self.record_time_series(
            "service_starts",
            1.0,
            Some({
                let mut labels = std::collections::HashMap::new();
                labels.insert("service".to_string(), service_name.to_string());
                labels
            })
        ).await;
        Ok(())
    }

    pub async fn record_service_error(&self, service_name: &str, error: &str) -> Result<()> {
        self.record_time_series(
            "service_errors",
            1.0,
            Some({
                let mut labels = std::collections::HashMap::new();
                labels.insert("service".to_string(), service_name.to_string());
                labels.insert("error".to_string(), error.to_string());
                labels
            })
        ).await;
        Ok(())
    }

    // Business metrics
    pub fn set_active_users(&self, count: u64) {
        self.active_users.store(count, Ordering::Relaxed);
    }

    pub fn increment_ai_operations(&self) {
        self.ai_operations.fetch_add(1, Ordering::Relaxed);
    }

    pub fn add_data_processed(&self, gb: f64) {
        self.data_processed_gb.fetch_add((gb * 1000.0) as u64, Ordering::Relaxed);
    }

    // Custom metrics
    pub fn set_custom_metric(&self, name: String, value: MetricValue) {
        self.custom_metrics.insert(name, value);
    }

    pub fn get_custom_metric(&self, name: &str) -> Option<MetricValue> {
        self.custom_metrics.get(name).map(|entry| entry.value().clone())
    }

    pub async fn get_platform_summary(&self) -> Result<MetricsSummary> {
        let uptime = self.start_time.elapsed().as_secs();
        let current_time = Utc::now();

        // Calculate rates
        let request_count = self.request_count.load(Ordering::Relaxed);
        let requests_per_second = if uptime > 0 {
            request_count as f64 / uptime as f64
        } else {
            0.0
        };

        let duration_sum = self.request_duration_sum.load(Ordering::Relaxed);
        let average_response_time = if request_count > 0 {
            duration_sum as f64 / request_count as f64
        } else {
            0.0
        };

        let error_count = self.error_count.load(Ordering::Relaxed);
        let error_rate = if request_count > 0 {
            (error_count as f64 / request_count as f64) * 100.0
        } else {
            0.0
        };

        // Get current system metrics
        let memory_mb = self.memory_usage.load(Ordering::Relaxed);
        let cpu_percent = self.cpu_usage.load(Ordering::Relaxed);
        let disk_percent = self.disk_usage.load(Ordering::Relaxed);

        // Calculate health score
        let health_score = self.calculate_health_score(
            cpu_percent as f64,
            memory_mb as f64,
            disk_percent as f64,
            error_rate,
        );

        // Generate alerts
        let alerts = self.generate_alerts(
            cpu_percent as f64,
            memory_mb as f64,
            disk_percent as f64,
            error_rate,
        );

        Ok(MetricsSummary {
            collection_time: current_time,
            uptime_seconds: uptime,
            requests_per_second,
            average_response_time_ms: average_response_time,
            error_rate_percent: error_rate,
            memory_usage_mb: memory_mb as f64,
            memory_usage_percent: 0.0, // Would calculate from total system memory
            cpu_usage_percent: cpu_percent as f64,
            disk_usage_percent: disk_percent as f64,
            network_rx_mbps: 0.0, // Would calculate from network stats
            network_tx_mbps: 0.0,
            active_users: self.active_users.load(Ordering::Relaxed),
            ai_operations_per_minute: 0.0, // Would calculate rate
            data_processed_gb_per_hour: 0.0, // Would calculate rate
            health_score,
            alerts,
        })
    }

    fn calculate_health_score(
        &self,
        cpu_percent: f64,
        memory_mb: f64,
        disk_percent: f64,
        error_rate: f64,
    ) -> f64 {
        let mut score: f64 = 100.0;

        // Penalize high CPU usage
        if cpu_percent > 80.0 {
            score -= 20.0;
        } else if cpu_percent > 60.0 {
            score -= 10.0;
        }

        // Penalize high memory usage
        if memory_mb > 2048.0 {
            score -= 15.0;
        } else if memory_mb > 1024.0 {
            score -= 5.0;
        }

        // Penalize high disk usage
        if disk_percent > 90.0 {
            score -= 25.0;
        } else if disk_percent > 80.0 {
            score -= 10.0;
        }

        // Penalize high error rate
        if error_rate > 5.0 {
            score -= 30.0;
        } else if error_rate > 1.0 {
            score -= 10.0;
        }

        score.max(0.0)
    }

    fn generate_alerts(
        &self,
        cpu_percent: f64,
        memory_mb: f64,
        disk_percent: f64,
        error_rate: f64,
    ) -> Vec<MetricAlert> {
        let mut alerts = Vec::new();
        let now = Utc::now();

        if cpu_percent > 90.0 {
            alerts.push(MetricAlert {
                alert_id: "cpu_critical".to_string(),
                severity: AlertSeverity::Critical,
                metric_name: "cpu_usage_percent".to_string(),
                current_value: cpu_percent,
                threshold: 90.0,
                message: "CPU usage is critically high".to_string(),
                timestamp: now,
            });
        }

        if memory_mb > 4096.0 {
            alerts.push(MetricAlert {
                alert_id: "memory_warning".to_string(),
                severity: AlertSeverity::Warning,
                metric_name: "memory_usage_mb".to_string(),
                current_value: memory_mb,
                threshold: 4096.0,
                message: "Memory usage is high".to_string(),
                timestamp: now,
            });
        }

        if disk_percent > 95.0 {
            alerts.push(MetricAlert {
                alert_id: "disk_critical".to_string(),
                severity: AlertSeverity::Critical,
                metric_name: "disk_usage_percent".to_string(),
                current_value: disk_percent,
                threshold: 95.0,
                message: "Disk space is critically low".to_string(),
                timestamp: now,
            });
        }

        if error_rate > 10.0 {
            alerts.push(MetricAlert {
                alert_id: "error_rate_critical".to_string(),
                severity: AlertSeverity::Critical,
                metric_name: "error_rate_percent".to_string(),
                current_value: error_rate,
                threshold: 10.0,
                message: "Error rate is critically high".to_string(),
                timestamp: now,
            });
        }

        alerts
    }

    pub fn get_time_series(&self, metric_name: &str) -> Option<Vec<TimeSeriesPoint>> {
        self.time_series.get(metric_name).map(|entry| entry.value().clone())
    }
}

impl Clone for EnterpriseMetrics {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            metrics_config: self.metrics_config.clone(),
            request_count: AtomicU64::new(self.request_count.load(Ordering::Relaxed)),
            request_duration_sum: AtomicU64::new(self.request_duration_sum.load(Ordering::Relaxed)),
            error_count: AtomicU64::new(self.error_count.load(Ordering::Relaxed)),
            memory_usage: AtomicU64::new(self.memory_usage.load(Ordering::Relaxed)),
            cpu_usage: AtomicU64::new(self.cpu_usage.load(Ordering::Relaxed)),
            disk_usage: AtomicU64::new(self.disk_usage.load(Ordering::Relaxed)),
            network_rx_bytes: AtomicU64::new(self.network_rx_bytes.load(Ordering::Relaxed)),
            network_tx_bytes: AtomicU64::new(self.network_tx_bytes.load(Ordering::Relaxed)),
            active_users: AtomicU64::new(self.active_users.load(Ordering::Relaxed)),
            ai_operations: AtomicU64::new(self.ai_operations.load(Ordering::Relaxed)),
            data_processed_gb: AtomicU64::new(self.data_processed_gb.load(Ordering::Relaxed)),
            custom_metrics: self.custom_metrics.clone(),
            time_series: self.time_series.clone(),
            system: self.system.clone(),
            start_time: self.start_time,
        }
    }
}