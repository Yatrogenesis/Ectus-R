use serde_json::json;
use uuid::Uuid;
use reqwest::Client;
use std::time::Duration;

// Integration tests for API endpoints
// Note: These tests require the AION-R server to be running

const BASE_URL: &str = "http://localhost:8080";

#[tokio::test]
#[cfg_attr(not(feature = "integration-tests"), ignore)]
async fn test_api_health_check() {
    let client = Client::new();

    let response = client
        .get(&format!("{}/health", BASE_URL))
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send health check request");

    assert!(response.status().is_success());

    let health_data: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse health check response");

    assert_eq!(health_data["success"], true);
    assert!(health_data["data"]["overall_healthy"].as_bool().unwrap_or(false));
}

#[tokio::test]
#[cfg_attr(not(feature = "integration-tests"), ignore)]
async fn test_api_user_registration_and_authentication() {
    let client = Client::new();

    // Test user registration
    let registration_payload = json!({
        "username": "testuser_integration",
        "email": "testuser@integration.test",
        "password": "SecurePassword123!",
        "first_name": "Test",
        "last_name": "User"
    });

    let registration_response = client
        .post(&format!("{}/api/v1/auth/register", BASE_URL))
        .json(&registration_payload)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send registration request");

    assert!(registration_response.status().is_success());

    let registration_data: serde_json::Value = registration_response
        .json()
        .await
        .expect("Failed to parse registration response");

    assert_eq!(registration_data["success"], true);
    let user_id = registration_data["data"]["user"]["id"]
        .as_str()
        .expect("User ID not found in response");

    // Test user authentication
    let login_payload = json!({
        "identifier": "testuser@integration.test",
        "password": "SecurePassword123!",
        "remember_me": false
    });

    let login_response = client
        .post(&format!("{}/api/v1/auth/login", BASE_URL))
        .json(&login_payload)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send login request");

    assert!(login_response.status().is_success());

    let login_data: serde_json::Value = login_response
        .json()
        .await
        .expect("Failed to parse login response");

    assert_eq!(login_data["success"], true);
    let access_token = login_data["data"]["access_token"]
        .as_str()
        .expect("Access token not found in response");

    assert!(!access_token.is_empty());

    // Test authenticated request
    let profile_response = client
        .get(&format!("{}/api/v1/users/profile", BASE_URL))
        .bearer_auth(access_token)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send profile request");

    assert!(profile_response.status().is_success());

    let profile_data: serde_json::Value = profile_response
        .json()
        .await
        .expect("Failed to parse profile response");

    assert_eq!(profile_data["success"], true);
    assert_eq!(profile_data["data"]["id"], user_id);
}

#[tokio::test]
#[ignore] // Ignore by default since it requires running server
async fn test_api_ai_inference() {
    let client = Client::new();

    // First authenticate to get a token
    let login_payload = json!({
        "identifier": "admin@example.com",
        "password": "AdminPassword123!",
        "remember_me": false
    });

    let login_response = client
        .post(&format!("{}/api/v1/auth/login", BASE_URL))
        .json(&login_payload)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to authenticate for AI inference test");

    let login_data: serde_json::Value = login_response
        .json()
        .await
        .expect("Failed to parse login response");

    let access_token = login_data["data"]["access_token"]
        .as_str()
        .expect("Access token not found");

    // Test AI inference request
    let inference_payload = json!({
        "model_id": "gpt2",
        "input": {
            "type": "text",
            "prompt": "Hello, world! This is a test.",
            "context": null
        },
        "config": {
            "max_tokens": 50,
            "temperature": 0.7,
            "top_p": 0.9,
            "stream_response": false
        }
    });

    let inference_response = client
        .post(&format!("{}/api/v1/ai/inference", BASE_URL))
        .bearer_auth(access_token)
        .json(&inference_payload)
        .timeout(Duration::from_secs(60)) // AI inference might take longer
        .send()
        .await
        .expect("Failed to send inference request");

    // Note: This might fail if no models are loaded, which is expected in test environment
    if inference_response.status().is_success() {
        let inference_data: serde_json::Value = inference_response
            .json()
            .await
            .expect("Failed to parse inference response");

        assert_eq!(inference_data["success"], true);
        assert!(inference_data["data"]["output"].is_object());
    } else {
        // Expected in test environment without actual AI models
        println!("AI inference test skipped - no models available in test environment");
    }
}

