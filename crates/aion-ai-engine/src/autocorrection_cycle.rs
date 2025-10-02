// AION-R Autocorrection Cycle: Complete self-healing implementation
// Integrates test execution with fix generation and validation
// NOW WITH REAL LLM INTEGRATION (Groq, OpenAI, HuggingFace, GitHub, Cloudflare)

use std::path::Path;
use anyhow::{Result, Context, anyhow};
use tracing::{info, warn, error, debug};
use serde::{Serialize, Deserialize};

use crate::test_integration::{TestIntegrationEngine, DetailedTestResults, TestFailure};
use crate::code_generation::GeneratedCode;
use crate::llm_providers::{MultiProviderLLM, LLMRequest, LLMClient, GroqClient, OpenAIClient, HuggingFaceClient, GitHubModelsClient, CloudflareAIClient};
use crate::locked_files::LockedFilesManager;

/// Maximum iterations before giving up
const MAX_AUTOCORRECTION_ITERATIONS: u32 = 5;

/// Minimum improvement threshold to continue (percentage)
const MIN_IMPROVEMENT_THRESHOLD: f64 = 5.0;

/// Autocorrection cycle manager with real LLM integration
pub struct AutocorrectionCycle {
    test_engine: TestIntegrationEngine,
    max_iterations: u32,
    llm: MultiProviderLLM,
    locked_files: LockedFilesManager,
}

/// Result of autocorrection attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocorrectionResult {
    pub success: bool,
    pub iterations_completed: u32,
    pub final_test_results: DetailedTestResults,
    pub corrections_applied: Vec<CorrectionAttempt>,
    pub convergence_achieved: bool,
    pub final_code: String,
}

/// Single correction attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionAttempt {
    pub iteration: u32,
    pub failures_before: usize,
    pub failures_after: usize,
    pub improvement_percentage: f64,
    pub fixes_applied: Vec<FixDescription>,
    pub success: bool,
}

/// Description of a fix applied
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixDescription {
    pub failure_type: String,
    pub fix_strategy: String,
    pub code_change: String,
    pub llm_provider_used: Option<String>,
}

impl AutocorrectionCycle {
    /// Create new autocorrection cycle with LLM providers from environment
    pub fn new() -> Result<Self> {
        let mut llm = MultiProviderLLM::new();

        // Try to initialize all available providers from environment variables
        if let Ok(groq_key) = std::env::var("GROQ_API_KEY") {
            info!("🚀 Initializing Groq LLM client");
            llm.add_provider(Box::new(GroqClient::new(groq_key)?));
        }

        if let Ok(openai_key) = std::env::var("OPENAI_API_KEY") {
            info!("🚀 Initializing OpenAI LLM client");
            llm.add_provider(Box::new(OpenAIClient::new(openai_key)?));
        }

        if let Ok(hf_key) = std::env::var("HUGGINGFACE_API_KEY") {
            info!("🚀 Initializing Hugging Face LLM client");
            llm.add_provider(Box::new(HuggingFaceClient::new(hf_key)?));
        }

        if let Ok(github_key) = std::env::var("GITHUB_TOKEN") {
            info!("🚀 Initializing GitHub Models LLM client");
            llm.add_provider(Box::new(GitHubModelsClient::new(github_key)?));
        }

        if let (Ok(cf_key), Ok(cf_account)) = (std::env::var("CLOUDFLARE_API_KEY"), std::env::var("CLOUDFLARE_ACCOUNT_ID")) {
            info!("🚀 Initializing Cloudflare AI client");
            llm.add_provider(Box::new(CloudflareAIClient::new(cf_key, cf_account)?));
        }

        let available = llm.available_providers();
        if available.is_empty() {
            warn!("⚠️  No LLM providers configured. Set API keys in environment variables.");
            warn!("   Supported: GROQ_API_KEY, OPENAI_API_KEY, HUGGINGFACE_API_KEY, GITHUB_TOKEN, CLOUDFLARE_API_KEY");
        } else {
            info!("✅ Available LLM providers: {:?}", available);
        }

        let project_root = std::env::current_dir()?;
        let mut locked_files = LockedFilesManager::new(&project_root);

        // Load locked files from config
        if let Err(e) = locked_files.load_from_config() {
            warn!("Failed to load locked files config: {}", e);
        }

        // Scan for git modifications
        if let Err(e) = locked_files.scan_git_modifications() {
            warn!("Failed to scan git modifications: {}", e);
        }

        Ok(Self {
            test_engine: TestIntegrationEngine::new(),
            max_iterations: MAX_AUTOCORRECTION_ITERATIONS,
            llm,
            locked_files,
        })
    }

