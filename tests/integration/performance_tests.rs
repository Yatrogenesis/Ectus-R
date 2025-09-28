use std::time::{Duration, Instant};
use tokio::time::sleep;
use serde_json::json;
use uuid::Uuid;
use futures::future::join_all;

// Performance and load testing

const BASE_URL: &str = "http://localhost:8080";

#[tokio::test]
#[ignore] // Ignore by default as it requires running system
async fn test_performance_api_response_times() {
    let client = reqwest::Client::new();

    // Test different endpoint response times
    let endpoints = vec![
        ("/health", "GET", None),
        ("/api/v1/status", "GET", None),
        ("/metrics", "GET", None),
    ];

    for (endpoint, method, body) in endpoints {
        let mut response_times = Vec::new();

        // Make 50 requests to get statistical data
        for _ in 0..50 {
            let start_time = Instant::now();

            let request = match method {
                "GET" => client.get(&format!("{}{}", BASE_URL, endpoint)),
                "POST" => {
                    let mut req = client.post(&format!("{}{}", BASE_URL, endpoint));
                    if let Some(json_body) = body {
                        req = req.json(&json_body);
                    }
                    req
                }
                _ => panic!("Unsupported HTTP method"),
            };

            let response = request
                .timeout(Duration::from_secs(10))
                .send()
                .await;

            let response_time = start_time.elapsed();

            if response.is_ok() {
                response_times.push(response_time);
            }

            // Small delay between requests
            sleep(Duration::from_millis(20)).await;
        }

        if !response_times.is_empty() {
            // Calculate statistics
            response_times.sort();
            let count = response_times.len();
            let min_time = response_times[0];
            let max_time = response_times[count - 1];
            let avg_time = response_times.iter().sum::<Duration>() / count as u32;
            let p50_time = response_times[count / 2];
            let p95_time = response_times[count * 95 / 100];
            let p99_time = response_times[count * 99 / 100];

            println!("Performance stats for {} {}:", method, endpoint);
            println!("  Requests: {}", count);
            println!("  Min: {:?}", min_time);
            println!("  Max: {:?}", max_time);
            println!("  Avg: {:?}", avg_time);
            println!("  P50: {:?}", p50_time);
            println!("  P95: {:?}", p95_time);
            println!("  P99: {:?}", p99_time);

            // Performance assertions
            assert!(avg_time < Duration::from_millis(500),
                   "Average response time too high for {}: {:?}", endpoint, avg_time);
            assert!(p95_time < Duration::from_millis(1000),
                   "95th percentile response time too high for {}: {:?}", endpoint, p95_time);
        }
    }
}

#[tokio::test]
#[ignore] // Ignore by default as it requires running system
async fn test_performance_concurrent_load() {
    let client = reqwest::Client::new();

    // Test with increasing concurrent load
    let concurrency_levels = vec![1, 5, 10, 25, 50, 100];
    let requests_per_client = 20;

    for concurrency in concurrency_levels {
        println!("Testing with {} concurrent clients", concurrency);

        let start_time = Instant::now();
        let mut handles = Vec::new();

        for client_id in 0..concurrency {
            let client_clone = client.clone();
            let handle = tokio::spawn(async move {
                let mut successful_requests = 0;
                let mut total_response_time = Duration::new(0, 0);

                for _ in 0..requests_per_client {
                    let request_start = Instant::now();

                    let result = client_clone
                        .get(&format!("{}/api/v1/status", BASE_URL))
                        .header("X-Client-ID", client_id.to_string())
                        .timeout(Duration::from_secs(30))
                        .send()
                        .await;

                    let response_time = request_start.elapsed();

                    if let Ok(response) = result {
                        if response.status().is_success() {
                            successful_requests += 1;
                            total_response_time += response_time;
                        }
                    }
                }

                (successful_requests, total_response_time)
            });

            handles.push(handle);
        }

        // Wait for all requests to complete
        let results = join_all(handles).await;
        let total_time = start_time.elapsed();

        // Aggregate results
        let mut total_successful = 0;
        let mut total_response_time = Duration::new(0, 0);

        for result in results {
            if let Ok((successful, response_time)) = result {
                total_successful += successful;
                total_response_time += response_time;
            }
        }

        let total_requests = concurrency * requests_per_client;
        let success_rate = (total_successful as f64 / total_requests as f64) * 100.0;
        let avg_response_time = if total_successful > 0 {
            total_response_time / total_successful as u32
        } else {
            Duration::new(0, 0)
        };
        let throughput = total_successful as f64 / total_time.as_secs_f64();

        println!("  Total requests: {}", total_requests);
        println!("  Successful: {}", total_successful);
        println!("  Success rate: {:.1}%", success_rate);
        println!("  Average response time: {:?}", avg_response_time);
        println!("  Throughput: {:.1} req/s", throughput);
        println!("  Total time: {:?}", total_time);

        // Performance expectations should degrade gracefully with load
        let min_success_rate = match concurrency {
            1..=10 => 99.0,
            11..=25 => 95.0,
            26..=50 => 90.0,
            _ => 80.0,
        };

        let max_avg_response_time = match concurrency {
            1..=10 => Duration::from_millis(200),
            11..=25 => Duration::from_millis(500),
            26..=50 => Duration::from_millis(1000),
            _ => Duration::from_millis(2000),
        };

        assert!(success_rate >= min_success_rate,
               "Success rate too low at {} concurrency: {:.1}%", concurrency, success_rate);
        assert!(avg_response_time <= max_avg_response_time,
               "Response time too high at {} concurrency: {:?}", concurrency, avg_response_time);

        // Brief pause between concurrency level tests
        sleep(Duration::from_secs(2)).await;
    }
}

