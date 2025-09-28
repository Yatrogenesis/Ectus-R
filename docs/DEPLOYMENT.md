# AION-R Enterprise Platform Deployment Guide

## Overview

This guide covers deployment options for AION-R Enterprise Platform, from development environments to production-scale enterprise deployments.

## Prerequisites

### System Requirements

#### Minimum Requirements (Development)
- **CPU:** 2 cores, 2.0 GHz
- **Memory:** 4 GB RAM
- **Storage:** 20 GB available space
- **OS:** Linux (Ubuntu 20.04+), macOS 11+, Windows 10+

#### Recommended Production Requirements
- **CPU:** 8+ cores, 3.0 GHz
- **Memory:** 16+ GB RAM
- **Storage:** 100+ GB SSD
- **Network:** 1 Gbps connection

#### Large-Scale Production Requirements
- **CPU:** 16+ cores per node
- **Memory:** 32+ GB RAM per node
- **Storage:** 500+ GB NVMe SSD
- **Network:** 10 Gbps connection

### Software Dependencies
- **Docker:** 20.10+
- **Docker Compose:** 2.0+
- **Kubernetes:** 1.25+ (for K8s deployments)
- **PostgreSQL:** 14+
- **Redis:** 6+
- **NGINX:** 1.20+ (for reverse proxy)

## Development Deployment

### Local Development with Docker Compose

1. **Clone the repository:**
```bash
git clone https://github.com/yatrogenesis/AION-R.git
cd AION-R
```

2. **Set up environment variables:**
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. **Start development environment:**
```bash
cd docker
docker-compose up -d
```

4. **Verify deployment:**
```bash
curl http://localhost:8080/health
```

### Configuration Files

#### Environment Variables (.env)
```bash
# Database Configuration
DB_PASSWORD=your_secure_db_password
REDIS_PASSWORD=your_redis_password

# Application Configuration
AION_ENVIRONMENT=development
AION_LOG_LEVEL=debug
JWT_SECRET=your_jwt_secret_key

# Monitoring
GRAFANA_PASSWORD=admin_password

# External Services
OPENAI_API_KEY=your_openai_key  # Optional
```

#### Development Configuration (docker/config/development.toml)
```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[database]
url = "postgresql://aion_user:password@postgres:5432/aion"
max_connections = 10
min_connections = 2

[redis]
url = "redis://redis:6379"
max_connections = 10

[ai_engine]
default_backend = "candle"
model_cache_size = "1GB"
max_concurrent_inferences = 4

[auth]
jwt_secret = "your_jwt_secret"
token_expiry = "1h"
refresh_token_expiry = "7d"

[logging]
level = "debug"
format = "json"
```

## Production Deployment

### Option 1: Docker Swarm Deployment

#### 1. Initialize Docker Swarm
```bash
docker swarm init
```

#### 2. Create Docker Secrets
```bash
echo "your_db_password" | docker secret create db_password -
echo "your_jwt_secret" | docker secret create jwt_secret -
echo "your_redis_password" | docker secret create redis_password -
```

#### 3. Deploy Stack
```bash
docker stack deploy -c docker-compose.prod.yml aion-r
```

#### 4. Scale Services
```bash
docker service scale aion-r_aion-server=3
docker service scale aion-r_aion-worker=2
```

### Option 2: Kubernetes Deployment

#### Prerequisites
- Kubernetes cluster (EKS, GKE, AKS, or self-managed)
- `kubectl` configured
- Ingress controller (NGINX, Traefik, etc.)
- Certificate manager (cert-manager)

#### 1. Create Namespace
```bash
kubectl create namespace aion-r
```

#### 2. Create Secrets
```bash
kubectl create secret generic aion-db-secret \
  --from-literal=username=aion_user \
  --from-literal=password=your_secure_password \
  -n aion-r

kubectl create secret generic aion-jwt-secret \
  --from-literal=secret=your_jwt_secret \
  -n aion-r

kubectl create secret generic aion-redis-secret \
  --from-literal=password=your_redis_password \
  -n aion-r
```

#### 3. Apply Configuration
```bash
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/postgres.yaml
kubectl apply -f k8s/redis.yaml
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/ingress.yaml
```

#### 4. Verify Deployment
```bash
kubectl get pods -n aion-r
kubectl get services -n aion-r
kubectl get ingress -n aion-r
```

### Production Configuration

#### Production Environment Variables
```bash
# Application
AION_ENVIRONMENT=production
AION_LOG_LEVEL=info
RUST_LOG=info

# Database
DB_HOST=postgres-service
DB_PASSWORD=secure_production_password
DB_MAX_CONNECTIONS=50

# Security
JWT_SECRET=highly_secure_jwt_secret_32_chars
CORS_ORIGINS=https://your-domain.com

# Performance
WORKER_THREADS=8
MAX_REQUEST_SIZE=50MB

# Monitoring
PROMETHEUS_ENABLED=true
JAEGER_ENABLED=true
```