    /// Run complete autocorrection cycle with real LLM-powered fixes
    pub async fn run_autocorrection(
        &self,
        project_path: &Path,
        language: &str,
        mut code: GeneratedCode,
    ) -> Result<AutocorrectionResult> {
        info!("🔄 Starting autocorrection cycle for project: {:?}", project_path);

        let mut corrections = Vec::new();
        let mut previous_failures = usize::MAX;

        for iteration in 1..=self.max_iterations {
            info!("📊 Autocorrection iteration {}/{}", iteration, self.max_iterations);

            // Step 1: Execute tests and get detailed results
            let test_results = self.test_engine
                .execute_and_parse_tests(project_path, language)
                .await
                .context("Failed to execute tests")?;

            let current_failures = test_results.failures.len();

            info!("   Tests: {} passed, {} failed",
                test_results.tests_passed,
                current_failures
            );

            // Step 2: Check if all tests pass
            if test_results.all_passed {
                info!("✅ All tests passed! Autocorrection successful.");

                corrections.push(CorrectionAttempt {
                    iteration,
                    failures_before: previous_failures,
                    failures_after: 0,
                    improvement_percentage: 100.0,
                    fixes_applied: vec![],
                    success: true,
                });

                return Ok(AutocorrectionResult {
                    success: true,
                    iterations_completed: iteration,
                    final_test_results: test_results,
                    corrections_applied: corrections,
                    convergence_achieved: true,
                    final_code: code.code.clone(),
                });
            }

            // Step 3: Check for convergence (diminishing returns)
            if current_failures >= previous_failures && iteration > 1 {
                warn!("⚠️  No improvement detected. Convergence check...");

                let improvement = if previous_failures > 0 {
                    ((previous_failures - current_failures) as f64 / previous_failures as f64) * 100.0
                } else {
                    0.0
                };

                if improvement < MIN_IMPROVEMENT_THRESHOLD {
                    warn!("❌ Convergence threshold not met. Stopping autocorrection.");

                    return Ok(AutocorrectionResult {
                        success: false,
                        iterations_completed: iteration,
                        final_test_results: test_results,
                        corrections_applied: corrections,
                        convergence_achieved: false,
                        final_code: code.code.clone(),
                    });
                }
            }

            // Step 4: Generate fixes using LLM
            info!("🔧 Generating fixes for {} failures using LLM...", current_failures);

            let fixes = self.generate_fixes_with_llm(
                &test_results.failures,
                &code,
                language,
            ).await?;

            // Step 5: Apply fixes to code
            let (updated_code, applied_fixes) = self.apply_fixes_to_code(
                code.clone(),
                fixes,
            ).await?;

            info!("   Applied {} fixes", applied_fixes.len());

            // Step 6: Write updated code to project
            self.write_code_to_project(project_path, &updated_code, language).await?;

            // Track correction attempt
            let improvement = if previous_failures > 0 {
                ((previous_failures - current_failures) as f64 / previous_failures as f64) * 100.0
            } else {
                0.0
            };

            corrections.push(CorrectionAttempt {
                iteration,
                failures_before: previous_failures,
                failures_after: current_failures,
                improvement_percentage: improvement,
                fixes_applied: applied_fixes,
                success: current_failures < previous_failures,
            });

            previous_failures = current_failures;
            code = updated_code;
        }

        // Maximum iterations reached
        warn!("⚠️  Maximum iterations ({}) reached", self.max_iterations);

        // Final test run
        let final_results = self.test_engine
            .execute_and_parse_tests(project_path, language)
            .await
            .context("Failed to execute final tests")?;

        Ok(AutocorrectionResult {
            success: final_results.all_passed,
            iterations_completed: self.max_iterations,
            final_test_results: final_results,
            corrections_applied: corrections,
            convergence_achieved: false,
            final_code: code.code,
        })
    }

    /// Generate fixes using real LLM (with fallback between providers)
    async fn generate_fixes_with_llm(
        &self,
        failures: &[TestFailure],
        code: &GeneratedCode,
        language: &str,
    ) -> Result<Vec<FixDescription>> {
        let mut fixes = Vec::new();

        for (idx, failure) in failures.iter().enumerate().take(5) { // Limit to 5 failures per iteration
            debug!("Analyzing failure {}: {}", idx + 1, failure.test_name);

            let system_prompt = format!(
                "You are an expert {} developer and debugger. \
                Your task is to analyze test failures and provide precise code fixes. \
                Return ONLY the corrected code, no explanations.",
                language
            );

            let user_prompt = format!(
                "Test Failure:\n\
                Test: {}\n\
                Error: {}\n\
                {}{}
                \n\n\
                Current Code:\n\
                ```{}\n\
                {}\n\
                ```\n\
                \n\
                Provide the corrected code that will make this test pass. \
                Return ONLY the fixed code, no markdown, no explanations.",
                failure.test_name,
                failure.failure_message,
                failure.file_path.as_ref().map(|p| format!("File: {}\n", p)).unwrap_or_default(),
                failure.line_number.map(|l| format!("Line: {}\n", l)).unwrap_or_default(),
                language,
                code.code
            );

            let request = LLMRequest {
                prompt: user_prompt,
                system_prompt: Some(system_prompt),
                max_tokens: Some(2000),
                temperature: Some(0.2), // Low temperature for precise fixes
                model: None,
            };

            match self.llm.generate_with_fallback(&request).await {
                Ok(response) => {
                    info!("✅ Generated fix using {:?}", response.provider);

                    fixes.push(FixDescription {
                        failure_type: failure.test_name.clone(),
                        fix_strategy: format!("LLM-generated fix via {:?}", response.provider),
                        code_change: response.content.clone(),
                        llm_provider_used: Some(format!("{:?}", response.provider)),
                    });
                }
                Err(e) => {
                    warn!("⚠️  Failed to generate fix for '{}': {}", failure.test_name, e);

                    // Fallback to heuristic fix
                    fixes.push(self.generate_heuristic_fix(failure, code, language));
                }
            }
        }

        Ok(fixes)
    }

