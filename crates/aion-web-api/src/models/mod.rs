//! Data models for API requests and responses

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// System status response
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemStatus {
    pub overall_status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub version: String,
    pub uptime: chrono::Duration,
    pub services: Vec<ServiceStatus>,
    pub system_metrics: SystemMetrics,
    pub active_alerts: Vec<Alert>,
    pub recent_deployments: u32,
    pub active_projects: u32,
}

/// Individual service status
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServiceStatus {
    pub name: String,
    pub status: String, // "operational", "degraded", "down"
    pub uptime: chrono::Duration,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub error_rate: f64,
    pub response_time: f64, // milliseconds
}

/// System metrics
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_io: NetworkIO,
    pub active_connections: u32,
    pub requests_per_second: f64,
}

/// Network I/O metrics
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkIO {
    pub bytes_in: u64,
    pub bytes_out: u64,
    pub packets_in: u64,
    pub packets_out: u64,
}

/// Alert information
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Alert {
    pub id: Uuid,
    pub level: AlertLevel,
    pub title: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: String,
    pub acknowledged: bool,
}

/// Alert severity levels
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// Metrics response with time series data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetricsResponse {
    pub timerange: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metrics: HashMap<String, MetricData>,
}

/// Time series metric data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetricData {
    pub name: String,
    pub unit: String,
    pub data_points: Vec<DataPoint>,
    pub summary: MetricSummary,
}

/// Individual data point
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataPoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub value: f64,
    pub tags: HashMap<String, String>,
}

/// Metric summary statistics
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetricSummary {
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub current: f64,
    pub trend: String, // "increasing", "decreasing", "stable"
}

/// Dashboard data aggregation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DashboardData {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub system_health: SystemHealth,
    pub recent_deployments: Vec<Deployment>,
    pub ai_statistics: AIStatistics,
    pub performance_metrics: PerformanceMetrics,
    pub resource_usage: ResourceUsage,
}

/// System health overview
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemHealth {
    pub status: String,
    pub metrics: SystemMetrics,
    pub active_alerts: Vec<Alert>,
}

/// AI engine statistics
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AIStatistics {
    pub total_generations: u64,
    pub successful_generations: u64,
    pub average_generation_time: f64,
    pub active_models: Vec<String>,
    pub queue_length: u32,
}

impl Default for AIStatistics {
    fn default() -> Self {
        Self {
            total_generations: 0,
            successful_generations: 0,
            average_generation_time: 0.0,
            active_models: vec!["gpt-4".to_string(), "claude-3".to_string()],
            queue_length: 0,
        }
    }
}

/// Performance metrics
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PerformanceMetrics {
    pub api_response_time: f64,
    pub throughput: f64,
    pub error_rate: f64,
    pub uptime_percentage: f64,
}

/// Resource usage statistics
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResourceUsage {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub disk_percent: f64,
    pub network_io: NetworkIO,
}

/// Deployment information
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Deployment {
    pub id: Uuid,
    pub name: String,
    pub status: String,
    pub environment: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: Option<String>,
    pub health_score: f64,
}

/// Project information
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub language: String,
    pub framework: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub repository_url: Option<String>,
    pub deployment_count: u32,
    // Additional fields for dashboard
    #[serde(rename = "lastDeployment")]
    pub last_deployment: String,
    #[serde(rename = "createdAt")]
    pub created_at_iso: String,
    pub repository: String,
    pub environment: String,
    pub team: Vec<String>,
    #[serde(rename = "deploymentUrl")]
    pub deployment_url: Option<String>,
    pub visibility: String,
    pub tags: Vec<String>,
}

/// AI generation request
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GenerateRequest {
    pub prompt: String,
    pub language: Option<String>,
    pub framework: Option<String>,
    pub features: Vec<String>,
    pub deployment_target: Option<String>,
}

/// AI generation response
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GenerateResponse {
    pub id: Uuid,
    pub status: String,
    pub generated_files: Vec<GeneratedFile>,
    pub deployment_instructions: Option<String>,
    pub estimated_time: u64,
    pub confidence_score: f64,
}

/// Generated file
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeneratedFile {
    pub path: String,
    pub content: String,
    pub language: String,
    pub size: usize,
}

/// Authentication request
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Authentication response
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub user: User,
}

/// User information
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_login: chrono::DateTime<chrono::Utc>,
}

/// Error response
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_id: Option<Uuid>,
}