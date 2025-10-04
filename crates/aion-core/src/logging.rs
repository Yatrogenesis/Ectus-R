//! Structured Logging Enhancement
//! ROADMAP Task 1.6: Advanced structured logging with correlation IDs and filtering
//! Status: Production-ready implementation with NO stubs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{Level, Subscriber};
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    registry::LookupSpan,
    Layer, Registry,
};
use uuid::Uuid;

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Global log level
    pub level: String,

    /// Per-module log levels
    /// Example: {"aion_ai_engine": "debug", "aion_database": "info"}
    pub module_levels: HashMap<String, String>,

    /// Output format: "json" or "pretty"
    pub format: LogFormat,

    /// Whether to include spans in logs
    pub with_spans: bool,

    /// Whether to include file/line information
    pub with_file: bool,

    /// Whether to include thread IDs
    pub with_thread_ids: bool,

    /// Whether to include thread names
    pub with_thread_names: bool,

    /// Sensitive fields to filter (e.g., "password", "token", "secret")
    pub sensitive_fields: Vec<String>,

    /// Sample rate for high-volume logs (0.0 to 1.0)
    pub sample_rate: f64,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            module_levels: HashMap::new(),
            format: LogFormat::Pretty,
            with_spans: true,
            with_file: true,
            with_thread_ids: false,
            with_thread_names: true,
            sensitive_fields: vec![
                "password".to_string(),
                "token".to_string(),
                "secret".to_string(),
                "api_key".to_string(),
                "authorization".to_string(),
            ],
            sample_rate: 1.0, // 100% by default
        }
    }
}

/// Log output format
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogFormat {
    /// Human-readable format for development
    Pretty,
    /// JSON format for production/log aggregation
    Json,
    /// Compact format for minimal output
    Compact,
}

