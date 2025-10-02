//! # Database Migrations
//!
//! Database migration system for AION-R Enterprise Platform.

use anyhow::{Context, Result};
use sqlx::{Pool, Postgres, Row};
use tracing::{info, warn};

/// Migration struct containing SQL and metadata
#[derive(Debug, Clone)]
pub struct Migration {
    pub version: u32,
    pub name: String,
    pub up_sql: &'static str,
    pub down_sql: &'static str,
}

/// Migration manager for handling database schema changes
pub struct MigrationManager {
    pool: Pool<Postgres>,
    migrations: Vec<Migration>,
}

impl MigrationManager {
    /// Create a new migration manager
    pub fn new(pool: Pool<Postgres>) -> Self {
        let migrations = get_all_migrations();
        Self { pool, migrations }
    }

    /// Initialize the migration system (create migration table)
    pub async fn init(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create schema_migrations table")?;

        info!("Migration system initialized");
        Ok(())
    }

    /// Get current schema version
    pub async fn get_current_version(&self) -> Result<u32> {
        let result = sqlx::query("SELECT COALESCE(MAX(version), 0) as version FROM schema_migrations")
            .fetch_one(&self.pool)
            .await
            .context("Failed to get current schema version")?;

        Ok(result.get::<i32, _>("version") as u32)
    }

    /// Run all pending migrations
    pub async fn migrate(&self) -> Result<()> {
        self.init().await?;

        let current_version = self.get_current_version().await?;
        info!("Current schema version: {}", current_version);

        let pending_migrations: Vec<_> = self
            .migrations
            .iter()
            .filter(|m| m.version > current_version)
            .collect();

        if pending_migrations.is_empty() {
            info!("No pending migrations");
            return Ok(());
        }

        info!("Running {} pending migrations", pending_migrations.len());

        for migration in pending_migrations {
            self.apply_migration(migration).await
                .with_context(|| format!("Failed to apply migration {}: {}", migration.version, migration.name))?;
        }

        info!("All migrations completed successfully");
        Ok(())
    }

    /// Apply a single migration
    async fn apply_migration(&self, migration: &Migration) -> Result<()> {
        info!("Applying migration {}: {}", migration.version, migration.name);

        // Start transaction
        let mut tx = self.pool.begin().await
            .context("Failed to start transaction")?;

        // Run migration SQL
        sqlx::query(migration.up_sql)
            .execute(&mut *tx)
            .await
            .with_context(|| format!("Failed to execute migration SQL for {}", migration.name))?;

        // Record migration as applied
        sqlx::query(
            "INSERT INTO schema_migrations (version, name) VALUES ($1, $2)"
        )
        .bind(migration.version as i32)
        .bind(&migration.name)
        .execute(&mut *tx)
        .await
        .context("Failed to record migration")?;

        // Commit transaction
        tx.commit().await
            .context("Failed to commit migration transaction")?;

        info!("Successfully applied migration {}: {}", migration.version, migration.name);
        Ok(())
    }

    /// Rollback to a specific version
    pub async fn rollback_to(&self, target_version: u32) -> Result<()> {
        let current_version = self.get_current_version().await?;

        if target_version >= current_version {
            warn!("Target version {} is not less than current version {}", target_version, current_version);
            return Ok(());
        }

        let rollback_migrations: Vec<_> = self
            .migrations
            .iter()
            .filter(|m| m.version > target_version && m.version <= current_version)
            .rev() // Apply rollbacks in reverse order
            .collect();

        info!("Rolling back {} migrations to version {}", rollback_migrations.len(), target_version);

        for migration in rollback_migrations {
            self.rollback_migration(migration).await
                .with_context(|| format!("Failed to rollback migration {}: {}", migration.version, migration.name))?;
        }

        info!("Rollback completed successfully");
        Ok(())
    }

    /// Rollback a single migration
    async fn rollback_migration(&self, migration: &Migration) -> Result<()> {
        info!("Rolling back migration {}: {}", migration.version, migration.name);

        // Start transaction
        let mut tx = self.pool.begin().await
            .context("Failed to start rollback transaction")?;

        // Run rollback SQL
        sqlx::query(migration.down_sql)
            .execute(&mut *tx)
            .await
            .with_context(|| format!("Failed to execute rollback SQL for {}", migration.name))?;

        // Remove migration record
        sqlx::query("DELETE FROM schema_migrations WHERE version = $1")
            .bind(migration.version as i32)
            .execute(&mut *tx)
            .await
            .context("Failed to remove migration record")?;

        // Commit transaction
        tx.commit().await
            .context("Failed to commit rollback transaction")?;

        info!("Successfully rolled back migration {}: {}", migration.version, migration.name);
        Ok(())
    }

