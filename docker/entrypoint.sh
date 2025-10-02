#!/bin/bash
set -e

# AION-R Enterprise Platform Entrypoint Script

# Default values
COMPONENT=${1:-server}
WAIT_FOR_DB=${WAIT_FOR_DB:-true}
RUN_MIGRATIONS=${RUN_MIGRATIONS:-true}

# Database connection parameters
DB_HOST=${DB_HOST:-localhost}
DB_PORT=${DB_PORT:-5432}
DB_NAME=${DB_NAME:-aion}
DB_USER=${DB_USER:-aion_user}
DB_TIMEOUT=${DB_TIMEOUT:-30}

# Redis connection parameters
REDIS_HOST=${REDIS_HOST:-localhost}
REDIS_PORT=${REDIS_PORT:-6379}

echo "Starting AION-R Enterprise Platform - Component: $COMPONENT"
echo "Environment: $AION_ENVIRONMENT"
echo "Log Level: $AION_LOG_LEVEL"

# Function to wait for service to be ready
wait_for_service() {
    local host=$1
    local port=$2
    local service_name=$3
    local timeout=$4

    echo "Waiting for $service_name at $host:$port..."

    for i in $(seq 1 $timeout); do
        if nc -z "$host" "$port" 2>/dev/null; then
            echo "$service_name is ready!"
            return 0
        fi
        echo "Waiting for $service_name... ($i/$timeout)"
        sleep 1
    done

    echo "ERROR: $service_name at $host:$port is not available after $timeout seconds"
    return 1
}

# Function to check database connectivity
check_database() {
    echo "Checking database connectivity..."

    if ! command -v psql &> /dev/null; then
        echo "Warning: psql not available, skipping database connectivity check"
        return 0
    fi

    PGPASSWORD=$DB_PASSWORD psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -c "SELECT 1;" &>/dev/null
    if [ $? -eq 0 ]; then
        echo "Database connectivity check passed"
        return 0
    else
        echo "ERROR: Database connectivity check failed"
        return 1
    fi
}

# Function to run database migrations
run_migrations() {
    echo "Running database migrations..."

    /app/bin/aion-migration migrate --config "$AION_CONFIG_PATH"
    if [ $? -eq 0 ]; then
        echo "Database migrations completed successfully"
    else
        echo "ERROR: Database migrations failed"
        exit 1
    fi
}

# Function to validate configuration
validate_config() {
    echo "Validating configuration..."

    if [ ! -f "$AION_CONFIG_PATH" ]; then
        echo "ERROR: Configuration file not found at $AION_CONFIG_PATH"
        exit 1
    fi

    # Basic configuration validation
    if ! grep -q "^\[server\]" "$AION_CONFIG_PATH"; then
        echo "ERROR: Invalid configuration - missing [server] section"
        exit 1
    fi

    if ! grep -q "^\[database\]" "$AION_CONFIG_PATH"; then
        echo "ERROR: Invalid configuration - missing [database] section"
        exit 1
    fi

    echo "Configuration validation passed"
}

# Function to setup directories and permissions
setup_environment() {
    echo "Setting up environment..."

    # Ensure log directory exists
    mkdir -p /app/logs

    # Ensure data directory exists
    mkdir -p /app/data

    # Set appropriate permissions
    chmod 755 /app/logs /app/data

    echo "Environment setup completed"
}

# Function to start the main server
start_server() {
    echo "Starting AION-R Server..."
    exec /app/bin/aion-server --config "$AION_CONFIG_PATH"
}

# Function to start worker process
start_worker() {
    echo "Starting AION-R Worker..."
    exec /app/bin/aion-worker --config "$AION_CONFIG_PATH"
}

# Function to run migrations only
run_migration_only() {
    echo "Running migrations and exiting..."
    run_migrations
    echo "Migrations completed, exiting"
    exit 0
}

# Function to display version and exit
show_version() {
    echo "AION-R Enterprise Platform"
    echo "Version: $(cat /app/VERSION 2>/dev/null || echo 'unknown')"
    echo "Build: $(cat /app/BUILD_INFO 2>/dev/null || echo 'unknown')"
    exit 0
}

# Main execution logic
main() {
    # Handle special commands
    case "$COMPONENT" in
        version|--version|-v)
            show_version
            ;;
        migrate|migration)
            validate_config
            if [ "$WAIT_FOR_DB" = "true" ]; then
                wait_for_service "$DB_HOST" "$DB_PORT" "PostgreSQL" "$DB_TIMEOUT"
            fi
            run_migration_only
            ;;
        help|--help|-h)
            echo "AION-R Enterprise Platform"
            echo ""
            echo "Usage: $0 [COMPONENT]"
            echo ""
            echo "Components:"
            echo "  server    - Start the main API server (default)"
            echo "  worker    - Start the background worker"
            echo "  migrate   - Run database migrations and exit"
            echo "  version   - Show version information"
            echo "  help      - Show this help message"
            echo ""
            echo "Environment Variables:"
            echo "  WAIT_FOR_DB      - Wait for database before starting (default: true)"
            echo "  RUN_MIGRATIONS   - Run migrations before starting (default: true)"
            echo "  DB_HOST          - Database host (default: localhost)"
            echo "  DB_PORT          - Database port (default: 5432)"
            echo "  DB_NAME          - Database name (default: aion)"
            echo "  DB_USER          - Database user (default: aion_user)"
            echo "  DB_TIMEOUT       - Database connection timeout (default: 30)"
            echo "  REDIS_HOST       - Redis host (default: localhost)"
            echo "  REDIS_PORT       - Redis port (default: 6379)"
            exit 0
            ;;
    esac

    # Validate configuration
    validate_config

    # Setup environment
    setup_environment

    # Wait for dependencies if required
    if [ "$WAIT_FOR_DB" = "true" ]; then
        wait_for_service "$DB_HOST" "$DB_PORT" "PostgreSQL" "$DB_TIMEOUT"
        check_database
    fi

    if [ -n "$REDIS_HOST" ] && [ "$REDIS_HOST" != "localhost" ]; then
        wait_for_service "$REDIS_HOST" "$REDIS_PORT" "Redis" "30"
    fi

    # Run migrations if required
    if [ "$RUN_MIGRATIONS" = "true" ] && [ "$COMPONENT" != "worker" ]; then
        run_migrations
    fi

    # Start the appropriate component
    case "$COMPONENT" in
        server)
            start_server
            ;;
        worker)
            start_worker
            ;;
        *)
            echo "ERROR: Unknown component '$COMPONENT'"
            echo "Use '$0 help' for usage information"
            exit 1
            ;;
    esac
}

# Handle signals for graceful shutdown
trap 'echo "Received SIGTERM, shutting down gracefully..."; exit 0' TERM
trap 'echo "Received SIGINT, shutting down gracefully..."; exit 0' INT

# Run main function
main "$@"