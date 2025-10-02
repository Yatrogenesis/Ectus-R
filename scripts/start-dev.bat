@echo off
echo 🚀 Starting AION-R Enterprise Platform...
echo ==========================================

REM Set environment variables
set RUST_LOG=info
set AION_ENVIRONMENT=development
set DATABASE_URL=postgresql://aion_user:aion_pass@localhost:5432/aion_r
set REDIS_URL=redis://:aion_redis_pass@localhost:6379
set JWT_SECRET=your-256-bit-secret-key-for-development

echo 📋 Starting infrastructure services...

REM Check if Docker is available
docker --version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Docker not found. Please install Docker Desktop.
    pause
    exit /b 1
)

echo ⚠️  Windows detected. Using Docker for development...
echo 🔧 Starting services with Docker Compose...

REM Start infrastructure services
docker-compose up -d postgres redis rabbitmq minio prometheus grafana jaeger

echo ⏳ Waiting for services to be ready...
timeout /t 15 /nobreak >nul

echo 🚀 Starting AION-R services with Docker...
docker-compose --profile services up --build -d

echo.
echo 🎉 AION-R Enterprise Platform is running!
echo.
echo 📊 Service URLs:
echo   • API Gateway:    http://localhost:8080
echo   • Auth Service:   http://localhost:8081
echo   • AI Service:     http://localhost:8082
echo   • Monitoring:     http://localhost:8083
echo.
echo 🛠️  Infrastructure URLs:
echo   • Grafana:        http://localhost:3000 (admin/aion_grafana_pass)
echo   • Prometheus:     http://localhost:9090
echo   • Jaeger:         http://localhost:16686
echo   • RabbitMQ:       http://localhost:15672 (aion_user/aion_pass)
echo   • MinIO:          http://localhost:9001 (aion_access_key/aion_secret_key)
echo.
echo 📋 Management Commands:
echo   • scripts\health-check.bat  - Check all services
echo   • scripts\stop-dev.bat      - Stop all services
echo   • scripts\logs.bat          - View service logs
echo.
echo Press any key to view running services...
pause

docker-compose ps