    /// List all migrations and their status
    pub async fn status(&self) -> Result<Vec<MigrationStatus>> {
        self.init().await?;

        let applied_versions: Vec<u32> = sqlx::query("SELECT version FROM schema_migrations")
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch applied migrations")?
            .into_iter()
            .map(|row| row.get::<i32, _>("version") as u32)
            .collect();

        let mut statuses = Vec::new();

        for migration in &self.migrations {
            let is_applied = applied_versions.contains(&migration.version);
            statuses.push(MigrationStatus {
                version: migration.version,
                name: migration.name.clone(),
                applied: is_applied,
            });
        }

        Ok(statuses)
    }
}

#[derive(Debug)]
pub struct MigrationStatus {
    pub version: u32,
    pub name: String,
    pub applied: bool,
}

/// Get all migrations in order
fn get_all_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            name: "initial_schema".to_string(),
            up_sql: MIGRATION_001_UP,
            down_sql: MIGRATION_001_DOWN,
        },
        Migration {
            version: 2,
            name: "add_indexes".to_string(),
            up_sql: MIGRATION_002_UP,
            down_sql: MIGRATION_002_DOWN,
        },
        Migration {
            version: 3,
            name: "add_audit_triggers".to_string(),
            up_sql: MIGRATION_003_UP,
            down_sql: MIGRATION_003_DOWN,
        },
        Migration {
            version: 4,
            name: "add_performance_tables".to_string(),
            up_sql: MIGRATION_004_UP,
            down_sql: MIGRATION_004_DOWN,
        },
    ]
}

// Migration 001: Initial schema
const MIGRATION_001_UP: &str = r#"
-- Create custom types
CREATE TYPE user_role AS ENUM ('admin', 'user', 'developer', 'viewer');
CREATE TYPE user_status AS ENUM ('active', 'inactive', 'suspended', 'pending');
CREATE TYPE subscription_tier AS ENUM ('free', 'pro', 'enterprise', 'custom');
CREATE TYPE tenant_status AS ENUM ('active', 'suspended', 'trial', 'expired');
CREATE TYPE model_type AS ENUM ('text', 'image', 'audio', 'multimodal', 'traditional');
CREATE TYPE request_status AS ENUM ('pending', 'processing', 'completed', 'failed', 'cancelled');
CREATE TYPE job_status AS ENUM ('queued', 'running', 'completed', 'failed', 'cancelled');
CREATE TYPE dataset_type AS ENUM ('text', 'image', 'audio', 'video', 'structured', 'multimodal');
CREATE TYPE notification_type AS ENUM ('info', 'warning', 'error', 'success', 'system');

-- Tenants table (multi-tenancy)
CREATE TABLE tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    subscription_tier subscription_tier NOT NULL DEFAULT 'free',
    status tenant_status NOT NULL DEFAULT 'active',
    settings JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    role user_role NOT NULL DEFAULT 'user',
    status user_status NOT NULL DEFAULT 'pending',
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    last_login TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE
);

-- API keys table
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    key_hash VARCHAR(255) NOT NULL,
    permissions TEXT[] DEFAULT '{}',
    expires_at TIMESTAMP WITH TIME ZONE,
    last_used TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Models table
CREATE TABLE models (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    description TEXT,
    model_type model_type NOT NULL,
    provider VARCHAR(100) NOT NULL,
    version VARCHAR(50) NOT NULL,
    size_bytes BIGINT,
    parameters BIGINT,
    supported_tasks TEXT[] DEFAULT '{}',
    input_modalities TEXT[] DEFAULT '{}',
    output_modalities TEXT[] DEFAULT '{}',
    config JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_public BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Inference requests table
CREATE TABLE inference_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    model_id UUID NOT NULL REFERENCES models(id) ON DELETE RESTRICT,
    request_data JSONB NOT NULL,
    response_data JSONB,
    status request_status NOT NULL DEFAULT 'pending',
    error_message TEXT,
    processing_time_ms INTEGER,
    tokens_used INTEGER,
    cost_cents INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE
);

