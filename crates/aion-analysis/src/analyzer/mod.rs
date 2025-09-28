pub mod rust_analyzer;
pub mod javascript_analyzer;
pub mod typescript_analyzer;
pub mod python_analyzer;
pub mod multi_language_analyzer;

pub use rust_analyzer::*;
pub use javascript_analyzer::*;
pub use typescript_analyzer::*;
pub use python_analyzer::*;
pub use multi_language_analyzer::*;

use crate::{
    AnalysisProject, SourceFile, FileAnalysisResult, ProjectAnalysisResult, AnalysisIssue,
    CodeAnalyzer, Language, Severity, RuleCategory, CodeLocation, ProjectMetrics, SecurityFinding,
    RefactoringOpportunity, AISuggestion, Result, FileMetrics, ProjectInsight, ProjectRecommendation,
    QualityRating, TechnicalDebtMetrics, ComplexityMetrics
};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;
use chrono::Utc;
use async_trait::async_trait;

pub struct DefaultCodeAnalyzer {
    rust_analyzer: RustAnalyzer,
    javascript_analyzer: JavaScriptAnalyzer,
    typescript_analyzer: TypeScriptAnalyzer,
    python_analyzer: PythonAnalyzer,
    ai_enabled: bool,
    security_enabled: bool,
}

impl DefaultCodeAnalyzer {
    pub fn new() -> Self {
        Self {
            rust_analyzer: RustAnalyzer::new(),
            javascript_analyzer: JavaScriptAnalyzer::new(),
            typescript_analyzer: TypeScriptAnalyzer::new(),
            python_analyzer: PythonAnalyzer::new(),
            ai_enabled: true,
            security_enabled: true,
        }
    }

    pub fn with_config(ai_enabled: bool, security_enabled: bool) -> Self {
        Self {
            rust_analyzer: RustAnalyzer::new(),
            javascript_analyzer: JavaScriptAnalyzer::new(),
            typescript_analyzer: TypeScriptAnalyzer::new(),
            python_analyzer: PythonAnalyzer::new(),
            ai_enabled,
            security_enabled,
        }
    }

    async fn analyze_file_by_language(&self, file: &SourceFile) -> Result<FileAnalysisResult> {
        match file.language {
            Language::Rust => self.rust_analyzer.analyze_file(file).await,
            Language::JavaScript => self.javascript_analyzer.analyze_file(file).await,
            Language::TypeScript => self.typescript_analyzer.analyze_file(file).await,
            Language::Python => self.python_analyzer.analyze_file(file).await,
            _ => self.analyze_generic_file(file).await,
        }
    }

    async fn analyze_generic_file(&self, file: &SourceFile) -> Result<FileAnalysisResult> {
        let start_time = std::time::Instant::now();

        // Basic analysis for unsupported languages
        let metrics = self.calculate_basic_metrics(file);
        let issues = self.find_generic_issues(file);

        Ok(FileAnalysisResult {
            file_id: file.id,
            issues,
            metrics,
            security_findings: Vec::new(),
            performance_insights: Vec::new(),
            refactoring_opportunities: Vec::new(),
            ai_suggestions: Vec::new(),
            dependencies: Vec::new(),
            exports: Vec::new(),
            analysis_duration_ms: start_time.elapsed().as_millis() as u64,
            analyzed_at: Utc::now(),
        })
    }

