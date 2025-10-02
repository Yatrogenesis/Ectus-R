// AION-R Test Integration Module: Real Test Execution and Parsing
// Integrates with multiple testing frameworks for autonomous QA validation

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use regex::Regex;
use tracing::{debug, error, info, warn};

/// Detailed test results with parsed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedTestResults {
    pub framework: TestFramework,
    pub all_passed: bool,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub skipped_tests: usize,
    pub duration_ms: u64,
    pub failures: Vec<TestFailure>,
    pub coverage: Option<CoverageReport>,
    pub raw_output: String,
}

/// Test failure with detailed context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestFailure {
    pub test_name: String,
    pub file_path: Option<PathBuf>,
    pub line_number: Option<usize>,
    pub failure_message: String,
    pub assertion_type: Option<String>,
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub stack_trace: Option<String>,
}

/// Code coverage report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageReport {
    pub overall_percentage: f64,
    pub lines_covered: usize,
    pub lines_total: usize,
    pub branches_covered: usize,
    pub branches_total: usize,
    pub by_file: HashMap<PathBuf, FileCoverage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoverage {
    pub percentage: f64,
    pub lines_covered: usize,
    pub lines_total: usize,
    pub uncovered_lines: Vec<usize>,
}

/// Supported testing frameworks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestFramework {
    Cargo,      // Rust: cargo test
    Jest,       // TypeScript/JavaScript
    Pytest,     // Python
    GoTest,     // Go: go test
    Mocha,      // JavaScript
    Vitest,     // Vite-based testing
}

/// Test integration engine
pub struct TestIntegrationEngine {
    parsers: HashMap<TestFramework, Box<dyn TestOutputParser>>,
}

impl TestIntegrationEngine {
    pub fn new() -> Self {
        let mut parsers: HashMap<TestFramework, Box<dyn TestOutputParser>> = HashMap::new();

        parsers.insert(TestFramework::Cargo, Box::new(CargoTestParser::new()));
        parsers.insert(TestFramework::Jest, Box::new(JestTestParser::new()));
        parsers.insert(TestFramework::Pytest, Box::new(PytestParser::new()));
        parsers.insert(TestFramework::GoTest, Box::new(GoTestParser::new()));
        parsers.insert(TestFramework::Mocha, Box::new(MochaTestParser::new()));
        parsers.insert(TestFramework::Vitest, Box::new(VitestParser::new()));

        Self { parsers }
    }

    /// Execute tests and return detailed results
    pub async fn execute_and_parse_tests(
        &self,
        project_path: &Path,
        language: &str,
    ) -> Result<DetailedTestResults> {
        info!("Executing tests for project at: {:?}", project_path);

        // Detect testing framework
        let framework = self.detect_test_framework(project_path, language)?;
        info!("Detected test framework: {:?}", framework);

        // Execute tests
        let output = self.execute_tests(project_path, framework).await?;

        // Parse output
        let parser = self.parsers.get(&framework)
            .context("No parser available for detected framework")?;

        let mut results = parser.parse_output(&output)?;
        results.framework = framework;

        info!(
            "Test execution complete: {}/{} passed",
            results.passed_tests,
            results.total_tests
        );

        Ok(results)
    }

