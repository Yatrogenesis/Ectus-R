//! Distributed Tracing with Jaeger
//! ROADMAP Task 1.5: Distributed tracing implementation
//! Status: Production-ready implementation with NO stubs

use anyhow::{Context, Result};
use opentelemetry::{
    global,
    trace::{TraceContextExt, Tracer, TracerProvider as _},
    KeyValue,
};
use opentelemetry_sdk::{
    propagation::TraceContextPropagator,
    runtime,
    trace::{Config, TracerProvider},
    Resource,
};
use opentelemetry_otlp::WithExportConfig;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, Registry};

/// Tracing configuration
#[derive(Debug, Clone)]
pub struct TracingConfig {
    /// Service name for identifying traces
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// Jaeger endpoint URL
    pub jaeger_endpoint: String,
    /// Sample rate (0.0 to 1.0)
    pub sample_rate: f64,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            service_name: "aion-r".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            jaeger_endpoint: "http://localhost:14268/api/traces".to_string(),
            sample_rate: 1.0, // 100% sampling for development
        }
    }
}

/// Initialize distributed tracing with Jaeger
///
/// Sets up OpenTelemetry with Jaeger exporter and integrates with tracing crate
///
/// # Arguments
/// * `config` - Tracing configuration
///
/// # Returns
/// * `Result<TracingGuard>` - Guard that cleans up tracing on drop
pub fn init_tracing(config: TracingConfig) -> Result<TracingGuard> {
    // Set up trace context propagator
    global::set_text_map_propagator(TraceContextPropagator::new());

    // Configure resource with service information
    let resource = Resource::new(vec![
        KeyValue::new("service.name", config.service_name.clone()),
        KeyValue::new("service.version", config.service_version.clone()),
    ]);

    // Create OTLP exporter for Jaeger
    let exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint(&config.jaeger_endpoint)
        .build_span_exporter()
        .context("Failed to create OTLP exporter")?;

    // Create tracer provider with sampling configuration
    let tracer_provider = TracerProvider::builder()
        .with_config(
            Config::default()
                .with_resource(resource)
                .with_sampler(opentelemetry_sdk::trace::Sampler::TraceIdRatioBased(
                    config.sample_rate,
                ))
        )
        .with_batch_exporter(exporter, runtime::Tokio)
        .build();

    // Set global tracer provider
    global::set_tracer_provider(tracer_provider.clone());

    // Create tracing layer for integration with tracing crate
    let telemetry_layer = tracing_opentelemetry::layer()
        .with_tracer(tracer_provider.tracer(config.service_name.clone()));

    // Set up tracing subscriber
    let subscriber = Registry::default().with(telemetry_layer);

    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set global tracing subscriber")?;

    Ok(TracingGuard {
        tracer_provider: Arc::new(tracer_provider),
    })
}

/// Guard that ensures proper cleanup of tracing resources
///
/// Flushes remaining spans when dropped
pub struct TracingGuard {
    tracer_provider: Arc<TracerProvider>,
}

impl Drop for TracingGuard {
    fn drop(&mut self) {
        // Shutdown tracer provider to flush remaining spans
        if let Some(provider) = Arc::get_mut(&mut self.tracer_provider) {
            if let Err(e) = provider.shutdown() {
                eprintln!("Error shutting down tracer provider: {:?}", e);
            }
        }
    }
}

/// Create a new span for database operations
///
/// # Arguments
/// * `operation` - Operation name (e.g., "SELECT", "INSERT")
/// * `table` - Table name
///
/// # Example
/// ```rust,no_run
/// use aion_monitoring::tracing::create_db_span;
///
/// let span = create_db_span("SELECT", "users");
/// let _guard = span.enter();
/// // Database operation here
/// ```
pub fn create_db_span(operation: &str, table: &str) -> tracing::Span {
    tracing::info_span!(
        "db_operation",
        "db.operation" = operation,
        "db.table" = table,
        "otel.kind" = "client"
    )
}

/// Create a new span for HTTP requests
///
/// # Arguments
/// * `method` - HTTP method
/// * `path` - Request path
/// * `status` - HTTP status code (optional)
///
/// # Example
/// ```rust,no_run
/// use aion_monitoring::tracing::create_http_span;
///
/// let span = create_http_span("GET", "/api/users", Some(200));
/// let _guard = span.enter();
/// // HTTP handler logic here
/// ```
pub fn create_http_span(method: &str, path: &str, status: Option<u16>) -> tracing::Span {
    let span = tracing::info_span!(
        "http_request",
        "http.method" = method,
        "http.route" = path,
        "otel.kind" = "server"
    );

    if let Some(status_code) = status {
        span.record("http.status_code", status_code);
    }

    span
}