-- Usage tracking table
CREATE TABLE usage_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    model_id UUID NOT NULL REFERENCES models(id) ON DELETE RESTRICT,
    date DATE NOT NULL,
    request_count INTEGER NOT NULL DEFAULT 0,
    total_tokens INTEGER NOT NULL DEFAULT 0,
    total_processing_time_ms BIGINT NOT NULL DEFAULT 0,
    total_cost_cents INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(tenant_id, model_id, date)
);

-- Audit logs table
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(100) NOT NULL,
    resource_id UUID,
    details JSONB NOT NULL DEFAULT '{}',
    ip_address INET,
    user_agent TEXT,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- System configuration table
CREATE TABLE system_config (
    key VARCHAR(255) PRIMARY KEY,
    value JSONB NOT NULL,
    description TEXT,
    is_sensitive BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Performance metrics table
CREATE TABLE performance_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    metric_name VARCHAR(255) NOT NULL,
    metric_value DOUBLE PRECISION NOT NULL,
    dimensions JSONB NOT NULL DEFAULT '{}',
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    model_id UUID REFERENCES models(id) ON DELETE SET NULL
);

-- Cache entries table
CREATE TABLE cache_entries (
    key VARCHAR(512) PRIMARY KEY,
    value JSONB NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    access_count INTEGER NOT NULL DEFAULT 0,
    last_accessed TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Training jobs table
CREATE TABLE training_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    model_type model_type NOT NULL,
    training_config JSONB NOT NULL DEFAULT '{}',
    dataset_config JSONB NOT NULL DEFAULT '{}',
    status job_status NOT NULL DEFAULT 'queued',
    progress_percentage SMALLINT NOT NULL DEFAULT 0 CHECK (progress_percentage >= 0 AND progress_percentage <= 100),
    logs TEXT,
    result_model_id UUID REFERENCES models(id) ON DELETE SET NULL,
    error_message TEXT,
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Datasets table
CREATE TABLE datasets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    dataset_type dataset_type NOT NULL,
    format VARCHAR(50) NOT NULL,
    size_bytes BIGINT NOT NULL,
    record_count BIGINT,
    schema_info JSONB,
    storage_path VARCHAR(1000) NOT NULL,
    tags TEXT[] DEFAULT '{}',
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Feature flags table
CREATE TABLE feature_flags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    is_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    rules JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Notifications table
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    notification_type notification_type NOT NULL DEFAULT 'info',
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    data JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    read_at TIMESTAMP WITH TIME ZONE
);

-- Create updated_at triggers
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_tenants_updated_at BEFORE UPDATE ON tenants
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_api_keys_updated_at BEFORE UPDATE ON api_keys
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_models_updated_at BEFORE UPDATE ON models
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_usage_records_updated_at BEFORE UPDATE ON usage_records
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_system_config_updated_at BEFORE UPDATE ON system_config
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_training_jobs_updated_at BEFORE UPDATE ON training_jobs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_datasets_updated_at BEFORE UPDATE ON datasets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_feature_flags_updated_at BEFORE UPDATE ON feature_flags
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
"#;

const MIGRATION_001_DOWN: &str = r#"
-- Drop tables in reverse order
DROP TABLE IF EXISTS notifications CASCADE;
DROP TABLE IF EXISTS feature_flags CASCADE;
DROP TABLE IF EXISTS datasets CASCADE;
DROP TABLE IF EXISTS training_jobs CASCADE;
DROP TABLE IF EXISTS cache_entries CASCADE;
DROP TABLE IF EXISTS performance_metrics CASCADE;
DROP TABLE IF EXISTS system_config CASCADE;
DROP TABLE IF EXISTS audit_logs CASCADE;
DROP TABLE IF EXISTS usage_records CASCADE;
DROP TABLE IF EXISTS inference_requests CASCADE;
DROP TABLE IF EXISTS models CASCADE;
DROP TABLE IF EXISTS api_keys CASCADE;
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS tenants CASCADE;

-- Drop function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop types
DROP TYPE IF EXISTS notification_type;
DROP TYPE IF EXISTS dataset_type;
DROP TYPE IF EXISTS job_status;
DROP TYPE IF EXISTS request_status;
DROP TYPE IF EXISTS model_type;
DROP TYPE IF EXISTS tenant_status;
DROP TYPE IF EXISTS subscription_tier;
DROP TYPE IF EXISTS user_status;
DROP TYPE IF EXISTS user_role;
"#;

