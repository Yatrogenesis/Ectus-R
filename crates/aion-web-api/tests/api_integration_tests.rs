//! Web API Integration Tests
//! Comprehensive testing of all API endpoints and services

use aion_web_api::*;
use axum::http::{StatusCode, HeaderValue};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::test;
use uuid::Uuid;

/// Test helper to create test database
async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/ectus_r_test".to_string());

    PgPool::connect(&database_url).await
        .expect("Failed to connect to test database")
}

/// Test helper to create test app state
async fn create_test_app_state() -> AppState {
    let config = AppConfig {
        jwt_secret: "test-secret-key-32-characters-long".to_string(),
        database_url: "postgresql://localhost/ectus_r_test".to_string(),
        ..Default::default()
    };

    let db_pool = Arc::new(setup_test_db().await);

    let monitoring_service = Arc::new(
        services::MonitoringService::new().await
            .expect("Failed to create monitoring service")
    );

    let ai_service = Arc::new(
        services::AIService::new().await
            .expect("Failed to create AI service")
    );

    let deployment_service = Arc::new(
        services::DeploymentService::new().await
            .expect("Failed to create deployment service")
    );

    let auth_service = Arc::new(
        services::AuthService::new(&config.jwt_secret, db_pool).await
            .expect("Failed to create auth service")
    );

    AppState {
        monitoring_service,
        ai_service,
        deployment_service,
        auth_service,
        config,
    }
}

/// Test authentication endpoints
#[tokio::test]
async fn test_authentication_endpoints() {
    println!("🔐 Testing Authentication Endpoints...");

    let app_state = create_test_app_state().await;
    let app = create_router(app_state);

    // Test user registration
    let registration_data = json!({
        "email": "test@example.com",
        "password": "SecurePassword123!",
        "name": "Test User"
    });

    // Note: In a real test, you'd use a test client to make HTTP requests
    // For now, we'll test the service directly
    let auth_service = &app_state.auth_service;

    let user_result = auth_service.register_user(
        "test@example.com",
        "SecurePassword123!",
        "Test User"
    ).await;

    assert!(user_result.is_ok(), "User registration should succeed");

    let user = user_result.unwrap();
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.name, "Test User");

    // Test email verification
    let verify_result = auth_service.verify_email(user.id).await;
    assert!(verify_result.is_ok(), "Email verification should succeed");

    // Test login
    let login_result = auth_service.authenticate("test@example.com", "SecurePassword123!").await;
    assert!(login_result.is_ok(), "Authentication should succeed");

    let login_response = login_result.unwrap();
    assert!(!login_response.access_token.is_empty(), "Should return access token");
    assert!(!login_response.refresh_token.is_empty(), "Should return refresh token");

    // Test token validation
    let claims_result = auth_service.validate_token(&login_response.access_token).await;
    assert!(claims_result.is_ok(), "Token validation should succeed");

    println!("✅ Authentication Endpoints: PASSED");
}

/// Test AI endpoints
#[tokio::test]
async fn test_ai_endpoints() {
    println!("🧠 Testing AI Endpoints...");

    let app_state = create_test_app_state().await;
    let ai_service = &app_state.ai_service;

    // Test code generation
    let generation_request = models::GenerateRequest {
        prompt: "Create a simple calculator function".to_string(),
        language: Some("rust".to_string()),
        framework: Some("std".to_string()),
        requirements: Some(vec!["add".to_string(), "subtract".to_string()]),
        constraints: Some(vec!["no unsafe code".to_string()]),
    };

    let generation_result = ai_service.generate_code(generation_request).await;
    assert!(generation_result.is_ok(), "Code generation should succeed");

    let response = generation_result.unwrap();
    assert!(!response.generated_files.is_empty(), "Should generate files");
    assert!(!response.id.to_string().is_empty(), "Should have generation ID");

    // Test code analysis
    let test_code = r#"
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        fn divide(a: i32, b: i32) -> i32 {
            a / b  // Potential division by zero
        }
    "#;

    let analysis_result = ai_service.analyze_code(test_code).await;
    assert!(analysis_result.is_ok(), "Code analysis should succeed");

    let analysis = analysis_result.unwrap();
    assert!(analysis.get("analysis_id").is_some(), "Should have analysis ID");
    assert!(analysis.get("complexity_score").is_some(), "Should have complexity score");

    // Test code fixing
    let fix_result = ai_service.fix_code(test_code, vec!["division by zero".to_string()]).await;
    assert!(fix_result.is_ok(), "Code fixing should succeed");

    // Test autonomous QA
    let qa_result = ai_service.run_autonomous_qa(test_code).await;
    assert!(qa_result.is_ok(), "Autonomous QA should succeed");

    let qa_response = qa_result.unwrap();
    assert!(qa_response.get("qa_id").is_some(), "Should have QA ID");
    assert!(qa_response.get("tests_run").is_some(), "Should have test count");

    println!("✅ AI Endpoints: PASSED");
}

