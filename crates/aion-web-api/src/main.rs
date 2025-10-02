//! Ectus-R Web API Server
//! Provides REST API endpoints for the web dashboard and external integrations

use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderValue, Method, StatusCode},
    middleware::DefaultBodyLimit,
    response::{Html, Json},
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, TraceLayer},
    compression::CompressionLayer,
};
use uuid::Uuid;

mod handlers;
mod models;
mod middleware;
mod services;
mod openapi;
mod secrets_manager;

use handlers::*;
use models::*;
use services::*;
pub use openapi::*;
pub use secrets_manager::*;

// Import optimization engine
// use aion_optimization_engine::{OptimizationEngine, OptimizationConfig};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub monitoring_service: Arc<MonitoringService>,
    pub ai_service: Arc<AIService>,
    pub deployment_service: Arc<DeploymentService>,
    pub auth_service: Arc<AuthService>,
    // pub optimization_engine: Arc<RwLock<OptimizationEngine>>,
    pub config: AppConfig,
}

/// Application configuration
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub admin_token: String,
    pub cors_origins: Vec<String>,
    pub rate_limit: u32,
    pub max_request_size: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            database_url: "postgresql://localhost/ectus_r".to_string(),
            jwt_secret: "your-secret-key-here".to_string(),
            admin_token: "ectus-admin-2025".to_string(),
            cors_origins: vec![
                "http://localhost:3000".to_string(),
                "https://dashboard.ectus.ai".to_string(),
                "https://yatrogenesis.github.io".to_string(),
            ],
            rate_limit: 1000,
            max_request_size: 50 * 1024 * 1024, // 50MB
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "aion_web_api=debug,tower_http=debug".into()),
        )
        .init();

    println!("🚀 Initializing Ectus-R Web API Server...");

    // Load configuration
    let config = load_config().await?;

    // Initialize services
    println!("🔧 Initializing services...");
    let monitoring_service = Arc::new(MonitoringService::new().await?);
    // Initialize secrets manager
    println!("🔐 Initializing secrets manager...");
    let secrets_manager = SecretsManager::from_env()
        .unwrap_or_else(|_| SecretsManager::new(SecretBackend::Environment));

    // Load secrets configuration
    let secrets_config = SecretsConfig::load(&secrets_manager).await
        .unwrap_or_else(|_| {
            println!("⚠️  Warning: Using fallback configuration");
            SecretsConfig {
                database_url: std::env::var("DATABASE_URL").unwrap_or_default(),
                redis_url: std::env::var("REDIS_URL").unwrap_or_default(),
                jwt_secret: config.jwt_secret.clone(),
                encryption_key: "fallback_key_32_chars_minimum".to_string(),
                api_keys: HashMap::new(),
            }
        });

    let ai_service = Arc::new(AIService::new().await?);
    let deployment_service = Arc::new(DeploymentService::new().await?);
    let auth_service = Arc::new(AuthService::new(&secrets_config.jwt_secret)?);

    println!("✅ Secrets manager initialized with {} backend",
        match secrets_manager.backend {
            SecretBackend::Environment => "Environment",
            SecretBackend::Vault { .. } => "HashiCorp Vault",
            SecretBackend::AwsSecretsManager { .. } => "AWS Secrets Manager",
            SecretBackend::AzureKeyVault { .. } => "Azure Key Vault",
        }
    );

    // Initialize optimization engine
    // println!("🧠 Initializing optimization engine...");
    // let optimization_config = OptimizationConfig::default();
    // let optimization_engine = Arc::new(RwLock::new(
    //     OptimizationEngine::new(&optimization_config).await?
    // ));

    // Start optimization engine
    // optimization_engine.write().await.start().await?;

    let app_state = AppState {
        monitoring_service,
        ai_service,
        deployment_service,
        auth_service,
        // optimization_engine,
        config: config.clone(),
    };

    // Create router with all routes
    let app = create_router(app_state.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    println!("🌐 Starting server at http://{}:{}", config.host, config.port);
    println!("📊 Dashboard available at: https://dashboard.ectus.ai");
    println!("🔗 API Documentation: http://{}:{}/api/v1/docs", config.host, config.port);
    println!("✅ Server is ready to accept connections!");

    // Start the server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Create the main application router
fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check endpoint
        .route("/health", get(health_check))

        // API v1 routes
        .nest("/api/v1", create_api_v1_router())

        // WebSocket endpoint for real-time updates
        .route("/ws", get(websocket_handler))

        // OpenAPI documentation endpoints
        .route("/api/openapi.json", get(serve_openapi_json))
        .route("/api/openapi.yaml", get(serve_openapi_yaml))
        .route("/docs", get(serve_api_docs))
        .route("/", get(root_handler))

        // Add middleware
        .layer(
            ServiceBuilder::new()
                .layer(DefaultBodyLimit::max(state.config.max_request_size))
                .layer(CompressionLayer::new())
                .layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                        .allow_headers(Any)
                        .allow_origin(Any), // In production, restrict this
                )
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::default().include_headers(true)),
                )
        )
        .with_state(state)
}

