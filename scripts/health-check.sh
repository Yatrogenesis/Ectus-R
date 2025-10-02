#!/bin/bash

# AION-R Health Check Script

echo "🏥 AION-R Enterprise Platform Health Check"
echo "==========================================="

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

check_service() {
    local service_name=$1
    local url=$2
    local expected_status=${3:-200}

    echo -n "Checking $service_name... "

    if command -v curl &> /dev/null; then
        response=$(curl -s -o /dev/null -w "%{http_code}" "$url" 2>/dev/null)
        if [ "$response" = "$expected_status" ]; then
            echo -e "${GREEN}✅ Healthy${NC}"
            return 0
        else
            echo -e "${RED}❌ Unhealthy (HTTP $response)${NC}"
            return 1
        fi
    else
        echo -e "${YELLOW}⚠️  curl not available${NC}"
        return 2
    fi
}

echo "📊 Core Services:"
check_service "API Gateway" "http://localhost:8080/gateway/health"
check_service "Auth Service" "http://localhost:8081/health"
check_service "AI Service" "http://localhost:8082/health"
check_service "Monitoring" "http://localhost:8083/health"

echo ""
echo "🛠️  Infrastructure Services:"
check_service "Grafana" "http://localhost:3000/api/health"
check_service "Prometheus" "http://localhost:9090/-/healthy"
check_service "Jaeger" "http://localhost:16686/"
check_service "RabbitMQ" "http://localhost:15672/"
check_service "MinIO" "http://localhost:9000/minio/health/live"

echo ""
echo "💾 Database Services:"
if command -v psql &> /dev/null; then
    echo -n "PostgreSQL... "
    if PGPASSWORD=aion_pass psql -h localhost -U aion_user -d aion_r -c "SELECT 1;" &>/dev/null; then
        echo -e "${GREEN}✅ Connected${NC}"
    else
        echo -e "${RED}❌ Connection failed${NC}"
    fi
else
    echo -e "${YELLOW}PostgreSQL... ⚠️  psql not available${NC}"
fi

if command -v redis-cli &> /dev/null; then
    echo -n "Redis... "
    if redis-cli -a aion_redis_pass ping &>/dev/null; then
        echo -e "${GREEN}✅ Connected${NC}"
    else
        echo -e "${RED}❌ Connection failed${NC}"
    fi
else
    echo -e "${YELLOW}Redis... ⚠️  redis-cli not available${NC}"
fi

echo ""
echo "📊 System Status:"
echo "Docker containers:"
docker-compose ps

echo ""
echo "📈 Quick Performance Check:"
if command -v curl &> /dev/null; then
    echo -n "API Gateway response time... "
    response_time=$(curl -o /dev/null -s -w "%{time_total}" http://localhost:8080/gateway/status 2>/dev/null)
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}${response_time}s${NC}"
    else
        echo -e "${RED}Failed${NC}"
    fi
fi

echo ""
echo "Health check completed! 🏁"