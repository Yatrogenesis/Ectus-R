// AION-R Enterprise Platform Server
// Main application entry point

use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
};
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod state;
mod errors;
mod middleware;

use config::AppConfig;
use state::AppState;
use api::build_api_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "aion_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting AION-R Enterprise Platform Server");

    // Load configuration
    let config = AppConfig::load()?;
    info!("Configuration loaded successfully");

    // Initialize application state
    let state = AppState::new(config.clone()).await?;
    info!("Application state initialized");

    // Initialize AI Engine (commented out - crate not available)
    // aion_ai_engine::initialize_ai_engine(config.ai_engine.clone()).await?;
    // info!("AI Engine initialized");

    // Build router
    let app = build_api_router(Arc::new(state))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
                .layer(CorsLayer::permissive())
        );

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await?;

    info!("Server listening on {}", addr);
    info!("API documentation available at http://{}/docs", addr);
    info!("Health check available at http://{}/health", addr);

    axum::serve(listener, app).await?;

    Ok(())
}