    fn calculate_basic_metrics(&self, file: &SourceFile) -> FileMetrics {
        let lines: Vec<&str> = file.content.lines().collect();
        let total_lines = lines.len() as u32;

        let mut code_lines = 0;
        let mut comment_lines = 0;
        let mut blank_lines = 0;

        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                blank_lines += 1;
            } else if trimmed.starts_with("//") || trimmed.starts_with("#") || trimmed.starts_with("/*") {
                comment_lines += 1;
            } else {
                code_lines += 1;
            }
        }

        FileMetrics {
            lines_of_code: code_lines,
            comment_lines,
            blank_lines,
            cyclomatic_complexity: 1, // Basic default
            cognitive_complexity: 1,
            maintainability_index: 100.0,
            technical_debt_minutes: 0,
            duplication_percentage: 0.0,
            function_count: 0,
            class_count: 0,
            interface_count: 0,
            variable_count: 0,
            import_count: 0,
            export_count: 0,
        }
    }

    fn find_generic_issues(&self, file: &SourceFile) -> Vec<AnalysisIssue> {
        let mut issues = Vec::new();

        // Check for very long lines
        for (line_num, line) in file.content.lines().enumerate() {
            if line.len() > 120 {
                issues.push(AnalysisIssue {
                    id: Uuid::new_v4(),
                    rule_id: "line-length".to_string(),
                    rule_name: "Line Length".to_string(),
                    severity: Severity::Warning,
                    category: RuleCategory::Style,
                    message: format!("Line is {} characters long, consider breaking it up", line.len()),
                    description: Some("Long lines can be difficult to read and may not display well in some editors".to_string()),
                    location: CodeLocation {
                        file_path: file.path.clone(),
                        start_line: line_num as u32 + 1,
                        start_column: 1,
                        end_line: line_num as u32 + 1,
                        end_column: line.len() as u32,
                        start_byte: 0,
                        end_byte: line.len() as u32,
                    },
                    suggested_fix: None,
                    related_issues: Vec::new(),
                    external_references: Vec::new(),
                });
            }
        }

        // Check for potential TODO/FIXME comments
        for (line_num, line) in file.content.lines().enumerate() {
            let line_lower = line.to_lowercase();
            if line_lower.contains("todo") || line_lower.contains("fixme") || line_lower.contains("hack") {
                issues.push(AnalysisIssue {
                    id: Uuid::new_v4(),
                    rule_id: "todo-comment".to_string(),
                    rule_name: "TODO Comment".to_string(),
                    severity: Severity::Info,
                    category: RuleCategory::Maintainability,
                    message: "TODO/FIXME comment found".to_string(),
                    description: Some("Consider creating a proper issue or task to track this work".to_string()),
                    location: CodeLocation {
                        file_path: file.path.clone(),
                        start_line: line_num as u32 + 1,
                        start_column: 1,
                        end_line: line_num as u32 + 1,
                        end_column: line.len() as u32,
                        start_byte: 0,
                        end_byte: line.len() as u32,
                    },
                    suggested_fix: None,
                    related_issues: Vec::new(),
                    external_references: Vec::new(),
                });
            }
        }

        issues
    }

    fn calculate_project_health_score(&self, results: &HashMap<PathBuf, FileAnalysisResult>) -> f64 {
        if results.is_empty() {
            return 0.0;
        }

        let mut total_score = 0.0;
        let mut file_count = 0;

        for result in results.values() {
            let mut file_score = 100.0;

            // Deduct points for issues
            for issue in &result.issues {
                match issue.severity {
                    Severity::Error => file_score -= 10.0,
                    Severity::Warning => file_score -= 5.0,
                    Severity::Info => file_score -= 1.0,
                    Severity::Hint => file_score -= 0.5,
                }
            }

            // Deduct points for security findings
            for finding in &result.security_findings {
                match finding.severity {
                    crate::SecuritySeverity::Critical => file_score -= 25.0,
                    crate::SecuritySeverity::High => file_score -= 15.0,
                    crate::SecuritySeverity::Medium => file_score -= 10.0,
                    crate::SecuritySeverity::Low => file_score -= 5.0,
                    crate::SecuritySeverity::Info => file_score -= 1.0,
                }
            }

            // Factor in complexity
            if result.metrics.cyclomatic_complexity > 10 {
                file_score -= (result.metrics.cyclomatic_complexity as f64 - 10.0) * 2.0;
            }

            total_score += file_score.max(0.0);
            file_count += 1;
        }

        (total_score / file_count as f64).min(100.0).max(0.0)
    }

    fn generate_project_insights(&self, results: &HashMap<PathBuf, FileAnalysisResult>) -> Vec<ProjectInsight> {
        let mut insights = Vec::new();

        // Analyze complexity distribution
        let mut high_complexity_files = 0;
        let mut total_complexity = 0;
        let mut file_count = 0;

        for (path, result) in results {
            if result.metrics.cyclomatic_complexity > 15 {
                high_complexity_files += 1;
            }
            total_complexity += result.metrics.cyclomatic_complexity;
            file_count += 1;
        }

        if high_complexity_files > 0 {
            insights.push(ProjectInsight {
                id: Uuid::new_v4(),
                insight_type: crate::InsightType::CodeQualityTrend,
                priority: if high_complexity_files > file_count / 4 {
                    crate::InsightPriority::High
                } else {
                    crate::InsightPriority::Medium
                },
                title: "High Complexity Files Detected".to_string(),
                description: format!(
                    "{} files have high cyclomatic complexity (>15). Average complexity is {:.1}",
                    high_complexity_files,
                    total_complexity as f64 / file_count as f64
                ),
                affected_files: results
                    .iter()
                    .filter(|(_, result)| result.metrics.cyclomatic_complexity > 15)
                    .map(|(path, _)| path.clone())
                    .collect(),
                metrics: HashMap::from([
                    ("high_complexity_files".to_string(), high_complexity_files as f64),
                    ("average_complexity".to_string(), total_complexity as f64 / file_count as f64),
                ]),
                recommendations: vec![
                    "Consider breaking down complex functions into smaller, more manageable pieces".to_string(),
                    "Review control flow and try to reduce nesting levels".to_string(),
                    "Extract common functionality into reusable functions".to_string(),
                ],
            });
        }

        // Analyze security findings
        let total_security_findings: usize = results.values()
            .map(|result| result.security_findings.len())
            .sum();

        if total_security_findings > 0 {
            insights.push(ProjectInsight {
                id: Uuid::new_v4(),
                insight_type: crate::InsightType::SecurityRisk,
                priority: crate::InsightPriority::High,
                title: "Security Issues Found".to_string(),
                description: format!("{} security issues detected across the project", total_security_findings),
                affected_files: results
                    .iter()
                    .filter(|(_, result)| !result.security_findings.is_empty())
                    .map(|(path, _)| path.clone())
                    .collect(),
                metrics: HashMap::from([
                    ("total_security_findings".to_string(), total_security_findings as f64),
                ]),
                recommendations: vec![
                    "Review and address all security findings immediately".to_string(),
                    "Implement security scanning in your CI/CD pipeline".to_string(),
                    "Consider security training for the development team".to_string(),
                ],
            });
        }

        insights
    }

    fn generate_project_recommendations(&self, results: &HashMap<PathBuf, FileAnalysisResult>) -> Vec<ProjectRecommendation> {
        let mut recommendations = Vec::new();

        // Analyze technical debt
        let total_debt_minutes: u32 = results.values()
            .map(|result| result.metrics.technical_debt_minutes)
            .sum();

        if total_debt_minutes > 480 { // More than 8 hours
            recommendations.push(ProjectRecommendation {
                id: Uuid::new_v4(),
                category: crate::RecommendationCategory::Maintainability,
                priority: crate::InsightPriority::High,
                title: "Address Technical Debt".to_string(),
                description: format!(
                    "Project has {:.1} hours of technical debt. This will slow down future development.",
                    total_debt_minutes as f64 / 60.0
                ),
                estimated_effort_hours: (total_debt_minutes as f64 / 60.0) * 0.8, // 80% of debt time
                expected_benefits: vec![
                    "Faster development velocity".to_string(),
                    "Reduced bug rate".to_string(),
                    "Improved code maintainability".to_string(),
                    "Better developer experience".to_string(),
                ],
                implementation_steps: vec![
                    "Prioritize high-impact, low-effort refactorings".to_string(),
                    "Allocate dedicated time for technical debt reduction".to_string(),
                    "Set up automated code quality gates".to_string(),
                    "Train team on clean code practices".to_string(),
                ],
                resources: vec![],
            });
        }

        // Analyze test coverage (if available)
        let files_with_tests = results.values()
            .filter(|result| {
                result.exports.iter().any(|export| {
                    export.name.to_lowercase().contains("test") ||
                    export.name.to_lowercase().contains("spec")
                })
            })
            .count();

        let total_files = results.len();
        let test_coverage_ratio = files_with_tests as f64 / total_files as f64;

        if test_coverage_ratio < 0.6 {
            recommendations.push(ProjectRecommendation {
                id: Uuid::new_v4(),
                category: crate::RecommendationCategory::Testing,
                priority: crate::InsightPriority::Medium,
                title: "Improve Test Coverage".to_string(),
                description: format!(
                    "Test coverage appears low ({:.1}%). Consider adding more tests.",
                    test_coverage_ratio * 100.0
                ),
                estimated_effort_hours: (total_files as f64 - files_with_tests as f64) * 2.0,
                expected_benefits: vec![
                    "Reduced bug rate in production".to_string(),
                    "Faster regression detection".to_string(),
                    "Improved confidence in refactoring".to_string(),
                    "Better documentation through tests".to_string(),
                ],
                implementation_steps: vec![
                    "Set up testing framework if not already present".to_string(),
                    "Identify critical paths and add tests for them first".to_string(),
                    "Add tests for new features as they're developed".to_string(),
                    "Consider test-driven development approach".to_string(),
                ],
                resources: vec![],
            });
        }

        recommendations
    }
}