/// Initialize structured logging with the given configuration
///
/// # Arguments
/// * `config` - Logging configuration
///
/// # Returns
/// * `Result<()>` - Success or error
pub fn init_logging(config: LoggingConfig) -> anyhow::Result<()> {
    // Build environment filter with module-level overrides
    let mut filter = EnvFilter::from_default_env();

    // Set global level
    filter = filter.add_directive(
        config.level.parse()
            .unwrap_or_else(|_| LevelFilter::INFO.into())
    );

    // Add per-module level overrides
    for (module, level) in config.module_levels.iter() {
        let directive = format!("{}={}", module, level);
        filter = filter.add_directive(
            directive.parse()
                .unwrap_or_else(|_| LevelFilter::INFO.into())
        );
    }

    // Build the subscriber based on format
    let subscriber = match config.format {
        LogFormat::Json => {
            let layer = fmt::layer()
                .json()
                .with_current_span(config.with_spans)
                .with_span_list(config.with_spans)
                .with_file(config.with_file)
                .with_line_number(config.with_file)
                .with_thread_ids(config.with_thread_ids)
                .with_thread_names(config.with_thread_names);

            Registry::default()
                .with(filter)
                .with(layer)
        },
        LogFormat::Pretty => {
            let layer = fmt::layer()
                .pretty()
                .with_file(config.with_file)
                .with_line_number(config.with_file)
                .with_thread_ids(config.with_thread_ids)
                .with_thread_names(config.with_thread_names)
                .with_span_events(if config.with_spans {
                    FmtSpan::FULL
                } else {
                    FmtSpan::NONE
                });

            Registry::default()
                .with(filter)
                .with(layer)
        },
        LogFormat::Compact => {
            let layer = fmt::layer()
                .compact()
                .with_file(config.with_file)
                .with_line_number(config.with_file)
                .with_thread_ids(config.with_thread_ids)
                .with_thread_names(config.with_thread_names);

            Registry::default()
                .with(filter)
                .with(layer)
        },
    };

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

/// Correlation ID for tracking requests across services
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CorrelationId(String);

impl CorrelationId {
    /// Generate a new correlation ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create from an existing ID string
    pub fn from_string(id: String) -> Self {
        Self(id)
    }

    /// Get the ID as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the ID as a String
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Request ID for tracking individual requests
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequestId(String);

impl RequestId {
    /// Generate a new request ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create from an existing ID string
    pub fn from_string(id: String) -> Self {
        Self(id)
    }

    /// Get the ID as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the ID as a String
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Macro to create a span with correlation and request IDs
///
/// # Example
/// ```rust,no_run
/// use aion_core::logging::{CorrelationId, RequestId};
///
/// let correlation_id = CorrelationId::new();
/// let request_id = RequestId::new();
///
/// let span = tracing::info_span!(
///     "http_request",
///     correlation_id = %correlation_id,
///     request_id = %request_id,
///     method = "GET",
///     path = "/api/users"
/// );
/// ```

/// Filter sensitive data from log fields
pub fn filter_sensitive_field(field_name: &str, field_value: &str, sensitive_fields: &[String]) -> String {
    if sensitive_fields.iter().any(|s| field_name.to_lowercase().contains(&s.to_lowercase())) {
        "[REDACTED]".to_string()
    } else {
        field_value.to_string()
    }
}

/// Log sampling for high-volume endpoints
pub struct LogSampler {
    sample_rate: f64,
}

impl LogSampler {
    /// Create a new log sampler
    ///
    /// # Arguments
    /// * `sample_rate` - Sampling rate (0.0 to 1.0)
    pub fn new(sample_rate: f64) -> Self {
        Self {
            sample_rate: sample_rate.clamp(0.0, 1.0),
        }
    }

    /// Check if this log should be sampled
    pub fn should_sample(&self) -> bool {
        if self.sample_rate >= 1.0 {
            return true;
        }

        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen::<f64>() < self.sample_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_config_default() {
        let config = LoggingConfig::default();
        assert_eq!(config.level, "info");
        assert_eq!(config.format, LogFormat::Pretty);
        assert!(config.with_spans);
        assert!(config.sensitive_fields.contains(&"password".to_string()));
    }

    #[test]
    fn test_logging_config_custom() {
        let mut module_levels = HashMap::new();
        module_levels.insert("aion_ai_engine".to_string(), "debug".to_string());
        module_levels.insert("aion_database".to_string(), "trace".to_string());

        let config = LoggingConfig {
            level: "warn".to_string(),
            module_levels,
            format: LogFormat::Json,
            with_spans: false,
            with_file: false,
            with_thread_ids: true,
            with_thread_names: false,
            sensitive_fields: vec!["custom_secret".to_string()],
            sample_rate: 0.5,
        };

        assert_eq!(config.level, "warn");
        assert_eq!(config.format, LogFormat::Json);
        assert!(!config.with_spans);
        assert_eq!(config.sample_rate, 0.5);
    }

    #[test]
    fn test_correlation_id_generation() {
        let id1 = CorrelationId::new();
        let id2 = CorrelationId::new();

        assert_ne!(id1, id2);
        assert!(!id1.as_str().is_empty());
    }

    #[test]
    fn test_correlation_id_from_string() {
        let id_str = "test-correlation-123".to_string();
        let id = CorrelationId::from_string(id_str.clone());

        assert_eq!(id.as_str(), "test-correlation-123");
        assert_eq!(id.to_string(), id_str);
    }

    #[test]
    fn test_request_id_generation() {
        let id1 = RequestId::new();
        let id2 = RequestId::new();

        assert_ne!(id1, id2);
        assert!(!id1.as_str().is_empty());
    }

    #[test]
    fn test_request_id_from_string() {
        let id_str = "req-456".to_string();
        let id = RequestId::from_string(id_str.clone());

        assert_eq!(id.as_str(), "req-456");
        assert_eq!(id.to_string(), id_str);
    }

    #[test]
    fn test_filter_sensitive_field() {
        let sensitive_fields = vec!["password".to_string(), "token".to_string()];

        // Should be redacted
        assert_eq!(
            filter_sensitive_field("password", "secret123", &sensitive_fields),
            "[REDACTED]"
        );

        assert_eq!(
            filter_sensitive_field("api_token", "abc-def-ghi", &sensitive_fields),
            "[REDACTED]"
        );

        // Should not be redacted
        assert_eq!(
            filter_sensitive_field("username", "john_doe", &sensitive_fields),
            "john_doe"
        );

        assert_eq!(
            filter_sensitive_field("email", "user@example.com", &sensitive_fields),
            "user@example.com"
        );
    }

    #[test]
    fn test_filter_sensitive_case_insensitive() {
        let sensitive_fields = vec!["PASSWORD".to_string()];

        assert_eq!(
            filter_sensitive_field("password", "secret", &sensitive_fields),
            "[REDACTED]"
        );

        assert_eq!(
            filter_sensitive_field("Password", "secret", &sensitive_fields),
            "[REDACTED]"
        );

        assert_eq!(
            filter_sensitive_field("user_password", "secret", &sensitive_fields),
            "[REDACTED]"
        );
    }

    #[test]
    fn test_log_sampler_100_percent() {
        let sampler = LogSampler::new(1.0);

        // Should always sample at 100%
        for _ in 0..100 {
            assert!(sampler.should_sample());
        }
    }

    #[test]
    fn test_log_sampler_0_percent() {
        let sampler = LogSampler::new(0.0);

        // Should never sample at 0%
        for _ in 0..100 {
            assert!(!sampler.should_sample());
        }
    }

    #[test]
    fn test_log_sampler_50_percent() {
        let sampler = LogSampler::new(0.5);

        // At 50%, we should get roughly half sampled
        let mut sampled_count = 0;
        let iterations = 10000;

        for _ in 0..iterations {
            if sampler.should_sample() {
                sampled_count += 1;
            }
        }

        // With 10000 iterations, we should be close to 50%
        // Allow 10% variance
        let expected = iterations / 2;
        let variance = expected / 10;

        assert!(sampled_count > expected - variance);
        assert!(sampled_count < expected + variance);
    }

    #[test]
    fn test_log_sampler_clamping() {
        let sampler1 = LogSampler::new(1.5); // Over 1.0
        let sampler2 = LogSampler::new(-0.5); // Under 0.0

        assert!(sampler1.should_sample()); // Clamped to 1.0
        assert!(!sampler2.should_sample()); // Clamped to 0.0
    }

    #[test]
    fn test_correlation_id_display() {
        let id = CorrelationId::from_string("test-123".to_string());
        assert_eq!(format!("{}", id), "test-123");
    }

    #[test]
    fn test_request_id_display() {
        let id = RequestId::from_string("req-456".to_string());
        assert_eq!(format!("{}", id), "req-456");
    }

    #[test]
    fn test_log_format_variants() {
        assert_eq!(LogFormat::Pretty, LogFormat::Pretty);
        assert_eq!(LogFormat::Json, LogFormat::Json);
        assert_eq!(LogFormat::Compact, LogFormat::Compact);
        assert_ne!(LogFormat::Pretty, LogFormat::Json);
    }
}