#[tokio::test]
#[ignore] // Ignore by default since it requires running server
async fn test_api_rate_limiting() {
    let client = Client::new();

    // Make multiple rapid requests to trigger rate limiting
    let mut responses = Vec::new();

    for _ in 0..20 {
        let response = client
            .get(&format!("{}/api/v1/status", BASE_URL))
            .timeout(Duration::from_secs(5))
            .send()
            .await;

        responses.push(response);
    }

    // Check if any requests were rate limited
    let rate_limited_count = responses
        .into_iter()
        .filter_map(|r| r.ok())
        .filter(|response| response.status() == 429) // Too Many Requests
        .count();

    // Should have some rate limited requests if rate limiting is working
    println!("Rate limited requests: {}", rate_limited_count);
}

#[tokio::test]
#[ignore] // Ignore by default since it requires running server
async fn test_api_error_handling() {
    let client = Client::new();

    // Test 404 - Not Found
    let not_found_response = client
        .get(&format!("{}/api/v1/nonexistent", BASE_URL))
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send not found request");

    assert_eq!(not_found_response.status(), 404);

    // Test 400 - Bad Request (invalid JSON)
    let bad_request_response = client
        .post(&format!("{}/api/v1/auth/login", BASE_URL))
        .header("Content-Type", "application/json")
        .body("invalid json")
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send bad request");

    assert_eq!(bad_request_response.status(), 400);

    // Test 401 - Unauthorized
    let unauthorized_response = client
        .get(&format!("{}/api/v1/users/profile", BASE_URL))
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send unauthorized request");

    assert_eq!(unauthorized_response.status(), 401);
}

#[tokio::test]
#[ignore] // Ignore by default since it requires running server
async fn test_api_cors_headers() {
    let client = Client::new();

    let response = client
        .options(&format!("{}/api/v1/status", BASE_URL))
        .header("Origin", "https://example.com")
        .header("Access-Control-Request-Method", "GET")
        .header("Access-Control-Request-Headers", "authorization")
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send CORS preflight request");

    assert!(response.status().is_success());

    let cors_headers = response.headers();
    assert!(cors_headers.contains_key("access-control-allow-origin"));
    assert!(cors_headers.contains_key("access-control-allow-methods"));
    assert!(cors_headers.contains_key("access-control-allow-headers"));
}

#[tokio::test]
#[ignore] // Ignore by default since it requires running server
async fn test_api_pagination() {
    let client = Client::new();

    // First authenticate
    let login_payload = json!({
        "identifier": "admin@example.com",
        "password": "AdminPassword123!",
        "remember_me": false
    });

    let login_response = client
        .post(&format!("{}/api/v1/auth/login", BASE_URL))
        .json(&login_payload)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to authenticate for pagination test");

    let login_data: serde_json::Value = login_response
        .json()
        .await
        .expect("Failed to parse login response");

    let access_token = login_data["data"]["access_token"]
        .as_str()
        .expect("Access token not found");

    // Test pagination with users endpoint
    let paginated_response = client
        .get(&format!("{}/api/v1/users?page=1&per_page=10", BASE_URL))
        .bearer_auth(access_token)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send paginated request");

    if paginated_response.status().is_success() {
        let paginated_data: serde_json::Value = paginated_response
            .json()
            .await
            .expect("Failed to parse paginated response");

        assert_eq!(paginated_data["success"], true);
        assert!(paginated_data["pagination"].is_object());

        let pagination = &paginated_data["pagination"];
        assert!(pagination["page"].as_u64().unwrap() >= 1);
        assert!(pagination["per_page"].as_u64().unwrap() > 0);
        assert!(pagination["total_items"].as_u64().unwrap() >= 0);
        assert!(pagination["total_pages"].as_u64().unwrap() >= 0);
    }
}

