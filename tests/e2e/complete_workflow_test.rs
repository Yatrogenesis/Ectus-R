// AION-R Complete E2E Workflow Test
// Tests the entire platform: Frontend -> Backend -> AI Engine -> QA -> Refactoring -> Deployment

use std::time::Duration;
use tokio::time::sleep;
use serde_json::json;
use reqwest::Client;

const API_BASE_URL: &str = "http://localhost:8080";
const FRONTEND_URL: &str = "http://localhost:3000";

#[derive(Debug, serde::Deserialize)]
struct Project {
    id: String,
    name: String,
    status: String,
}

#[derive(Debug, serde::Deserialize)]
struct QAResult {
    success: bool,
    tests_run: Option<usize>,
    tests_passed: Option<usize>,
    tests_failed: Option<usize>,
}

#[derive(Debug, serde::Deserialize)]
struct RefactoringResult {
    success: bool,
    changes_applied: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
struct DeploymentResult {
    deployment_url: String,
    status: String,
}

/// Complete E2E Test: User creates project, runs QA, applies refactoring, deploys
#[tokio::test]
#[ignore] // Run with: cargo test --ignored e2e_complete_workflow
async fn e2e_complete_workflow() {
    println!("🚀 Starting Complete E2E Workflow Test");
    println!("================================================\n");

    let client = Client::new();

    // Step 0: Wait for services to be ready
    println!("📡 Step 0: Checking service health...");
    wait_for_services(&client).await;
    println!("   ✓ All services healthy\n");

    // Step 1: Create a new project via API
    println!("📦 Step 1: Creating new project...");
    let project = create_project(&client).await;
    println!("   ✓ Project created: {} (ID: {})", project.name, project.id);
    println!("   Status: {}\n", project.status);

    // Step 2: Generate code for the project using AI
    println!("🤖 Step 2: Generating code with AI engine...");
    let generated_code = generate_code(&client, &project.id).await;
    println!("   ✓ Code generated: {} lines", generated_code.len());
    println!("   Language: Rust, Framework: Axum\n");

    // Step 3: Run autonomous QA on generated code
    println!("🔍 Step 3: Running autonomous QA...");
    let qa_result = run_qa(&client, &project.id).await;
    println!("   ✓ QA completed");
    println!("   Tests run: {}", qa_result.tests_run.unwrap_or(0));
    println!("   Tests passed: {}", qa_result.tests_passed.unwrap_or(0));
    println!("   Tests failed: {}", qa_result.tests_failed.unwrap_or(0));
    println!("   Success: {}\n", qa_result.success);

    // Step 4: Run refactoring analysis
    println!("🔧 Step 4: Analyzing code for refactoring opportunities...");
    let analysis = analyze_code(&client, &project.id).await;
    println!("   ✓ Analysis completed");
    println!("   Quality score: {:.1}%", analysis.quality_score);
    println!("   Issues found: {}", analysis.issues_found);
    println!("   Refactoring opportunities: {}\n", analysis.refactoring_opportunities);

    // Step 5: Apply automated refactoring
    if analysis.refactoring_opportunities > 0 {
        println!("🔨 Step 5: Applying automated refactoring...");
        let refactoring_result = apply_refactoring(&client, &project.id).await;
        println!("   ✓ Refactoring applied");
        println!("   Success: {}", refactoring_result.success);
        if let Some(changes) = refactoring_result.changes_applied {
            println!("   Changes applied: {}", changes.len());
            for (i, change) in changes.iter().take(3).enumerate() {
                println!("     {}. {}", i + 1, change);
            }
        }
        println!();
    } else {
        println!("⏭️  Step 5: Skipping refactoring (no opportunities found)\n");
    }

    // Step 6: Re-run QA after refactoring
    println!("🔍 Step 6: Re-running QA after refactoring...");
    let qa_result_after = run_qa(&client, &project.id).await;
    println!("   ✓ QA completed");
    println!("   Tests passed: {}", qa_result_after.tests_passed.unwrap_or(0));
    println!("   All tests passing: {}\n", qa_result_after.success);

    // Step 7: Deploy to staging
    println!("🚀 Step 7: Deploying to staging environment...");
    let deployment = deploy_project(&client, &project.id, "staging").await;
    println!("   ✓ Deployment initiated");
    println!("   Status: {}", deployment.status);
    println!("   Deployment URL: {}\n", deployment.deployment_url);

    // Step 8: Wait for deployment to complete
    println!("⏳ Step 8: Waiting for deployment to complete...");
    wait_for_deployment(&client, &project.id).await;
    println!("   ✓ Deployment completed successfully\n");

    // Step 9: Verify deployed application is accessible
    println!("🌐 Step 9: Verifying deployed application...");
    verify_deployment(&client, &deployment.deployment_url).await;
    println!("   ✓ Deployment is accessible and healthy\n");

    // Step 10: Cleanup
    println!("🧹 Step 10: Cleaning up test project...");
    delete_project(&client, &project.id).await;
    println!("   ✓ Test project deleted\n");

    println!("================================================");
    println!("✅ Complete E2E Workflow Test: PASSED");
    println!("   All steps completed successfully");
}

/// E2E Test: Frontend to Backend Integration
#[tokio::test]
#[ignore] // Run with: cargo test --ignored e2e_frontend_backend_integration
async fn e2e_frontend_backend_integration() {
    println!("🌐 Starting Frontend-Backend Integration Test\n");

    let client = Client::new();

    // Wait for services
    wait_for_services(&client).await;

    // Test frontend is accessible
    println!("📱 Testing frontend accessibility...");
    let frontend_response = client
        .get(FRONTEND_URL)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Frontend not accessible");

    assert!(frontend_response.status().is_success(), "Frontend returned error status");
    println!("   ✓ Frontend accessible\n");

    // Test API is accessible from frontend perspective
    println!("🔌 Testing API accessibility...");
    let api_response = client
        .get(format!("{}/api/projects", API_BASE_URL))
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("API not accessible");

    assert!(api_response.status().is_success(), "API returned error status");
    println!("   ✓ API accessible\n");

    // Test CORS headers
    println!("🔐 Testing CORS configuration...");
    let cors_response = client
        .options(format!("{}/api/projects", API_BASE_URL))
        .header("Origin", FRONTEND_URL)
        .header("Access-Control-Request-Method", "POST")
        .send()
        .await
        .expect("CORS preflight failed");

    assert!(cors_response.status().is_success(), "CORS not properly configured");
    println!("   ✓ CORS properly configured\n");

    println!("✅ Frontend-Backend Integration Test: PASSED\n");
}

/// E2E Test: QA Engine Workflow
#[tokio::test]
#[ignore] // Run with: cargo test --ignored e2e_qa_engine_workflow
async fn e2e_qa_engine_workflow() {
    println!("🔍 Starting QA Engine Workflow Test\n");

    let client = Client::new();
    wait_for_services(&client).await;

    // Create project
    let project = create_project(&client).await;
    println!("✓ Project created: {}\n", project.id);

    // Generate code with intentional bug
    println!("🐛 Generating code with intentional bug...");
    let buggy_code = r#"
    pub fn add(a: i32, b: i32) -> i32 {
        a - b  // Bug: should be a + b
    }
    "#;

    upload_code(&client, &project.id, buggy_code).await;
    println!("   ✓ Buggy code uploaded\n");

    // Run QA - should detect bug
    println!("🔍 Running QA (should detect bug)...");
    let qa_result_1 = run_qa(&client, &project.id).await;
    assert!(!qa_result_1.success, "QA should have detected bug");
    assert!(qa_result_1.tests_failed.unwrap_or(0) > 0, "Should have failing tests");
    println!("   ✓ Bug detected: {} tests failed\n", qa_result_1.tests_failed.unwrap_or(0));

    // Run autocorrection
    println!("🔧 Running autocorrection...");
    let correction_result = run_autocorrection(&client, &project.id).await;
    assert!(correction_result.success, "Autocorrection should succeed");
    println!("   ✓ Autocorrection completed\n");

    // Re-run QA - should pass
    println!("🔍 Re-running QA (should pass after correction)...");
    let qa_result_2 = run_qa(&client, &project.id).await;
    assert!(qa_result_2.success, "QA should pass after correction");
    assert_eq!(qa_result_2.tests_failed.unwrap_or(1), 0, "No tests should fail");
    println!("   ✓ All tests passing after correction\n");

    // Cleanup
    delete_project(&client, &project.id).await;

    println!("✅ QA Engine Workflow Test: PASSED\n");
}

/// E2E Test: Refactoring Engine Workflow
#[tokio::test]
#[ignore] // Run with: cargo test --ignored e2e_refactoring_workflow
async fn e2e_refactoring_workflow() {
    println!("🔨 Starting Refactoring Engine Workflow Test\n");

    let client = Client::new();
    wait_for_services(&client).await;

    // Create project
    let project = create_project(&client).await;
    println!("✓ Project created: {}\n", project.id);

    // Upload code with refactoring opportunities
    println!("📝 Uploading code with refactoring opportunities...");
    let code_with_smells = r#"
    pub fn calculate(x: i32, y: i32, z: i32) -> i32 {
        let result = x + y;
        let result2 = result * z;
        let result3 = result2 + 42; // Magic number
        result3
    }
    "#;

    upload_code(&client, &project.id, code_with_smells).await;
    println!("   ✓ Code uploaded\n");

    // Analyze for refactoring opportunities
    println!("🔍 Analyzing code...");
    let analysis = analyze_code(&client, &project.id).await;
    assert!(analysis.refactoring_opportunities > 0, "Should find refactoring opportunities");
    println!("   ✓ Found {} refactoring opportunities\n", analysis.refactoring_opportunities);

    // Apply refactorings
    println!("🔧 Applying refactorings...");
    let refactoring_result = apply_refactoring(&client, &project.id).await;
    assert!(refactoring_result.success, "Refactoring should succeed");
    println!("   ✓ Refactoring applied successfully\n");

    // Re-analyze - should have fewer opportunities
    println!("🔍 Re-analyzing code...");
    let analysis_after = analyze_code(&client, &project.id).await;
    assert!(analysis_after.quality_score > analysis.quality_score, "Quality should improve");
    println!("   ✓ Quality improved: {:.1}% -> {:.1}%\n", analysis.quality_score, analysis_after.quality_score);

    // Cleanup
    delete_project(&client, &project.id).await;

    println!("✅ Refactoring Engine Workflow Test: PASSED\n");
}

// Helper functions

async fn wait_for_services(client: &Client) {
    for attempt in 1..=30 {
        if let Ok(response) = client
            .get(format!("{}/health", API_BASE_URL))
            .timeout(Duration::from_secs(2))
            .send()
            .await
        {
            if response.status().is_success() {
                return;
            }
        }

        if attempt % 5 == 0 {
            println!("   Waiting for services... (attempt {}/30)", attempt);
        }

        sleep(Duration::from_secs(1)).await;
    }

    panic!("Services did not become ready within 30 seconds");
}

async fn create_project(client: &Client) -> Project {
    let response = client
        .post(format!("{}/api/projects", API_BASE_URL))
        .json(&json!({
            "name": "E2E Test Project",
            "description": "Automated E2E test project",
            "language": "Rust",
            "framework": "Axum",
            "repository": "https://github.com/test/e2e-project"
        }))
        .send()
        .await
        .expect("Failed to create project");

    assert!(response.status().is_success());
    response.json::<Project>().await.expect("Failed to parse project response")
}

async fn generate_code(client: &Client, project_id: &str) -> String {
    let response = client
        .post(format!("{}/api/ai/generate", API_BASE_URL))
        .json(&json!({
            "project_id": project_id,
            "requirements": "Create a simple REST API with CRUD operations",
            "language": "Rust",
            "framework": "Axum"
        }))
        .send()
        .await
        .expect("Failed to generate code");

    assert!(response.status().is_success());
    response.text().await.expect("Failed to read generated code")
}

async fn run_qa(client: &Client, project_id: &str) -> QAResult {
    let response = client
        .post(format!("{}/api/projects/{}/qa", API_BASE_URL, project_id))
        .send()
        .await
        .expect("Failed to run QA");

    assert!(response.status().is_success());
    response.json::<QAResult>().await.expect("Failed to parse QA result")
}

async fn analyze_code(client: &Client, project_id: &str) -> CodeAnalysis {
    let response = client
        .post(format!("{}/api/projects/{}/analyze", API_BASE_URL, project_id))
        .send()
        .await
        .expect("Failed to analyze code");

    assert!(response.status().is_success());
    response.json::<CodeAnalysis>().await.expect("Failed to parse analysis result")
}

async fn apply_refactoring(client: &Client, project_id: &str) -> RefactoringResult {
    let response = client
        .post(format!("{}/api/projects/{}/refactor", API_BASE_URL, project_id))
        .json(&json!({
            "operation_type": "extract_method",
            "target_file": "src/main.rs",
            "parameters": {}
        }))
        .send()
        .await
        .expect("Failed to apply refactoring");

    assert!(response.status().is_success());
    response.json::<RefactoringResult>().await.expect("Failed to parse refactoring result")
}

async fn deploy_project(client: &Client, project_id: &str, environment: &str) -> DeploymentResult {
    let response = client
        .post(format!("{}/api/projects/{}/deploy", API_BASE_URL, project_id))
        .json(&json!({
            "environment": environment
        }))
        .send()
        .await
        .expect("Failed to deploy project");

    assert!(response.status().is_success());
    response.json::<DeploymentResult>().await.expect("Failed to parse deployment result")
}

async fn wait_for_deployment(client: &Client, project_id: &str) {
    for _ in 0..60 { // Wait up to 60 seconds
        let response = client
            .get(format!("{}/api/projects/{}", API_BASE_URL, project_id))
            .send()
            .await
            .expect("Failed to check project status");

        if response.status().is_success() {
            let project: Project = response.json().await.expect("Failed to parse project");

            if project.status == "deployed" {
                return;
            }
        }

        sleep(Duration::from_secs(1)).await;
    }

    panic!("Deployment did not complete within 60 seconds");
}

async fn verify_deployment(client: &Client, deployment_url: &str) {
    let response = client
        .get(format!("{}/health", deployment_url))
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Deployed application not accessible");

    assert!(response.status().is_success(), "Deployed application is not healthy");
}

async fn delete_project(client: &Client, project_id: &str) {
    let response = client
        .delete(format!("{}/api/projects/{}", API_BASE_URL, project_id))
        .send()
        .await
        .expect("Failed to delete project");

    assert!(response.status().is_success());
}

async fn upload_code(client: &Client, project_id: &str, code: &str) {
    let response = client
        .post(format!("{}/api/projects/{}/code", API_BASE_URL, project_id))
        .json(&json!({
            "file_path": "src/lib.rs",
            "content": code
        }))
        .send()
        .await
        .expect("Failed to upload code");

    assert!(response.status().is_success());
}

async fn run_autocorrection(client: &Client, project_id: &str) -> AutocorrectionResult {
    let response = client
        .post(format!("{}/api/projects/{}/autocorrect", API_BASE_URL, project_id))
        .send()
        .await
        .expect("Failed to run autocorrection");

    assert!(response.status().is_success());
    response.json::<AutocorrectionResult>().await.expect("Failed to parse autocorrection result")
}

// Supporting types

#[derive(Debug, serde::Deserialize)]
struct CodeAnalysis {
    quality_score: f64,
    issues_found: usize,
    refactoring_opportunities: usize,
}

#[derive(Debug, serde::Deserialize)]
struct AutocorrectionResult {
    success: bool,
    iterations_completed: usize,
}
