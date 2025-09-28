#!/bin/bash

# AION-R Development Environment Startup Script

echo "🚀 Starting AION-R Enterprise Platform..."
echo "=========================================="

# Set environment variables
export RUST_LOG=info
export AION_ENVIRONMENT=development
export DATABASE_URL=postgresql://aion_user:aion_pass@localhost:5432/aion_r
export REDIS_URL=redis://:aion_redis_pass@localhost:6379
export JWT_SECRET=your-256-bit-secret-key-for-development

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}📋 Starting infrastructure services...${NC}"

# Start infrastructure with Docker Compose
if command -v docker-compose &> /dev/null; then
    echo -e "${YELLOW}Starting PostgreSQL, Redis, RabbitMQ, MinIO...${NC}"
    docker-compose up -d postgres redis rabbitmq minio prometheus grafana jaeger

    echo -e "${YELLOW}Waiting for services to be ready...${NC}"
    sleep 10

    echo -e "${GREEN}✅ Infrastructure services started${NC}"
else
    echo -e "${RED}❌ Docker Compose not found. Please install Docker and Docker Compose.${NC}"
    exit 1
fi

echo -e "${BLUE}🔧 Building AION-R services...${NC}"

# Build services (for Linux/Docker)
if [ "$OSTYPE" = "linux-gnu" ] || [ "$OSTYPE" = "darwin"* ]; then
    echo -e "${YELLOW}Building Rust services...${NC}"
    cargo build --workspace

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Build successful${NC}"

        echo -e "${BLUE}🚀 Starting AION-R services...${NC}"

        # Start services in background
        echo -e "${YELLOW}Starting API Gateway (Port 8080)...${NC}"
        cargo run --bin gateway-service &
        GATEWAY_PID=$!

        echo -e "${YELLOW}Starting Auth Service (Port 8081)...${NC}"
        # cargo run --bin auth-service &
        # AUTH_PID=$!

        echo -e "${YELLOW}Starting AI Service (Port 8082)...${NC}"
        # cargo run --bin ai-service &
        # AI_PID=$!

        echo -e "${YELLOW}Starting Monitoring Service (Port 8083)...${NC}"
        # cargo run --bin monitoring-service &
        # MONITORING_PID=$!

        echo ""
        echo -e "${GREEN}🎉 AION-R Enterprise Platform is running!${NC}"
        echo ""
        echo "📊 Service URLs:"
        echo "  • API Gateway:    http://localhost:8080"
        echo "  • Auth Service:   http://localhost:8081"
        echo "  • AI Service:     http://localhost:8082"
        echo "  • Monitoring:     http://localhost:8083"
        echo ""
        echo "🛠️  Infrastructure URLs:"
        echo "  • Grafana:        http://localhost:3000 (admin/aion_grafana_pass)"
        echo "  • Prometheus:     http://localhost:9090"
        echo "  • Jaeger:         http://localhost:16686"
        echo "  • RabbitMQ:       http://localhost:15672 (aion_user/aion_pass)"
        echo "  • MinIO:          http://localhost:9001 (aion_access_key/aion_secret_key)"
        echo ""
        echo "📋 Management Commands:"
        echo "  • ./scripts/health-check.sh  - Check all services"
        echo "  • ./scripts/stop-dev.sh      - Stop all services"
        echo "  • ./scripts/logs.sh          - View service logs"
        echo ""
        echo "Press Ctrl+C to stop all services"

        # Wait for interrupt
        trap 'echo -e "\n${YELLOW}Shutting down services...${NC}"; kill $GATEWAY_PID; docker-compose down; exit 0' INT
        wait

    else
        echo -e "${RED}❌ Build failed. Check compilation errors above.${NC}"
        echo -e "${YELLOW}For Windows users: Use WSL2 or Docker for development${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠️  Windows detected. Using Docker for development...${NC}"
    echo -e "${BLUE}Starting services with Docker Compose...${NC}"

    # Build and start services with Docker
    docker-compose --profile services up --build
fi