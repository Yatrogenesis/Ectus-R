use aion_api_gateway::{EnterpriseApiGateway, GatewayConfig, UpstreamService};
use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gateway_service=info,aion_api_gateway=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 Starting AION Gateway Service");

    // Create gateway configuration
    let config = GatewayConfig {
        listen_address: "0.0.0.0:8080".parse()?,
        upstream_services: vec![
            UpstreamService {
                name: "auth-service".to_string(),
                base_url: "http://localhost:8081".to_string(),
                health_check_path: "/health".to_string(),
                weight: 100,
                max_connections: 50,
                timeout_seconds: 30,
            },
            UpstreamService {
                name: "ai-service".to_string(),
                base_url: "http://localhost:8082".to_string(),
                health_check_path: "/health".to_string(),
                weight: 100,
                max_connections: 50,
                timeout_seconds: 30,
            },
        ],
        ..Default::default()
    };

    // Create and start gateway
    let gateway = EnterpriseApiGateway::new(config).await?;
    gateway.start().await?;

    Ok(())
}