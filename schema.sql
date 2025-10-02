-- Ectus-R SaaS Database Schema
-- Core tables for user management, deployment tracking, and analytics

-- Users table for authentication and subscription management
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    name TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    plan TEXT DEFAULT 'free' CHECK (plan IN ('free', 'pro', 'enterprise')),
    api_key TEXT UNIQUE,
    usage_quota INTEGER DEFAULT 0,
    last_login INTEGER,
    is_active BOOLEAN DEFAULT true
);

-- Deployments table for tracking Magic Loop deployments
CREATE TABLE IF NOT EXISTS deployments (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    prompt TEXT NOT NULL,
    status TEXT DEFAULT 'pending' CHECK (status IN ('pending', 'generating', 'deploying', 'completed', 'failed')),
    worker_url TEXT,
    custom_domain TEXT,
    created_at INTEGER NOT NULL,
    completed_at INTEGER,
    deployment_time INTEGER,
    cost_estimate REAL DEFAULT 0.0,
    resources_created TEXT, -- JSON array of created resources
    ai_model_used TEXT,
    error_message TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- API keys for programmatic access
CREATE TABLE IF NOT EXISTS api_keys (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    key_hash TEXT UNIQUE NOT NULL,
    created_at INTEGER NOT NULL,
    last_used INTEGER,
    is_active BOOLEAN DEFAULT true,
    permissions TEXT, -- JSON array of permissions
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Usage tracking for billing and quotas
CREATE TABLE IF NOT EXISTS usage_records (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    deployment_id TEXT,
    resource_type TEXT NOT NULL, -- 'compute', 'storage', 'bandwidth', 'ai_tokens'
    quantity INTEGER NOT NULL,
    cost REAL DEFAULT 0.0,
    recorded_at INTEGER NOT NULL,
    billing_period TEXT, -- YYYY-MM format
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (deployment_id) REFERENCES deployments(id)
);

-- Audit log for security and compliance
CREATE TABLE IF NOT EXISTS audit_log (
    id TEXT PRIMARY KEY,
    user_id TEXT,
    action TEXT NOT NULL,
    resource_type TEXT,
    resource_id TEXT,
    ip_address TEXT,
    user_agent TEXT,
    timestamp INTEGER NOT NULL,
    details TEXT -- JSON object with additional context
);

-- Settings and configuration
CREATE TABLE IF NOT EXISTS user_settings (
    user_id TEXT PRIMARY KEY,
    default_environment TEXT DEFAULT 'production',
    auto_deploy BOOLEAN DEFAULT false,
    notification_preferences TEXT, -- JSON object
    cloudflare_account_id TEXT,
    github_integration TEXT, -- JSON object with OAuth tokens
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Domain management for custom domains
CREATE TABLE IF NOT EXISTS domains (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    domain_name TEXT UNIQUE NOT NULL,
    status TEXT DEFAULT 'pending' CHECK (status IN ('pending', 'active', 'failed', 'suspended')),
    dns_configured BOOLEAN DEFAULT false,
    ssl_certificate TEXT,
    created_at INTEGER NOT NULL,
    verified_at INTEGER,
    expires_at INTEGER,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Performance metrics for analytics
CREATE TABLE IF NOT EXISTS performance_metrics (
    id TEXT PRIMARY KEY,
    deployment_id TEXT NOT NULL,
    metric_type TEXT NOT NULL, -- 'response_time', 'throughput', 'error_rate'
    value REAL NOT NULL,
    unit TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    region TEXT,
    FOREIGN KEY (deployment_id) REFERENCES deployments(id)
);

-- Billing and subscription management
CREATE TABLE IF NOT EXISTS billing_records (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    billing_period TEXT NOT NULL, -- YYYY-MM format
    plan TEXT NOT NULL,
    base_cost REAL DEFAULT 0.0,
    usage_cost REAL DEFAULT 0.0,
    total_cost REAL DEFAULT 0.0,
    status TEXT DEFAULT 'pending' CHECK (status IN ('pending', 'paid', 'overdue', 'cancelled')),
    created_at INTEGER NOT NULL,
    paid_at INTEGER,
    due_date INTEGER,
    stripe_invoice_id TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Indexes for performance optimization
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_api_key ON users(api_key);
CREATE INDEX IF NOT EXISTS idx_deployments_user_id ON deployments(user_id);
CREATE INDEX IF NOT EXISTS idx_deployments_status ON deployments(status);
CREATE INDEX IF NOT EXISTS idx_deployments_created_at ON deployments(created_at);
CREATE INDEX IF NOT EXISTS idx_api_keys_user_id ON api_keys(user_id);
CREATE INDEX IF NOT EXISTS idx_api_keys_hash ON api_keys(key_hash);
CREATE INDEX IF NOT EXISTS idx_usage_records_user_id ON usage_records(user_id);
CREATE INDEX IF NOT EXISTS idx_usage_records_billing_period ON usage_records(billing_period);
CREATE INDEX IF NOT EXISTS idx_audit_log_user_id ON audit_log(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_timestamp ON audit_log(timestamp);
CREATE INDEX IF NOT EXISTS idx_domains_user_id ON domains(user_id);
CREATE INDEX IF NOT EXISTS idx_domains_name ON domains(domain_name);
CREATE INDEX IF NOT EXISTS idx_performance_metrics_deployment_id ON performance_metrics(deployment_id);
CREATE INDEX IF NOT EXISTS idx_billing_records_user_id ON billing_records(user_id);
CREATE INDEX IF NOT EXISTS idx_billing_records_period ON billing_records(billing_period);