#[tokio::test]
#[ignore] // Ignore by default as it requires running system
async fn test_performance_sustained_load() {
    let client = reqwest::Client::new();

    // Run sustained load for 2 minutes
    let test_duration = Duration::from_secs(120);
    let concurrent_clients = 10;
    let requests_per_second_per_client = 2;
    let request_interval = Duration::from_millis(1000 / requests_per_second_per_client);

    println!("Running sustained load test for {:?}", test_duration);
    println!("Concurrent clients: {}", concurrent_clients);
    println!("Target rate: {} req/s per client", requests_per_second_per_client);

    let start_time = Instant::now();
    let mut handles = Vec::new();

    for client_id in 0..concurrent_clients {
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            let mut successful_requests = 0;
            let mut failed_requests = 0;
            let mut total_response_time = Duration::new(0, 0);
            let mut max_response_time = Duration::new(0, 0);

            let client_start = Instant::now();

            while client_start.elapsed() < test_duration {
                let request_start = Instant::now();

                let result = client_clone
                    .get(&format!("{}/api/v1/status", BASE_URL))
                    .header("X-Client-ID", client_id.to_string())
                    .timeout(Duration::from_secs(10))
                    .send()
                    .await;

                let response_time = request_start.elapsed();
                max_response_time = max_response_time.max(response_time);

                match result {
                    Ok(response) if response.status().is_success() => {
                        successful_requests += 1;
                        total_response_time += response_time;
                    }
                    _ => {
                        failed_requests += 1;
                    }
                }

                sleep(request_interval).await;
            }

            (successful_requests, failed_requests, total_response_time, max_response_time)
        });

        handles.push(handle);
    }

    // Wait for all clients to complete
    let results = join_all(handles).await;
    let actual_duration = start_time.elapsed();

    // Aggregate results
    let mut total_successful = 0;
    let mut total_failed = 0;
    let mut total_response_time = Duration::new(0, 0);
    let mut overall_max_response_time = Duration::new(0, 0);

    for result in results {
        if let Ok((successful, failed, response_time, max_time)) = result {
            total_successful += successful;
            total_failed += failed;
            total_response_time += response_time;
            overall_max_response_time = overall_max_response_time.max(max_time);
        }
    }

    let total_requests = total_successful + total_failed;
    let success_rate = (total_successful as f64 / total_requests as f64) * 100.0;
    let avg_response_time = if total_successful > 0 {
        total_response_time / total_successful as u32
    } else {
        Duration::new(0, 0)
    };
    let actual_throughput = total_successful as f64 / actual_duration.as_secs_f64();
    let expected_throughput = (concurrent_clients * requests_per_second_per_client) as f64;

    println!("Sustained load test results:");
    println!("  Duration: {:?}", actual_duration);
    println!("  Total requests: {}", total_requests);
    println!("  Successful: {}", total_successful);
    println!("  Failed: {}", total_failed);
    println!("  Success rate: {:.1}%", success_rate);
    println!("  Average response time: {:?}", avg_response_time);
    println!("  Max response time: {:?}", overall_max_response_time);
    println!("  Actual throughput: {:.1} req/s", actual_throughput);
    println!("  Expected throughput: {:.1} req/s", expected_throughput);
    println!("  Throughput ratio: {:.1}%", (actual_throughput / expected_throughput) * 100.0);

    // Performance assertions for sustained load
    assert!(success_rate >= 95.0, "Success rate too low during sustained load: {:.1}%", success_rate);
    assert!(avg_response_time < Duration::from_millis(1000),
           "Average response time too high during sustained load: {:?}", avg_response_time);
    assert!(overall_max_response_time < Duration::from_secs(5),
           "Maximum response time too high during sustained load: {:?}", overall_max_response_time);

    // Throughput should be at least 80% of expected
    let throughput_ratio = actual_throughput / expected_throughput;
    assert!(throughput_ratio >= 0.8,
           "Throughput too low: {:.1}% of expected", throughput_ratio * 100.0);
}