/// Create API v1 routes
fn create_api_v1_router() -> Router<AppState> {
    Router::new()
        // System status and monitoring
        .route("/status", get(get_system_status))
        .route("/metrics", get(get_metrics))
        .route("/metrics/:metric_name", get(get_specific_metric))

        // Real-time monitoring
        .route("/monitoring/alerts", get(get_active_alerts))
        .route("/monitoring/dashboard", get(get_dashboard_data))

        // Dashboard endpoints
        .route("/dashboard/stats", get(get_dashboard_stats))
        .route("/dashboard/live-metrics", get(get_live_metrics))
        .route("/dashboard/ai-health", get(get_ai_health))

        // AI Engine endpoints
        .route("/ai/generate", post(generate_code))
        .route("/ai/analyze", post(analyze_code))
        .route("/ai/fix", post(fix_code))
        .route("/ai/refactor", post(refactor_code))
        .route("/ai/qa", post(run_autonomous_qa))

        // Deployment management
        .route("/deployments", get(list_deployments))
        .route("/deployments", post(create_deployment))
        .route("/deployments/:id", get(get_deployment))
        .route("/deployments/:id", put(update_deployment))
        .route("/deployments/:id", delete(delete_deployment))
        .route("/deployments/:id/logs", get(get_deployment_logs))

        // Project management
        .route("/projects", get(list_projects))
        .route("/projects", post(create_project))
        .route("/projects/:id", get(get_project))
        .route("/projects/:id", put(update_project))
        .route("/projects/:id", delete(delete_project))

        // Authentication
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh_token))
        .route("/auth/logout", post(logout))

        // Optimization endpoints
        .route("/optimization/status", get(get_optimization_status))
        .route("/optimization/config", get(get_optimization_config))
        .route("/optimization/config", put(update_optimization_config))
        .route("/optimization/metrics", get(get_optimization_metrics))
        .route("/optimization/recommendations", get(get_optimization_recommendations))
        .route("/optimization/start", post(start_optimization))
        .route("/optimization/stop", post(stop_optimization))

        // Admin endpoints
        .route("/admin/stats", get(get_admin_stats))
        .route("/admin/users", get(list_users))
        .route("/admin/system", get(get_system_info))

        // Payment and subscription endpoints
        .route("/pricing", get(get_pricing_plans))
        .route("/checkout", post(create_checkout_session))
        .route("/webhooks/stripe", post(handle_stripe_webhook))
        .route("/subscription/:user_id", get(get_subscription_status))
        .route("/subscription/cancel/:user_id", post(cancel_subscription))
        .route("/payment-method/:user_id", put(update_payment_method))
        .route("/customer-portal/:user_id", get(get_customer_portal))

        // Analytics and conversion tracking endpoints
        .route("/analytics/track", post(track_event))
        .route("/analytics/metrics", get(get_analytics_metrics))
        .route("/analytics/funnel", get(get_conversion_funnel))
        .route("/analytics/cohorts", get(get_cohort_analysis))
        .route("/analytics/ab-tests", get(get_ab_test_results))
        .route("/analytics/realtime", get(get_realtime_analytics))
        .route("/analytics/feature-flag", post(track_feature_flag))
}

