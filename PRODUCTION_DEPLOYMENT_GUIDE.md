# 🚀 Ectus-R Production Deployment Guide
## Complete Production-Ready Deployment with Testing & Monitoring

This guide provides comprehensive instructions for deploying Ectus-R in a production environment with enterprise-grade security, monitoring, and reliability.

## 📋 Prerequisites

### System Requirements
- **CPU:** 8+ cores (16+ recommended)
- **RAM:** 32GB minimum (64GB recommended)
- **Storage:** 500GB NVMe SSD (1TB+ recommended)
- **Network:** 1Gbps connection minimum
- **OS:** Ubuntu 22.04 LTS, CentOS 8+, or RHEL 8+

### Software Requirements
- Docker 24.0+
- Docker Compose 2.20+
- PostgreSQL 15+ (managed or self-hosted)
- Redis 7+ (managed or self-hosted)
- NGINX or similar reverse proxy
- SSL/TLS certificates

## 🔧 Pre-Deployment Setup

### 1. Environment Configuration

Create `.env` file in the project root:

```bash
# Database Configuration
POSTGRES_DB=ectus_r
POSTGRES_USER=ectus_prod
POSTGRES_PASSWORD=<SECURE_PASSWORD_32_CHARS>
POSTGRES_REPLICATION_USER=replicator
POSTGRES_REPLICATION_PASSWORD=<SECURE_REPLICATION_PASSWORD>

# Redis Configuration
REDIS_PASSWORD=<SECURE_REDIS_PASSWORD>

# Application Configuration
JWT_SECRET=<SECURE_JWT_SECRET_64_CHARS>
ADMIN_TOKEN=<SECURE_ADMIN_TOKEN>
RUST_LOG=info

# Monitoring
GRAFANA_PASSWORD=<SECURE_GRAFANA_PASSWORD>

# Security
FAIL_ON_DEGRADED=false
ECTUS_API_HOST=0.0.0.0
ECTUS_API_PORT=8080
```

### 2. Generate Secure Secrets

```bash
# Generate JWT secret (64 characters)
openssl rand -hex 32

# Generate database passwords
openssl rand -base64 32

# Generate admin tokens
uuidgen | tr -d '-' | head -c 32
```

### 3. SSL Certificate Setup

```bash
# For production, use Let's Encrypt or your certificate authority
sudo apt install certbot
sudo certbot --nginx -d api.ectus.ai -d dashboard.ectus.ai

# Or use provided certificates
mkdir -p config/nginx/ssl
cp your-certificate.crt config/nginx/ssl/
cp your-private-key.key config/nginx/ssl/
```

## 🐳 Docker Deployment

### 1. Build Production Images

```bash
# Build optimized production images
docker build -f Dockerfile.production -t ectus-r:latest .

# Verify image security
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  aquasec/trivy image ectus-r:latest
```

### 2. Deploy with Docker Compose

```bash
# Start the full production stack
docker-compose -f docker-compose.production.yml up -d

# Verify all services are healthy
docker-compose -f docker-compose.production.yml ps
```

### 3. Initialize Database

```bash
# Run database migrations
docker-compose exec ectus-api-1 ./migrate_database.sh

# Create initial admin user
docker-compose exec ectus-api-1 ./create_admin_user.sh
```

## 🧪 Testing & Validation

### 1. Health Checks

```bash
# Basic health check
curl -f http://localhost:8080/health

# Detailed health check
HEALTH_CHECK_VERBOSE=true ./scripts/health-check.sh

# Test all endpoints
./tests/smoke-test.sh
```

### 2. Load Testing

```bash
# Install K6
sudo apt install k6

# Run load tests
cd tests/load
k6 run --vus 50 --duration 5m api-load-test.js

# Stress testing
k6 run --vus 100 --duration 10m --rps 1000 api-load-test.js
```

### 3. Security Testing

```bash
# Run security scans
docker run --rm -v $(pwd):/app \
  owasp/zap2docker-weekly zap-baseline.py \
  -t http://localhost:8080

# Test rate limiting
for i in {1..200}; do curl -s http://localhost:8080/api/v1/status; done

# Test authentication
./tests/security/auth-test.sh
```