// Migration 002: Add indexes for performance
const MIGRATION_002_UP: &str = r#"
-- Indexes for users table
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_tenant_id ON users(tenant_id);
CREATE INDEX idx_users_status ON users(status);
CREATE INDEX idx_users_created_at ON users(created_at);

-- Indexes for tenants table
CREATE INDEX idx_tenants_slug ON tenants(slug);
CREATE INDEX idx_tenants_status ON tenants(status);
CREATE INDEX idx_tenants_subscription_tier ON tenants(subscription_tier);

-- Indexes for api_keys table
CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);
CREATE INDEX idx_api_keys_tenant_id ON api_keys(tenant_id);
CREATE INDEX idx_api_keys_is_active ON api_keys(is_active);
CREATE INDEX idx_api_keys_expires_at ON api_keys(expires_at);

-- Indexes for models table
CREATE INDEX idx_models_name ON models(name);
CREATE INDEX idx_models_model_type ON models(model_type);
CREATE INDEX idx_models_provider ON models(provider);
CREATE INDEX idx_models_is_active ON models(is_active);
CREATE INDEX idx_models_is_public ON models(is_public);

-- Indexes for inference_requests table
CREATE INDEX idx_inference_requests_user_id ON inference_requests(user_id);
CREATE INDEX idx_inference_requests_tenant_id ON inference_requests(tenant_id);
CREATE INDEX idx_inference_requests_model_id ON inference_requests(model_id);
CREATE INDEX idx_inference_requests_status ON inference_requests(status);
CREATE INDEX idx_inference_requests_created_at ON inference_requests(created_at);
CREATE INDEX idx_inference_requests_completed_at ON inference_requests(completed_at);

-- Indexes for usage_records table
CREATE INDEX idx_usage_records_tenant_id ON usage_records(tenant_id);
CREATE INDEX idx_usage_records_model_id ON usage_records(model_id);
CREATE INDEX idx_usage_records_date ON usage_records(date);
CREATE INDEX idx_usage_records_tenant_date ON usage_records(tenant_id, date);

-- Indexes for audit_logs table
CREATE INDEX idx_audit_logs_tenant_id ON audit_logs(tenant_id);
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_resource_type ON audit_logs(resource_type);
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(timestamp);

-- Indexes for performance_metrics table
CREATE INDEX idx_performance_metrics_metric_name ON performance_metrics(metric_name);
CREATE INDEX idx_performance_metrics_timestamp ON performance_metrics(timestamp);
CREATE INDEX idx_performance_metrics_tenant_id ON performance_metrics(tenant_id);
CREATE INDEX idx_performance_metrics_model_id ON performance_metrics(model_id);

-- Indexes for cache_entries table
CREATE INDEX idx_cache_entries_expires_at ON cache_entries(expires_at);
CREATE INDEX idx_cache_entries_last_accessed ON cache_entries(last_accessed);

-- Indexes for training_jobs table
CREATE INDEX idx_training_jobs_user_id ON training_jobs(user_id);
CREATE INDEX idx_training_jobs_tenant_id ON training_jobs(tenant_id);
CREATE INDEX idx_training_jobs_status ON training_jobs(status);
CREATE INDEX idx_training_jobs_created_at ON training_jobs(created_at);

-- Indexes for datasets table
CREATE INDEX idx_datasets_tenant_id ON datasets(tenant_id);
CREATE INDEX idx_datasets_user_id ON datasets(user_id);
CREATE INDEX idx_datasets_dataset_type ON datasets(dataset_type);
CREATE INDEX idx_datasets_is_public ON datasets(is_public);
CREATE INDEX idx_datasets_tags ON datasets USING GIN(tags);

-- Indexes for feature_flags table
CREATE INDEX idx_feature_flags_name ON feature_flags(name);
CREATE INDEX idx_feature_flags_is_enabled ON feature_flags(is_enabled);

-- Indexes for notifications table
CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_notifications_tenant_id ON notifications(tenant_id);
CREATE INDEX idx_notifications_is_read ON notifications(is_read);
CREATE INDEX idx_notifications_created_at ON notifications(created_at);
CREATE INDEX idx_notifications_notification_type ON notifications(notification_type);
"#;

