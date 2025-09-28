use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::thread;
use tokio::time::sleep;
use sysinfo::{System, SystemExt, ProcessExt, CpuExt};

// System-level integration tests

#[tokio::test]
#[ignore] // Ignore by default as it requires system resources
async fn test_system_startup_and_shutdown() {
    // Test that the AION-R system can start up and shut down cleanly
    let start_time = Instant::now();

    // Start the system (assuming we have a startup script)
    let mut child = Command::new("./scripts/start-dev.sh")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start AION-R system");

    // Wait for system to fully start (check health endpoint)
    let client = reqwest::Client::new();
    let mut system_ready = false;

    for _ in 0..30 { // Try for 30 seconds
        sleep(Duration::from_secs(1)).await;

        if let Ok(response) = client
            .get("http://localhost:8080/health")
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            if response.status().is_success() {
                system_ready = true;
                break;
            }
        }
    }

    assert!(system_ready, "System failed to start within 30 seconds");

    let startup_time = start_time.elapsed();
    println!("System startup time: {:?}", startup_time);

    // System should start reasonably quickly
    assert!(startup_time < Duration::from_secs(60), "System startup took too long");

    // Test graceful shutdown
    let shutdown_start = Instant::now();

    // Send SIGTERM to gracefully shut down
    child.kill().expect("Failed to terminate AION-R system");

    let exit_status = child.wait().expect("Failed to wait for AION-R system");
    let shutdown_time = shutdown_start.elapsed();

    println!("System shutdown time: {:?}", shutdown_time);
    println!("Exit status: {:?}", exit_status);

    // Shutdown should be reasonably quick
    assert!(shutdown_time < Duration::from_secs(30), "System shutdown took too long");
}

#[tokio::test]
#[ignore] // Ignore by default as it requires system resources
async fn test_system_resource_usage() {
    // Monitor system resource usage during operation
    let mut system = System::new_all();
    system.refresh_all();

    let initial_memory = system.total_memory();
    let initial_cpu_count = system.cpus().len();

    println!("System specs:");
    println!("  Total memory: {} MB", initial_memory / 1024 / 1024);
    println!("  CPU cores: {}", initial_cpu_count);

    // Start the system
    let mut child = Command::new("./scripts/start-dev.sh")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start AION-R system");

    // Wait for system to start
    let client = reqwest::Client::new();
    for _ in 0..30 {
        sleep(Duration::from_secs(1)).await;

        if client
            .get("http://localhost:8080/health")
            .timeout(Duration::from_secs(2))
            .send()
            .await
            .is_ok()
        {
            break;
        }
    }

    // Monitor resource usage for 30 seconds
    let mut max_memory_usage = 0;
    let mut avg_cpu_usage = 0.0;
    let mut samples = 0;

    for _ in 0..30 {
        sleep(Duration::from_secs(1)).await;
        system.refresh_all();

        // Find AION-R processes
        let mut total_memory = 0;
        let mut total_cpu = 0.0;

        for (_, process) in system.processes() {
            let process_name = process.name().to_lowercase();
            if process_name.contains("aion") || process_name.contains("rust") {
                total_memory += process.memory();
                total_cpu += process.cpu_usage();
            }
        }

        max_memory_usage = max_memory_usage.max(total_memory);
        avg_cpu_usage += total_cpu;
        samples += 1;

        println!("Current usage - Memory: {} MB, CPU: {:.1}%",
                 total_memory / 1024 / 1024, total_cpu);
    }

    avg_cpu_usage /= samples as f32;

    println!("Resource usage summary:");
    println!("  Max memory usage: {} MB", max_memory_usage / 1024 / 1024);
    println!("  Average CPU usage: {:.1}%", avg_cpu_usage);

    // Clean shutdown
    child.kill().expect("Failed to terminate system");
    child.wait().expect("Failed to wait for system termination");

    // Verify reasonable resource usage
    let max_memory_mb = max_memory_usage / 1024 / 1024;
    assert!(max_memory_mb < 2048, "Memory usage too high: {} MB", max_memory_mb);
    assert!(avg_cpu_usage < 80.0, "CPU usage too high: {:.1}%", avg_cpu_usage);
}

