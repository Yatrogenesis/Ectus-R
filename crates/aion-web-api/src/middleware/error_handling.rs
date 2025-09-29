//! Advanced Error Handling and Graceful Degradation
//! Provides comprehensive error handling for production reliability

use axum::{
    extract::Request,
    http::{StatusCode, HeaderValue},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tracing::{error, warn, info};
use uuid::Uuid;

/// Comprehensive error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error_id: Uuid,
    pub error_code: String,
    pub message: String,
    pub details: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub path: Option<String>,
    pub request_id: Option<String>,
    pub suggestions: Vec<String>,
    pub retry_after: Option<u64>,
}

/// Error categories for different handling strategies
#[derive(Debug, Clone)]
pub enum ErrorCategory {
    UserInput,       // 4xx errors - user can fix
    SystemError,     // 5xx errors - system issue
    RateLimit,       // 429 - too many requests
    Authentication,  // 401/403 - auth issues
    NotFound,        // 404 - resource not found
    Validation,      // 422 - validation failed
    ServiceUnavailable, // 503 - temporary failure
}

/// Circuit breaker states
#[derive(Debug, Clone)]
pub enum CircuitState {
    Closed,    // Normal operation
    Open,      // Failing, rejecting requests
    HalfOpen,  // Testing if service recovered
}

/// Circuit breaker for graceful degradation
#[derive(Debug)]
pub struct CircuitBreaker {
    pub state: CircuitState,
    pub failure_count: u32,
    pub success_count: u32,
    pub last_failure_time: Option<Instant>,
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
    pub success_threshold: u32,
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
            success_threshold: 3,
        }
    }
}

impl CircuitBreaker {
    /// Record a successful operation
    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.success_threshold {
                    self.state = CircuitState::Closed;
                    self.failure_count = 0;
                    self.success_count = 0;
                    info!("Circuit breaker closed - service recovered");
                }
            }
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            CircuitState::Open => {}
        }
    }

    /// Record a failed operation
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(Instant::now());

        if self.failure_count >= self.failure_threshold {
            self.state = CircuitState::Open;
            warn!("Circuit breaker opened due to {} failures", self.failure_count);
        }
    }

    /// Check if the circuit should allow requests
    pub fn should_allow_request(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if let Some(last_failure) = self.last_failure_time {
                    if last_failure.elapsed() >= self.recovery_timeout {
                        self.state = CircuitState::HalfOpen;
                        self.success_count = 0;
                        info!("Circuit breaker half-open - testing service");
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            }
            CircuitState::HalfOpen => true,
        }
    }
}

/// Global error handler middleware
pub async fn error_handler_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start_time = Instant::now();

    // Extract request ID if present
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|h| h.to_str().ok())
        .map(String::from);

    let response = next.run(request).await;
    let duration = start_time.elapsed();

    // Log request details
    info!(
        method = %method,
        uri = %uri,
        status = %response.status(),
        duration_ms = duration.as_millis(),
        request_id = request_id.as_deref().unwrap_or("none"),
        "Request processed"
    );

    // Add security headers
    let mut response = response;
    let headers = response.headers_mut();

    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff")
    );
    headers.insert(
        "X-Frame-Options",
        HeaderValue::from_static("DENY")
    );
    headers.insert(
        "X-XSS-Protection",
        HeaderValue::from_static("1; mode=block")
    );
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin")
    );

    if let Some(request_id) = request_id {
        headers.insert(
            "X-Request-ID",
            HeaderValue::from_str(&request_id).unwrap_or(HeaderValue::from_static("invalid"))
        );
    }

    Ok(response)
}