const MIGRATION_002_DOWN: &str = r#"
-- Drop all indexes
DROP INDEX IF EXISTS idx_notifications_notification_type;
DROP INDEX IF EXISTS idx_notifications_created_at;
DROP INDEX IF EXISTS idx_notifications_is_read;
DROP INDEX IF EXISTS idx_notifications_tenant_id;
DROP INDEX IF EXISTS idx_notifications_user_id;
DROP INDEX IF EXISTS idx_feature_flags_is_enabled;
DROP INDEX IF EXISTS idx_feature_flags_name;
DROP INDEX IF EXISTS idx_datasets_tags;
DROP INDEX IF EXISTS idx_datasets_is_public;
DROP INDEX IF EXISTS idx_datasets_dataset_type;
DROP INDEX IF EXISTS idx_datasets_user_id;
DROP INDEX IF EXISTS idx_datasets_tenant_id;
DROP INDEX IF EXISTS idx_training_jobs_created_at;
DROP INDEX IF EXISTS idx_training_jobs_status;
DROP INDEX IF EXISTS idx_training_jobs_tenant_id;
DROP INDEX IF EXISTS idx_training_jobs_user_id;
DROP INDEX IF EXISTS idx_cache_entries_last_accessed;
DROP INDEX IF EXISTS idx_cache_entries_expires_at;
DROP INDEX IF EXISTS idx_performance_metrics_model_id;
DROP INDEX IF EXISTS idx_performance_metrics_tenant_id;
DROP INDEX IF EXISTS idx_performance_metrics_timestamp;
DROP INDEX IF EXISTS idx_performance_metrics_metric_name;
DROP INDEX IF EXISTS idx_audit_logs_timestamp;
DROP INDEX IF EXISTS idx_audit_logs_resource_type;
DROP INDEX IF EXISTS idx_audit_logs_action;
DROP INDEX IF EXISTS idx_audit_logs_user_id;
DROP INDEX IF EXISTS idx_audit_logs_tenant_id;
DROP INDEX IF EXISTS idx_usage_records_tenant_date;
DROP INDEX IF EXISTS idx_usage_records_date;
DROP INDEX IF EXISTS idx_usage_records_model_id;
DROP INDEX IF EXISTS idx_usage_records_tenant_id;
DROP INDEX IF EXISTS idx_inference_requests_completed_at;
DROP INDEX IF EXISTS idx_inference_requests_created_at;
DROP INDEX IF EXISTS idx_inference_requests_status;
DROP INDEX IF EXISTS idx_inference_requests_model_id;
DROP INDEX IF EXISTS idx_inference_requests_tenant_id;
DROP INDEX IF EXISTS idx_inference_requests_user_id;
DROP INDEX IF EXISTS idx_models_is_public;
DROP INDEX IF EXISTS idx_models_is_active;
DROP INDEX IF EXISTS idx_models_provider;
DROP INDEX IF EXISTS idx_models_model_type;
DROP INDEX IF EXISTS idx_models_name;
DROP INDEX IF EXISTS idx_api_keys_expires_at;
DROP INDEX IF EXISTS idx_api_keys_is_active;
DROP INDEX IF EXISTS idx_api_keys_tenant_id;
DROP INDEX IF EXISTS idx_api_keys_user_id;
DROP INDEX IF EXISTS idx_tenants_subscription_tier;
DROP INDEX IF EXISTS idx_tenants_status;
DROP INDEX IF EXISTS idx_tenants_slug;
DROP INDEX IF EXISTS idx_users_created_at;
DROP INDEX IF EXISTS idx_users_status;
DROP INDEX IF EXISTS idx_users_tenant_id;
DROP INDEX IF EXISTS idx_users_username;
DROP INDEX IF EXISTS idx_users_email;
"#;

// Migration 003: Add audit triggers
const MIGRATION_003_UP: &str = r#"
-- Create audit trigger function
CREATE OR REPLACE FUNCTION audit_trigger_function()
RETURNS TRIGGER AS $$
DECLARE
    old_data JSONB;
    new_data JSONB;
    action_type TEXT;
