#!/usr/bin/env cargo run --bin test-runner --features test-runner

use aion_r_tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    aion_r_tests::run_test_suite().await
}