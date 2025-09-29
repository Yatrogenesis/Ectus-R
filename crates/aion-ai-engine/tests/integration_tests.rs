//! Comprehensive Integration Tests for AION AI Engine
//! Tests all critical functionality to ensure production readiness

use aion_ai_engine::*;
use tokio::test;
use uuid::Uuid;
use std::sync::Arc;

/// Test AI Engine initialization and configuration
#[tokio::test]
async fn test_ai_engine_initialization() {
    let config = AIEngineConfig {
        max_memory: 1024 * 1024 * 1024, // 1GB for testing
        default_backend: InferenceBackend::Candle,
        cache_dir: std::path::PathBuf::from("./test_models"),
        max_concurrent_inferences: 5,
        enable_monitoring: true,
    };

    let result = initialize_ai_engine(config).await;
    assert!(result.is_ok(), "AI Engine should initialize successfully");
}

/// Test Bug Prediction System
#[tokio::test]
async fn test_bug_prediction_comprehensive() {
    println!("🐛 Testing Bug Prediction System...");

    let inference_engine = Arc::new(
        InferenceEngine::new(AIEngineConfig::default()).await
            .expect("Failed to create inference engine")
    );

    let bug_predictor = BugPredictor::new(inference_engine).await
        .expect("Failed to create bug predictor");

    // Test with sample Rust code containing common bugs
    let test_code = GeneratedCode {
        generation_id: Uuid::new_v4(),
        files: vec![code_generation::CodeFile {
            path: "test.rs".to_string(),
            content: r#"
                fn main() {
                    let mut v = Vec::new();
                    v.push(1);
                    println!("{}", v[10]); // Index out of bounds

                    let x = Box::new(5);
                    let y = *x;
                    drop(x);
                    println!("{}", y); // Use after free
                }
            "#.to_string(),
            language: "rust".to_string(),
            file_type: code_generation::FileType::Source,
        }],
        metadata: code_generation::GenerationMetadata {
            prompt: "Test code".to_string(),
            language: "rust".to_string(),
            framework: None,
            generation_time_ms: 100,
            confidence_score: 1.0,
            complexity_score: 3.0,
            lines_of_code: 10,
            estimated_runtime_performance: code_generation::PerformanceMetrics::default(),
        },
        deployment_instructions: None,
    };

    let predictions = bug_predictor.predict_bugs(&test_code).await
        .expect("Bug prediction should work");

    assert!(!predictions.is_empty(), "Should detect bugs in problematic code");

    // Test auto-correction
    let critical_bugs: Vec<_> = predictions.iter()
        .filter(|p| p.severity == bug_prediction::BugSeverity::Critical)
        .collect();

    if !critical_bugs.is_empty() {
        let fixed_code = bug_predictor.apply_auto_fixes(&test_code, critical_bugs).await
            .expect("Auto-fixing should work");

        assert_ne!(fixed_code.files[0].content, test_code.files[0].content,
                  "Code should be modified after fixing");
    }

    println!("✅ Bug Prediction System: PASSED");
}

/// Test Vulnerability Scanner
#[tokio::test]
async fn test_vulnerability_scanner() {
    println!("🛡️ Testing Vulnerability Scanner...");

    let scanner = VulnerabilityScanner::new().await
        .expect("Failed to create vulnerability scanner");

    // Test with code containing security vulnerabilities
    let vulnerable_code = GeneratedCode {
        generation_id: Uuid::new_v4(),
        files: vec![code_generation::CodeFile {
            path: "vulnerable.rs".to_string(),
            content: r#"
                use std::process::Command;

                fn execute_user_input(input: &str) {
                    // SQL Injection vulnerability
                    let query = format!("SELECT * FROM users WHERE name = '{}'", input);

                    // Command injection vulnerability
                    Command::new("sh")
                        .arg("-c")
                        .arg(format!("echo {}", input))
                        .spawn()
                        .expect("Failed to execute command");
                }
            "#.to_string(),
            language: "rust".to_string(),
            file_type: code_generation::FileType::Source,
        }],
        metadata: code_generation::GenerationMetadata::default(),
        deployment_instructions: None,
    };

    let vulnerabilities = scanner.scan_code(&vulnerable_code).await
        .expect("Vulnerability scanning should work");

    assert!(!vulnerabilities.is_empty(), "Should detect security vulnerabilities");

    // Verify we detect injection vulnerabilities
    let has_injection_vuln = vulnerabilities.iter()
        .any(|v| v.vulnerability_type.to_lowercase().contains("injection"));

    assert!(has_injection_vuln, "Should detect injection vulnerabilities");

    println!("✅ Vulnerability Scanner: PASSED");
}

