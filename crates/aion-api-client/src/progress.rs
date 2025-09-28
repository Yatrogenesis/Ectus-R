use crate::{types::*, error::Result, client::AionClient};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ProgressApi {
    client: AionClient,
}

impl ProgressApi {
    pub(crate) fn new(client: AionClient) -> Self {
        Self { client }
    }

    /// Get active progress sessions
    pub async fn list_sessions(&self, params: Option<ProgressListParams>) -> Result<PaginatedResponse<ProgressSession>> {
        let mut path = "/api/v1/progress/sessions".to_string();
        let mut query_params = Vec::new();

        if let Some(params) = params {
            if let Some(project_id) = params.project_id {
                query_params.push(format!("project_id={}", project_id));
            }

            if let Some(status) = params.status {
                let status_str = match status {
                    ProgressStatus::Running => "running",
                    ProgressStatus::Completed => "completed",
                    ProgressStatus::Failed => "failed",
                    ProgressStatus::Paused => "paused",
                };
                query_params.push(format!("status={}", status_str));
            }

            if let Some(page) = params.pagination.page {
                query_params.push(format!("page={}", page));
            }

            if let Some(per_page) = params.pagination.per_page {
                query_params.push(format!("per_page={}", per_page));
            }
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.client.get(&path).await
    }

    /// Get specific progress session
    pub async fn get_session(&self, session_id: Uuid) -> Result<ProgressSession> {
        let path = format!("/api/v1/progress/sessions/{}", session_id);
        self.client.get(&path).await
    }

    /// Create a new progress session
    pub async fn create_session(&self, request: CreateProgressSessionRequest) -> Result<ProgressSession> {
        self.client.post("/api/v1/progress/sessions", &request).await
    }

    /// Update progress session
    pub async fn update_session(&self, session_id: Uuid, update: ProgressSessionUpdate) -> Result<ProgressSession> {
        let path = format!("/api/v1/progress/sessions/{}", session_id);
        self.client.put(&path, &update).await
    }

    /// Delete progress session
    pub async fn delete_session(&self, session_id: Uuid) -> Result<()> {
        let path = format!("/api/v1/progress/sessions/{}", session_id);
        self.client.delete::<serde_json::Value>(&path).await?;
        Ok(())
    }

    /// Start progress session
    pub async fn start_session(&self, session_id: Uuid) -> Result<()> {
        let path = format!("/api/v1/progress/sessions/{}/start", session_id);
        self.client.post::<serde_json::Value, _>(&path, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Pause progress session
    pub async fn pause_session(&self, session_id: Uuid) -> Result<()> {
        let path = format!("/api/v1/progress/sessions/{}/pause", session_id);
        self.client.post::<serde_json::Value, _>(&path, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Resume progress session
    pub async fn resume_session(&self, session_id: Uuid) -> Result<()> {
        let path = format!("/api/v1/progress/sessions/{}/resume", session_id);
        self.client.post::<serde_json::Value, _>(&path, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Complete progress session
    pub async fn complete_session(&self, session_id: Uuid) -> Result<()> {
        let path = format!("/api/v1/progress/sessions/{}/complete", session_id);
        self.client.post::<serde_json::Value, _>(&path, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Fail progress session
    pub async fn fail_session(&self, session_id: Uuid, reason: Option<String>) -> Result<()> {
        let path = format!("/api/v1/progress/sessions/{}/fail", session_id);
        let body = serde_json::json!({ "reason": reason });
        self.client.post::<serde_json::Value, _>(&path, &body).await?;
        Ok(())
    }

    /// Get session events
    pub async fn get_events(&self, session_id: Uuid, params: Option<EventListParams>) -> Result<PaginatedResponse<ProgressEvent>> {
        let mut path = format!("/api/v1/progress/sessions/{}/events", session_id);
        let mut query_params = Vec::new();

        if let Some(params) = params {
            if let Some(event_type) = params.event_type {
                let event_str = match event_type {
                    ProgressEventType::SessionStarted => "session_started",
                    ProgressEventType::TaskStarted => "task_started",
                    ProgressEventType::TaskProgress => "task_progress",
                    ProgressEventType::TaskCompleted => "task_completed",
                    ProgressEventType::TaskFailed => "task_failed",
                    ProgressEventType::SessionCompleted => "session_completed",
                    ProgressEventType::SessionFailed => "session_failed",
                    ProgressEventType::MetricsUpdate => "metrics_update",
                    ProgressEventType::LogMessage => "log_message",
                };
                query_params.push(format!("event_type={}", event_str));
            }

            if let Some(since) = params.since {
                query_params.push(format!("since={}", since.to_rfc3339()));
            }

            if let Some(until) = params.until {
                query_params.push(format!("until={}", until.to_rfc3339()));
            }

            if let Some(page) = params.pagination.page {
                query_params.push(format!("page={}", page));
            }

            if let Some(per_page) = params.pagination.per_page {
                query_params.push(format!("per_page={}", per_page));
            }
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.client.get(&path).await
    }

    /// Send progress event
    pub async fn send_event(&self, session_id: Uuid, event: SendProgressEventRequest) -> Result<()> {
        let path = format!("/api/v1/progress/sessions/{}/events", session_id);
        self.client.post::<serde_json::Value, _>(&path, &event).await?;
        Ok(())
    }

    /// Get session metrics
    pub async fn get_metrics(&self, session_id: Uuid) -> Result<ProgressMetrics> {
        let path = format!("/api/v1/progress/sessions/{}/metrics", session_id);
        self.client.get(&path).await
    }

    /// Update session metrics
    pub async fn update_metrics(&self, session_id: Uuid, metrics: ProgressMetrics) -> Result<()> {
        let path = format!("/api/v1/progress/sessions/{}/metrics", session_id);
        self.client.put::<serde_json::Value, _>(&path, &metrics).await?;
        Ok(())
    }

    /// Get session logs
    pub async fn get_logs(&self, session_id: Uuid, params: Option<LogListParams>) -> Result<PaginatedResponse<ProgressLogEntry>> {
        let mut path = format!("/api/v1/progress/sessions/{}/logs", session_id);
        let mut query_params = Vec::new();

        if let Some(params) = params {
            if let Some(level) = params.level {
                let level_str = match level {
                    LogLevel::Debug => "debug",
                    LogLevel::Info => "info",
                    LogLevel::Warn => "warn",
                    LogLevel::Error => "error",
                };
                query_params.push(format!("level={}", level_str));
            }

            if let Some(since) = params.since {
                query_params.push(format!("since={}", since.to_rfc3339()));
            }

            if let Some(page) = params.pagination.page {
                query_params.push(format!("page={}", page));
            }

            if let Some(per_page) = params.pagination.per_page {
                query_params.push(format!("per_page={}", per_page));
            }
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.client.get(&path).await
    }

    /// Add log entry
    pub async fn add_log(&self, session_id: Uuid, log: AddLogRequest) -> Result<()> {
        let path = format!("/api/v1/progress/sessions/{}/logs", session_id);
        self.client.post::<serde_json::Value, _>(&path, &log).await?;
        Ok(())
    }

    /// Get progress statistics
    pub async fn get_stats(&self, project_id: Option<Uuid>) -> Result<ProgressStats> {
        let path = if let Some(project_id) = project_id {
            format!("/api/v1/progress/stats?project_id={}", project_id)
        } else {
            "/api/v1/progress/stats".to_string()
        };
        self.client.get(&path).await
    }

    /// Export session data
    pub async fn export_session(&self, session_id: Uuid, format: ExportFormat) -> Result<bytes::Bytes> {
        let format_str = match format {
            ExportFormat::Json => "json",
            ExportFormat::Csv => "csv",
            ExportFormat::Excel => "excel",
        };

        let path = format!("/api/v1/progress/sessions/{}/export?format={}", session_id, format_str);
        let url = self.client.base_url.join(&path)?;
        let response = self.client.client.get(url).send().await?;

        if response.status().is_success() {
            Ok(response.bytes().await?)
        } else {
            Err(crate::error::AionError::Api {
                status: response.status().as_u16(),
                message: "Failed to export session data".to_string(),
            })
        }
    }

    /// Subscribe to session updates via Server-Sent Events
    pub async fn subscribe_sse(&self, session_id: Uuid) -> Result<reqwest::Response> {
        let path = format!("/api/v1/progress/sessions/{}/subscribe", session_id);
        let url = self.client.base_url.join(&path)?;

        let response = self.client.client
            .get(url)
            .header("Accept", "text/event-stream")
            .header("Cache-Control", "no-cache")
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(crate::error::AionError::Api {
                status: response.status().as_u16(),
                message: "Failed to subscribe to session updates".to_string(),
            })
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProgressListParams {
    pub project_id: Option<Uuid>,
    pub status: Option<ProgressStatus>,
    pub pagination: PaginationParams,
}

#[derive(Debug, Clone, Default)]
pub struct EventListParams {
    pub event_type: Option<ProgressEventType>,
    pub since: Option<chrono::DateTime<chrono::Utc>>,
    pub until: Option<chrono::DateTime<chrono::Utc>>,
    pub pagination: PaginationParams,
}

#[derive(Debug, Clone, Default)]
pub struct LogListParams {
    pub level: Option<LogLevel>,
    pub since: Option<chrono::DateTime<chrono::Utc>>,
    pub pagination: PaginationParams,
}

#[derive(Debug, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub enum ExportFormat {
    Json,
    Csv,
    Excel,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateProgressSessionRequest {
    pub project_id: Uuid,
    pub name: String,
    pub total_tasks: u32,
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProgressSessionUpdate {
    pub name: Option<String>,
    pub current_task: Option<String>,
    pub completed_tasks: Option<u32>,
    pub total_tasks: Option<u32>,
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SendProgressEventRequest {
    pub event_type: ProgressEventType,
    pub data: serde_json::Value,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProgressLogEntry {
    pub id: Uuid,
    pub session_id: Uuid,
    pub level: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddLogRequest {
    pub level: String,
    pub message: String,
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProgressStats {
    pub total_sessions: u64,
    pub active_sessions: u64,
    pub completed_sessions: u64,
    pub failed_sessions: u64,
    pub average_duration_minutes: f32,
    pub average_tasks_per_session: f32,
    pub success_rate: f32,
    pub most_active_projects: Vec<ProjectActivity>,
    pub performance_trends: Vec<PerformanceTrend>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectActivity {
    pub project_id: Uuid,
    pub project_name: String,
    pub session_count: u64,
    pub average_duration_minutes: f32,
    pub success_rate: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceTrend {
    pub date: chrono::DateTime<chrono::Utc>,
    pub session_count: u64,
    pub average_duration_minutes: f32,
    pub success_rate: f32,
    pub average_cpu_usage: f32,
    pub average_memory_usage: u64,
}