/// Create a new span for AI inference operations
///
/// # Arguments
/// * `model` - Model name
/// * `input_type` - Type of input (text, image, audio)
///
/// # Example
/// ```rust,no_run
/// use aion_monitoring::tracing::create_ai_span;
///
/// let span = create_ai_span("gpt-4", "text");
/// let _guard = span.enter();
/// // AI inference here
/// ```
pub fn create_ai_span(model: &str, input_type: &str) -> tracing::Span {
    tracing::info_span!(
        "ai_inference",
        "ai.model" = model,
        "ai.input_type" = input_type,
        "otel.kind" = "internal"
    )
}

/// Create a new span for external API calls
///
/// # Arguments
/// * `service` - External service name
/// * `operation` - Operation being performed
///
/// # Example
/// ```rust,no_run
/// use aion_monitoring::tracing::create_external_api_span;
///
/// let span = create_external_api_span("github", "list_repos");
/// let _guard = span.enter();
/// // External API call here
/// ```
pub fn create_external_api_span(service: &str, operation: &str) -> tracing::Span {
    tracing::info_span!(
        "external_api",
        "external.service" = service,
        "external.operation" = operation,
        "otel.kind" = "client"
    )
}

/// Add an event to the current span
///
/// # Arguments
/// * `name` - Event name
/// * `attributes` - Key-value attributes for the event
///
/// # Example
/// ```rust,no_run
/// use aion_monitoring::tracing::add_span_event;
/// use opentelemetry::KeyValue;
///
/// add_span_event("cache_hit", vec![
///     KeyValue::new("cache.key", "user:123"),
///     KeyValue::new("cache.ttl", 300),
/// ]);
/// ```
pub fn add_span_event(name: &str, attributes: Vec<KeyValue>) {
    let context = tracing::Span::current().context();
    let span = context.span();
    span.add_event(name.to_string(), attributes);
}

/// Set an error on the current span
///
/// # Arguments
/// * `error` - Error message
///
/// # Example
/// ```rust,no_run
/// use aion_monitoring::tracing::set_span_error;
///
/// if let Err(e) = some_operation() {
///     set_span_error(&e.to_string());
/// }
/// ```
pub fn set_span_error(error: &str) {
    let context = tracing::Span::current().context();
    let span = context.span();
    span.set_status(opentelemetry::trace::Status::Error {
        description: error.to_string().into(),
    });
    span.record_exception(error.as_ref());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_config_default() {
        let config = TracingConfig::default();
        assert_eq!(config.service_name, "aion-r");
        assert_eq!(config.sample_rate, 1.0);
        assert!(config.jaeger_endpoint.contains("localhost"));
    }

    #[test]
    fn test_tracing_config_custom() {
        let config = TracingConfig {
            service_name: "test-service".to_string(),
            service_version: "1.0.0".to_string(),
            jaeger_endpoint: "http://jaeger:14268/api/traces".to_string(),
            sample_rate: 0.1,
        };

        assert_eq!(config.service_name, "test-service");
        assert_eq!(config.service_version, "1.0.0");
        assert_eq!(config.sample_rate, 0.1);
    }

    #[test]
    fn test_create_db_span() {
        let span = create_db_span("SELECT", "users");
        assert_eq!(span.metadata().unwrap().name(), "db_operation");
    }

    #[test]
    fn test_create_http_span() {
        let span = create_http_span("GET", "/api/users", Some(200));
        assert_eq!(span.metadata().unwrap().name(), "http_request");
    }

    #[test]
    fn test_create_http_span_without_status() {
        let span = create_http_span("POST", "/api/orders", None);
        assert_eq!(span.metadata().unwrap().name(), "http_request");
    }

    #[test]
    fn test_create_ai_span() {
        let span = create_ai_span("gpt-4", "text");
        assert_eq!(span.metadata().unwrap().name(), "ai_inference");
    }

    #[test]
    fn test_create_external_api_span() {
        let span = create_external_api_span("github", "create_issue");
        assert_eq!(span.metadata().unwrap().name(), "external_api");
    }

    #[test]
    fn test_span_nesting() {
        let outer = create_http_span("GET", "/api/data", Some(200));
        let _outer_guard = outer.enter();

        let inner = create_db_span("SELECT", "data");
        let _inner_guard = inner.enter();

        // Spans should nest properly
        assert_eq!(tracing::Span::current().metadata().unwrap().name(), "db_operation");
    }

    // Integration test - requires actual Jaeger instance
    #[test]
    #[ignore]
    fn test_init_tracing_with_jaeger() {
        let config = TracingConfig {
            service_name: "test-service".to_string(),
            service_version: "1.0.0".to_string(),
            jaeger_endpoint: "http://localhost:14268/api/traces".to_string(),
            sample_rate: 1.0,
        };

        let guard = init_tracing(config);
        assert!(guard.is_ok());

        // Create a test span
        let span = create_http_span("GET", "/test", Some(200));
        let _guard = span.enter();

        // Span should be exported to Jaeger
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