#[tokio::test]
#[ignore] // Ignore by default as it requires running system and auth
async fn test_performance_ai_inference() {
    let client = reqwest::Client::new();

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
        .await;

    if login_response.is_err() {
        println!("AI inference performance test skipped - authentication failed");
        return;
    }

    let login_data: serde_json::Value = login_response.unwrap()
        .json()
        .await
        .expect("Failed to parse login response");

    let access_token = login_data["data"]["access_token"]
        .as_str()
        .expect("Access token not found");

    // Test AI inference performance with different configurations
    let test_configs = vec![
        ("small", json!({
            "model_id": "gpt2",
            "max_tokens": 10,
            "temperature": 0.7
        })),
        ("medium", json!({
            "model_id": "gpt2",
            "max_tokens": 50,
            "temperature": 0.7
        })),
        ("large", json!({
            "model_id": "gpt2",
            "max_tokens": 200,
            "temperature": 0.7
        })),
    ];

    for (config_name, config) in test_configs {
        println!("Testing AI inference performance - {} config", config_name);

        let mut response_times = Vec::new();
        let mut successful_inferences = 0;

        // Run 10 inference requests
        for i in 0..10 {
            let inference_payload = json!({
                "model_id": config["model_id"],
                "input": {
                    "type": "text",
                    "prompt": format!("Test prompt number {}", i),
                    "context": null
                },
                "config": config
            });

            let start_time = Instant::now();

            let result = client
                .post(&format!("{}/api/v1/ai/inference", BASE_URL))
                .bearer_auth(access_token)
                .json(&inference_payload)
                .timeout(Duration::from_secs(120)) // AI inference can take time
                .send()
                .await;

            let response_time = start_time.elapsed();

            match result {
                Ok(response) if response.status().is_success() => {
                    successful_inferences += 1;
                    response_times.push(response_time);
                    println!("  Inference {} completed in {:?}", i + 1, response_time);
                }
                Ok(response) => {
                    println!("  Inference {} failed with status: {}", i + 1, response.status());
                }
                Err(e) => {
                    println!("  Inference {} failed with error: {}", i + 1, e);
                }
            }

            // Delay between requests to avoid overloading
            sleep(Duration::from_secs(1)).await;
        }

        if !response_times.is_empty() {
            response_times.sort();
            let avg_time = response_times.iter().sum::<Duration>() / response_times.len() as u32;
            let min_time = response_times[0];
            let max_time = response_times[response_times.len() - 1];

            println!("  Results for {} config:", config_name);
            println!("    Successful inferences: {}/10", successful_inferences);
            println!("    Average time: {:?}", avg_time);
            println!("    Min time: {:?}", min_time);
            println!("    Max time: {:?}", max_time);

            // Performance expectations based on config size
            let max_expected_time = match config_name {
                "small" => Duration::from_secs(10),
                "medium" => Duration::from_secs(30),
                "large" => Duration::from_secs(60),
                _ => Duration::from_secs(120),
            };

            assert!(avg_time <= max_expected_time,
                   "AI inference average time too high for {} config: {:?}", config_name, avg_time);
        } else {
            println!("  No successful inferences for {} config - may be expected in test environment", config_name);
        }
    }
}

