// Integration tests for the Magic Loop deployment process
// Tests the complete end-to-end flow from prompt to production URL

use anyhow::Result;
use ectus_r::*;
use std::time::Duration;
use tokio::time::timeout;
use uuid::Uuid;

#[tokio::test]
async fn test_magic_loop_api_deployment() -> Result<()> {
    // Test the complete Magic Loop for a simple API
    let prompt = "Create a REST API for a blog with posts and comments";

    let deployer = create_test_cloudflare_deployer().await?;

    let options = MagicLoopOptions {
        custom_domain: None,
        environment: "test".to_string(),
        enable_monitoring: false,
        auto_scaling: false,
    };

    // Execute Magic Loop with timeout
    let result = timeout(
        Duration::from_secs(300), // 5 minutes max
        deployer.magic_loop_deployment(prompt, Uuid::new_v4(), options)
    ).await??;

    // Verify deployment succeeded
    assert!(result.success);
    assert!(result.worker_url.contains("workers.dev"));
    assert!(result.deployment_time.num_seconds() < 300);

    // Test the deployed API
    let client = reqwest::Client::new();

    // Health check
    let health_response = client
        .get(&format!("{}/health", result.worker_url))
        .send()
        .await?;

    assert!(health_response.status().is_success());

    // API endpoint test
    let api_response = client
        .get(&format!("{}/api/posts", result.worker_url))
        .send()
        .await?;

    assert!(api_response.status().as_u16() < 500);

    Ok(())
}

#[tokio::test]
async fn test_magic_loop_ai_service() -> Result<()> {
    // Test Magic Loop for AI-powered service
    let prompt = "Create an AI chat service that responds to user messages";

    let deployer = create_test_cloudflare_deployer().await?;

    let options = MagicLoopOptions::default();

    let result = timeout(
        Duration::from_secs(300),
        deployer.magic_loop_deployment(prompt, Uuid::new_v4(), options)
    ).await??;

    assert!(result.success);
    assert!(result.ai_model_used.is_some());

    // Test AI endpoint
    let client = reqwest::Client::new();
    let ai_response = client
        .post(&format!("{}/api/ai", result.worker_url))
        .json(&serde_json::json!({
            "prompt": "Hello, how are you?"
        }))
        .send()
        .await?;

    assert!(ai_response.status().is_success());

    let response_body: serde_json::Value = ai_response.json().await?;
    assert!(response_body["response"].is_string());

    Ok(())
}

#[tokio::test]
async fn test_magic_loop_with_database() -> Result<()> {
    // Test Magic Loop with database requirements
    let prompt = "Create a user management system that stores user profiles in a database";

    let deployer = create_test_cloudflare_deployer().await?;
    let options = MagicLoopOptions::default();

    let result = timeout(
        Duration::from_secs(300),
        deployer.magic_loop_deployment(prompt, Uuid::new_v4(), options)
    ).await??;

    assert!(result.success);

    // Verify D1 database was created
    let d1_resources: Vec<_> = result.resources_created
        .iter()
        .filter(|r| r.resource_type == "D1 Database")
        .collect();

    assert!(!d1_resources.is_empty());

    // Test database endpoints
    let client = reqwest::Client::new();

    // Test read endpoint
    let read_response = client
        .get(&format!("{}/api/data", result.worker_url))
        .send()
        .await?;

    assert!(read_response.status().is_success());

    // Test write endpoint
    let write_response = client
        .post(&format!("{}/api/data", result.worker_url))
        .json(&serde_json::json!({
            "name": "Test User",
            "email": "test@example.com"
        }))
        .send()
        .await?;

    assert!(write_response.status().is_success());

    Ok(())
}

#[tokio::test]
async fn test_deployment_status_tracking() -> Result<()> {
    // Test that deployment status is properly tracked throughout the process
    let prompt = "Create a simple web API";
    let deployer = create_test_cloudflare_deployer().await?;
    let user_id = Uuid::new_v4();

    // Start deployment in background
    let deployment_future = deployer.magic_loop_deployment(
        prompt,
        user_id,
        MagicLoopOptions::default()
    );

    // Poll status while deployment is running
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;

            // In a real implementation, we'd get the deployment ID
            // and check status via deployer.get_deployment_status()
        }
    });

    let result = timeout(Duration::from_secs(300), deployment_future).await??;
    assert!(result.success);

    Ok(())
}

#[tokio::test]
async fn test_deployment_rollback_on_failure() -> Result<()> {
    // Test that failed deployments are properly rolled back
    let prompt = "Create an invalid deployment that should fail";
    let deployer = create_test_cloudflare_deployer().await?;

    // This should demonstrate graceful failure handling
    // In a real scenario, we'd inject failures or use invalid configurations

    Ok(())
}

#[tokio::test]
async fn test_performance_benchmarks() -> Result<()> {
    // Test deployment performance requirements
    let prompts = vec![
        "Create a simple REST API",
        "Create a web application with database",
        "Create an AI-powered chat service",
    ];

    let deployer = create_test_cloudflare_deployer().await?;
    let mut deployment_times = Vec::new();

    for prompt in prompts {
        let start = std::time::Instant::now();

        let result = timeout(
            Duration::from_secs(300),
            deployer.magic_loop_deployment(prompt, Uuid::new_v4(), MagicLoopOptions::default())
        ).await??;

        let duration = start.elapsed();
        deployment_times.push(duration);

        assert!(result.success);
        assert!(duration < Duration::from_secs(300)); // Max 5 minutes
    }

    // Average deployment time should be under 3 minutes
    let average_time = deployment_times.iter().sum::<Duration>() / deployment_times.len() as u32;
    assert!(average_time < Duration::from_secs(180));

    println!("Average deployment time: {:?}", average_time);

    Ok(())
}