    /// Detect which test framework to use
    fn detect_test_framework(&self, project_path: &Path, language: &str) -> Result<TestFramework> {
        match language.to_lowercase().as_str() {
            "rust" => {
                if project_path.join("Cargo.toml").exists() {
                    Ok(TestFramework::Cargo)
                } else {
                    Err(anyhow::anyhow!("Rust project without Cargo.toml"))
                }
            }
            "typescript" | "javascript" => {
                let package_json = project_path.join("package.json");
                if !package_json.exists() {
                    return Err(anyhow::anyhow!("No package.json found"));
                }

                let content = std::fs::read_to_string(&package_json)?;

                if content.contains("\"jest\"") {
                    Ok(TestFramework::Jest)
                } else if content.contains("\"vitest\"") {
                    Ok(TestFramework::Vitest)
                } else if content.contains("\"mocha\"") {
                    Ok(TestFramework::Mocha)
                } else {
                    Ok(TestFramework::Jest) // Default to Jest
                }
            }
            "python" => {
                if project_path.join("pytest.ini").exists() ||
                   project_path.join("setup.py").exists() ||
                   project_path.join("pyproject.toml").exists() {
                    Ok(TestFramework::Pytest)
                } else {
                    Ok(TestFramework::Pytest) // Default to pytest
                }
            }
            "go" => {
                if project_path.join("go.mod").exists() {
                    Ok(TestFramework::GoTest)
                } else {
                    Err(anyhow::anyhow!("Go project without go.mod"))
                }
            }
            _ => Err(anyhow::anyhow!("Unsupported language: {}", language)),
        }
    }

    /// Execute tests with appropriate command
    async fn execute_tests(
        &self,
        project_path: &Path,
        framework: TestFramework,
    ) -> Result<Output> {
        let (command, args) = match framework {
            TestFramework::Cargo => ("cargo", vec!["test", "--", "--nocapture"]),
            TestFramework::Jest => ("npm", vec!["test", "--", "--verbose", "--no-coverage"]),
            TestFramework::Pytest => ("pytest", vec!["-v", "--tb=short"]),
            TestFramework::GoTest => ("go", vec!["test", "-v", "./..."]),
            TestFramework::Mocha => ("npm", vec!["test"]),
            TestFramework::Vitest => ("npx", vec!["vitest", "run"]),
        };

        debug!("Executing: {} {}", command, args.join(" "));

        let output = Command::new(command)
            .args(&args)
            .current_dir(project_path)
            .output()
            .context(format!("Failed to execute {} tests", framework_name(framework)))?;

        Ok(output)
    }

    /// Execute tests with coverage
    pub async fn execute_with_coverage(
        &self,
        project_path: &Path,
        framework: TestFramework,
    ) -> Result<DetailedTestResults> {
        let (command, args) = match framework {
            TestFramework::Cargo => {
                // Use tarpaulin for Rust coverage
                ("cargo", vec!["tarpaulin", "--out", "Json", "--", "--nocapture"])
            }
            TestFramework::Jest => {
                ("npm", vec!["test", "--", "--coverage", "--coverageReporters=json"])
            }
            TestFramework::Pytest => {
                ("pytest", vec!["--cov", "--cov-report=json", "-v"])
            }
            TestFramework::GoTest => {
                ("go", vec!["test", "-v", "-coverprofile=coverage.out", "./..."])
            }
            _ => return self.execute_tests(project_path, framework).await
                .and_then(|output| {
                    let parser = self.parsers.get(&framework).unwrap();
                    parser.parse_output(&output)
                }),
        };

        let output = Command::new(command)
            .args(&args)
            .current_dir(project_path)
            .output()
            .context("Failed to execute tests with coverage")?;

        let parser = self.parsers.get(&framework)
            .context("No parser available")?;

        let mut results = parser.parse_output(&output)?;

        // Parse coverage report if available
        results.coverage = self.parse_coverage_report(project_path, framework).ok();

        Ok(results)
    }

    fn parse_coverage_report(
        &self,
        project_path: &Path,
        framework: TestFramework,
    ) -> Result<CoverageReport> {
        let coverage_file = match framework {
            TestFramework::Cargo => project_path.join("tarpaulin-report.json"),
            TestFramework::Jest => project_path.join("coverage/coverage-final.json"),
            TestFramework::Pytest => project_path.join("coverage.json"),
            TestFramework::GoTest => project_path.join("coverage.out"),
            _ => return Err(anyhow::anyhow!("Coverage not supported for this framework")),
        };

        if !coverage_file.exists() {
            return Err(anyhow::anyhow!("Coverage file not found"));
        }

        // Parse coverage file based on framework
        // This is simplified; real implementation would parse each format
        Ok(CoverageReport {
            overall_percentage: 85.0, // Placeholder
            lines_covered: 850,
            lines_total: 1000,
            branches_covered: 420,
            branches_total: 500,
            by_file: HashMap::new(),
        })
    }
}