#### Production Configuration (config/production.toml)
```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 8
keep_alive = 75
max_connections = 1000

[database]
url = "postgresql://aion_user:password@postgres:5432/aion"
max_connections = 50
min_connections = 10
connect_timeout = "5s"
idle_timeout = "10m"

[redis]
url = "redis://redis:6379"
max_connections = 50
connection_timeout = "3s"

[ai_engine]
default_backend = "candle"
model_cache_size = "8GB"
max_concurrent_inferences = 16
inference_timeout = "30s"

[auth]
jwt_secret = "${JWT_SECRET}"
token_expiry = "1h"
refresh_token_expiry = "7d"
max_failed_attempts = 5
lockout_duration = "15m"

[rate_limiting]
enabled = true
requests_per_minute = 60
burst_size = 10

[logging]
level = "info"
format = "json"
max_file_size = "100MB"
max_files = 10

[monitoring]
prometheus_enabled = true
metrics_port = 9090
health_check_interval = "30s"

[security]
cors_origins = ["https://your-domain.com"]
max_request_size = "50MB"
enable_https_redirect = true
```

## Cloud Provider Deployments

### Amazon Web Services (AWS)

#### Using EKS (Elastic Kubernetes Service)
```bash
# Create EKS cluster
eksctl create cluster --name aion-r-cluster --region us-west-2

# Configure kubectl
aws eks update-kubeconfig --region us-west-2 --name aion-r-cluster

# Deploy AION-R
kubectl apply -f k8s/aws/
```

#### Using ECS (Elastic Container Service)
```bash
# Register task definition
aws ecs register-task-definition --cli-input-json file://aws/task-definition.json

# Create service
aws ecs create-service --cluster aion-r --service-name aion-r-service \
  --task-definition aion-r:1 --desired-count 3
```

### Google Cloud Platform (GCP)

#### Using GKE (Google Kubernetes Engine)
```bash
# Create GKE cluster
gcloud container clusters create aion-r-cluster \
  --zone us-central1-a --num-nodes 3

# Get credentials
gcloud container clusters get-credentials aion-r-cluster --zone us-central1-a

# Deploy AION-R
kubectl apply -f k8s/gcp/
```

### Microsoft Azure

#### Using AKS (Azure Kubernetes Service)
```bash
# Create resource group
az group create --name aion-r-rg --location eastus

# Create AKS cluster
az aks create --resource-group aion-r-rg --name aion-r-cluster \
  --node-count 3 --enable-addons monitoring

# Get credentials
az aks get-credentials --resource-group aion-r-rg --name aion-r-cluster

# Deploy AION-R
kubectl apply -f k8s/azure/
```

## Database Setup

### PostgreSQL Configuration

#### Production PostgreSQL Setup
```sql
-- Create database and user
CREATE DATABASE aion;
CREATE USER aion_user WITH ENCRYPTED PASSWORD 'secure_password';

-- Grant permissions
GRANT ALL PRIVILEGES ON DATABASE aion TO aion_user;
ALTER USER aion_user CREATEDB;

-- Configure for performance
ALTER SYSTEM SET shared_buffers = '256MB';
ALTER SYSTEM SET effective_cache_size = '1GB';
ALTER SYSTEM SET maintenance_work_mem = '64MB';
ALTER SYSTEM SET checkpoint_completion_target = 0.9;
ALTER SYSTEM SET wal_buffers = '16MB';
SELECT pg_reload_conf();
```

#### Database Migration
```bash
# Run migrations
./scripts/migrate.sh

# Or using the binary
./target/release/aion-migration migrate --config config/production.toml
```

### Redis Configuration

#### Production Redis Configuration (redis.conf)
```
# Memory management
maxmemory 2gb
maxmemory-policy allkeys-lru

# Persistence
save 900 1
save 300 10
save 60 10000

# Security
requirepass your_secure_redis_password
bind 127.0.0.1 10.0.0.1

# Performance
tcp-keepalive 300
timeout 300
```

## SSL/TLS Configuration

### Using Let's Encrypt with cert-manager

#### 1. Install cert-manager
```bash
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.11.0/cert-manager.yaml
```

#### 2. Create ClusterIssuer
```yaml
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: letsencrypt-prod
spec:
  acme:
    server: https://acme-v02.api.letsencrypt.org/directory
    email: admin@your-domain.com
    privateKeySecretRef:
      name: letsencrypt-prod
    solvers:
    - http01:
        ingress:
          class: nginx
```