/// Test Documentation Generator
#[tokio::test]
async fn test_documentation_generator() {
    println!("📚 Testing Documentation Generator...");

    let inference_engine = Arc::new(
        InferenceEngine::new(AIEngineConfig::default()).await
            .expect("Failed to create inference engine")
    );

    let doc_generator = DocumentationGenerator::new(inference_engine).await
        .expect("Failed to create documentation generator");

    let sample_code = GeneratedCode {
        generation_id: Uuid::new_v4(),
        files: vec![code_generation::CodeFile {
            path: "api.rs".to_string(),
            content: r#"
                /// User management API
                pub struct UserAPI {
                    database: Database,
                }

                impl UserAPI {
                    /// Create a new user
                    pub async fn create_user(&self, user: CreateUserRequest) -> Result<User, Error> {
                        // Implementation here
                        todo!()
                    }

                    /// Get user by ID
                    pub async fn get_user(&self, id: u64) -> Result<User, Error> {
                        // Implementation here
                        todo!()
                    }
                }
            "#.to_string(),
            language: "rust".to_string(),
            file_type: code_generation::FileType::Source,
        }],
        metadata: code_generation::GenerationMetadata::default(),
        deployment_instructions: None,
    };

    let options = documentation_generator::DocumentationOptions {
        document_types: vec![
            documentation_generator::DocumentType::APIReference,
            documentation_generator::DocumentType::UserGuide,
        ],
        output_format: documentation_generator::OutputFormat::Markdown,
        include_examples: true,
        include_diagrams: true,
        target_audience: documentation_generator::TargetAudience::Developers,
        language: "en".to_string(),
    };

    let documentation = doc_generator.generate_documentation(&sample_code, options).await
        .expect("Documentation generation should work");

    assert!(!documentation.documents.is_empty(), "Should generate documentation");
    assert!(documentation.documents.len() >= 2, "Should generate multiple document types");

    println!("✅ Documentation Generator: PASSED");
}

/// Test Autonomous QA System
#[tokio::test]
async fn test_autonomous_qa() {
    println!("🧪 Testing Autonomous QA System...");

    // Initialize all required components
    let inference_engine = Arc::new(
        InferenceEngine::new(AIEngineConfig::default()).await
            .expect("Failed to create inference engine")
    );

    let performance_monitor = Arc::new(PerformanceMonitor::new());

    let code_generator = Arc::new(
        CodeGenerator::new(inference_engine.clone(), performance_monitor).await
            .expect("Failed to create code generator")
    );

    let bug_predictor = Arc::new(
        BugPredictor::new(inference_engine.clone()).await
            .expect("Failed to create bug predictor")
    );

    let refactoring_engine = Arc::new(
        RefactoringEngine::new(inference_engine, code_generator.clone()).await
            .expect("Failed to create refactoring engine")
    );

    let qa_engine = AutonomousQA::new(
        code_generator,
        bug_predictor,
        refactoring_engine,
    ).await.expect("Failed to create QA engine");

    let test_code = GeneratedCode {
        generation_id: Uuid::new_v4(),
        files: vec![code_generation::CodeFile {
            path: "calculator.rs".to_string(),
            content: r#"
                pub fn add(a: i32, b: i32) -> i32 {
                    a + b
                }

                pub fn divide(a: i32, b: i32) -> i32 {
                    a / b  // Potential division by zero
                }
            "#.to_string(),
            language: "rust".to_string(),
            file_type: code_generation::FileType::Source,
        }],
        metadata: code_generation::GenerationMetadata::default(),
        deployment_instructions: None,
    };

    let qa_result = qa_engine.run_autonomous_qa(&test_code).await
        .expect("Autonomous QA should work");

    assert!(qa_result.tests_executed > 0, "Should execute tests");
    assert!(qa_result.overall_quality_score >= 0.0, "Should calculate quality score");

    println!("✅ Autonomous QA System: PASSED");
}

/// Performance benchmark test
#[tokio::test]
async fn test_performance_benchmarks() {
    println!("⚡ Running Performance Benchmarks...");

    let start_time = std::time::Instant::now();

    // Initialize AI engine
    let config = AIEngineConfig {
        max_memory: 512 * 1024 * 1024, // 512MB
        default_backend: InferenceBackend::Candle,
        cache_dir: std::path::PathBuf::from("./bench_models"),
        max_concurrent_inferences: 3,
        enable_monitoring: true,
    };

    let init_result = initialize_ai_engine(config).await;
    assert!(init_result.is_ok(), "AI Engine should initialize quickly");

    let init_time = start_time.elapsed();
    assert!(init_time.as_secs() < 30, "Initialization should complete within 30 seconds");

    println!("✅ Performance Benchmarks: PASSED (Init: {}ms)", init_time.as_millis());
}