#[async_trait]
impl CodeAnalyzer for DefaultCodeAnalyzer {
    async fn analyze_project(&self, project: &AnalysisProject) -> Result<ProjectAnalysisResult> {
        let start_time = std::time::Instant::now();
        let mut file_results = HashMap::new();

        // Analyze each file
        for file in &project.files {
            let result = self.analyze_file_by_language(file).await?;
            file_results.insert(file.relative_path.clone(), result);
        }

        // Calculate project-level metrics
        let overall_health_score = self.calculate_project_health_score(&file_results);

        let total_issues = file_results.values()
            .map(|result| result.issues.len() as u32)
            .sum();

        let critical_issues = file_results.values()
            .map(|result| {
                result.issues.iter()
                    .filter(|issue| matches!(issue.severity, Severity::Error))
                    .count() as u32
            })
            .sum();

        let security_score = 100.0 - (file_results.values()
            .map(|result| result.security_findings.len() as f64 * 10.0)
            .sum::<f64>()
            .min(100.0));

        let maintainability_score = file_results.values()
            .map(|result| result.metrics.maintainability_index)
            .sum::<f64>() / file_results.len() as f64;

        let performance_score = 100.0 - (file_results.values()
            .map(|result| result.performance_insights.len() as f64 * 5.0)
            .sum::<f64>()
            .min(100.0));

        // Generate insights and recommendations
        let project_level_insights = self.generate_project_insights(&file_results);
        let recommendations = self.generate_project_recommendations(&file_results);

        Ok(ProjectAnalysisResult {
            project_id: project.id,
            overall_health_score,
            total_issues,
            critical_issues,
            security_score,
            maintainability_score,
            performance_score,
            test_coverage_score: 0.0, // TODO: Calculate actual test coverage
            file_results,
            project_level_insights,
            recommendations,
            trends: None, // TODO: Implement trend analysis
            analysis_duration_ms: start_time.elapsed().as_millis() as u64,
            analyzed_at: Utc::now(),
        })
    }

