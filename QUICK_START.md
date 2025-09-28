# AION-R Quick Start Guide

🚀 **Get AION-R Enterprise Platform running in 5 minutes!**

## Prerequisites

- **Docker** & **Docker Compose** (recommended)
- **Rust** 1.70+ (for local development)
- **Git** for cloning

## 🔥 Fastest Start (Docker)

### Windows Users

```batch
# Clone and start
git clone https://github.com/Yatrogenesis/AION-R.git
cd AION-R
scripts\start-dev.bat
```

### Linux/macOS Users

```bash
# Clone and start
git clone https://github.com/Yatrogenesis/AION-R.git
cd AION-R
chmod +x scripts/*.sh
./scripts/start-dev.sh
```

## 🎯 What Gets Started

### Core Services
- **🌐 API Gateway** (Port 8080) - Enterprise API gateway with load balancing
- **🔐 Auth Service** (Port 8081) - Multi-tenant authentication
- **🤖 AI Service** (Port 8082) - AI/ML processing engine
- **📊 Monitoring** (Port 8083) - Metrics and observability

### Infrastructure
- **📊 Grafana** (Port 3000) - Dashboards and visualization
- **📈 Prometheus** (Port 9090) - Metrics collection
- **🔍 Jaeger** (Port 16686) - Distributed tracing
- **🐰 RabbitMQ** (Port 15672) - Message queuing
- **💾 MinIO** (Port 9001) - S3-compatible storage
- **🗄️ PostgreSQL** (Port 5432) - Primary database
- **⚡ Redis** (Port 6379) - Caching and sessions

## 📋 Quick Commands

```bash
# Health check all services
./scripts/health-check.sh        # Linux/macOS
scripts\health-check.bat         # Windows

# View service logs
docker-compose logs -f

# Stop all services
docker-compose down

# Restart specific service
docker-compose restart aion-gateway
```

## 🔗 Access URLs

| Service | URL | Credentials |
|---------|-----|-------------|
| **API Gateway** | http://localhost:8080 | - |
| **Grafana** | http://localhost:3000 | admin / aion_grafana_pass |
| **Prometheus** | http://localhost:9090 | - |
| **Jaeger** | http://localhost:16686 | - |
| **RabbitMQ** | http://localhost:15672 | aion_user / aion_pass |
| **MinIO Console** | http://localhost:9001 | aion_access_key / aion_secret_key |

## 🧪 Test the Platform

### 1. Check System Status
```bash
curl http://localhost:8080/gateway/status
```

### 2. Health Check
```bash
curl http://localhost:8080/gateway/health
```

### 3. View Metrics
```bash
curl http://localhost:8080/gateway/metrics
```

### 4. Test Authentication
```bash
curl -X POST http://localhost:8081/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123"}'
```

## ⚡ Performance Expectations

| Metric | Expected Value |
|--------|----------------|
| **Startup Time** | < 30 seconds |
| **API Response** | < 5ms |
| **Memory Usage** | < 100MB per service |
| **Throughput** | 10,000+ req/s |

## 🛠️ Development Mode

### Local Rust Development
```bash
# Install dependencies
cargo check

# Run specific service
cargo run --bin gateway-service

# Run tests
cargo test

# Format code
cargo fmt

# Security audit
cargo audit
```

### Configuration

Create `.env` file:
```env
AION_ENVIRONMENT=development
DATABASE_URL=postgresql://aion_user:aion_pass@localhost:5432/aion_r
REDIS_URL=redis://:aion_redis_pass@localhost:6379
JWT_SECRET=your-256-bit-secret-key
RUST_LOG=debug
```

## 🏥 Troubleshooting

### Common Issues

**Port already in use:**
```bash
# Check what's using port 8080
netstat -tulpn | grep :8080

# Kill process using port
sudo kill -9 $(sudo lsof -t -i:8080)
```

**Docker issues:**
```bash
# Clean Docker environment
docker-compose down --volumes --remove-orphans
docker system prune -a

# Restart Docker Desktop (Windows/macOS)
```

**Compilation errors (Windows):**
- Use WSL2 or Docker for development
- Install Visual Studio Build Tools
- Use `cargo build` in WSL2

## 📚 Next Steps

1. **[Architecture Guide](docs/architecture/system-design.md)** - Understand the system
2. **[API Documentation](docs/api/README.md)** - Explore the APIs
3. **[Security Guide](docs/security/README.md)** - Configure security
4. **[Deployment Guide](docs/deployment/README.md)** - Deploy to production
5. **[Monitoring Guide](docs/monitoring/README.md)** - Set up monitoring

## 🤝 Support

- **Issues**: [GitHub Issues](https://github.com/Yatrogenesis/AION-R/issues)
- **Documentation**: [Wiki](https://github.com/Yatrogenesis/AION-R/wiki)
- **Enterprise Support**: enterprise@yatrogenesis.com

---

**🎉 Welcome to AION-R Enterprise Platform!**