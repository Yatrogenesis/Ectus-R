//! Test runner for AION monitoring system integration tests

use aion_monitoring::test_integration::run_integration_tests;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize basic logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting AION Monitoring Integration Tests...");

    // Run all integration tests
    match run_integration_tests().await {
        Ok(()) => {
            println!("🎉 All tests completed successfully!");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("❌ Tests failed: {}", e);
            std::process::exit(1);
        }
    }
}