    async fn analyze_file(&self, file: &SourceFile) -> Result<FileAnalysisResult> {
        self.analyze_file_by_language(file).await
    }

    async fn analyze_code_snippet(&self, code: &str, language: Language) -> Result<Vec<AnalysisIssue>> {
        // Create a temporary file for analysis
        let temp_file = SourceFile {
            id: Uuid::new_v4(),
            path: PathBuf::from("temp.code"),
            relative_path: PathBuf::from("temp.code"),
            language,
            content: code.to_string(),
            size_bytes: code.len() as u64,
            line_count: code.lines().count() as u32,
            hash: "temp".to_string(),
            last_modified: Utc::now(),
            analysis_results: None,
        };

        let result = self.analyze_file(&temp_file).await?;
        Ok(result.issues)
    }

    async fn get_metrics(&self, project: &AnalysisProject) -> Result<ProjectMetrics> {
        let mut language_distribution = HashMap::new();
        let mut total_lines = 0;
        let mut total_files = 0;

        for file in &project.files {
            total_files += 1;
            total_lines += file.line_count;

            let stats = language_distribution
                .entry(file.language.clone())
                .or_insert(crate::LanguageStats {
                    file_count: 0,
                    line_count: 0,
                    percentage: 0.0,
                });

            stats.file_count += 1;
            stats.line_count += file.line_count;
        }

        // Calculate percentages
        for stats in language_distribution.values_mut() {
            stats.percentage = (stats.line_count as f64 / total_lines as f64) * 100.0;
        }

        Ok(ProjectMetrics {
            total_lines_of_code: total_lines,
            total_files,
            language_distribution,
            complexity_distribution: crate::ComplexityDistribution {
                low_complexity_files: 0,
                medium_complexity_files: 0,
                high_complexity_files: 0,
                average_complexity: 0.0,
                complexity_histogram: Vec::new(),
            },
            dependency_metrics: crate::DependencyMetrics {
                total_dependencies: project.dependencies.len() as u32,
                direct_dependencies: project.dependencies.len() as u32,
                transitive_dependencies: 0,
                outdated_dependencies: 0,
                vulnerable_dependencies: project.dependencies.iter()
                    .filter(|dep| !dep.vulnerabilities.is_empty())
                    .count() as u32,
                license_distribution: HashMap::new(),
                dependency_tree_depth: 1,
            },
            quality_metrics: crate::QualityMetrics {
                duplication_percentage: 0.0,
                test_coverage_percentage: 0.0,
                documentation_percentage: 0.0,
                code_smells: 0,
                bugs: 0,
                vulnerabilities: 0,
                maintainability_rating: QualityRating::A,
                reliability_rating: QualityRating::A,
                security_rating: QualityRating::A,
            },
            technical_debt: TechnicalDebtMetrics {
                total_debt_hours: 0.0,
                debt_ratio_percentage: 0.0,
                sqale_rating: QualityRating::A,
                remediation_cost: 0.0,
                debt_by_category: HashMap::new(),
                debt_trends: None,
            },
        })
    }

    async fn find_security_issues(&self, project: &AnalysisProject) -> Result<Vec<SecurityFinding>> {
        let mut all_findings = Vec::new();

        for file in &project.files {
            let result = self.analyze_file(file).await?;
            all_findings.extend(result.security_findings);
        }

        Ok(all_findings)
    }

    async fn suggest_refactorings(&self, file: &SourceFile) -> Result<Vec<RefactoringOpportunity>> {
        let result = self.analyze_file(file).await?;
        Ok(result.refactoring_opportunities)
    }

    async fn get_ai_suggestions(&self, file: &SourceFile) -> Result<Vec<AISuggestion>> {
        if !self.ai_enabled {
            return Ok(Vec::new());
        }

        let result = self.analyze_file(file).await?;
        Ok(result.ai_suggestions)
    }
}