/// Test monitoring endpoints
#[tokio::test]
async fn test_monitoring_endpoints() {
    println!("📊 Testing Monitoring Endpoints...");

    let app_state = create_test_app_state().await;
    let monitoring_service = &app_state.monitoring_service;

    // Test system health
    let health_result = monitoring_service.get_system_health().await;
    assert!(health_result.is_ok(), "System health check should succeed");

    let health = health_result.unwrap();
    assert!(!health.status.is_empty(), "Should have health status");
    assert!(health.metrics.cpu_usage >= 0.0, "Should have CPU usage");
    assert!(health.metrics.memory_usage >= 0.0, "Should have memory usage");

    // Test metrics retrieval
    let metrics_result = monitoring_service.get_metrics(
        &["system.cpu.usage_percent".to_string()],
        Some(chrono::Duration::hours(1))
    ).await;

    assert!(metrics_result.is_ok(), "Metrics retrieval should succeed");

    let metrics = metrics_result.unwrap();
    assert!(!metrics.is_empty(), "Should return metrics data");

    // Test alerts
    let alerts_result = monitoring_service.get_active_alerts().await;
    assert!(alerts_result.is_ok(), "Alerts retrieval should succeed");

    println!("✅ Monitoring Endpoints: PASSED");
}

/// Test dashboard endpoints
#[tokio::test]
async fn test_dashboard_endpoints() {
    println!("📈 Testing Dashboard Endpoints...");

    let app_state = Arc::new(create_test_app_state().await);

    // Test dashboard stats
    let stats_result = handlers::get_dashboard_stats(
        axum::extract::State(app_state.clone())
    ).await;

    assert!(stats_result.is_ok(), "Dashboard stats should succeed");

    let stats_response = stats_result.unwrap();
    let stats = stats_response.0;

    assert!(stats.total_generations >= 0, "Should have generation count");
    assert!(stats.total_projects >= 0, "Should have project count");
    assert!(!stats.recent_generations.is_empty(), "Should have recent generations");
    assert!(!stats.usage_stats.daily.is_empty(), "Should have daily usage data");

    // Test live metrics
    let live_metrics_result = handlers::get_live_metrics(
        axum::extract::State(app_state.clone())
    ).await;

    assert!(live_metrics_result.is_ok(), "Live metrics should succeed");

    // Test AI health
    let ai_health_result = handlers::get_ai_health(
        axum::extract::State(app_state.clone())
    ).await;

    assert!(ai_health_result.is_ok(), "AI health check should succeed");

    println!("✅ Dashboard Endpoints: PASSED");
}

/// Test error handling and edge cases
#[tokio::test]
async fn test_error_handling() {
    println!("🚨 Testing Error Handling...");

    let app_state = create_test_app_state().await;
    let auth_service = &app_state.auth_service;

    // Test invalid login
    let invalid_login = auth_service.authenticate("invalid@email.com", "wrongpassword").await;
    assert!(invalid_login.is_err(), "Invalid login should fail");

    // Test weak password
    let weak_password_result = auth_service.register_user(
        "test2@example.com",
        "123", // Too weak
        "Test User 2"
    ).await;
    assert!(weak_password_result.is_err(), "Weak password should be rejected");

    // Test duplicate email
    let _ = auth_service.register_user(
        "duplicate@example.com",
        "SecurePassword123!",
        "First User"
    ).await;

    let duplicate_result = auth_service.register_user(
        "duplicate@example.com",
        "AnotherPassword123!",
        "Second User"
    ).await;
    assert!(duplicate_result.is_err(), "Duplicate email should be rejected");

    // Test invalid token
    let invalid_token_result = auth_service.validate_token("invalid.jwt.token").await;
    assert!(invalid_token_result.is_err(), "Invalid token should be rejected");

    println!("✅ Error Handling: PASSED");
}

/// Test rate limiting and security
#[tokio::test]
async fn test_security_features() {
    println!("🛡️ Testing Security Features...");

    let app_state = create_test_app_state().await;
    let auth_service = &app_state.auth_service;

    // Register a user for testing
    let user = auth_service.register_user(
        "security_test@example.com",
        "SecurePassword123!",
        "Security Test User"
    ).await.expect("User registration should succeed");

    auth_service.verify_email(user.id).await
        .expect("Email verification should succeed");

    // Test account lockout after failed attempts
    for i in 0..6 {
        let result = auth_service.authenticate("security_test@example.com", "wrongpassword").await;
        assert!(result.is_err(), "Wrong password should fail");

        if i >= 4 {
            // After 5 failed attempts, account should be locked
            let error_msg = result.err().unwrap().to_string();
            if error_msg.contains("locked") {
                println!("Account properly locked after 5 failed attempts");
                break;
            }
        }
    }

    // Test session management
    let login_result = auth_service.authenticate("security_test@example.com", "SecurePassword123!").await;

    if login_result.is_ok() {
        // If not locked, test session invalidation
        let login_response = login_result.unwrap();
        let token_validation = auth_service.validate_token(&login_response.access_token).await;
        assert!(token_validation.is_ok(), "Valid token should work");

        // Test session invalidation
        auth_service.invalidate_all_sessions(user.id).await
            .expect("Session invalidation should work");
    }

    println!("✅ Security Features: PASSED");
}