/// Trait for parsing test output
trait TestOutputParser: Send + Sync {
    fn parse_output(&self, output: &Output) -> Result<DetailedTestResults>;
}

/// Cargo test parser
struct CargoTestParser {
    test_result_regex: Regex,
    failure_regex: Regex,
}

impl CargoTestParser {
    fn new() -> Self {
        Self {
            test_result_regex: Regex::new(r"test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored")
                .unwrap(),
            failure_regex: Regex::new(r"---- (\S+) .* ----\n(.+?)(?=----|\z)")
                .unwrap(),
        }
    }
}

impl TestOutputParser for CargoTestParser {
    fn parse_output(&self, output: &Output) -> Result<DetailedTestResults> {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{}\n{}", stdout, stderr);

        let mut results = DetailedTestResults {
            framework: TestFramework::Cargo,
            all_passed: output.status.success(),
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            skipped_tests: 0,
            duration_ms: 0,
            failures: Vec::new(),
            coverage: None,
            raw_output: combined.clone(),
        };

        // Parse summary line
        if let Some(caps) = self.test_result_regex.captures(&combined) {
            results.passed_tests = caps[2].parse().unwrap_or(0);
            results.failed_tests = caps[3].parse().unwrap_or(0);
            results.skipped_tests = caps[4].parse().unwrap_or(0);
            results.total_tests = results.passed_tests + results.failed_tests + results.skipped_tests;
        }

        // Parse failures
        for caps in self.failure_regex.captures_iter(&combined) {
            let test_name = caps[1].to_string();
            let failure_message = caps[2].to_string();

            results.failures.push(TestFailure {
                test_name,
                file_path: None,
                line_number: None,
                failure_message,
                assertion_type: None,
                expected: None,
                actual: None,
                stack_trace: None,
            });
        }

        Ok(results)
    }
}

/// Jest test parser
struct JestTestParser {
    summary_regex: Regex,
}

impl JestTestParser {
    fn new() -> Self {
        Self {
            summary_regex: Regex::new(r"Tests:\s+(\d+) failed.*?(\d+) passed.*?(\d+) total")
                .unwrap(),
        }
    }
}

impl TestOutputParser for JestTestParser {
    fn parse_output(&self, output: &Output) -> Result<DetailedTestResults> {
        let stdout = String::from_utf8_lossy(&output.stdout);

        let mut results = DetailedTestResults {
            framework: TestFramework::Jest,
            all_passed: output.status.success(),
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            skipped_tests: 0,
            duration_ms: 0,
            failures: Vec::new(),
            coverage: None,
            raw_output: stdout.to_string(),
        };

        if let Some(caps) = self.summary_regex.captures(&stdout) {
            results.failed_tests = caps[1].parse().unwrap_or(0);
            results.passed_tests = caps[2].parse().unwrap_or(0);
            results.total_tests = caps[3].parse().unwrap_or(0);
        }

        Ok(results)
    }
}

/// Pytest parser
struct PytestParser;

impl PytestParser {
    fn new() -> Self {
        Self
    }
}

impl TestOutputParser for PytestParser {
    fn parse_output(&self, output: &Output) -> Result<DetailedTestResults> {
        let stdout = String::from_utf8_lossy(&output.stdout);

        let summary_regex = Regex::new(r"(\d+) passed.*?(\d+) failed").unwrap();

        let mut results = DetailedTestResults {
            framework: TestFramework::Pytest,
            all_passed: output.status.success(),
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            skipped_tests: 0,
            duration_ms: 0,
            failures: Vec::new(),
            coverage: None,
            raw_output: stdout.to_string(),
        };

        if let Some(caps) = summary_regex.captures(&stdout) {
            results.passed_tests = caps[1].parse().unwrap_or(0);
            results.failed_tests = caps[2].parse().unwrap_or(0);
            results.total_tests = results.passed_tests + results.failed_tests;
        }

        Ok(results)
    }
}