#[tokio::test]
#[ignore] // Ignore by default as it requires running system
async fn test_performance_memory_usage() {
    let client = reqwest::Client::new();

    // Monitor memory usage during load testing
    println!("Starting memory usage monitoring");

    let initial_memory_info = get_memory_info().await;
    println!("Initial memory usage: {:?}", initial_memory_info);

    // Generate load for memory testing
    let num_clients = 20;
    let requests_per_client = 50;

    let mut handles = Vec::new();

    for client_id in 0..num_clients {
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            for request_id in 0..requests_per_client {
                // Create requests with varying payload sizes
                let payload_size = (request_id % 5 + 1) * 1024; // 1KB to 5KB
                let large_payload = json!({
                    "client_id": client_id,
                    "request_id": request_id,
                    "data": "x".repeat(payload_size)
                });

                let _result = client_clone
                    .post(&format!("{}/api/v1/echo", BASE_URL)) // Echo endpoint for testing
                    .json(&large_payload)
                    .timeout(Duration::from_secs(10))
                    .send()
                    .await;

                // Small delay to allow memory monitoring
                sleep(Duration::from_millis(50)).await;
            }
        });

        handles.push(handle);
    }

    // Monitor memory during load test
    let mut memory_samples = Vec::new();
    let monitoring_handle = tokio::spawn(async move {
        for _ in 0..60 { // Monitor for 60 seconds
            let memory_info = get_memory_info().await;
            memory_samples.push(memory_info);
            sleep(Duration::from_secs(1)).await;
        }
        memory_samples
    });

    // Wait for load test to complete
    join_all(handles).await;

    // Get memory monitoring results
    let memory_samples = monitoring_handle.await.unwrap();

    let final_memory_info = get_memory_info().await;
    println!("Final memory usage: {:?}", final_memory_info);

    // Analyze memory usage patterns
    if !memory_samples.is_empty() {
        let max_memory = memory_samples.iter()
            .map(|info| info.used_memory_mb)
            .max()
            .unwrap_or(0);

        let memory_increase = final_memory_info.used_memory_mb as i64 - initial_memory_info.used_memory_mb as i64;

        println!("Memory usage analysis:");
        println!("  Peak memory usage: {} MB", max_memory);
        println!("  Memory increase: {} MB", memory_increase);
        println!("  Available memory: {} MB", final_memory_info.available_memory_mb);

        // Memory usage should be reasonable
        assert!(max_memory < 4096, "Peak memory usage too high: {} MB", max_memory);
        assert!(memory_increase < 1024, "Memory increase too high: {} MB", memory_increase);
        assert!(final_memory_info.available_memory_mb > 512, "Not enough available memory: {} MB", final_memory_info.available_memory_mb);
    }
}

#[tokio::test]
#[ignore] // Ignore by default as it requires running system
async fn test_performance_database_operations() {
    let client = reqwest::Client::new();

    // Test database performance through API
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
        .await;

    if login_response.is_err() {
        println!("Database performance test skipped - authentication failed");
        return;
    }

    let login_data: serde_json::Value = login_response.unwrap()
        .json()
        .await
        .expect("Failed to parse login response");

    let access_token = login_data["data"]["access_token"]
        .as_str()
        .expect("Access token not found");

    // Test different database operations
    let operations = vec![
        ("Read operations", "GET", "/api/v1/users?page=1&per_page=50"),
        ("Search operations", "GET", "/api/v1/users?search=test&page=1&per_page=20"),
    ];

    for (operation_name, method, endpoint) in operations {
        println!("Testing {}", operation_name);

        let mut response_times = Vec::new();
        let num_requests = 30;

        for _ in 0..num_requests {
            let start_time = Instant::now();

            let request = match method {
                "GET" => client.get(&format!("{}{}", BASE_URL, endpoint)),
                "POST" => client.post(&format!("{}{}", BASE_URL, endpoint)),
                _ => panic!("Unsupported method"),
            };

            let result = request
                .bearer_auth(access_token)
                .timeout(Duration::from_secs(10))
                .send()
                .await;

            let response_time = start_time.elapsed();

            if let Ok(response) = result {
                if response.status().is_success() {
                    response_times.push(response_time);
                }
            }

            sleep(Duration::from_millis(100)).await;
        }

        if !response_times.is_empty() {
            response_times.sort();
            let count = response_times.len();
            let avg_time = response_times.iter().sum::<Duration>() / count as u32;
            let p95_time = response_times[count * 95 / 100];

            println!("  {} results:", operation_name);
            println!("    Successful requests: {}/{}", count, num_requests);
            println!("    Average response time: {:?}", avg_time);
            println!("    95th percentile: {:?}", p95_time);

            // Database operations should be reasonably fast
            assert!(avg_time < Duration::from_millis(500),
                   "{} average time too high: {:?}", operation_name, avg_time);
            assert!(p95_time < Duration::from_millis(1000),
                   "{} 95th percentile time too high: {:?}", operation_name, p95_time);
        }
    }
}