BEGIN
    -- Determine action type
    IF TG_OP = 'DELETE' THEN
        old_data = to_jsonb(OLD);
        new_data = NULL;
        action_type = 'DELETE';
    ELSIF TG_OP = 'UPDATE' THEN
        old_data = to_jsonb(OLD);
        new_data = to_jsonb(NEW);
        action_type = 'UPDATE';
    ELSIF TG_OP = 'INSERT' THEN
        old_data = NULL;
        new_data = to_jsonb(NEW);
        action_type = 'INSERT';
    END IF;

    -- Insert audit record
    INSERT INTO audit_logs (
        tenant_id,
        user_id,
        action,
        resource_type,
        resource_id,
        details,
        timestamp
    ) VALUES (
        COALESCE(
            (new_data->>'tenant_id')::UUID,
            (old_data->>'tenant_id')::UUID
        ),
        COALESCE(
            (new_data->>'user_id')::UUID,
            (old_data->>'user_id')::UUID,
            current_setting('app.current_user_id', true)::UUID
        ),
        action_type,
        TG_TABLE_NAME,
        COALESCE(
            (new_data->>'id')::UUID,
            (old_data->>'id')::UUID
        ),
        jsonb_build_object(
            'old', old_data,
            'new', new_data
        ),
        NOW()
    );

    -- Return appropriate record
    IF TG_OP = 'DELETE' THEN
        RETURN OLD;
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Add audit triggers to sensitive tables
CREATE TRIGGER audit_users_trigger
    AFTER INSERT OR UPDATE OR DELETE ON users
    FOR EACH ROW EXECUTE FUNCTION audit_trigger_function();

CREATE TRIGGER audit_tenants_trigger
    AFTER INSERT OR UPDATE OR DELETE ON tenants
    FOR EACH ROW EXECUTE FUNCTION audit_trigger_function();

CREATE TRIGGER audit_api_keys_trigger
    AFTER INSERT OR UPDATE OR DELETE ON api_keys
    FOR EACH ROW EXECUTE FUNCTION audit_trigger_function();

CREATE TRIGGER audit_models_trigger
    AFTER INSERT OR UPDATE OR DELETE ON models
    FOR EACH ROW EXECUTE FUNCTION audit_trigger_function();

CREATE TRIGGER audit_system_config_trigger
    AFTER INSERT OR UPDATE OR DELETE ON system_config
    FOR EACH ROW EXECUTE FUNCTION audit_trigger_function();
"#;

const MIGRATION_003_DOWN: &str = r#"
-- Drop audit triggers
DROP TRIGGER IF EXISTS audit_system_config_trigger ON system_config;
DROP TRIGGER IF EXISTS audit_models_trigger ON models;
DROP TRIGGER IF EXISTS audit_api_keys_trigger ON api_keys;
DROP TRIGGER IF EXISTS audit_tenants_trigger ON tenants;
DROP TRIGGER IF EXISTS audit_users_trigger ON users;

-- Drop audit function
DROP FUNCTION IF EXISTS audit_trigger_function();
"#;

// Migration 004: Add performance tables
const MIGRATION_004_UP: &str = r#"
-- Create time-series tables for performance monitoring
CREATE TABLE system_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    cpu_usage_percent DOUBLE PRECISION,
    memory_usage_percent DOUBLE PRECISION,
    disk_usage_percent DOUBLE PRECISION,
    network_in_bytes BIGINT,
    network_out_bytes BIGINT,
    active_connections INTEGER,
    query_count INTEGER,
    avg_query_time_ms DOUBLE PRECISION
);

CREATE TABLE model_performance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    model_id UUID NOT NULL REFERENCES models(id) ON DELETE CASCADE,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    inference_count INTEGER NOT NULL DEFAULT 0,
    avg_latency_ms DOUBLE PRECISION,
    p95_latency_ms DOUBLE PRECISION,
    p99_latency_ms DOUBLE PRECISION,
    error_rate DOUBLE PRECISION,
    throughput_req_per_sec DOUBLE PRECISION,
    memory_usage_bytes BIGINT
);

-- Add indexes for time-series queries
CREATE INDEX idx_system_metrics_timestamp ON system_metrics(timestamp);
CREATE INDEX idx_model_performance_timestamp ON model_performance(timestamp);
CREATE INDEX idx_model_performance_model_id ON model_performance(model_id);
CREATE INDEX idx_model_performance_model_timestamp ON model_performance(model_id, timestamp);