/// Convert various error types to standardized error responses
pub fn create_error_response(
    error: &anyhow::Error,
    category: ErrorCategory,
    path: Option<String>,
    request_id: Option<String>,
) -> (StatusCode, Json<ErrorResponse>) {
    let error_id = Uuid::new_v4();
    let timestamp = chrono::Utc::now();

    let (status_code, error_code, message, suggestions, retry_after) = match category {
        ErrorCategory::UserInput => (
            StatusCode::BAD_REQUEST,
            "USER_INPUT_ERROR".to_string(),
            "Invalid input provided".to_string(),
            vec![
                "Check your request format".to_string(),
                "Ensure all required fields are provided".to_string(),
                "Verify data types and constraints".to_string(),
            ],
            None,
        ),
        ErrorCategory::Authentication => (
            StatusCode::UNAUTHORIZED,
            "AUTHENTICATION_ERROR".to_string(),
            "Authentication required or invalid".to_string(),
            vec![
                "Provide a valid access token".to_string(),
                "Check if your token has expired".to_string(),
                "Re-authenticate if necessary".to_string(),
            ],
            None,
        ),
        ErrorCategory::NotFound => (
            StatusCode::NOT_FOUND,
            "RESOURCE_NOT_FOUND".to_string(),
            "Requested resource not found".to_string(),
            vec![
                "Check the resource ID or path".to_string(),
                "Ensure the resource exists".to_string(),
                "Verify your permissions".to_string(),
            ],
            None,
        ),
        ErrorCategory::RateLimit => (
            StatusCode::TOO_MANY_REQUESTS,
            "RATE_LIMIT_EXCEEDED".to_string(),
            "Too many requests".to_string(),
            vec![
                "Reduce request frequency".to_string(),
                "Implement exponential backoff".to_string(),
                "Consider upgrading your plan".to_string(),
            ],
            Some(60), // Retry after 60 seconds
        ),
        ErrorCategory::Validation => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "VALIDATION_ERROR".to_string(),
            "Request validation failed".to_string(),
            vec![
                "Check field formats and constraints".to_string(),
                "Ensure all required fields are present".to_string(),
                "Verify enum values are valid".to_string(),
            ],
            None,
        ),
        ErrorCategory::ServiceUnavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            "SERVICE_UNAVAILABLE".to_string(),
            "Service temporarily unavailable".to_string(),
            vec![
                "Try again in a few moments".to_string(),
                "Check service status page".to_string(),
                "Contact support if issue persists".to_string(),
            ],
            Some(30), // Retry after 30 seconds
        ),
        ErrorCategory::SystemError => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR".to_string(),
            "Internal server error occurred".to_string(),
            vec![
                "Try again later".to_string(),
                "Contact support with error ID".to_string(),
                "Check service status".to_string(),
            ],
            Some(60),
        ),
    };

    // Log error with context
    error!(
        error_id = %error_id,
        error_code = %error_code,
        path = path.as_deref().unwrap_or("unknown"),
        request_id = request_id.as_deref().unwrap_or("none"),
        error = %error,
        "Error occurred"
    );

    let error_response = ErrorResponse {
        error_id,
        error_code,
        message,
        details: Some(error.to_string()),
        timestamp,
        path,
        request_id,
        suggestions,
        retry_after,
    };

    (status_code, Json(error_response))
}

/// Graceful degradation helper
pub struct GracefulDegradation {
    pub ai_circuit: CircuitBreaker,
    pub db_circuit: CircuitBreaker,
    pub monitoring_circuit: CircuitBreaker,
}

impl Default for GracefulDegradation {
    fn default() -> Self {
        Self {
            ai_circuit: CircuitBreaker::default(),
            db_circuit: CircuitBreaker::default(),
            monitoring_circuit: CircuitBreaker::default(),
        }
    }
}

impl GracefulDegradation {
    /// Check if AI service is available
    pub fn is_ai_available(&mut self) -> bool {
        self.ai_circuit.should_allow_request()
    }

    /// Check if database is available
    pub fn is_database_available(&mut self) -> bool {
        self.db_circuit.should_allow_request()
    }

    /// Check if monitoring is available
    pub fn is_monitoring_available(&mut self) -> bool {
        self.monitoring_circuit.should_allow_request()
    }

