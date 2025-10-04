# Ectus-R - Deployment Guide

**Version:** 2.0
**Last Updated:** 2025-10-04
**Target Environment:** Production, Staging, Development

---

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Local Development](#local-development)
4. [Docker Deployment](#docker-deployment)
5. [Kubernetes Deployment](#kubernetes-deployment)
6. [Monitoring Stack Deployment](#monitoring-stack-deployment)
7. [Database Setup](#database-setup)
8. [Configuration](#configuration)
9. [Health Checks](#health-checks)
10. [Troubleshooting](#troubleshooting)

---

## Overview

This guide covers deploying Ectus-R in various environments:
- **Local Development:** Running services locally for development
- **Docker:** Containerized deployment with Docker Compose
- **Kubernetes:** Production deployment on Kubernetes clusters
- **Monitoring:** Complete observability stack deployment

---

## Prerequisites

### Required Software

| Software | Version | Purpose |
|----------|---------|---------|
| Rust | 1.75+ | Backend compilation |
| Node.js | 18+ | Frontend build |
| Docker | 24.0+ | Containerization |
| Kubernetes | 1.28+ | Orchestration |
| kubectl | 1.28+ | K8s CLI |
| Helm | 3.12+ | K8s package manager |
| PostgreSQL | 15+ | Database |
| Redis | 7+ | Cache |

### Optional Tools

- **k6:** Load testing
- **cargo-tarpaulin:** Test coverage
- **cargo-audit:** Security scanning

---

## Local Development

### 1. Clone Repository

```bash
git clone https://github.com/Yatrogenesis/Ectus-R.git
cd Ectus-R
```

### 2. Install Dependencies

**Backend:**
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
cargo build
```

**Frontend:**
```bash
cd web-dashboard
npm install
```

### 3. Setup Database

```bash
# Start PostgreSQL
docker run -d \
  --name ectus-postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=ectus_dev \
  -p 5432:5432 \
  postgres:15

# Run migrations
cargo sqlx migrate run
```

### 4. Setup Redis

```bash
docker run -d \
  --name ectus-redis \
  -p 6379:6379 \
  redis:7-alpine
```

### 5. Configure Environment

Create `.env` file:
```bash
# Database
DATABASE_URL=postgres://postgres:postgres@localhost:5432/ectus_dev

# Redis
REDIS_URL=redis://localhost:6379

# API
API_HOST=0.0.0.0
API_PORT=8080

# JWT
JWT_SECRET=your-secret-key-change-in-production
JWT_EXPIRY=86400

# LLM Providers
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...

# Monitoring
JAEGER_ENDPOINT=http://localhost:4318/v1/traces
PROMETHEUS_PUSHGATEWAY=http://localhost:9091

# Log Level
RUST_LOG=info,aion_web_api=debug
```

### 6. Run Backend

```bash
cargo run --bin aion-web-api
```

API available at: http://localhost:8080

### 7. Run Frontend

```bash
cd web-dashboard
npm run dev
```

Frontend available at: http://localhost:5173

### 8. Run Monitoring Stack (Optional)

```bash
cd monitoring
docker-compose up -d
```

Services available:
- Prometheus: http://localhost:9090
- Jaeger: http://localhost:16686
- Grafana: http://localhost:3000 (admin/admin)

---

## Docker Deployment

### 1. Build Images

**Backend:**
```bash
docker build -t ectus-r-api:latest -f Dockerfile.production .
```

**Frontend:**
```bash
cd web-dashboard
docker build -t ectus-r-web:latest .
```

### 2. Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: ectus_production
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5

  api:
    image: ectus-r-api:latest
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
    environment:
      DATABASE_URL: postgres://postgres:${POSTGRES_PASSWORD}@postgres:5432/ectus_production
      REDIS_URL: redis://redis:6379
      RUST_LOG: info
      JAEGER_ENDPOINT: http://jaeger:4318/v1/traces
    ports:
      - "8080:8080"
      - "9091:9091"
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
    restart: unless-stopped

  web:
    image: ectus-r-web:latest
    depends_on:
      - api
    environment:
      VITE_API_BASE_URL: http://api:8080
    ports:
      - "80:80"
    restart: unless-stopped

  prometheus:
    image: prom/prometheus:v2.45.0
    volumes:
      - ./monitoring/prometheus/prometheus.yml:/etc/prometheus/prometheus.yml
      - ./monitoring/prometheus/alerts:/etc/prometheus/alerts
      - prometheus_data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--storage.tsdb.retention.time=30d'
    ports:
      - "9090:9090"
    restart: unless-stopped

  jaeger:
    image: jaegertracing/all-in-one:1.51
    environment:
      COLLECTOR_OTLP_ENABLED: true
      SPAN_STORAGE_TYPE: badger
      BADGER_EPHEMERAL: false
      BADGER_DIRECTORY_VALUE: /badger/data
      BADGER_DIRECTORY_KEY: /badger/key
    volumes:
      - jaeger_data:/badger
    ports:
      - "16686:16686"  # UI
      - "4317:4317"    # OTLP gRPC
      - "4318:4318"    # OTLP HTTP
    restart: unless-stopped

  grafana:
    image: grafana/grafana:10.0.0
    depends_on:
      - prometheus
    environment:
      GF_SECURITY_ADMIN_PASSWORD: ${GRAFANA_PASSWORD:-admin}
      GF_INSTALL_PLUGINS: grafana-clock-panel
    volumes:
      - ./monitoring/grafana/provisioning:/etc/grafana/provisioning
      - ./monitoring/grafana/dashboards:/var/lib/grafana/dashboards
      - grafana_data:/var/lib/grafana
    ports:
      - "3000:3000"
    restart: unless-stopped

  alertmanager:
    image: prom/alertmanager:v0.26.0
    volumes:
      - ./monitoring/alertmanager/alertmanager.yml:/etc/alertmanager/alertmanager.yml
      - alertmanager_data:/alertmanager
    command:
      - '--config.file=/etc/alertmanager/alertmanager.yml'
      - '--storage.path=/alertmanager'
    ports:
      - "9093:9093"
    restart: unless-stopped

volumes:
  postgres_data:
  prometheus_data:
  jaeger_data:
  grafana_data:
  alertmanager_data:
```

### 3. Start Stack

```bash
# Create .env file with secrets
cat > .env <<EOF
POSTGRES_PASSWORD=your-secure-password
GRAFANA_PASSWORD=your-secure-password
EOF

# Start all services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f api
```

### 4. Initialize Database

```bash
docker-compose exec api cargo sqlx migrate run
```

---

## Kubernetes Deployment

### 1. Cluster Setup

**Prerequisites:**
- Kubernetes cluster (GKE, EKS, AKS, or local minikube)
- kubectl configured
- Helm 3 installed

**Verify cluster:**
```bash
kubectl cluster-info
kubectl get nodes
```

### 2. Create Namespaces

```bash
kubectl create namespace aion-production
kubectl create namespace aion-monitoring
```

### 3. Deploy Database (PostgreSQL)

**Using Helm:**
```bash
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update

helm install postgres bitnami/postgresql \
  --namespace aion-production \
  --set auth.postgresPassword=your-secure-password \
  --set auth.database=ectus_production \
  --set primary.persistence.size=100Gi \
  --set primary.persistence.storageClass=fast-ssd \
  --set metrics.enabled=true
```

**Or using manifests:**
```bash
kubectl apply -f k8s/postgres/
```

### 4. Deploy Redis

```bash
helm install redis bitnami/redis \
  --namespace aion-production \
  --set auth.enabled=false \
  --set master.persistence.enabled=true \
  --set master.persistence.size=10Gi
```

### 5. Create Secrets

```bash
# Database credentials
kubectl create secret generic postgres-credentials \
  --namespace aion-production \
  --from-literal=username=postgres \
  --from-literal=password=your-secure-password \
  --from-literal=database=ectus_production

# JWT secret
kubectl create secret generic jwt-secret \
  --namespace aion-production \
  --from-literal=secret=your-jwt-secret

# LLM API keys
kubectl create secret generic llm-api-keys \
  --namespace aion-production \
  --from-literal=openai-key=sk-... \
  --from-literal=anthropic-key=sk-ant-...
```

### 6. Deploy API Server

```bash
kubectl apply -f k8s/api/deployment.yaml
kubectl apply -f k8s/api/service.yaml
kubectl apply -f k8s/api/hpa.yaml
```

**k8s/api/deployment.yaml:**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: aion-web-api
  namespace: aion-production
spec:
  replicas: 3
  selector:
    matchLabels:
      app: aion-web-api
  template:
    metadata:
      labels:
        app: aion-web-api
    spec:
      containers:
      - name: api
        image: your-registry/ectus-r-api:latest
        ports:
        - containerPort: 8080
          name: http
        - containerPort: 9091
          name: metrics
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: postgres-credentials
              key: url
        - name: REDIS_URL
          value: redis://redis-master:6379
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: jwt-secret
              key: secret
        - name: RUST_LOG
          value: info
        - name: JAEGER_ENDPOINT
          value: http://jaeger-collector.aion-monitoring:4318/v1/traces
        resources:
          requests:
            memory: "2Gi"
            cpu: "1"
          limits:
            memory: "4Gi"
            cpu: "2"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 5
```

### 7. Deploy Monitoring Stack

#### Prometheus

```bash
kubectl apply -f k8s/prometheus/deployment.yaml
```

Verify:
```bash
kubectl get pods -n aion-monitoring -l app=prometheus
kubectl port-forward -n aion-monitoring svc/prometheus 9090:9090
```

#### Jaeger

```bash
kubectl apply -f k8s/jaeger/deployment.yaml
```

Verify:
```bash
kubectl get pods -n aion-monitoring -l app=jaeger
kubectl port-forward -n aion-monitoring svc/jaeger-query 16686:16686
```

#### Grafana

```bash
kubectl apply -f k8s/grafana/deployment.yaml
```

Verify:
```bash
kubectl get pods -n aion-monitoring -l app=grafana
kubectl port-forward -n aion-monitoring svc/grafana 3000:3000
```

### 8. Setup Ingress

**Install NGINX Ingress Controller:**
```bash
helm install ingress-nginx ingress-nginx/ingress-nginx \
  --namespace ingress-nginx \
  --create-namespace \
  --set controller.service.type=LoadBalancer
```

**Create Ingress:**
```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: aion-ingress
  namespace: aion-production
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
    nginx.ingress.kubernetes.io/rate-limit: "100"
spec:
  ingressClassName: nginx
  tls:
  - hosts:
    - api.ectus-r.com
    secretName: api-tls
  rules:
  - host: api.ectus-r.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: aion-api-svc
            port:
              number: 8080
```

### 9. Verify Deployment

```bash
# Check all pods
kubectl get pods -n aion-production
kubectl get pods -n aion-monitoring

# Check services
kubectl get svc -n aion-production
kubectl get svc -n aion-monitoring

# Check ingress
kubectl get ingress -n aion-production

# View logs
kubectl logs -n aion-production -l app=aion-web-api -f

# Check metrics
kubectl port-forward -n aion-production svc/aion-api-svc 9091:9091
curl http://localhost:9091/metrics
```

---

## Monitoring Stack Deployment

### Complete Monitoring Stack

The monitoring stack consists of:
1. **Prometheus** - Metrics collection and storage
2. **Jaeger** - Distributed tracing
3. **Grafana** - Visualization
4. **AlertManager** - Alert routing
5. **Loki** (optional) - Log aggregation

### Deployment Steps

#### 1. Deploy Prometheus

```bash
cd k8s/prometheus
kubectl apply -f namespace.yaml
kubectl apply -f serviceaccount.yaml
kubectl apply -f clusterrole.yaml
kubectl apply -f clusterrolebinding.yaml
kubectl apply -f configmap.yaml
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
```

**Verify:**
```bash
kubectl get pods -n aion-monitoring -l app=prometheus
kubectl logs -n aion-monitoring -l app=prometheus

# Port forward to access UI
kubectl port-forward -n aion-monitoring svc/prometheus 9090:9090
# Open http://localhost:9090
```

#### 2. Deploy Jaeger

```bash
cd k8s/jaeger
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
```

**Verify:**
```bash
kubectl get pods -n aion-monitoring -l app=jaeger
kubectl logs -n aion-monitoring -l app=jaeger

# Port forward to access UI
kubectl port-forward -n aion-monitoring svc/jaeger-query 16686:16686
# Open http://localhost:16686
```

#### 3. Deploy Grafana

```bash
cd k8s/grafana
kubectl apply -f configmap.yaml
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
```

**Verify:**
```bash
kubectl get pods -n aion-monitoring -l app=grafana

# Port forward to access UI
kubectl port-forward -n aion-monitoring svc/grafana 3000:3000
# Open http://localhost:3000 (admin/admin)
```

#### 4. Deploy AlertManager

```bash
cd monitoring/alertmanager
kubectl create configmap alertmanager-config \
  --namespace aion-monitoring \
  --from-file=alertmanager.yml

kubectl apply -f k8s/alertmanager/deployment.yaml
kubectl apply -f k8s/alertmanager/service.yaml
```

### Configure Data Sources in Grafana

1. Login to Grafana (admin/admin)
2. Go to Configuration → Data Sources
3. Add Prometheus:
   - URL: http://prometheus.aion-monitoring:9090
   - Access: Server
4. Add Jaeger:
   - URL: http://jaeger-query.aion-monitoring:16686
5. Import dashboards from `monitoring/grafana/dashboards/`

---

## Database Setup

### PostgreSQL Initialization

#### Local Setup

```bash
# Start PostgreSQL
docker run -d \
  --name ectus-postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=ectus_dev \
  -p 5432:5432 \
  -v postgres_data:/var/lib/postgresql/data \
  postgres:15

# Wait for startup
sleep 5

# Run migrations
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/ectus_dev
cargo sqlx migrate run
```

#### Kubernetes Setup

```bash
# Migrations job
kubectl create job migrate-db \
  --namespace aion-production \
  --image=your-registry/ectus-r-api:latest \
  -- cargo sqlx migrate run

# Check job status
kubectl get jobs -n aion-production
kubectl logs -n aion-production job/migrate-db
```

### Database Backup

**Manual backup:**
```bash
kubectl exec -n aion-production postgres-0 -- \
  pg_dump -U postgres ectus_production > backup-$(date +%Y%m%d).sql
```

**Automated backup (CronJob):**
```yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: postgres-backup
  namespace: aion-production
spec:
  schedule: "0 2 * * *"  # Daily at 2 AM
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: backup
            image: postgres:15
            command:
            - /bin/sh
            - -c
            - |
              pg_dump -h postgres -U postgres ectus_production | \
              gzip > /backup/backup-$(date +%Y%m%d-%H%M%S).sql.gz
            env:
            - name: PGPASSWORD
              valueFrom:
                secretKeyRef:
                  name: postgres-credentials
                  key: password
            volumeMounts:
            - name: backup
              mountPath: /backup
          volumes:
          - name: backup
            persistentVolumeClaim:
              claimName: postgres-backup-pvc
          restartPolicy: OnFailure
```

---

## Configuration

### Environment Variables

**Required:**
```bash
DATABASE_URL=postgres://user:pass@host:5432/db
REDIS_URL=redis://host:6379
JWT_SECRET=your-secret-key
```

**Optional:**
```bash
API_HOST=0.0.0.0
API_PORT=8080
RUST_LOG=info
JAEGER_ENDPOINT=http://localhost:4318/v1/traces
PROMETHEUS_PUSHGATEWAY=http://localhost:9091
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
```

### ConfigMaps

**Prometheus config:**
```bash
kubectl create configmap prometheus-config \
  --namespace aion-monitoring \
  --from-file=monitoring/prometheus/prometheus.yml
```

**Alert rules:**
```bash
kubectl create configmap prometheus-alerts \
  --namespace aion-monitoring \
  --from-file=monitoring/prometheus/alerts/
```

---

## Health Checks

### API Health

```bash
curl http://localhost:8080/health
```

Expected response:
```json
{
  "status": "healthy",
  "database": "connected",
  "redis": "connected",
  "version": "2.0.0"
}
```

### Metrics Health

```bash
curl http://localhost:9091/metrics | grep "^# TYPE"
```

Should show metric definitions.

### Monitoring Stack Health

**Prometheus:**
```bash
curl http://localhost:9090/-/healthy
```

**Jaeger:**
```bash
curl http://localhost:14269/
```

**Grafana:**
```bash
curl http://localhost:3000/api/health
```

---

## Troubleshooting

### Common Issues

#### Pod CrashLoopBackOff

```bash
# View logs
kubectl logs -n aion-production <pod-name> --previous

# Describe pod
kubectl describe pod -n aion-production <pod-name>

# Common causes:
# - Database connection failed
# - Missing secrets
# - Resource limits too low
# - Liveness probe failing
```

#### Database Connection Issues

```bash
# Test connection from pod
kubectl exec -n aion-production -it <api-pod> -- /bin/sh
pg_isready -h postgres -p 5432 -U postgres

# Check postgres logs
kubectl logs -n aion-production postgres-0
```

#### Metrics Not Appearing

```bash
# Check Prometheus targets
kubectl port-forward -n aion-monitoring svc/prometheus 9090:9090
# Open http://localhost:9090/targets

# Verify metrics endpoint
kubectl port-forward -n aion-production svc/aion-api-svc 9091:9091
curl http://localhost:9091/metrics

# Check scrape config
kubectl get configmap -n aion-monitoring prometheus-config -o yaml
```

#### Traces Not Showing

```bash
# Check Jaeger is receiving traces
kubectl logs -n aion-monitoring -l app=jaeger

# Verify OTLP endpoint
kubectl port-forward -n aion-monitoring svc/jaeger-collector 4318:4318
curl http://localhost:4318/v1/traces

# Check app environment
kubectl get deployment -n aion-production aion-web-api -o yaml | grep JAEGER
```

### Performance Tuning

**Increase replicas:**
```bash
kubectl scale deployment -n aion-production aion-web-api --replicas=5
```

**Update HPA:**
```bash
kubectl autoscale deployment -n aion-production aion-web-api \
  --min=3 --max=10 --cpu-percent=70
```

**Database connection pool:**
```yaml
env:
- name: DATABASE_MAX_CONNECTIONS
  value: "100"
```

---

## Rollback

### Kubernetes Rollback

```bash
# View rollout history
kubectl rollout history deployment -n aion-production aion-web-api

# Rollback to previous version
kubectl rollout undo deployment -n aion-production aion-web-api

# Rollback to specific revision
kubectl rollout undo deployment -n aion-production aion-web-api --to-revision=2

# Check rollout status
kubectl rollout status deployment -n aion-production aion-web-api
```

### Docker Rollback

```bash
# Tag previous image
docker tag ectus-r-api:v1.9.0 ectus-r-api:latest

# Restart with docker-compose
docker-compose up -d api
```

---

## Monitoring Deployment

### Verify All Components

```bash
# Check all monitoring pods
kubectl get pods -n aion-monitoring

# Expected output:
# prometheus-0                    1/1     Running
# jaeger-xxx                      1/1     Running
# grafana-xxx                     1/1     Running
# alertmanager-xxx                1/1     Running

# Check metrics collection
kubectl port-forward -n aion-monitoring svc/prometheus 9090:9090
# Query: up{job="aion-api"}
# Should show all API pods with value 1

# Check traces
kubectl port-forward -n aion-monitoring svc/jaeger-query 16686:16686
# Search for service "aion-r"

# Check dashboards
kubectl port-forward -n aion-monitoring svc/grafana 3000:3000
# Login and verify datasources connected
```

---

## Appendix

### Useful Commands

```bash
# Get all resources in namespace
kubectl get all -n aion-production

# View resource usage
kubectl top pods -n aion-production
kubectl top nodes

# Execute command in pod
kubectl exec -n aion-production -it <pod-name> -- /bin/sh

# Copy files from pod
kubectl cp aion-production/<pod-name>:/path/to/file ./local-file

# View events
kubectl get events -n aion-production --sort-by='.lastTimestamp'

# Force delete stuck pod
kubectl delete pod -n aion-production <pod-name> --grace-period=0 --force
```

### Configuration Files Location

- K8s manifests: `k8s/`
- Monitoring config: `monitoring/`
- Docker Compose: `docker-compose.yml`
- Environment: `.env` (not in git)

### Related Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture
- [MONITORING.md](docs/MONITORING.md) - Monitoring guide
- [INCIDENT_RESPONSE.md](docs/operations/incident-response.md) - Incident procedures

---

**Document Version:** 2.0
**Last Review:** 2025-10-04
**Next Review:** 2025-11-04