    /// Fallback heuristic fix when LLM is unavailable
    fn generate_heuristic_fix(
        &self,
        failure: &TestFailure,
        _code: &GeneratedCode,
        language: &str,
    ) -> FixDescription {
        // Simple heuristic patterns
        let fix_code = if failure.failure_message.contains("assertion") {
            format!("// Heuristic fix for assertion failure in {}", language)
        } else if failure.failure_message.contains("null") || failure.failure_message.contains("None") {
            format!("// Heuristic fix for null/None error in {}", language)
        } else if failure.failure_message.contains("type") {
            format!("// Heuristic fix for type error in {}", language)
        } else {
            format!("// Generic heuristic fix for {}", language)
        };

        FixDescription {
            failure_type: failure.test_name.clone(),
            fix_strategy: "Heuristic fallback".to_string(),
            code_change: fix_code,
            llm_provider_used: None,
        }
    }

    /// Apply fixes to code
    async fn apply_fixes_to_code(
        &self,
        mut code: GeneratedCode,
        fixes: Vec<FixDescription>,
    ) -> Result<(GeneratedCode, Vec<FixDescription>)> {
        let mut applied_fixes = Vec::new();

        for fix in fixes {
            // For now, replace entire code with fixed version
            // In production, would do smarter merging
            if !fix.code_change.is_empty() && fix.llm_provider_used.is_some() {
                code.code = fix.code_change.clone();
                applied_fixes.push(fix);
            }
        }

        Ok((code, applied_fixes))
    }

    /// Write code to project filesystem (respects locked files)
    async fn write_code_to_project(
        &self,
        project_path: &Path,
        code: &GeneratedCode,
        language: &str,
    ) -> Result<()> {
        let file_extension = match language.to_lowercase().as_str() {
            "rust" => "rs",
            "typescript" | "javascript" => "ts",
            "python" => "py",
            "go" => "go",
            _ => "txt",
        };

        let main_file = project_path.join(format!("src/main.{}", file_extension));

        // Check if file is locked
        if self.locked_files.is_locked(&main_file) {
            warn!("🔒 File is locked: {:?}", main_file);
            warn!("   Creating suggestion file instead...");

            // Create suggestion file instead of overwriting
            let suggestion_path = self.locked_files.create_suggestion_file(&main_file, &code.code)?;

            info!("💡 Suggestion written to: {:?}", suggestion_path);
            info!("   Review and merge manually if needed");

            return Ok(());
        }

        tokio::fs::create_dir_all(main_file.parent().unwrap()).await?;
        tokio::fs::write(&main_file, &code.code).await
            .context(format!("Failed to write code to {:?}", main_file))?;

        debug!("✅ Wrote updated code to {:?}", main_file);
        Ok(())
    }
}

impl Default for AutocorrectionCycle {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            warn!("Failed to initialize autocorrection cycle with LLM: {}", e);
            warn!("Continuing without LLM support (heuristic mode only)");

            let project_root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let locked_files = LockedFilesManager::new(&project_root);

            Self {
                test_engine: TestIntegrationEngine::new(),
                max_iterations: MAX_AUTOCORRECTION_ITERATIONS,
                llm: MultiProviderLLM::new(),
                locked_files,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    #[ignore] // Requires LLM API keys
    async fn test_autocorrection_with_llm() {
        let cycle = AutocorrectionCycle::new().unwrap();

        let buggy_code = GeneratedCode {
            language: "rust".to_string(),
            framework: Some("none".to_string()),
            code: r#"
                pub fn add(a: i32, b: i32) -> i32 {
                    a - b  // BUG: should be +
                }
            "#.to_string(),
            files: vec![],
            tests: vec![],
        };

        let project_path = PathBuf::from("/tmp/test_project");

        let result = cycle.run_autocorrection(
            &project_path,
            "rust",
            buggy_code,
        ).await;

        assert!(result.is_ok());
        let result = result.unwrap();

        if cycle.llm.available_providers().is_empty() {
            println!("Skipping LLM test - no providers configured");
        } else {
            assert!(result.iterations_completed > 0);
            assert!(!result.corrections_applied.is_empty());
        }
    }
}