/// Integration test for all systems working together
#[tokio::test]
async fn test_full_system_integration() {
    println!("🔄 Testing Full System Integration...");

    // This test ensures all components work together seamlessly
    let config = AIEngineConfig::default();

    // Initialize AI engine
    initialize_ai_engine(config.clone()).await
        .expect("AI Engine initialization failed");

    // Create all components
    let inference_engine = Arc::new(
        InferenceEngine::new(config).await
            .expect("Failed to create inference engine")
    );

    let performance_monitor = Arc::new(PerformanceMonitor::new());

    let code_generator = Arc::new(
        CodeGenerator::new(inference_engine.clone(), performance_monitor).await
            .expect("Failed to create code generator")
    );

    let bug_predictor = Arc::new(
        BugPredictor::new(inference_engine.clone()).await
            .expect("Failed to create bug predictor")
    );

    let vulnerability_scanner = Arc::new(
        VulnerabilityScanner::new().await
            .expect("Failed to create vulnerability scanner")
    );

    // Test complete workflow: Generate -> Analyze -> Fix -> Document
    let generation_request = code_generation::GenerationRequest {
        prompt: "Create a simple REST API for user management".to_string(),
        language: Some("rust".to_string()),
        framework: Some("axum".to_string()),
        requirements: vec!["authentication".to_string(), "CRUD operations".to_string()],
        constraints: vec!["no unsafe code".to_string()],
        optimization_level: code_generation::OptimizationLevel::Balanced,
        include_tests: true,
        include_docs: true,
        include_ci: false,
    };

    // Step 1: Generate code
    let generated_code = code_generator.generate_code(generation_request).await
        .expect("Code generation should work");

    assert!(!generated_code.files.is_empty(), "Should generate files");

    // Step 2: Predict bugs
    let bugs = bug_predictor.predict_bugs(&generated_code).await
        .expect("Bug prediction should work");

    // Step 3: Scan for vulnerabilities
    let vulnerabilities = vulnerability_scanner.scan_code(&generated_code).await
        .expect("Vulnerability scanning should work");

    println!("Generated {} files, found {} potential bugs, {} vulnerabilities",
             generated_code.files.len(), bugs.len(), vulnerabilities.len());

    println!("✅ Full System Integration: PASSED");
}

/// Test error handling and edge cases
#[tokio::test]
async fn test_error_handling() {
    println!("🚨 Testing Error Handling...");

    // Test with invalid configuration
    let invalid_config = AIEngineConfig {
        max_memory: 0, // Invalid memory limit
        default_backend: InferenceBackend::Candle,
        cache_dir: std::path::PathBuf::from("/invalid/path/that/does/not/exist"),
        max_concurrent_inferences: 0, // Invalid concurrency
        enable_monitoring: true,
    };

    // This should handle the error gracefully
    let result = initialize_ai_engine(invalid_config).await;
    // We expect this to either work with fallbacks or fail gracefully
    if result.is_err() {
        println!("Expected error handled gracefully: {:?}", result.err());
    }

    println!("✅ Error Handling: PASSED");
}

#[tokio::test]
async fn test_concurrent_operations() {
    println!("🔄 Testing Concurrent Operations...");

    let config = AIEngineConfig {
        max_memory: 1024 * 1024 * 1024,
        default_backend: InferenceBackend::Candle,
        cache_dir: std::path::PathBuf::from("./concurrent_test_models"),
        max_concurrent_inferences: 10,
        enable_monitoring: true,
    };

    initialize_ai_engine(config.clone()).await
        .expect("AI Engine initialization failed");

    // Create multiple tasks running concurrently
    let mut handles = Vec::new();

    for i in 0..5 {
        let config_clone = config.clone();
        let handle = tokio::spawn(async move {
            let inference_engine = Arc::new(
                InferenceEngine::new(config_clone).await
                    .expect("Failed to create inference engine")
            );

            let bug_predictor = BugPredictor::new(inference_engine).await
                .expect("Failed to create bug predictor");

            let test_code = GeneratedCode {
                generation_id: Uuid::new_v4(),
                files: vec![code_generation::CodeFile {
                    path: format!("test_{}.rs", i),
                    content: "fn main() { println!(); }".to_string(),
                    language: "rust".to_string(),
                    file_type: code_generation::FileType::Source,
                }],
                metadata: code_generation::GenerationMetadata::default(),
                deployment_instructions: None,
            };

            bug_predictor.predict_bugs(&test_code).await
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await.expect("Task should complete");
        assert!(result.is_ok(), "Concurrent operation should succeed");
    }

    println!("✅ Concurrent Operations: PASSED");
}