    /// Record AI service result
    pub fn record_ai_result(&mut self, success: bool) {
        if success {
            self.ai_circuit.record_success();
        } else {
            self.ai_circuit.record_failure();
        }
    }

    /// Record database result
    pub fn record_db_result(&mut self, success: bool) {
        if success {
            self.db_circuit.record_success();
        } else {
            self.db_circuit.record_failure();
        }
    }

    /// Record monitoring result
    pub fn record_monitoring_result(&mut self, success: bool) {
        if success {
            self.monitoring_circuit.record_success();
        } else {
            self.monitoring_circuit.record_failure();
        }
    }

    /// Get fallback response when service is unavailable
    pub fn get_fallback_response(&self, service: &str) -> Json<serde_json::Value> {
        let response = match service {
            "ai" => serde_json::json!({
                "status": "degraded",
                "message": "AI service temporarily unavailable",
                "fallback": true,
                "estimated_recovery": "1-2 minutes"
            }),
            "database" => serde_json::json!({
                "status": "degraded",
                "message": "Database service temporarily unavailable",
                "fallback": true,
                "cached_data": true
            }),
            "monitoring" => serde_json::json!({
                "status": "degraded",
                "message": "Monitoring service temporarily unavailable",
                "fallback": true,
                "basic_metrics": true
            }),
            _ => serde_json::json!({
                "status": "degraded",
                "message": "Service temporarily unavailable",
                "fallback": true
            }),
        };

        Json(response)
    }
}

/// Timeout handler for requests
pub async fn timeout_middleware(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    // Set timeout based on endpoint
    let timeout_duration = if request.uri().path().contains("/ai/") {
        Duration::from_secs(120) // AI operations need more time
    } else if request.uri().path().contains("/dashboard/") {
        Duration::from_secs(30) // Dashboard should be fast
    } else {
        Duration::from_secs(60) // Default timeout
    };

    match tokio::time::timeout(timeout_duration, next.run(request)).await {
        Ok(response) => Ok(response),
        Err(_) => {
            let error = anyhow::anyhow!("Request timeout after {} seconds", timeout_duration.as_secs());
            let (status, response) = create_error_response(
                &error,
                ErrorCategory::ServiceUnavailable,
                None,
                None,
            );
            Err((status, response))
        }
    }
}

/// Health check endpoint that considers circuit breaker states
pub async fn health_check_with_degradation(
    degradation: &GracefulDegradation,
) -> Json<serde_json::Value> {
    let ai_status = match degradation.ai_circuit.state {
        CircuitState::Closed => "healthy",
        CircuitState::HalfOpen => "recovering",
        CircuitState::Open => "unhealthy",
    };

    let db_status = match degradation.db_circuit.state {
        CircuitState::Closed => "healthy",
        CircuitState::HalfOpen => "recovering",
        CircuitState::Open => "unhealthy",
    };

    let monitoring_status = match degradation.monitoring_circuit.state {
        CircuitState::Closed => "healthy",
        CircuitState::HalfOpen => "recovering",
        CircuitState::Open => "unhealthy",
    };

    let overall_status = if ai_status == "healthy" && db_status == "healthy" && monitoring_status == "healthy" {
        "healthy"
    } else if ai_status == "unhealthy" || db_status == "unhealthy" {
        "unhealthy"
    } else {
        "degraded"
    };

    Json(serde_json::json!({
        "status": overall_status,
        "timestamp": chrono::Utc::now(),
        "services": {
            "ai_engine": {
                "status": ai_status,
                "failures": degradation.ai_circuit.failure_count
            },
            "database": {
                "status": db_status,
                "failures": degradation.db_circuit.failure_count
            },
            "monitoring": {
                "status": monitoring_status,
                "failures": degradation.monitoring_circuit.failure_count
            }
        },
        "degraded_features": match overall_status {
            "degraded" => vec!["ai_generation", "real_time_metrics"],
            "unhealthy" => vec!["ai_generation", "real_time_metrics", "user_data"],
            _ => vec![]
        }
    }))
}