# Ectus-R - System Architecture

**Version:** 2.0
**Last Updated:** 2025-10-04
**Status:** Production-Ready

---

## Table of Contents

1. [Overview](#overview)
2. [System Architecture](#system-architecture)
3. [Core Components](#core-components)
4. [Monitoring Architecture](#monitoring-architecture)
5. [Data Flow](#data-flow)
6. [Technology Stack](#technology-stack)
7. [Deployment Architecture](#deployment-architecture)
8. [Security Architecture](#security-architecture)
9. [Scalability & Performance](#scalability--performance)
10. [Integration Points](#integration-points)

---

## Overview

Ectus-R is an enterprise AI code generation platform built on the AION-R MLOps infrastructure. The system provides autonomous code generation, quality assurance, and project management capabilities with comprehensive monitoring and observability.

### Key Characteristics

- **Language:** Rust (primary), TypeScript (frontend)
- **Architecture:** Microservices with shared-nothing components
- **Deployment:** Kubernetes-native with Docker containers
- **Observability:** Full-stack monitoring (metrics, traces, logs)
- **Security:** Post-quantum cryptography, zero-trust architecture

---

## System Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Client Layer                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │ Web Dashboard│  │   CLI Tool   │  │  REST API    │          │
│  │  (React/TS)  │  │   (Rust)     │  │   Clients    │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      API Gateway Layer                           │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │         aion-web-api (Axum HTTP Server)                  │   │
│  │  - Authentication/Authorization                          │   │
│  │  - Request validation                                    │   │
│  │  - Rate limiting                                         │   │
│  │  - Metrics middleware                                    │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Service Layer                               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │ AI Engine   │  │ Optimizer   │  │  Analysis   │            │
│  │  Service    │  │  Service    │  │   Service   │            │
│  └─────────────┘  └─────────────┘  └─────────────┘            │
│                                                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │  Database   │  │    Cloud    │  │   Auth      │            │
│  │  Service    │  │   Service   │  │  Service    │            │
│  └─────────────┘  └─────────────┘  └─────────────┘            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Data Layer                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │ PostgreSQL  │  │    Redis    │  │   S3/Blob   │            │
│  │  (Primary)  │  │   (Cache)   │  │  (Storage)  │            │
│  └─────────────┘  └─────────────┘  └─────────────┘            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                   Monitoring Layer                               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │ Prometheus  │  │   Jaeger    │  │   Grafana   │            │
│  │  (Metrics)  │  │  (Traces)   │  │ (Dashboards)│            │
│  └─────────────┘  └─────────────┘  └─────────────┘            │
│                                                                  │
│  ┌─────────────┐  ┌─────────────┐                              │
│  │    Loki     │  │ AlertManager│                              │
│  │   (Logs)    │  │  (Alerts)   │                              │
│  └─────────────┘  └─────────────┘                              │
└─────────────────────────────────────────────────────────────────┘
```

---

## Core Components

### 1. API Gateway (aion-web-api)

**Responsibility:** HTTP request handling, authentication, routing

**Technology:** Axum (async HTTP framework)

**Key Features:**
- RESTful API endpoints
- JWT-based authentication
- Request/response validation
- Rate limiting per user/IP
- Automatic metrics collection
- Distributed tracing integration

**Endpoints:**
- `/api/v1/auth/*` - Authentication
- `/api/v1/ai/*` - AI code generation
- `/api/v1/projects/*` - Project management
- `/api/v1/status` - System status
- `/health` - Health checks
- `/metrics` - Prometheus metrics (port 9091)

**Configuration:**
```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 8

[metrics]
enabled = true
port = 9091
path = "/metrics"
```

### 2. AI Engine (aion-ai-engine)

**Responsibility:** AI model orchestration, code generation

**Technology:** Rust + Candle ML framework

**Key Features:**
- Multi-LLM support (OpenAI, Anthropic, local models)
- Code generation pipeline
- Quality assessment
- Model selection and routing
- Token usage tracking
- Inference metrics

**Metrics Tracked:**
- `ai_inference_requests_total` - Total inference requests
- `ai_inference_duration_seconds` - Inference latency histogram
- `ai_inference_errors_total` - Error counter by model
- `ai_active_sessions` - Current active sessions
- `ai_token_usage_total` - Token consumption

### 3. Database Service (aion-database)

**Responsibility:** Data persistence and retrieval

**Technology:** PostgreSQL with SQLx

**Key Features:**
- Connection pooling (r2d2)
- Query optimization
- Transaction management
- Migration support
- Metrics collection

**Metrics Tracked:**
- `database_query_duration_seconds` - Query latency
- `database_connections_active` - Active connections
- `database_connections_idle` - Idle connections
- `database_query_errors_total` - Query failures
- `database_slow_queries_total` - Slow query counter (>2s)

**Schema:**
```sql
-- Key tables
users (id, email, password_hash, created_at, ...)
projects (id, user_id, name, status, ...)
generations (id, project_id, prompt, result, ...)
sessions (id, user_id, expires_at, ...)
```

### 4. Optimizer Service (aion-optimization-engine)

**Responsibility:** Code optimization and analysis

**Key Features:**
- Static analysis
- Complexity calculation
- Performance optimization suggestions
- Security vulnerability detection

### 5. Cloud Service (aion-cloud)

**Responsibility:** Multi-cloud resource management

**Supported Providers:**
- AWS (S3, ECS, RDS)
- Azure (Blob, AKS, SQL)
- GCP (GCS, GKE, Cloud SQL)

### 6. Monitoring Service (aion-monitoring)

**Responsibility:** Observability infrastructure

**Components:**
- Prometheus exporter (HTTP server on port 9090)
- Distributed tracing (OpenTelemetry → Jaeger)
- Structured logging (JSON format)
- Metrics registry

---

## Monitoring Architecture

### Metrics Collection Flow

```
┌──────────────┐     HTTP Middleware      ┌─────────────┐
│   Request    │ ───────────────────────> │   Metrics   │
│              │                           │  Registry   │
└──────────────┘                           └─────────────┘
                                                  │
       ┌──────────────────────────────────────────┤
       │                                          │
       ▼                                          ▼
┌─────────────┐                           ┌─────────────┐
│  In-Memory  │                           │  Prometheus │
│   Buffers   │                           │   Exporter  │
└─────────────┘                           │  (Port 9091)│
                                          └─────────────┘
                                                  │
                                                  ▼
                                          ┌─────────────┐
                                          │ Prometheus  │
                                          │   Server    │
                                          │ (Port 9090) │
                                          └─────────────┘
                                                  │
                                                  ▼
                                          ┌─────────────┐
                                          │   Grafana   │
                                          │ (Port 3000) │
                                          └─────────────┘
```

### Distributed Tracing Flow

```
┌──────────────┐     Trace Context       ┌─────────────┐
│   Request    │ ───────────────────────> │    Span     │
│ (traceparent)│                           │   Context   │
└──────────────┘                           └─────────────┘
                                                  │
                                                  ▼
                                          ┌─────────────┐
                                          │ OpenTelemetry│
                                          │   SDK       │
                                          └─────────────┘
                                                  │
                                                  ▼
                                          ┌─────────────┐
                                          │    OTLP     │
                                          │  Exporter   │
                                          │ (Port 4318) │
                                          └─────────────┘
                                                  │
                                                  ▼
                                          ┌─────────────┐
                                          │   Jaeger    │
                                          │  Collector  │
                                          └─────────────┘
                                                  │
                                                  ▼
                                          ┌─────────────┐
                                          │   Jaeger    │
                                          │     UI      │
                                          │ (Port 16686)│
                                          └─────────────┘
```

### Logging Architecture

```
┌──────────────┐
│  Application │
│     Code     │
└──────────────┘
       │
       ▼
┌──────────────┐     tracing-subscriber
│   tracing    │ ────────────────────────>  ┌──────────────┐
│   macros     │                             │    Format    │
└──────────────┘                             │    Layer     │
                                             └──────────────┘
                                                    │
                        ┌───────────────────────────┼───────────────┐
                        │                           │               │
                        ▼                           ▼               ▼
                 ┌─────────────┐            ┌─────────────┐  ┌─────────────┐
                 │   Console   │            │    JSON     │  │    Loki     │
                 │   (stdout)  │            │    File     │  │   Exporter  │
                 └─────────────┘            └─────────────┘  └─────────────┘
```

### Alert Flow

```
┌──────────────┐
│  Prometheus  │
│    Rules     │
└──────────────┘
       │
       ▼ (evaluate every 1m)
┌──────────────┐
│ Alert State  │
│   Machine    │
└──────────────┘
       │
       ▼ (when firing)
┌──────────────┐
│ AlertManager │
└──────────────┘
       │
       ├──────────────> Slack (critical)
       ├──────────────> PagerDuty (critical)
       └──────────────> Email (warning/info)
```

---

## Data Flow

### Code Generation Request Flow

```
1. Client → API Gateway
   POST /api/v1/ai/generate
   {
     "prompt": "Create a REST API in Rust",
     "language": "rust",
     "framework": "axum"
   }

2. API Gateway → AI Engine
   - Validate JWT token
   - Check rate limits
   - Create trace span
   - Record request metric

3. AI Engine → LLM Provider
   - Select appropriate model
   - Format prompt
   - Track inference start time

4. LLM Provider → AI Engine
   - Receive generated code
   - Track inference end time
   - Record metrics (duration, tokens)

5. AI Engine → Optimizer
   - Analyze generated code
   - Calculate complexity
   - Check for issues

6. Optimizer → AI Engine
   - Return analysis results

7. AI Engine → Database
   - Store generation
   - Update usage stats

8. AI Engine → API Gateway
   - Return result with metadata

9. API Gateway → Client
   - Return JSON response
   - Record completion metric
   - Close trace span
```

### Metrics Collection Flow

```
1. HTTP Request arrives
   ↓
2. Metrics Middleware (start timer)
   ↓
3. Handler processes request
   ↓
4. Handler records business metrics
   - counter!("ai_requests_total").increment(1)
   - histogram!("ai_duration").record(elapsed)
   ↓
5. Metrics Middleware (stop timer)
   - histogram!("http_request_duration").record(duration)
   - counter!("http_requests_total", "status" => "200").increment(1)
   ↓
6. Response sent to client
   ↓
7. Prometheus scrapes /metrics (every 15s)
   ↓
8. Metrics stored in Prometheus TSDB
   ↓
9. Grafana queries Prometheus for dashboards
   ↓
10. Alert rules evaluated every 1m
```

---

## Technology Stack

### Backend

| Component | Technology | Version | Purpose |
|-----------|-----------|---------|---------|
| Language | Rust | 1.75+ | System programming |
| HTTP Framework | Axum | 0.7 | Web server |
| Database Driver | SQLx | 0.7 | PostgreSQL access |
| Async Runtime | Tokio | 1.0 | Async I/O |
| Serialization | Serde | 1.0 | JSON/TOML parsing |
| Metrics | metrics crate | 0.22 | Metrics collection |
| Metrics Export | metrics-exporter-prometheus | 0.13 | Prometheus format |
| Tracing | tracing | 0.1 | Distributed tracing |
| OpenTelemetry | opentelemetry | 0.21 | Trace export |
| Auth | jsonwebtoken | 9.0 | JWT handling |
| Hashing | argon2 | 0.5 | Password hashing |

### Frontend

| Component | Technology | Version | Purpose |
|-----------|-----------|---------|---------|
| Framework | React | 18.2 | UI framework |
| Language | TypeScript | 5.0 | Type safety |
| Build Tool | Vite | 4.5 | Fast builds |
| HTTP Client | Axios | 1.6 | API calls |
| State Management | Zustand | 4.4 | App state |
| UI Components | shadcn/ui | Latest | UI library |

### Infrastructure

| Component | Technology | Version | Purpose |
|-----------|-----------|---------|---------|
| Container | Docker | 24.0+ | Containerization |
| Orchestration | Kubernetes | 1.28+ | Container orchestration |
| Metrics | Prometheus | 2.45+ | Metrics storage |
| Tracing | Jaeger | 1.51+ | Trace storage |
| Visualization | Grafana | 10.0+ | Dashboards |
| Logging | Loki | 2.9+ | Log aggregation |
| Alerting | AlertManager | 0.26+ | Alert routing |
| Database | PostgreSQL | 15+ | Primary database |
| Cache | Redis | 7+ | Session cache |
| Storage | S3/Azure Blob | - | File storage |

### CI/CD

| Component | Technology | Purpose |
|-----------|-----------|---------|
| CI/CD | GitHub Actions | Automated testing/deployment |
| Testing | cargo test | Unit tests |
| Load Testing | k6 | Performance testing |
| Security Scan | cargo audit | Dependency scanning |
| Code Coverage | tarpaulin | Test coverage |
| Linting | clippy | Code quality |
| Formatting | rustfmt | Code formatting |

---

## Deployment Architecture

### Kubernetes Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Kubernetes Cluster                        │
│                                                              │
│  ┌────────────────────────────────────────────────────┐     │
│  │              Namespace: aion-production            │     │
│  │                                                     │     │
│  │  ┌──────────────────────────────────────────────┐  │     │
│  │  │      Deployment: aion-web-api                │  │     │
│  │  │  ┌────────┐  ┌────────┐  ┌────────┐         │  │     │
│  │  │  │  Pod   │  │  Pod   │  │  Pod   │         │  │     │
│  │  │  │ (api)  │  │ (api)  │  │ (api)  │         │  │     │
│  │  │  └────────┘  └────────┘  └────────┘         │  │     │
│  │  │       3 replicas, HPA 3-10                   │  │     │
│  │  └──────────────────────────────────────────────┘  │     │
│  │                                                     │     │
│  │  ┌──────────────────────────────────────────────┐  │     │
│  │  │          Service: aion-api-svc               │  │     │
│  │  │  Type: LoadBalancer, Port: 8080              │  │     │
│  │  └──────────────────────────────────────────────┘  │     │
│  │                                                     │     │
│  │  ┌──────────────────────────────────────────────┐  │     │
│  │  │       StatefulSet: postgres                  │  │     │
│  │  │  ┌────────┐     PVC: 100Gi                   │  │     │
│  │  │  │  Pod   │     StorageClass: fast-ssd       │  │     │
│  │  │  │  (db)  │                                   │  │     │
│  │  │  └────────┘                                   │  │     │
│  │  └──────────────────────────────────────────────┘  │     │
│  │                                                     │     │
│  │  ┌──────────────────────────────────────────────┐  │     │
│  │  │       Deployment: redis                      │  │     │
│  │  │  ┌────────┐                                   │  │     │
│  │  │  │  Pod   │     1 replica                     │  │     │
│  │  │  │(cache) │                                   │  │     │
│  │  │  └────────┘                                   │  │     │
│  │  └──────────────────────────────────────────────┘  │     │
│  └────────────────────────────────────────────────────┘     │
│                                                              │
│  ┌────────────────────────────────────────────────────┐     │
│  │            Namespace: aion-monitoring              │     │
│  │                                                     │     │
│  │  ┌──────────────────────────────────────────────┐  │     │
│  │  │      Deployment: prometheus                  │  │     │
│  │  │  PVC: 50Gi, Retention: 30d                   │  │     │
│  │  └──────────────────────────────────────────────┘  │     │
│  │                                                     │     │
│  │  ┌──────────────────────────────────────────────┐  │     │
│  │  │      Deployment: jaeger                      │  │     │
│  │  │  PVC: 20Gi, Retention: 7d                    │  │     │
│  │  └──────────────────────────────────────────────┘  │     │
│  │                                                     │     │
│  │  ┌──────────────────────────────────────────────┐  │     │
│  │  │      Deployment: grafana                     │  │     │
│  │  │  PVC: 10Gi                                   │  │     │
│  │  └──────────────────────────────────────────────┘  │     │
│  └────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

### Resource Requirements

**Production (per pod):**
- API Server: 2 CPU, 4Gi RAM
- Database: 4 CPU, 8Gi RAM, 100Gi disk
- Redis: 1 CPU, 2Gi RAM
- Prometheus: 2 CPU, 4Gi RAM, 50Gi disk
- Jaeger: 1 CPU, 2Gi RAM, 20Gi disk
- Grafana: 0.5 CPU, 1Gi RAM, 10Gi disk

**Staging/Development:**
- 50% of production resources

---

## Security Architecture

### Authentication & Authorization

**Flow:**
```
1. User Login → API Gateway
2. Verify credentials (Argon2 hash)
3. Generate JWT token (24h expiry)
4. Return token to client
5. Client sends token in Authorization header
6. API Gateway validates JWT
7. Extract user_id and roles
8. Check permissions for endpoint
9. Process request if authorized
```

**JWT Claims:**
```json
{
  "sub": "user_id",
  "email": "user@example.com",
  "roles": ["user", "admin"],
  "exp": 1730000000,
  "iat": 1729913600
}
```

### Encryption

**At Rest:**
- Database: PostgreSQL native encryption (AES-256)
- Storage: S3 server-side encryption (SSE-S3)
- Secrets: Kubernetes secrets (etcd encryption)

**In Transit:**
- HTTPS/TLS 1.3 for all external traffic
- mTLS for internal service communication
- gRPC with TLS for OpenTelemetry

**Post-Quantum:**
- CRYSTALS-Kyber (ML-KEM) for key exchange
- CRYSTALS-Dilithium5 (ML-DSA) for signatures
- SPHINCS+ (SLH-DSA) as backup

### Network Security

**Ingress:**
- External traffic through LoadBalancer
- TLS termination at ingress
- Rate limiting (100 req/min per IP)
- DDoS protection

**Egress:**
- Outbound traffic through NAT gateway
- Allow-list for external APIs
- No direct database access from internet

---

## Scalability & Performance

### Horizontal Scaling

**Auto-scaling:**
```yaml
HorizontalPodAutoscaler:
  minReplicas: 3
  maxReplicas: 10
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70
    - type: Resource
      resource:
        name: memory
        target:
          type: Utilization
          averageUtilization: 80
```

**Database Scaling:**
- Read replicas for query distribution
- Connection pooling (max 100 connections per pod)
- Query caching in Redis

### Performance Targets

| Metric | Target | Current |
|--------|--------|---------|
| API Response Time (p95) | <2000ms | TBD |
| API Response Time (p99) | <5000ms | TBD |
| Database Query Time (p95) | <100ms | TBD |
| AI Inference Time (p95) | <30s | TBD |
| Throughput | >1000 req/s | TBD |
| Availability | 99.9% | TBD |
| Error Rate | <0.1% | TBD |

---

## Integration Points

### External Services

**LLM Providers:**
- OpenAI API (GPT-4, GPT-3.5)
- Anthropic API (Claude)
- Local models (Ollama)

**Cloud Providers:**
- AWS (S3, RDS, ECS)
- Azure (Blob Storage, AKS)
- GCP (Cloud Storage, GKE)

**Monitoring:**
- Slack (alert notifications)
- PagerDuty (incident management)
- Email (notifications)

### API Integrations

**Webhooks:**
- GitHub webhook for CI/CD
- Slack incoming webhooks
- Custom user webhooks

**OAuth:**
- GitHub OAuth
- Google OAuth
- Microsoft OAuth

---

## Appendix

### Port Allocation

| Service | Port | Protocol | Purpose |
|---------|------|----------|---------|
| API Server | 8080 | HTTP | Main API |
| Metrics Exporter | 9091 | HTTP | Prometheus metrics |
| Prometheus | 9090 | HTTP | Metrics UI/API |
| Jaeger UI | 16686 | HTTP | Traces UI |
| Jaeger Collector (gRPC) | 4317 | gRPC | OTLP ingestion |
| Jaeger Collector (HTTP) | 4318 | HTTP | OTLP ingestion |
| Grafana | 3000 | HTTP | Dashboards |
| AlertManager | 9093 | HTTP | Alert management |
| PostgreSQL | 5432 | TCP | Database |
| Redis | 6379 | TCP | Cache |

### Configuration Files

- `Cargo.toml` - Rust workspace configuration
- `k8s/*.yaml` - Kubernetes manifests
- `monitoring/*.yml` - Monitoring stack config
- `.github/workflows/*.yml` - CI/CD pipelines

### Related Documentation

- [DEPLOYMENT.md](DEPLOYMENT.md) - Deployment guide
- [MONITORING.md](docs/MONITORING.md) - Monitoring guide
- [SECURITY.md](SECURITY.md) - Security practices
- [API.md](docs/API.md) - API documentation

---

**Document Version:** 2.0
**Last Review:** 2025-10-04
**Next Review:** 2025-11-04
