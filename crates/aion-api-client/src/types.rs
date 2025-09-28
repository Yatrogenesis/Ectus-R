use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub tech_stack: Vec<String>,
    pub architecture: Option<String>,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    #[serde(rename = "planning")]
    Planning,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "testing")]
    Testing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "paused")]
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub tech_stack: Vec<String>,
    pub architecture: Option<String>,
    pub requirements: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub tech_stack: String,
    pub architecture: String,
    pub category: TemplateCategory,
    pub tags: Vec<String>,
    pub version: String,
    pub author: String,
    pub downloads: u64,
    pub rating: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateCategory {
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "desktop")]
    Desktop,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "microservice")]
    Microservice,
    #[serde(rename = "library")]
    Library,
    #[serde(rename = "cli")]
    Cli,
    #[serde(rename = "game")]
    Game,
    #[serde(rename = "ai_ml")]
    AiMl,
    #[serde(rename = "blockchain")]
    Blockchain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateRequest {
    pub template_id: Uuid,
    pub project_name: String,
    pub customizations: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QASession {
    pub id: Uuid,
    pub project_id: Uuid,
    pub test_type: QATestType,
    pub status: QAStatus,
    pub results: Option<QAResults>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub configuration: QAConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QATestType {
    #[serde(rename = "unit")]
    Unit,
    #[serde(rename = "integration")]
    Integration,
    #[serde(rename = "e2e")]
    E2E,
    #[serde(rename = "performance")]
    Performance,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "accessibility")]
    Accessibility,
    #[serde(rename = "comprehensive")]
    Comprehensive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QAStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAResults {
    pub total_tests: u32,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub coverage: Option<f32>,
    pub duration_ms: u64,
    pub issues: Vec<QAIssue>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAIssue {
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "info")]
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueCategory {
    #[serde(rename = "bug")]
    Bug,
    #[serde(rename = "performance")]
    Performance,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "style")]
    Style,
    #[serde(rename = "maintainability")]
    Maintainability,
    #[serde(rename = "accessibility")]
    Accessibility,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QAConfiguration {
    pub timeout_seconds: Option<u32>,
    pub parallel_execution: Option<bool>,
    pub coverage_threshold: Option<f32>,
    pub custom_rules: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QARequest {
    pub project_id: Uuid,
    pub test_type: QATestType,
    pub configuration: Option<QAConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEvent {
    pub session_id: Uuid,
    pub event_type: ProgressEventType,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressEventType {
    #[serde(rename = "session_started")]
    SessionStarted,
    #[serde(rename = "task_started")]
    TaskStarted,
    #[serde(rename = "task_progress")]
    TaskProgress,
    #[serde(rename = "task_completed")]
    TaskCompleted,
    #[serde(rename = "task_failed")]
    TaskFailed,
    #[serde(rename = "session_completed")]
    SessionCompleted,
    #[serde(rename = "session_failed")]
    SessionFailed,
    #[serde(rename = "metrics_update")]
    MetricsUpdate,
    #[serde(rename = "log_message")]
    LogMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressSession {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub status: ProgressStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_tasks: u32,
    pub completed_tasks: u32,
    pub current_task: Option<String>,
    pub metrics: ProgressMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "paused")]
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressMetrics {
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub disk_io: u64,
    pub network_io: u64,
    pub tasks_per_minute: f32,
    pub estimated_completion: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
    pub code: Option<String>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(20),
            sort_by: None,
            sort_order: Some(SortOrder::Asc),
        }
    }
}