## 📊 Monitoring Setup

### 1. Grafana Dashboard

Access Grafana at `http://localhost:3000`:
- Username: `admin`
- Password: `<GRAFANA_PASSWORD>`

Import dashboards from `config/grafana/dashboards/`:
- System Metrics Dashboard
- Application Performance Dashboard
- Security Monitoring Dashboard
- Business Metrics Dashboard

### 2. Prometheus Metrics

Key metrics to monitor:
- `ectus_requests_total` - Total API requests
- `ectus_request_duration_seconds` - Request latency
- `ectus_ai_generation_duration_seconds` - AI processing time
- `ectus_database_connections` - Database connection pool
- `ectus_security_violations_total` - Security incidents

### 3. Alerting Rules

Configure alerts in `config/prometheus/alerts.yml`:
- High error rate (>5%)
- High response time (>2s)
- Database connection issues
- High memory usage (>80%)
- Failed authentication attempts (>100/min)

## 🔐 Security Configuration

### 1. Network Security

```bash
# Configure firewall
sudo ufw enable
sudo ufw allow ssh
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw deny 8080/tcp  # Block direct API access
```

### 2. API Rate Limiting

Default limits (configurable in `SecurityConfig`):
- 100 requests/minute per IP
- 20 burst capacity
- 5-minute ban for suspicious activity
- 1000 requests/minute triggers investigation

### 3. Authentication Security

- JWT tokens expire after 24 hours
- Refresh tokens expire after 30 days
- Account lockout after 5 failed attempts
- Email verification required
- Argon2 password hashing

## 🔄 CI/CD Pipeline

### 1. GitHub Actions Workflow

```yaml
# .github/workflows/deploy.yml
name: Production Deployment

on:
  push:
    branches: [main]
    tags: ['v*']

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run tests
        run: |
          cargo test --all-features
          npm test

  security-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Security audit
        run: |
          cargo audit
          npm audit

  deploy:
    needs: [test, security-scan]
    runs-on: ubuntu-latest
    steps:
      - name: Deploy to production
        run: |
          docker-compose -f docker-compose.production.yml up -d
          ./scripts/health-check.sh
```

### 2. Rolling Updates

```bash
# Zero-downtime deployment
./scripts/rolling-update.sh

# Rollback if needed
./scripts/rollback.sh
```

## 📈 Performance Optimization

### 1. Database Optimization

```sql
-- PostgreSQL optimizations
ALTER SYSTEM SET shared_buffers = '8GB';
ALTER SYSTEM SET effective_cache_size = '24GB';
ALTER SYSTEM SET maintenance_work_mem = '2GB';
ALTER SYSTEM SET random_page_cost = 1.1;
```

### 2. Application Tuning

```bash
# Set optimal worker count
export ECTUS_WORKER_COUNT=16

# Enable performance features
export ECTUS_ENABLE_CACHING=true
export ECTUS_ENABLE_COMPRESSION=true
```

### 3. NGINX Configuration

```nginx
# /etc/nginx/sites-available/ectus-r
upstream ectus_backend {
    least_conn;
    server 127.0.0.1:8080 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8081 max_fails=3 fail_timeout=30s;
}

server {
    listen 443 ssl http2;
    server_name api.ectus.ai;

    # SSL configuration
    ssl_certificate /etc/ssl/certs/ectus.crt;
    ssl_certificate_key /etc/ssl/private/ectus.key;

    # Security headers
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    add_header X-XSS-Protection "1; mode=block";

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
    limit_req zone=api burst=20 nodelay;

    location / {
        proxy_pass http://ectus_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## 🔍 Troubleshooting

### Common Issues

1. **High Memory Usage**
   ```bash
   # Check memory usage
   docker stats

   # Optimize AI model cache
   export ECTUS_AI_CACHE_SIZE=2048  # MB
   ```

2. **Database Connection Issues**
   ```bash
   # Check connection pool
   docker-compose logs postgres-primary

   # Increase pool size
   export DATABASE_MAX_CONNECTIONS=100
   ```

3. **Slow API Responses**
   ```bash
   # Profile requests
   curl -w "@curl-format.txt" http://localhost:8080/api/v1/status

   # Check AI inference times
   docker-compose logs ectus-api-1 | grep "inference_time"
   ```

### Log Analysis

```bash
# View application logs
docker-compose logs -f ectus-api-1