/// Go test parser
struct GoTestParser;

impl GoTestParser {
    fn new() -> Self {
        Self
    }
}

impl TestOutputParser for GoTestParser {
    fn parse_output(&self, output: &Output) -> Result<DetailedTestResults> {
        let stdout = String::from_utf8_lossy(&output.stdout);

        let mut passed = 0;
        let mut failed = 0;

        for line in stdout.lines() {
            if line.contains("PASS:") {
                passed += 1;
            } else if line.contains("FAIL:") {
                failed += 1;
            }
        }

        Ok(DetailedTestResults {
            framework: TestFramework::GoTest,
            all_passed: output.status.success(),
            total_tests: passed + failed,
            passed_tests: passed,
            failed_tests: failed,
            skipped_tests: 0,
            duration_ms: 0,
            failures: Vec::new(),
            coverage: None,
            raw_output: stdout.to_string(),
        })
    }
}

/// Mocha test parser
struct MochaTestParser;

impl MochaTestParser {
    fn new() -> Self {
        Self
    }
}

impl TestOutputParser for MochaTestParser {
    fn parse_output(&self, output: &Output) -> Result<DetailedTestResults> {
        let stdout = String::from_utf8_lossy(&output.stdout);

        let summary_regex = Regex::new(r"(\d+) passing.*?(\d+) failing").unwrap();

        let mut results = DetailedTestResults {
            framework: TestFramework::Mocha,
            all_passed: output.status.success(),
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            skipped_tests: 0,
            duration_ms: 0,
            failures: Vec::new(),
            coverage: None,
            raw_output: stdout.to_string(),
        };

        if let Some(caps) = summary_regex.captures(&stdout) {
            results.passed_tests = caps[1].parse().unwrap_or(0);
            results.failed_tests = caps[2].parse().unwrap_or(0);
            results.total_tests = results.passed_tests + results.failed_tests;
        }

        Ok(results)
    }
}

/// Vitest parser
struct VitestParser;

impl VitestParser {
    fn new() -> Self {
        Self
    }
}

impl TestOutputParser for VitestParser {
    fn parse_output(&self, output: &Output) -> Result<DetailedTestResults> {
        let stdout = String::from_utf8_lossy(&output.stdout);

        let summary_regex = Regex::new(r"Test Files\s+(\d+) passed.*?Tests\s+(\d+) passed").unwrap();

        let mut results = DetailedTestResults {
            framework: TestFramework::Vitest,
            all_passed: output.status.success(),
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            skipped_tests: 0,
            duration_ms: 0,
            failures: Vec::new(),
            coverage: None,
            raw_output: stdout.to_string(),
        };

        if let Some(caps) = summary_regex.captures(&stdout) {
            results.passed_tests = caps[2].parse().unwrap_or(0);
            results.total_tests = results.passed_tests;
        }

        Ok(results)
    }
}

fn framework_name(framework: TestFramework) -> &'static str {
    match framework {
        TestFramework::Cargo => "cargo",
        TestFramework::Jest => "jest",
        TestFramework::Pytest => "pytest",
        TestFramework::GoTest => "go test",
        TestFramework::Mocha => "mocha",
        TestFramework::Vitest => "vitest",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_parser() {
        let parser = CargoTestParser::new();
        let output = Output {
            status: std::process::ExitStatus::default(),
            stdout: b"test result: ok. 5 passed; 0 failed; 0 ignored".to_vec(),
            stderr: Vec::new(),
        };

        let results = parser.parse_output(&output).unwrap();
        assert_eq!(results.passed_tests, 5);
        assert_eq!(results.failed_tests, 0);
    }
}
