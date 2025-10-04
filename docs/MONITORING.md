# AION-R Monitoring & Observability Guide

**Version:** 2.0
**Last Updated:** 2025-10-04
**Status:** Production-Ready

---

## Table of Contents

1. [Overview](#overview)
2. [Metrics Collection](#metrics-collection)
3. [Distributed Tracing](#distributed-tracing)
4. [Structured Logging](#structured-logging)
5. [Alerting](#alerting)
6. [Dashboards](#dashboards)
7. [Deployment](#deployment)
8. [Usage Examples](#usage-examples)
9. [Troubleshooting](#troubleshooting)

---

## Overview

AION-R implements a comprehensive observability stack following production best practices:

- **Metrics:** Prometheus with custom business metrics
- **Tracing:** Jaeger with OpenTelemetry
- **Logging:** Structured logging with correlation IDs
- **Alerting:** Multi-channel alerting with PagerDuty, Slack, Email
- **Visualization:** Grafana dashboards

### Architecture

```
┌─────────────┐
│  AION Web   │────┐
│     API     │    │
└─────────────┘    │
                   │
┌─────────────┐    │    ┌──────────────┐    ┌─────────────┐
│  AION AI    │────┼───→│  Prometheus  │───→│   Grafana   │
│   Engine    │    │    └──────────────┘    └─────────────┘
└─────────────┘    │            │
                   │            ↓
┌─────────────┐    │    ┌──────────────┐
│  Database   │────┘    │ AlertManager │
└─────────────┘         └──────────────┘
        │                       │
        │                       ↓
        │               ┌───────────────┐
        │               │   PagerDuty   │
        ↓               │     Slack     │
┌──────────────┐        │     Email     │
│    Jaeger    │        └───────────────┘
└──────────────┘
```

### Key Features

- **Real-time metrics** with <15s granularity
- **Distributed tracing** across all services
- **Correlation IDs** for request tracking
- **Automatic instrumentation** with RAII patterns
- **Production-ready** with no stubs or placeholders

---

## Metrics Collection

### Prometheus Metrics

AION-R exports metrics in Prometheus format on port **9090**.

#### Endpoint

```
http://localhost:9090/metrics
http://localhost:9090/health
```

#### Available Metrics

**HTTP Metrics:**
```promql
# Request count by method, path, status
http_requests_total{method="GET", path="/api/users", status="200"}

# Request duration histogram
http_request_duration_seconds{method="POST", path="/api/inference"}

# Active connections
http_active_connections
```

**Database Metrics:**
```promql
# Query duration by type and table
db_query_duration_seconds{query_type="SELECT", table="users"}

# Connection pool statistics
db_connections_active
db_connections_idle
db_connection_pool_size
db_connection_pool_utilization

# Slow queries counter
db_slow_queries_total{query_type="SELECT", table="analytics"}

# Transaction metrics
db_transactions_total
db_transaction_rollbacks_total
db_transaction_duration_seconds
```

**AI Engine Metrics:**
```promql
# Inference requests by model
ai_inference_requests_total{model="gpt-4", input_type="text"}

# Inference duration
ai_inference_duration_seconds{model="gpt-4"}

# Token usage
ai_tokens_processed_total{model="gpt-4"}
ai_tokens_per_request{model="gpt-4"}

# Active sessions
ai_active_sessions

# Model loading
ai_model_loads_total{model="bert-base"}
ai_model_load_duration_seconds{model="bert-base"}
ai_loaded_models

# Errors
ai_inference_errors_total{model="gpt-4", error_type="timeout"}
```

**System Metrics:**
```promql
# Memory and CPU
process_resident_memory_bytes
process_cpu_seconds_total
```

### Using Metrics in Code

#### AI Engine Metrics

```rust
use aion_ai_engine::{AIMetrics, InferenceTracker};

// Create metrics collector
let metrics = AIMetrics::new();

// Automatic tracking with RAII
{
    let tracker = InferenceTracker::new(
        metrics.clone(),
        "gpt-4".to_string(),
        "text".to_string()
    );

    // Perform inference
    let result = model.infer(input).await?;

    // Track completion with token count
    tracker.complete(Some(150));
    // Metrics automatically recorded
}

// Or track errors
{
    let tracker = InferenceTracker::new(...);

    if let Err(e) = model.infer(input).await {
        tracker.fail("timeout");
    }
}
```

#### Database Metrics

```rust
use aion_database::{DatabaseMetrics, QueryTracker};

let metrics = DatabaseMetrics::new(2000); // 2s slow query threshold

// Automatic query tracking
{
    let tracker = QueryTracker::new(
        metrics.clone(),
        "SELECT".to_string(),
        "users".to_string()
    );

    let results = db.query("SELECT * FROM users").await?;

    tracker.complete();
    // Duration and slow query detection automatic
}

// Connection pool stats
metrics.update_connection_pool_stats(
    5,  // active
    5,  // idle
    10  // total
);
```

---

## Distributed Tracing

### Jaeger Integration

AION-R uses OpenTelemetry with Jaeger for distributed tracing.

#### Configuration

```rust
use aion_monitoring::tracing::{TracingConfig, init_tracing};

let config = TracingConfig {
    service_name: "aion-web-api".to_string(),
    service_version: "1.0.0".to_string(),
    jaeger_endpoint: "http://jaeger:14268/api/traces".to_string(),
    sample_rate: 0.1, // 10% sampling
};

let _guard = init_tracing(config)?;
// Guard ensures proper cleanup on drop
```

#### Creating Spans

**HTTP Request Spans:**
```rust
use aion_monitoring::tracing::create_http_span;

let span = create_http_span("GET", "/api/users", Some(200));
let _guard = span.enter();

// HTTP handler logic here
// Span automatically closed when guard drops
```

**Database Operation Spans:**
```rust
use aion_monitoring::tracing::create_db_span;

let span = create_db_span("SELECT", "users");
let _guard = span.enter();

let results = db.query(...).await?;
```

**AI Inference Spans:**
```rust
use aion_monitoring::tracing::create_ai_span;

let span = create_ai_span("gpt-4", "text");
let _guard = span.enter();

let output = ai_engine.infer(...).await?;
```

**External API Spans:**
```rust
use aion_monitoring::tracing::create_external_api_span;

let span = create_external_api_span("openai", "completions");
let _guard = span.enter();

let response = client.post(...).await?;
```

#### Adding Events and Errors

```rust
use aion_monitoring::tracing::{add_span_event, set_span_error};
use opentelemetry::KeyValue;

// Add event to current span
add_span_event("cache_hit", vec![
    KeyValue::new("cache.key", "user:123"),
    KeyValue::new("cache.ttl", 300),
]);

// Mark span as error
if let Err(e) = operation().await {
    set_span_error(&e.to_string());
}
```

#### Viewing Traces

**Jaeger UI:** http://jaeger.aion.internal:16686

Search traces by:
- Service name
- Operation name
- Tags
- Duration
- Time range

---

## Structured Logging

### Configuration

```rust
use aion_core::logging::{LoggingConfig, LogFormat, init_logging};

let config = LoggingConfig {
    level: "info".to_string(),
    format: LogFormat::Json, // or Pretty, Compact
    with_spans: true,
    with_file: true,
    sensitive_fields: vec![
        "password".to_string(),
        "token".to_string(),
        "api_key".to_string(),
    ],
    sample_rate: 1.0, // 100% logging
    ..Default::default()
};

init_logging(config)?;
```

### Correlation and Request IDs

```rust
use aion_core::logging::{CorrelationId, RequestId};

let correlation_id = CorrelationId::new();
let request_id = RequestId::new();

let span = tracing::info_span!(
    "http_request",
    correlation_id = %correlation_id,
    request_id = %request_id,
    method = "GET",
    path = "/api/users"
);

let _guard = span.enter();
tracing::info!("Processing request");
```

### Log Sampling

For high-volume endpoints:

```rust
use aion_core::logging::LogSampler;

let sampler = LogSampler::new(0.1); // 10% sampling

if sampler.should_sample() {
    tracing::debug!("Detailed debug info");
}
```

### Sensitive Data Filtering

```rust
use aion_core::logging::filter_sensitive_field;

let sensitive_fields = vec!["password".to_string()];

let filtered = filter_sensitive_field(
    "password",
    "secret123",
    &sensitive_fields
);
// Returns: "[REDACTED]"
```

---

## Alerting

### Alert Rules

AION-R includes 15 production-ready alert rules:

#### API Alerts

**HighHTTPErrorRate**
- Trigger: Error rate >5% for 5 minutes
- Severity: Critical
- Runbook: `/docs/runbooks/high_error_rate.md`

**HighAPILatency**
- Trigger: p95 latency >1s for 10 minutes
- Severity: Warning
- Runbook: `/docs/runbooks/high_latency.md`

**ServiceDown**
- Trigger: Service down for 2 minutes
- Severity: Critical
- Runbook: `/docs/runbooks/service_down.md`

#### Database Alerts

**DatabaseConnectionPoolExhausted**
- Trigger: Pool utilization >90% for 5 minutes
- Severity: Critical

**SlowDatabaseQueries**
- Trigger: p95 query duration >2s for 10 minutes
- Severity: Warning

#### AI Engine Alerts

**HighAIInferenceErrors**
- Trigger: Error rate >10% for 5 minutes
- Severity: Warning

**SlowAIInference**
- Trigger: p95 inference duration >30s for 10 minutes
- Severity: Warning

#### System Alerts

**HighMemoryUsage**
- Trigger: Memory usage >90% for 10 minutes
- Severity: Warning

**HighCPUUsage**
- Trigger: CPU usage >80% for 10 minutes
- Severity: Warning

### Alert Channels

**PagerDuty:** Critical alerts (SEV-1)
**Slack:** All alerts (#platform-alerts)
**Email:** Warning and above

### Alert Configuration

```yaml
# monitoring/alertmanager/alertmanager.yml
global:
  pagerduty_url: 'https://events.pagerduty.com/v2/enqueue'

route:
  group_by: ['alertname', 'cluster', 'service']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 12h
  receiver: 'default'

  routes:
    - match:
        severity: critical
      receiver: 'pagerduty-critical'

    - match:
        severity: warning
      receiver: 'slack-warnings'

receivers:
  - name: 'pagerduty-critical'
    pagerduty_configs:
      - service_key: $PAGERDUTY_SERVICE_KEY

  - name: 'slack-warnings'
    slack_configs:
      - api_url: $SLACK_WEBHOOK_URL
        channel: '#platform-alerts'
```

---

## Dashboards

### Grafana Overview Dashboard

**Access:** http://grafana.aion.internal:3000

**Panels:**

1. **Request Rate** (Gauge)
   ```promql
   rate(http_requests_total[5m])
   ```

2. **Error Rate** (Gauge with thresholds)
   ```promql
   sum(rate(http_requests_total{status=~"5.."}[5m]))
   /
   sum(rate(http_requests_total[5m]))
   ```

3. **API Latency** (Time series)
   ```promql
   histogram_quantile(0.50, sum(rate(http_request_duration_seconds_bucket[5m])) by (le))
   histogram_quantile(0.95, sum(rate(http_request_duration_seconds_bucket[5m])) by (le))
   histogram_quantile(0.99, sum(rate(http_request_duration_seconds_bucket[5m])) by (le))
   ```

4. **Database Connections** (Time series)
   ```promql
   db_connections_active
   db_connections_idle
   ```

5. **AI Inference Duration** (Time series by model)
   ```promql
   histogram_quantile(0.95,
     sum(rate(ai_inference_duration_seconds_bucket[5m])) by (le, model)
   )
   ```

6. **CPU Usage** (Gauge)
   ```promql
   100 - (avg(irate(node_cpu_seconds_total{mode="idle"}[5m])) * 100)
   ```

7. **Memory Usage** (Gauge)
   ```promql
   (node_memory_MemTotal_bytes - node_memory_MemAvailable_bytes)
   /
   node_memory_MemTotal_bytes * 100
   ```

---

## Deployment

### Kubernetes Deployment

**Deploy monitoring stack:**

```bash
# Create namespace
kubectl apply -f k8s/prometheus/deployment.yaml

# Deploy Prometheus
kubectl apply -f k8s/prometheus/deployment.yaml

# Deploy Jaeger
kubectl apply -f k8s/jaeger/deployment.yaml

# Verify deployment
kubectl get pods -n aion-monitoring

# Expected output:
# prometheus-xxx      1/1     Running
# jaeger-xxx          1/1     Running
```

### Access Services

**Prometheus:**
```bash
kubectl port-forward -n aion-monitoring svc/prometheus 9090:9090
# Access: http://localhost:9090
```

**Jaeger:**
```bash
kubectl port-forward -n aion-monitoring svc/jaeger-query 16686:16686
# Access: http://localhost:16686
```

**Grafana:**
```bash
kubectl port-forward -n aion-monitoring svc/grafana 3000:3000
# Access: http://localhost:3000
```

### Configuration

**Environment Variables:**

```bash
# Prometheus
PROMETHEUS_RETENTION_TIME=30d
PROMETHEUS_RETENTION_SIZE=45GB

# Jaeger
JAEGER_SAMPLING_RATE=0.1  # 10%
JAEGER_STORAGE_TYPE=badger
JAEGER_SPAN_STORE_TTL=168h  # 7 days

# AlertManager
PAGERDUTY_SERVICE_KEY=<your-key>
SLACK_WEBHOOK_URL=<your-webhook>
SMTP_FROM=alerts@aion.com
```

---

## Usage Examples

### Complete Monitoring Setup

```rust
use aion_monitoring::{TracingConfig, init_tracing};
use aion_core::logging::{LoggingConfig, LogFormat, init_logging};
use aion_ai_engine::AIMetrics;
use aion_database::DatabaseMetrics;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    let log_config = LoggingConfig {
        level: "info".to_string(),
        format: LogFormat::Json,
        ..Default::default()
    };
    init_logging(log_config)?;

    // Initialize tracing
    let trace_config = TracingConfig {
        service_name: "aion-web-api".to_string(),
        service_version: env!("CARGO_PKG_VERSION").to_string(),
        jaeger_endpoint: "http://jaeger:14268/api/traces".to_string(),
        sample_rate: 0.1,
    };
    let _tracing_guard = init_tracing(trace_config)?;

    // Initialize metrics
    let ai_metrics = AIMetrics::new();
    let db_metrics = DatabaseMetrics::new(2000);

    // Start Prometheus exporter
    let metrics_registry = aion_monitoring::MetricsRegistry::new();
    let exporter = aion_monitoring::PrometheusExporter::new(
        "0.0.0.0:9090".to_string(),
        metrics_registry.clone()
    );

    tokio::spawn(async move {
        if let Err(e) = exporter.start().await {
            tracing::error!("Metrics exporter failed: {}", e);
        }
    });

    // Your application code here
    run_application(ai_metrics, db_metrics).await?;

    Ok(())
}
```

### Monitored HTTP Handler

```rust
use axum::{Router, routing::get};
use aion_monitoring::tracing::create_http_span;
use aion_core::logging::{CorrelationId, RequestId};

async fn handler() -> &'static str {
    let correlation_id = CorrelationId::new();
    let request_id = RequestId::new();

    let span = create_http_span("GET", "/api/users", None);
    let _guard = span.enter();

    tracing::info!(
        correlation_id = %correlation_id,
        request_id = %request_id,
        "Handling request"
    );

    // Business logic
    "Hello, World!"
}
```

### Monitored Database Query

```rust
use aion_database::{DatabaseMetrics, QueryTracker};
use aion_monitoring::tracing::create_db_span;

async fn query_users(db: &Database, metrics: &DatabaseMetrics) -> Result<Vec<User>> {
    let _span = create_db_span("SELECT", "users").entered();
    let _tracker = QueryTracker::new(
        metrics.clone(),
        "SELECT".to_string(),
        "users".to_string()
    );

    let users = db.query("SELECT * FROM users").await?;

    Ok(users)
}
```

### Monitored AI Inference

```rust
use aion_ai_engine::{AIMetrics, InferenceTracker};
use aion_monitoring::tracing::create_ai_span;

async fn run_inference(
    model: &Model,
    input: &str,
    metrics: &AIMetrics
) -> Result<String> {
    let _span = create_ai_span("gpt-4", "text").entered();
    let tracker = InferenceTracker::new(
        metrics.clone(),
        "gpt-4".to_string(),
        "text".to_string()
    );

    match model.infer(input).await {
        Ok(output) => {
            tracker.complete(Some(output.token_count));
            Ok(output.text)
        }
        Err(e) => {
            tracker.fail("inference_error");
            Err(e)
        }
    }
}
```

---

## Troubleshooting

### Metrics Not Appearing

**Check Prometheus targets:**
```bash
curl http://localhost:9090/api/v1/targets
```

**Verify metrics endpoint:**
```bash
curl http://localhost:9090/metrics
```

**Check scrape configuration:**
```bash
kubectl get configmap -n aion-monitoring prometheus-config -o yaml
```

### Traces Not Showing in Jaeger

**Verify Jaeger collector:**
```bash
kubectl logs -n aion-monitoring deployment/jaeger
```

**Check OTLP endpoint:**
```bash
curl http://jaeger:4318/v1/traces -d '{}'
```

**Verify sampling rate:**
- Check `TracingConfig.sample_rate` (0.0-1.0)
- Low sample rates may not show all traces

### High Memory Usage

**Check Prometheus retention:**
```bash
kubectl exec -n aion-monitoring deployment/prometheus -- \
  promtool tsdb analyze /prometheus
```

**Reduce retention if needed:**
```yaml
args:
  - '--storage.tsdb.retention.time=7d'
  - '--storage.tsdb.retention.size=10GB'
```

### Alerts Not Firing

**Check AlertManager config:**
```bash
kubectl logs -n aion-monitoring deployment/alertmanager
```

**Verify alert rules:**
```bash
curl http://localhost:9090/api/v1/rules
```

**Test notification:**
```bash
curl -X POST http://alertmanager:9093/api/v1/alerts -d '[{
  "labels": {"alertname":"test","severity":"warning"},
  "annotations": {"summary":"Test alert"}
}]'
```

---

## Performance Considerations

### Metrics Cardinality

**Avoid high cardinality labels:**
```rust
// Bad: User ID in label (unbounded)
counter!("requests", "user_id" => user_id).increment(1);

// Good: Aggregate metrics
counter!("requests_total").increment(1);
```

### Sampling Rates

**Recommended sampling:**
- Development: 100% (sample_rate: 1.0)
- Staging: 50% (sample_rate: 0.5)
- Production: 10% (sample_rate: 0.1)
- High-traffic: 1% (sample_rate: 0.01)

### Log Sampling

```rust
use aion_core::logging::LogSampler;

let sampler = LogSampler::new(0.01); // 1% for high-volume logs

for request in requests {
    if sampler.should_sample() {
        tracing::debug!("Request details: {:?}", request);
    }
}
```

---

## Security Considerations

### Sensitive Data

**Always filter sensitive fields:**
```rust
let sensitive_fields = vec![
    "password",
    "token",
    "api_key",
    "authorization",
    "secret",
    "ssn",
    "credit_card",
];
```

### Access Control

**Prometheus:**
- Internal network only
- Basic auth or OAuth2

**Jaeger:**
- Internal network only
- Read-only access for developers

**Grafana:**
- SSO integration
- Role-based access control

---

## Additional Resources

- [Prometheus Best Practices](https://prometheus.io/docs/practices/)
- [OpenTelemetry Documentation](https://opentelemetry.io/docs/)
- [Jaeger Documentation](https://www.jaegertracing.io/docs/)
- [Grafana Documentation](https://grafana.com/docs/)

**Internal Documentation:**
- Incident Response: `/docs/operations/incident-response.md`
- On-Call Setup: `/docs/operations/on-call.md`
- Runbooks: `/docs/runbooks/`
- Architecture: `/docs/ARCHITECTURE.md`

---

**Document Version:** 2.0
**Last Reviewed:** 2025-10-04
**Next Review:** 2025-11-04
**Owner:** Platform Engineering Team
