use crate::{types::*, error::Result, client::AionClient};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct QaApi {
    client: AionClient,
}

impl QaApi {
    pub(crate) fn new(client: AionClient) -> Self {
        Self { client }
    }

    /// Start a new QA session
    pub async fn start_session(&self, request: QARequest) -> Result<QASession> {
        self.client.post("/api/v1/qa/sessions", &request).await
    }

    /// Get QA session details
    pub async fn get_session(&self, session_id: Uuid) -> Result<QASession> {
        let path = format!("/api/v1/qa/sessions/{}", session_id);
        self.client.get(&path).await
    }

    /// List QA sessions for a project
    pub async fn list_sessions(&self, project_id: Uuid, params: Option<PaginationParams>) -> Result<PaginatedResponse<QASession>> {
        let mut path = format!("/api/v1/qa/sessions?project_id={}", project_id);

        if let Some(params) = params {
            if let Some(page) = params.page {
                path.push_str(&format!("&page={}", page));
            }
            if let Some(per_page) = params.per_page {
                path.push_str(&format!("&per_page={}", per_page));
            }
        }

        self.client.get(&path).await
    }

    /// Stop a running QA session
    pub async fn stop_session(&self, session_id: Uuid) -> Result<()> {
        let path = format!("/api/v1/qa/sessions/{}/stop", session_id);
        self.client.post::<serde_json::Value, _>(&path, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Cancel a QA session
    pub async fn cancel_session(&self, session_id: Uuid) -> Result<()> {
        let path = format!("/api/v1/qa/sessions/{}/cancel", session_id);
        self.client.post::<serde_json::Value, _>(&path, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Get session results
    pub async fn get_results(&self, session_id: Uuid) -> Result<QAResults> {
        let path = format!("/api/v1/qa/sessions/{}/results", session_id);
        self.client.get(&path).await
    }

    /// Get session logs
    pub async fn get_logs(&self, session_id: Uuid, params: Option<LogParams>) -> Result<PaginatedResponse<QALogEntry>> {
        let mut path = format!("/api/v1/qa/sessions/{}/logs", session_id);
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

    /// Run unit tests
    pub async fn run_unit_tests(&self, project_id: Uuid, config: Option<QAConfiguration>) -> Result<QASession> {
        let request = QARequest {
            project_id,
            test_type: QATestType::Unit,
            configuration: config,
        };
        self.start_session(request).await
    }

    /// Run integration tests
    pub async fn run_integration_tests(&self, project_id: Uuid, config: Option<QAConfiguration>) -> Result<QASession> {
        let request = QARequest {
            project_id,
            test_type: QATestType::Integration,
            configuration: config,
        };
        self.start_session(request).await
    }

    /// Run end-to-end tests
    pub async fn run_e2e_tests(&self, project_id: Uuid, config: Option<QAConfiguration>) -> Result<QASession> {
        let request = QARequest {
            project_id,
            test_type: QATestType::E2E,
            configuration: config,
        };
        self.start_session(request).await
    }

    /// Run performance tests
    pub async fn run_performance_tests(&self, project_id: Uuid, config: Option<QAConfiguration>) -> Result<QASession> {
        let request = QARequest {
            project_id,
            test_type: QATestType::Performance,
            configuration: config,
        };
        self.start_session(request).await
    }

    /// Run security tests
    pub async fn run_security_tests(&self, project_id: Uuid, config: Option<QAConfiguration>) -> Result<QASession> {
        let request = QARequest {
            project_id,
            test_type: QATestType::Security,
            configuration: config,
        };
        self.start_session(request).await
    }

    /// Run accessibility tests
    pub async fn run_accessibility_tests(&self, project_id: Uuid, config: Option<QAConfiguration>) -> Result<QASession> {
        let request = QARequest {
            project_id,
            test_type: QATestType::Accessibility,
            configuration: config,
        };
        self.start_session(request).await
    }

    /// Run comprehensive test suite
    pub async fn run_comprehensive_tests(&self, project_id: Uuid, config: Option<QAConfiguration>) -> Result<QASession> {
        let request = QARequest {
            project_id,
            test_type: QATestType::Comprehensive,
            configuration: config,
        };
        self.start_session(request).await
    }

    /// Get test coverage report
    pub async fn get_coverage(&self, project_id: Uuid) -> Result<CoverageReport> {
        let path = format!("/api/v1/qa/projects/{}/coverage", project_id);
        self.client.get(&path).await
    }

    /// Get quality metrics
    pub async fn get_quality_metrics(&self, project_id: Uuid) -> Result<QualityMetrics> {
        let path = format!("/api/v1/qa/projects/{}/quality", project_id);
        self.client.get(&path).await
    }

    /// Get test recommendations
    pub async fn get_recommendations(&self, project_id: Uuid) -> Result<Vec<TestRecommendation>> {
        let path = format!("/api/v1/qa/projects/{}/recommendations", project_id);
        self.client.get(&path).await
    }

    /// Generate test cases
    pub async fn generate_tests(&self, request: TestGenerationRequest) -> Result<TestGenerationResponse> {
        self.client.post("/api/v1/qa/generate-tests", &request).await
    }

    /// Validate test configuration
    pub async fn validate_config(&self, config: QAConfiguration) -> Result<ConfigValidationResult> {
        self.client.post("/api/v1/qa/validate-config", &config).await
    }

    /// Get QA statistics
    pub async fn get_stats(&self, project_id: Option<Uuid>) -> Result<QAStats> {
        let path = if let Some(project_id) = project_id {
            format!("/api/v1/qa/stats?project_id={}", project_id)
        } else {
            "/api/v1/qa/stats".to_string()
        };
        self.client.get(&path).await
    }

    /// Export test results
    pub async fn export_results(&self, session_id: Uuid, format: ExportFormat) -> Result<bytes::Bytes> {
        let format_str = match format {
            ExportFormat::Json => "json",
            ExportFormat::Xml => "xml",
            ExportFormat::Html => "html",
            ExportFormat::Pdf => "pdf",
        };

        let path = format!("/api/v1/qa/sessions/{}/export?format={}", session_id, format_str);
        let url = self.client.base_url().join(&path)?;
        let response = self.client.http_client().get(url).send().await?;

        if response.status().is_success() {
            Ok(response.bytes().await?)
        } else {
            Err(crate::error::AionError::Api {
                status: response.status().as_u16(),
                message: "Failed to export results".to_string(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogParams {
    pub level: Option<LogLevel>,
    pub pagination: PaginationParams,
}

#[derive(Debug, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QALogEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: String,
    pub message: String,
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoverageReport {
    pub overall_coverage: f32,
    pub line_coverage: f32,
    pub branch_coverage: f32,
    pub function_coverage: f32,
    pub files: Vec<FileCoverage>,
    pub uncovered_lines: Vec<UncoveredLine>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileCoverage {
    pub file_path: String,
    pub line_coverage: f32,
    pub branch_coverage: f32,
    pub function_coverage: f32,
    pub total_lines: u32,
    pub covered_lines: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UncoveredLine {
    pub file_path: String,
    pub line_number: u32,
    pub line_content: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QualityMetrics {
    pub maintainability_index: f32,
    pub cyclomatic_complexity: f32,
    pub code_duplication: f32,
    pub technical_debt_ratio: f32,
    pub test_coverage: f32,
    pub security_rating: SecurityRating,
    pub reliability_rating: ReliabilityRating,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SecurityRating {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "B")]
    B,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "D")]
    D,
    #[serde(rename = "E")]
    E,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ReliabilityRating {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "B")]
    B,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "D")]
    D,
    #[serde(rename = "E")]
    E,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestRecommendation {
    pub category: RecommendationCategory,
    pub priority: RecommendationPriority,
    pub title: String,
    pub description: String,
    pub suggested_action: String,
    pub estimated_effort: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RecommendationCategory {
    #[serde(rename = "coverage")]
    Coverage,
    #[serde(rename = "performance")]
    Performance,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "maintainability")]
    Maintainability,
    #[serde(rename = "best_practices")]
    BestPractices,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RecommendationPriority {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestGenerationRequest {
    pub project_id: Uuid,
    pub file_patterns: Vec<String>,
    pub test_types: Vec<QATestType>,
    pub options: TestGenerationOptions,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestGenerationOptions {
    pub include_edge_cases: bool,
    pub generate_mocks: bool,
    pub coverage_target: Option<f32>,
    pub test_framework: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestGenerationResponse {
    pub generated_files: Vec<GeneratedTestFile>,
    pub statistics: TestGenerationStats,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneratedTestFile {
    pub file_path: String,
    pub content: String,
    pub test_count: u32,
    pub coverage_estimate: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestGenerationStats {
    pub total_files: u32,
    pub total_tests: u32,
    pub estimated_coverage: f32,
    pub generation_time_ms: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfigValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QAStats {
    pub total_sessions: u64,
    pub total_tests_run: u64,
    pub average_coverage: f32,
    pub success_rate: f32,
    pub most_common_issues: Vec<IssueStats>,
    pub performance_trends: Vec<PerformanceTrend>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IssueStats {
    pub category: IssueCategory,
    pub count: u64,
    pub percentage: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceTrend {
    pub date: chrono::DateTime<chrono::Utc>,
    pub average_duration: f32,
    pub success_rate: f32,
}

#[derive(Debug, Clone)]
pub enum ExportFormat {
    Json,
    Xml,
    Html,
    Pdf,
}