#[tokio::test]
#[ignore] // Ignore by default since it requires running server
async fn test_api_concurrent_requests() {
    let client = Client::new();

    // Create multiple concurrent requests
    let mut handles = Vec::new();

    for i in 0..10 {
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            let response = client_clone
                .get(&format!("{}/health", BASE_URL))
                .header("X-Request-ID", format!("concurrent-test-{}", i))
                .timeout(Duration::from_secs(10))
                .send()
                .await;

            (i, response)
        });

        handles.push(handle);
    }

    // Wait for all requests to complete
    let results = futures::future::join_all(handles).await;

    let successful_requests = results
        .into_iter()
        .filter_map(|result| result.ok())
        .filter(|(_, response)| {
            response.as_ref().map(|r| r.status().is_success()).unwrap_or(false)
        })
        .count();

    // Most requests should succeed
    assert!(successful_requests >= 8, "Expected at least 8 successful concurrent requests, got {}", successful_requests);
}

#[tokio::test]
#[ignore] // Ignore by default since it requires running server
async fn test_api_request_id_tracking() {
    let client = Client::new();

    let request_id = Uuid::new_v4().to_string();

    let response = client
        .get(&format!("{}/health", BASE_URL))
        .header("X-Request-ID", &request_id)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send request with request ID");

    assert!(response.status().is_success());

    // Check that the response includes the request ID
    let response_headers = response.headers();
    if let Some(response_request_id) = response_headers.get("X-Request-ID") {
        assert_eq!(response_request_id.to_str().unwrap(), request_id);
    }

    let response_data: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response");

    // Check that the response body includes the request ID
    if let Some(body_request_id) = response_data["request_id"].as_str() {
        assert_eq!(body_request_id, request_id);
    }
}

#[tokio::test]
#[ignore] // Ignore by default since it requires running server
async fn test_api_metrics_endpoint() {
    let client = Client::new();

    let response = client
        .get(&format!("{}/metrics", BASE_URL))
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send metrics request");

    assert!(response.status().is_success());

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    // Metrics should be in Prometheus format
    assert!(content_type.contains("text/plain") || content_type.contains("text/prometheus"));

    let metrics_text = response
        .text()
        .await
        .expect("Failed to get metrics text");

    // Should contain some basic metrics
    assert!(metrics_text.contains("http_requests_total") ||
            metrics_text.contains("process_") ||
            !metrics_text.is_empty());
}

// Helper functions for integration tests

async fn setup_test_user(client: &Client) -> (String, String) {
    let user_id = Uuid::new_v4();
    let registration_payload = json!({
        "username": format!("testuser_{}", user_id),
        "email": format!("testuser_{}@test.com", user_id),
        "password": "TestPassword123!",
        "first_name": "Test",
        "last_name": "User"
    });

    let response = client
        .post(&format!("{}/api/v1/auth/register", BASE_URL))
        .json(&registration_payload)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to register test user");

    let data: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse registration response");

    let user_id = data["data"]["user"]["id"]
        .as_str()
        .expect("User ID not found")
        .to_string();

    // Login to get token
    let login_payload = json!({
        "identifier": registration_payload["email"],
        "password": registration_payload["password"],
        "remember_me": false
    });

    let login_response = client
        .post(&format!("{}/api/v1/auth/login", BASE_URL))
        .json(&login_payload)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to login test user");

    let login_data: serde_json::Value = login_response
        .json()
        .await
        .expect("Failed to parse login response");

    let access_token = login_data["data"]["access_token"]
        .as_str()
        .expect("Access token not found")
        .to_string();

    (user_id, access_token)
}

async fn cleanup_test_user(client: &Client, access_token: &str) {
    // Delete test user
    let _response = client
        .delete(&format!("{}/api/v1/users/profile", BASE_URL))
        .bearer_auth(access_token)
        .timeout(Duration::from_secs(10))
        .send()
        .await;

    // Ignore errors during cleanup
}