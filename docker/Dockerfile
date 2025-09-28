# AION-R Enterprise Platform
# Multi-stage Docker build for production deployment

# Stage 1: Build dependencies and compile
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    cmake \
    build-essential \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Build dependencies (this layer will be cached unless Cargo.toml changes)
RUN cargo build --release --locked

# Stage 2: Runtime environment
FROM debian:bookworm-slim as runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r aion && useradd -r -g aion aion

# Create app directories
RUN mkdir -p /app/bin /app/config /app/logs /app/data \
    && chown -R aion:aion /app

# Copy compiled binaries
COPY --from=builder /app/target/release/aion-server /app/bin/
COPY --from=builder /app/target/release/aion-worker /app/bin/
COPY --from=builder /app/target/release/aion-migration /app/bin/

# Copy configuration files
COPY config/production.toml /app/config/default.toml
COPY docker/entrypoint.sh /app/entrypoint.sh

# Set permissions
RUN chmod +x /app/entrypoint.sh /app/bin/*
RUN chown -R aion:aion /app

# Switch to non-root user
USER aion

# Set working directory
WORKDIR /app

# Environment variables
ENV RUST_LOG=info
ENV AION_CONFIG_PATH=/app/config/default.toml
ENV AION_LOG_LEVEL=info
ENV AION_ENVIRONMENT=production

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Expose ports
EXPOSE 8080 9090

# Set entrypoint
ENTRYPOINT ["/app/entrypoint.sh"]

# Default command
CMD ["server"]