#### 3. Configure Ingress with TLS
```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: aion-ingress
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
  - hosts:
    - api.your-domain.com
    secretName: aion-tls-secret
  rules:
  - host: api.your-domain.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: aion-server-service
            port:
              number: 8080
```

## Monitoring and Logging

### Prometheus Configuration
```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  - "aion_rules.yml"

scrape_configs:
  - job_name: 'aion-server'
    static_configs:
      - targets: ['aion-server:9090']
    metrics_path: /metrics
    scrape_interval: 10s

  - job_name: 'aion-worker'
    static_configs:
      - targets: ['aion-worker:9090']
    metrics_path: /metrics
    scrape_interval: 10s
```

### Grafana Dashboard
Import the AION-R dashboard:
- Dashboard ID: `aion-r-overview`
- Includes metrics for API requests, AI processing, database performance

### Alerting Rules
```yaml
groups:
- name: aion-r-alerts
  rules:
  - alert: HighErrorRate
    expr: rate(aion_http_requests_total{status=~"5.."}[5m]) > 0.1
    for: 2m
    labels:
      severity: warning
    annotations:
      summary: High error rate detected

  - alert: DatabaseConnectionIssue
    expr: aion_database_connections_active / aion_database_connections_max > 0.8
    for: 1m
    labels:
      severity: critical
    annotations:
      summary: Database connection pool nearly exhausted
```

## Backup and Recovery

### Database Backup
```bash
# Daily backup script
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
pg_dump -h postgres-host -U aion_user aion > backup_$DATE.sql
aws s3 cp backup_$DATE.sql s3://your-backup-bucket/
```

### Restore Procedure
```bash
# Restore from backup
psql -h postgres-host -U aion_user -d aion < backup_20240120_120000.sql
```

## Security Hardening

### Network Security
- Use private subnets for database and Redis
- Configure security groups to restrict access
- Enable VPC Flow Logs for monitoring

### Application Security
- Keep all dependencies updated
- Regular security scans with `cargo audit`
- Enable HTTPS-only communication
- Implement proper input validation

### Secrets Management
- Use Kubernetes secrets or cloud provider secret managers
- Rotate secrets regularly
- Never commit secrets to version control

## Performance Tuning

### Application Tuning
```toml
[server]
workers = 16  # 2x CPU cores
keep_alive = 75
max_connections = 2000

[database]
max_connections = 100  # Tune based on load
connection_timeout = "3s"
statement_timeout = "30s"

[ai_engine]
max_concurrent_inferences = 32  # Tune based on memory
model_cache_size = "16GB"
```

### Database Tuning
```sql
-- PostgreSQL performance tuning
ALTER SYSTEM SET shared_buffers = '512MB';
ALTER SYSTEM SET effective_cache_size = '2GB';
ALTER SYSTEM SET work_mem = '16MB';
ALTER SYSTEM SET maintenance_work_mem = '128MB';
```

## Troubleshooting

### Common Issues

#### 1. High Memory Usage
```bash
# Check memory usage
kubectl top pods -n aion-r

# Adjust memory limits
kubectl patch deployment aion-server -p '{"spec":{"template":{"spec":{"containers":[{"name":"aion-server","resources":{"limits":{"memory":"2Gi"}}}]}}}}'
```

#### 2. Database Connection Issues
```bash
# Check database connectivity
kubectl exec -it deployment/aion-server -n aion-r -- /bin/bash
psql -h postgres-service -U aion_user -d aion

# Check connection pool
curl http://aion-server:9090/metrics | grep database_connections
```

#### 3. AI Model Loading Issues
```bash
# Check model status
curl -H "Authorization: Bearer $TOKEN" http://api.aion-r.com/api/v1/ai/models

# Check logs
kubectl logs deployment/aion-server -n aion-r | grep "model"
```

### Log Analysis
```bash
# Real-time logs
kubectl logs -f deployment/aion-server -n aion-r

# Search for errors
kubectl logs deployment/aion-server -n aion-r | grep ERROR

# Performance metrics
curl http://aion-server:9090/metrics
```

## Maintenance

### Regular Maintenance Tasks
1. **Weekly:** Review logs and metrics
2. **Monthly:** Update dependencies and security patches
3. **Quarterly:** Performance review and capacity planning
4. **Annually:** Security audit and disaster recovery testing

### Update Procedure
```bash
# Update AION-R to new version
kubectl set image deployment/aion-server aion-server=aion-r/server:v1.1.0 -n aion-r

# Monitor rollout
kubectl rollout status deployment/aion-server -n aion-r

# Rollback if needed
kubectl rollout undo deployment/aion-server -n aion-r
```

For additional support or specific deployment questions, please contact our enterprise support team or check the troubleshooting section in our documentation.