# Search for errors
docker-compose logs ectus-api-1 | grep ERROR

# Monitor security events
docker-compose logs ectus-api-1 | grep "security_violation"
```

## 📊 Performance Benchmarks

### Expected Performance Metrics

- **API Response Time:** <200ms (95th percentile)
- **AI Generation Time:** <30s (typical)
- **Database Queries:** <10ms (95th percentile)
- **Memory Usage:** <16GB (normal operation)
- **CPU Usage:** <60% (under load)

### Load Testing Results

```bash
# 100 concurrent users
k6 run --vus 100 --duration 5m api-load-test.js

# Expected results:
# - RPS: 1000+ requests/second
# - Response time: <500ms (p95)
# - Error rate: <0.1%
```

## 🔄 Backup & Recovery

### Database Backup

```bash
# Automated daily backups
docker-compose exec postgres-primary pg_dump \
  -U ectus_prod ectus_r | gzip > backup-$(date +%Y%m%d).sql.gz

# Point-in-time recovery
docker-compose exec postgres-primary pg_basebackup \
  -U replicator -D /backup -Ft -z -P
```

### Application Data Backup

```bash
# Backup AI models and data
tar -czf ectus-data-$(date +%Y%m%d).tar.gz \
  -C /var/lib/docker/volumes/ \
  ectus-r_ai_models \
  ectus-r_app_data_1 \
  ectus-r_app_data_2
```

## 🌐 Multi-Region Deployment

### Geographic Distribution

```yaml
# docker-compose.multi-region.yml
services:
  ectus-api-us-east:
    image: ectus-r:latest
    environment:
      - REGION=us-east-1
      - DATABASE_URL=postgresql://user:pass@db-us-east.ectus.ai/ectus_r

  ectus-api-eu-west:
    image: ectus-r:latest
    environment:
      - REGION=eu-west-1
      - DATABASE_URL=postgresql://user:pass@db-eu-west.ectus.ai/ectus_r
```

### Load Balancer Configuration

```nginx
# Global load balancer
upstream global_ectus {
    server us-east.ectus.ai:443 weight=3;
    server eu-west.ectus.ai:443 weight=2;
    server asia.ectus.ai:443 weight=1;
}
```

## ✅ Deployment Checklist

- [ ] Environment variables configured
- [ ] SSL certificates installed
- [ ] Database initialized and migrated
- [ ] Load balancer configured
- [ ] Monitoring dashboards imported
- [ ] Alerting rules configured
- [ ] Security scans completed
- [ ] Load tests passed
- [ ] Backup strategy implemented
- [ ] Documentation updated
- [ ] Team training completed

## 📞 Support & Maintenance

### Monitoring Contacts
- **Operations Team:** ops@ectus.ai
- **Security Team:** security@ectus.ai
- **Development Team:** dev@ectus.ai

### Emergency Procedures
1. Check system health: `./scripts/health-check.sh`
2. Review logs: `docker-compose logs -f`
3. Scale if needed: `docker-compose up -d --scale ectus-api=4`
4. Contact on-call engineer: +1-XXX-XXX-XXXX

---

## 🎯 Success Metrics

After deployment, expect:
- ✅ **99.9% uptime** with proper monitoring
- ✅ **Sub-second response times** for most endpoints
- ✅ **Zero security incidents** with proper configuration
- ✅ **Automatic scaling** based on load
- ✅ **Comprehensive observability** with metrics and logs

**Ectus-R is now production-ready with enterprise-grade reliability, security, and performance!** 🚀