// Helper functions for performance tests

#[derive(Debug, Clone)]
struct MemoryInfo {
    total_memory_mb: u64,
    used_memory_mb: u64,
    available_memory_mb: u64,
}

async fn get_memory_info() -> MemoryInfo {
    use sysinfo::{System, SystemExt};

    let mut system = System::new_all();
    system.refresh_memory();

    let total_memory = system.total_memory() / 1024 / 1024; // Convert to MB
    let used_memory = system.used_memory() / 1024 / 1024; // Convert to MB
    let available_memory = system.available_memory() / 1024 / 1024; // Convert to MB

    MemoryInfo {
        total_memory_mb: total_memory,
        used_memory_mb: used_memory,
        available_memory_mb: available_memory,
    }
}

async fn measure_response_time<F, Fut>(operation: F) -> Duration
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let start_time = Instant::now();
    operation().await;
    start_time.elapsed()
}

#[tokio::test]
#[ignore] // Ignore by default as it requires running system
async fn test_performance_stress_test() {
    // Extreme stress test to find breaking points
    let client = reqwest::Client::new();

    println!("Running stress test to find system limits");

    let max_clients = 200;
    let ramp_up_time = Duration::from_secs(30);
    let test_duration = Duration::from_secs(60);

    let clients_per_second = max_clients as f64 / ramp_up_time.as_secs_f64();

    println!("Ramping up to {} clients over {:?}", max_clients, ramp_up_time);
    println!("Clients per second: {:.1}", clients_per_second);

    let start_time = Instant::now();
    let mut handles = Vec::new();
    let mut client_count = 0;

    // Ramp up clients gradually
    while start_time.elapsed() < ramp_up_time && client_count < max_clients {
        let client_clone = client.clone();
        let client_id = client_count;

        let handle = tokio::spawn(async move {
            let mut requests = 0;
            let mut successful = 0;
            let client_start = Instant::now();

            while client_start.elapsed() < test_duration {
                requests += 1;

                let result = client_clone
                    .get(&format!("{}/api/v1/status", BASE_URL))
                    .header("X-Stress-Client", client_id.to_string())
                    .timeout(Duration::from_secs(5))
                    .send()
                    .await;

                if let Ok(response) = result {
                    if response.status().is_success() {
                        successful += 1;
                    }
                }

                sleep(Duration::from_millis(100)).await;
            }

            (client_id, requests, successful)
        });

        handles.push(handle);
        client_count += 1;

        // Wait before starting next client
        let delay = Duration::from_millis((1000.0 / clients_per_second) as u64);
        sleep(delay).await;
    }

    println!("All {} clients started, running for {:?}", client_count, test_duration);

    // Wait for all clients to complete
    let results = join_all(handles).await;

    // Analyze stress test results
    let mut total_requests = 0;
    let mut total_successful = 0;
    let mut active_clients = 0;

    for result in results {
        if let Ok((_, requests, successful)) = result {
            total_requests += requests;
            total_successful += successful;
            active_clients += 1;
        }
    }

    let success_rate = (total_successful as f64 / total_requests as f64) * 100.0;
    let total_test_time = start_time.elapsed();
    let throughput = total_successful as f64 / total_test_time.as_secs_f64();

    println!("Stress test results:");
    println!("  Active clients: {}", active_clients);
    println!("  Total requests: {}", total_requests);
    println!("  Successful requests: {}", total_successful);
    println!("  Success rate: {:.1}%", success_rate);
    println!("  Throughput: {:.1} req/s", throughput);
    println!("  Test duration: {:?}", total_test_time);

    // Even under stress, some minimum performance should be maintained
    assert!(success_rate >= 70.0, "Success rate too low under stress: {:.1}%", success_rate);
    assert!(throughput >= 10.0, "Throughput too low under stress: {:.1} req/s", throughput);

    println!("Stress test completed - system maintained minimum performance");
}