-- Create materialized view for daily aggregations
CREATE MATERIALIZED VIEW daily_usage_summary AS
SELECT
    date_trunc('day', created_at) as day,
    tenant_id,
    model_id,
    COUNT(*) as request_count,
    AVG(processing_time_ms) as avg_processing_time,
    SUM(tokens_used) as total_tokens,
    SUM(cost_cents) as total_cost
FROM inference_requests
WHERE completed_at IS NOT NULL
GROUP BY date_trunc('day', created_at), tenant_id, model_id;

-- Create unique index on materialized view
CREATE UNIQUE INDEX idx_daily_usage_summary_unique
    ON daily_usage_summary(day, tenant_id, model_id);

-- Function to refresh materialized view
CREATE OR REPLACE FUNCTION refresh_daily_usage_summary()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY daily_usage_summary;
END;
$$ LANGUAGE plpgsql;

-- Create alert rules table
CREATE TABLE alert_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    metric_name VARCHAR(255) NOT NULL,
    condition VARCHAR(50) NOT NULL, -- 'gt', 'lt', 'eq', 'gte', 'lte'
    threshold_value DOUBLE PRECISION NOT NULL,
    is_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    notification_channels TEXT[] DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create alerts table
CREATE TABLE alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_id UUID NOT NULL REFERENCES alert_rules(id) ON DELETE CASCADE,
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    severity VARCHAR(20) NOT NULL DEFAULT 'medium', -- 'low', 'medium', 'high', 'critical'
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    metric_value DOUBLE PRECISION,
    threshold_value DOUBLE PRECISION,
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- 'active', 'acknowledged', 'resolved'
    acknowledged_by UUID REFERENCES users(id) ON DELETE SET NULL,
    acknowledged_at TIMESTAMP WITH TIME ZONE,
    resolved_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Add indexes for alerts
CREATE INDEX idx_alerts_rule_id ON alerts(rule_id);
CREATE INDEX idx_alerts_tenant_id ON alerts(tenant_id);
CREATE INDEX idx_alerts_status ON alerts(status);
CREATE INDEX idx_alerts_severity ON alerts(severity);
CREATE INDEX idx_alerts_created_at ON alerts(created_at);

CREATE INDEX idx_alert_rules_tenant_id ON alert_rules(tenant_id);
CREATE INDEX idx_alert_rules_is_enabled ON alert_rules(is_enabled);
CREATE INDEX idx_alert_rules_metric_name ON alert_rules(metric_name);

-- Add updated_at triggers for new tables
CREATE TRIGGER update_alert_rules_updated_at BEFORE UPDATE ON alert_rules
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_alerts_updated_at BEFORE UPDATE ON alerts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
"#;

const MIGRATION_004_DOWN: &str = r#"
-- Drop triggers
DROP TRIGGER IF EXISTS update_alerts_updated_at ON alerts;
DROP TRIGGER IF EXISTS update_alert_rules_updated_at ON alert_rules;

-- Drop indexes
DROP INDEX IF EXISTS idx_alert_rules_metric_name;
DROP INDEX IF EXISTS idx_alert_rules_is_enabled;
DROP INDEX IF EXISTS idx_alert_rules_tenant_id;
DROP INDEX IF EXISTS idx_alerts_created_at;
DROP INDEX IF EXISTS idx_alerts_severity;
DROP INDEX IF EXISTS idx_alerts_status;
DROP INDEX IF EXISTS idx_alerts_tenant_id;
DROP INDEX IF EXISTS idx_alerts_rule_id;
DROP INDEX IF EXISTS idx_model_performance_model_timestamp;
DROP INDEX IF EXISTS idx_model_performance_model_id;
DROP INDEX IF EXISTS idx_model_performance_timestamp;
DROP INDEX IF EXISTS idx_system_metrics_timestamp;

-- Drop tables
DROP TABLE IF EXISTS alerts CASCADE;
DROP TABLE IF EXISTS alert_rules CASCADE;

-- Drop function
DROP FUNCTION IF EXISTS refresh_daily_usage_summary();

-- Drop materialized view
DROP MATERIALIZED VIEW IF EXISTS daily_usage_summary;

-- Drop performance tables
DROP TABLE IF EXISTS model_performance CASCADE;
DROP TABLE IF EXISTS system_metrics CASCADE;
"#;