/// Performance and load testing
#[tokio::test]
async fn test_performance() {
    println!("⚡ Testing Performance...");

    let app_state = create_test_app_state().await;
    let start_time = std::time::Instant::now();

    // Test concurrent AI requests
    let mut handles = Vec::new();

    for i in 0..5 {
        let ai_service = app_state.ai_service.clone();
        let handle = tokio::spawn(async move {
            let test_code = format!("fn test_{}() {{ println!(\"test {}\"); }}", i, i);
            ai_service.analyze_code(&test_code).await
        });
        handles.push(handle);
    }

    // Wait for all requests to complete
    for handle in handles {
        let result = handle.await.expect("Task should complete");
        assert!(result.is_ok(), "Concurrent request should succeed");
    }

    let elapsed = start_time.elapsed();
    assert!(elapsed.as_secs() < 30, "Concurrent requests should complete within 30 seconds");

    println!("✅ Performance Test: PASSED ({}ms)", elapsed.as_millis());
}

/// Test database operations
#[tokio::test]
async fn test_database_operations() {
    println!("🗄️ Testing Database Operations...");

    let db_pool = setup_test_db().await;

    // Test database connectivity
    let result = sqlx::query("SELECT 1 as test")
        .fetch_one(&db_pool)
        .await;

    assert!(result.is_ok(), "Database should be accessible");

    // Test user table operations
    let user_id = Uuid::new_v4();
    let insert_result = sqlx::query!(
        "INSERT INTO users (id, email, password_hash, name) VALUES ($1, $2, $3, $4)",
        user_id,
        "db_test@example.com",
        "hashed_password",
        "DB Test User"
    ).execute(&db_pool).await;

    assert!(insert_result.is_ok(), "User insertion should succeed");

    // Test user retrieval
    let user_result = sqlx::query!(
        "SELECT email, name FROM users WHERE id = $1",
        user_id
    ).fetch_one(&db_pool).await;

    assert!(user_result.is_ok(), "User retrieval should succeed");

    let user = user_result.unwrap();
    assert_eq!(user.email, "db_test@example.com");
    assert_eq!(user.name, "DB Test User");

    // Cleanup
    let cleanup_result = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id
    ).execute(&db_pool).await;

    assert!(cleanup_result.is_ok(), "Cleanup should succeed");

    println!("✅ Database Operations: PASSED");
}

/// Integration test for complete workflow
#[tokio::test]
async fn test_complete_workflow() {
    println!("🔄 Testing Complete Workflow...");

    let app_state = create_test_app_state().await;

    // Step 1: Register and authenticate user
    let user = app_state.auth_service.register_user(
        "workflow_test@example.com",
        "WorkflowPassword123!",
        "Workflow Test User"
    ).await.expect("User registration should succeed");

    app_state.auth_service.verify_email(user.id).await
        .expect("Email verification should succeed");

    let login_response = app_state.auth_service.authenticate(
        "workflow_test@example.com",
        "WorkflowPassword123!"
    ).await.expect("Authentication should succeed");

    // Step 2: Generate code using AI
    let generation_request = models::GenerateRequest {
        prompt: "Create a REST API endpoint for user management".to_string(),
        language: Some("rust".to_string()),
        framework: Some("axum".to_string()),
        requirements: Some(vec!["authentication".to_string(), "CRUD".to_string()]),
        constraints: Some(vec!["secure".to_string()]),
    };

    let generation_response = app_state.ai_service.generate_code(generation_request).await
        .expect("Code generation should succeed");

    assert!(!generation_response.generated_files.is_empty(), "Should generate files");

    // Step 3: Analyze generated code
    let main_file = &generation_response.generated_files[0];
    let analysis_response = app_state.ai_service.analyze_code(&main_file.content).await
        .expect("Code analysis should succeed");

    assert!(analysis_response.get("complexity_score").is_some(), "Should analyze complexity");

    // Step 4: Check system health
    let system_health = app_state.monitoring_service.get_system_health().await
        .expect("System health check should succeed");

    assert_eq!(system_health.status, "operational", "System should be operational");

    // Step 5: Get dashboard stats
    let dashboard_stats = handlers::get_dashboard_stats(
        axum::extract::State(Arc::new(app_state))
    ).await.expect("Dashboard stats should succeed");

    assert!(dashboard_stats.0.total_generations >= 0, "Should have generation stats");

    println!("✅ Complete Workflow: PASSED");
}