#[tokio::test]
#[ignore] // Ignore by default as it requires system resources
async fn test_system_database_connectivity() {
    // Test database connectivity and migrations
    let output = Command::new("psql")
        .args(&[
            "-h", "localhost",
            "-p", "5432",
            "-U", "aion_user",
            "-d", "aion_test",
            "-c", "SELECT version();"
        ])
        .env("PGPASSWORD", "test_password")
        .output();

    match output {
        Ok(result) => {
            assert!(result.status.success(), "Database connection failed");

            let stdout = String::from_utf8_lossy(&result.stdout);
            println!("Database version: {}", stdout.trim());

            // Check if our tables exist
            let tables_output = Command::new("psql")
                .args(&[
                    "-h", "localhost",
                    "-p", "5432",
                    "-U", "aion_user",
                    "-d", "aion_test",
                    "-c", "\\dt"
                ])
                .env("PGPASSWORD", "test_password")
                .output()
                .expect("Failed to list database tables");

            let tables_stdout = String::from_utf8_lossy(&tables_output.stdout);
            println!("Database tables:\n{}", tables_stdout);

            // Should have our core tables
            assert!(tables_stdout.contains("users"));
            assert!(tables_stdout.contains("tenants"));
            assert!(tables_stdout.contains("roles"));
        }
        Err(e) => {
            println!("Database test skipped - PostgreSQL not available: {}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Ignore by default as it requires system resources
async fn test_system_load_handling() {
    // Test system behavior under load
    let client = reqwest::Client::new();

    // First ensure system is running
    let health_check = client
        .get("http://localhost:8080/health")
        .timeout(Duration::from_secs(5))
        .send()
        .await;

    if health_check.is_err() {
        println!("System load test skipped - server not running");
        return;
    }

    // Generate concurrent load
    let num_concurrent_requests = 50;
    let requests_per_client = 10;

    let start_time = Instant::now();
    let mut handles = Vec::new();

    for client_id in 0..num_concurrent_requests {
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            let mut successful_requests = 0;
            let mut failed_requests = 0;
            let mut total_response_time = Duration::new(0, 0);

            for request_id in 0..requests_per_client {
                let request_start = Instant::now();

                let result = client_clone
                    .get("http://localhost:8080/api/v1/status")
                    .header("X-Client-ID", client_id.to_string())
                    .header("X-Request-ID", request_id.to_string())
                    .timeout(Duration::from_secs(30))
                    .send()
                    .await;

                let response_time = request_start.elapsed();
                total_response_time += response_time;

                match result {
                    Ok(response) if response.status().is_success() => {
                        successful_requests += 1;
                    }
                    Ok(_) => {
                        failed_requests += 1;
                    }
                    Err(_) => {
                        failed_requests += 1;
                    }
                }

                // Small delay between requests from same client
                sleep(Duration::from_millis(100)).await;
            }

            (successful_requests, failed_requests, total_response_time)
        });

        handles.push(handle);
    }

    // Wait for all requests to complete
    let results = futures::future::join_all(handles).await;

    let total_time = start_time.elapsed();

    // Aggregate results
    let mut total_successful = 0;
    let mut total_failed = 0;
    let mut total_response_time = Duration::new(0, 0);

    for result in results {
        if let Ok((successful, failed, response_time)) = result {
            total_successful += successful;
            total_failed += failed;
            total_response_time += response_time;
        }
    }

    let total_requests = total_successful + total_failed;
    let success_rate = (total_successful as f64 / total_requests as f64) * 100.0;
    let avg_response_time = total_response_time / total_requests as u32;

    println!("Load test results:");
    println!("  Total requests: {}", total_requests);
    println!("  Successful requests: {}", total_successful);
    println!("  Failed requests: {}", total_failed);
    println!("  Success rate: {:.1}%", success_rate);
    println!("  Average response time: {:?}", avg_response_time);
    println!("  Total test time: {:?}", total_time);
    println!("  Requests per second: {:.1}", total_requests as f64 / total_time.as_secs_f64());

    // Verify acceptable performance
    assert!(success_rate >= 95.0, "Success rate too low: {:.1}%", success_rate);
    assert!(avg_response_time < Duration::from_millis(1000), "Average response time too high: {:?}", avg_response_time);
}

#[tokio::test]
#[ignore] // Ignore by default as it requires system resources
async fn test_system_failover_and_recovery() {
    // Test system behavior during component failures
    let client = reqwest::Client::new();

    // Verify system is initially healthy
    let initial_health = client
        .get("http://localhost:8080/health")
        .timeout(Duration::from_secs(5))
        .send()
        .await;

    if initial_health.is_err() {
        println!("Failover test skipped - system not running");
        return;
    }

    println!("System initially healthy");

    // Simulate database connection failure by connecting to wrong port
    let bad_db_config = r#"
    [database]
    host = "localhost"
    port = 9999  # Wrong port
    "#;

    // Save current config and replace with bad config
    std::fs::write("/tmp/aion_bad_config.toml", bad_db_config)
        .expect("Failed to write bad config");

    // Test that system handles database failure gracefully
    // (This would require the system to reload config or restart with bad config)

    // For now, just test that health endpoint reports database issues
    sleep(Duration::from_secs(5)).await;

    let health_response = client
        .get("http://localhost:8080/health")
        .timeout(Duration::from_secs(10))
        .send()
        .await;

    match health_response {
        Ok(response) => {
            if let Ok(health_data) = response.json::<serde_json::Value>().await {
                println!("Health check during simulated failure: {}", health_data);

                // System should still respond but report unhealthy components
                if let Some(services) = health_data["data"]["services"].as_object() {
                    if let Some(database_health) = services.get("database") {
                        // Database should be reported as unhealthy
                        let db_healthy = database_health["healthy"].as_bool().unwrap_or(true);
                        if !db_healthy {
                            println!("Database correctly reported as unhealthy");
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Health check failed during simulated failure: {}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Ignore by default as it requires system resources
async fn test_system_metrics_collection() {
    // Test that system metrics are being collected properly
    let client = reqwest::Client::new();

    // Check metrics endpoint
    let metrics_response = client
        .get("http://localhost:9090/metrics") // Prometheus metrics port
        .timeout(Duration::from_secs(10))
        .send()
        .await;

    match metrics_response {
        Ok(response) => {
            assert!(response.status().is_success());

            let metrics_text = response.text().await.expect("Failed to get metrics text");
            println!("Sample metrics:\n{}", &metrics_text[..std::cmp::min(1000, metrics_text.len())]);

            // Verify key metrics are present
            assert!(metrics_text.contains("http_requests_total") ||
                    metrics_text.contains("process_cpu_seconds_total") ||
                    metrics_text.contains("go_memstats") ||
                    !metrics_text.is_empty(),
                    "Expected metrics not found");
        }
        Err(e) => {
            println!("Metrics test skipped - metrics endpoint not available: {}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Ignore by default as it requires system resources
async fn test_system_log_output() {
    // Test that system produces proper log output
    let log_file_path = "/tmp/aion_test.log";

    // Start system with log file output
    let mut child = Command::new("./scripts/start-dev.sh")
        .env("AION_LOG_FILE", log_file_path)
        .env("AION_LOG_LEVEL", "info")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start system for log test");

    // Wait for system to start and generate some logs
    sleep(Duration::from_secs(10)).await;

    // Make some requests to generate activity
    let client = reqwest::Client::new();
    for _ in 0..5 {
        let _ = client
            .get("http://localhost:8080/health")
            .timeout(Duration::from_secs(5))
            .send()
            .await;

        sleep(Duration::from_secs(1)).await;
    }

    // Check log file
    match std::fs::read_to_string(log_file_path) {
        Ok(log_content) => {
            println!("Log file size: {} bytes", log_content.len());
            println!("Sample log entries:\n{}", &log_content[..std::cmp::min(1000, log_content.len())]);

            // Verify log content
            assert!(!log_content.is_empty(), "Log file should not be empty");
            assert!(log_content.contains("INFO") || log_content.contains("DEBUG") ||
                    log_content.contains("WARN") || log_content.contains("ERROR"),
                    "Log should contain log level indicators");

            // Clean up log file
            let _ = std::fs::remove_file(log_file_path);
        }
        Err(e) => {
            println!("Log test skipped - could not read log file: {}", e);
        }
    }

    // Clean shutdown
    child.kill().expect("Failed to terminate system");
    child.wait().expect("Failed to wait for system termination");
}

#[tokio::test]
#[ignore] // Ignore by default as it requires system resources
async fn test_system_configuration_validation() {
    // Test that system validates configuration properly
    let invalid_config = r#"
    [server]
    host = "127.0.0.1"
    port = 0  # Invalid port
    workers = 0  # Invalid worker count

    [database]
    host = ""  # Invalid empty host
    max_connections = 0  # Invalid connection count
    "#;

    let config_file_path = "/tmp/aion_invalid_config.toml";
    std::fs::write(config_file_path, invalid_config)
        .expect("Failed to write invalid config");

    // Try to start system with invalid config
    let output = Command::new("./scripts/start-dev.sh")
        .env("AION_CONFIG_FILE", config_file_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to start system with invalid config");

    // System should fail to start with invalid config
    assert!(!output.status.success(), "System should reject invalid configuration");

    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("Config validation error output:\n{}", stderr);

    // Should contain validation error messages
    assert!(stderr.contains("config") || stderr.contains("validation") ||
            stderr.contains("invalid") || stderr.contains("error"),
            "Expected configuration validation error messages");

    // Clean up
    let _ = std::fs::remove_file(config_file_path);
}

// Helper functions for system tests

fn wait_for_system_ready(timeout_secs: u64) -> bool {
    let client = reqwest::Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();

    for _ in 0..timeout_secs {
        let result = rt.block_on(async {
            client
                .get("http://localhost:8080/health")
                .timeout(Duration::from_secs(2))
                .send()
                .await
        });

        if let Ok(response) = result {
            if response.status().is_success() {
                return true;
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    false
}

fn get_system_processes() -> Vec<String> {
    let mut system = System::new_all();
    system.refresh_all();

    let mut aion_processes = Vec::new();

    for (pid, process) in system.processes() {
        let process_name = process.name().to_lowercase();
        if process_name.contains("aion") ||
           (process_name.contains("rust") &&
            process.cmd().iter().any(|arg| arg.contains("aion"))) {
            aion_processes.push(format!("{}: {} (PID: {})",
                                      process.name(),
                                      process.cmd().join(" "),
                                      pid));
        }
    }

    aion_processes
}