/// Load application configuration
async fn load_config() -> anyhow::Result<AppConfig> {
    let mut config = AppConfig::default();

    // Override with environment variables if available
    if let Ok(host) = std::env::var("ECTUS_HOST") {
        config.host = host;
    }

    if let Ok(port) = std::env::var("ECTUS_PORT") {
        config.port = port.parse()?;
    }

    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        config.database_url = db_url;
    }

    if let Ok(jwt_secret) = std::env::var("JWT_SECRET") {
        config.jwt_secret = jwt_secret;
    }

    if let Ok(admin_token) = std::env::var("ADMIN_TOKEN") {
        config.admin_token = admin_token;
    }

    Ok(config)
}

/// Root handler
async fn root_handler() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Ectus-R API Server</title>
    <style>
        body { font-family: Arial, sans-serif; max-width: 800px; margin: 50px auto; padding: 20px; }
        .header { text-align: center; margin-bottom: 40px; }
        .endpoint { background: #f5f5f5; padding: 15px; margin: 10px 0; border-radius: 5px; }
        .method { font-weight: bold; color: #0066cc; }
    </style>
</head>
<body>
    <div class="header">
        <h1>🚀 Ectus-R API Server</h1>
        <p>Autonomous Software Engineering Platform</p>
    </div>

    <h2>Available Endpoints</h2>

    <div class="endpoint">
        <span class="method">GET</span> /health - Health check
    </div>

    <div class="endpoint">
        <span class="method">GET</span> /api/v1/status - System status
    </div>

    <div class="endpoint">
        <span class="method">GET</span> /api/v1/metrics - System metrics
    </div>

    <div class="endpoint">
        <span class="method">POST</span> /api/v1/ai/generate - Generate code
    </div>

    <div class="endpoint">
        <span class="method">GET</span> /api/v1/deployments - List deployments
    </div>

    <div class="endpoint">
        <span class="method">GET</span> /ws - WebSocket connection for real-time updates
    </div>

    <p><strong>Documentation:</strong> <a href="/docs">/docs</a></p>
    <p><strong>Dashboard:</strong> <a href="https://dashboard.ectus.ai">https://dashboard.ectus.ai</a></p>
</body>
</html>
    "#)
}

/// API documentation handler
async fn serve_api_docs() -> Html<&'static str> {
    Html(include_str!("../docs/api.html"))
}

/// Health check handler
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": "1.0.0",
        "services": {
            "api": "operational",
            "ai_engine": "operational",
            "monitoring": "operational",
            "deployments": "operational"
        }
    }))
}

/// Serve OpenAPI JSON specification
async fn serve_openapi_json() -> (StatusCode, Json<serde_json::Value>) {
    match openapi::export_json() {
        Ok(json_str) => {
            let value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
            (StatusCode::OK, Json(value))
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Failed to generate OpenAPI spec"}))
        )
    }
}

/// Serve OpenAPI YAML specification
async fn serve_openapi_yaml() -> (StatusCode, [(header::HeaderName, &'static str); 1], String) {
    match openapi::export_yaml() {
        Ok(yaml) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/x-yaml")],
            yaml
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, "text/plain")],
            "Failed to generate OpenAPI spec".to_string()
        )
    }
}

/// WebSocket handler for real-time updates
async fn websocket_handler() -> &'static str {
    "WebSocket upgrade not implemented in this handler"
}

/// Graceful shutdown signal
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("🛑 Shutdown signal received, starting graceful shutdown...");
}