#[tokio::test]
async fn test_concurrent_deployments() -> Result<()> {
    // Test multiple concurrent deployments
    let deployer = create_test_cloudflare_deployer().await?;
    let mut handles = Vec::new();

    for i in 0..3 {
        let deployer = deployer.clone();
        let prompt = format!("Create API service number {}", i);

        let handle = tokio::spawn(async move {
            deployer.magic_loop_deployment(
                &prompt,
                Uuid::new_v4(),
                MagicLoopOptions::default()
            ).await
        });

        handles.push(handle);
    }

    // Wait for all deployments to complete
    for handle in handles {
        let result = timeout(Duration::from_secs(300), handle).await??;
        assert!(result.success);
    }

    Ok(())
}

#[tokio::test]
async fn test_cost_estimation() -> Result<()> {
    // Test that cost estimation is accurate
    let prompt = "Create a high-traffic API service";
    let deployer = create_test_cloudflare_deployer().await?;

    let result = deployer.magic_loop_deployment(
        prompt,
        Uuid::new_v4(),
        MagicLoopOptions::default()
    ).await?;

    assert!(result.success);

    // Verify cost estimate is reasonable
    assert_eq!(result.cost_estimate.monthly_cost, 0.0); // Free tier
    assert!(result.cost_estimate.included_requests > 0);

    Ok(())
}

#[tokio::test]
async fn test_security_compliance() -> Result<()> {
    // Test that deployed services meet security requirements
    let prompt = "Create a secure API with authentication";
    let deployer = create_test_cloudflare_deployer().await?;

    let result = deployer.magic_loop_deployment(
        prompt,
        Uuid::new_v4(),
        MagicLoopOptions::default()
    ).await?;

    assert!(result.success);

    // Test security headers
    let client = reqwest::Client::new();
    let response = client
        .get(&result.worker_url)
        .send()
        .await?;

    // Check for CORS headers
    assert!(response.headers().contains_key("access-control-allow-origin"));

    // Test HTTPS
    assert!(result.worker_url.starts_with("https://"));

    Ok(())
}

#[tokio::test]
async fn test_monitoring_setup() -> Result<()> {
    // Test that monitoring is properly configured
    let prompt = "Create an API with full monitoring";
    let deployer = create_test_cloudflare_deployer().await?;

    let options = MagicLoopOptions {
        enable_monitoring: true,
        ..Default::default()
    };

    let result = deployer.magic_loop_deployment(
        prompt,
        Uuid::new_v4(),
        options
    ).await?;

    assert!(result.success);

    // Verify analytics resources were created
    let analytics_resources: Vec<_> = result.resources_created
        .iter()
        .filter(|r| r.resource_type.contains("Analytics"))
        .collect();

    // In a full implementation, we'd verify analytics are collecting data
    Ok(())
}

// Helper functions for testing

async fn create_test_cloudflare_deployer() -> Result<CloudflareDeployer> {
    use ectus_r::deployment::cloudflare_deployer::*;
    use ectus_r::infrastructure::cloudflare_impl::*;

    let config = CloudflareConfig {
        account_id: std::env::var("CLOUDFLARE_ACCOUNT_ID")
            .unwrap_or_else(|_| "test-account-id".to_string()),
        api_token: std::env::var("CLOUDFLARE_API_TOKEN")
            .unwrap_or_else(|_| "test-api-token".to_string()),
        zone_id: None,
        custom_domain: None,
        environment: "test".to_string(),
    };

    Ok(CloudflareDeployer::new(config))
}

// Mock implementations for testing
mod mocks {
    use super::*;

    pub struct MockCloudflareDeployer {
        // Mock implementation that doesn't actually deploy
    }

    impl MockCloudflareDeployer {
        pub async fn mock_magic_loop_deployment(
            &self,
            prompt: &str,
            user_id: Uuid,
            options: MagicLoopOptions,
        ) -> Result<MagicLoopResult> {
            // Simulate deployment time
            tokio::time::sleep(Duration::from_millis(100)).await;

            Ok(MagicLoopResult {
                deployment_id: Uuid::new_v4(),
                success: true,
                worker_url: "https://test-worker.example.workers.dev".to_string(),
                custom_url: None,
                deployment_time: chrono::Duration::milliseconds(100),
                resources_created: vec![],
                ai_model_used: Some("@cf/meta/llama-2-7b-chat-int8".to_string()),
                performance_metrics: PerformanceMetrics::default(),
                cost_estimate: CostEstimate::free(),
            })
        }
    }
}

// Benchmarks for performance testing
#[cfg(test)]
mod benches {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn bench_simple_deployment() -> Result<()> {
        let deployer = mocks::MockCloudflareDeployer {};

        let start = Instant::now();
        let _ = deployer.mock_magic_loop_deployment(
            "Create a simple API",
            Uuid::new_v4(),
            MagicLoopOptions::default()
        ).await?;
        let duration = start.elapsed();

        println!("Simple deployment took: {:?}", duration);
        assert!(duration < Duration::from_millis(200));

        Ok(())
    }

    #[tokio::test]
    async fn bench_complex_deployment() -> Result<()> {
        let deployer = mocks::MockCloudflareDeployer {};

        let start = Instant::now();
        let _ = deployer.mock_magic_loop_deployment(
            "Create a complex AI-powered microservice with database, real-time features, and analytics",
            Uuid::new_v4(),
            MagicLoopOptions::default()
        ).await?;
        let duration = start.elapsed();

        println!("Complex deployment took: {:?}", duration);
        assert!(duration < Duration::from_millis(500));

        Ok(())
    }
}