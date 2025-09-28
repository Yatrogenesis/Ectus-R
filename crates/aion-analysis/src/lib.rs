pub mod ast;
pub mod analyzer;
pub mod refactor;
pub mod language;
pub mod metrics;
pub mod security;
pub mod performance;
pub mod patterns;
pub mod suggestions;
pub mod ai;

pub use ast::*;
pub use analyzer::*;
pub use refactor::*;
pub use language::*;
pub use metrics::*;
pub use security::*;
pub use performance::*;
pub use patterns::*;
pub use suggestions::*;
pub use ai::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Go,
    Java,
    CPlusPlus,
    C,
    CSharp,
    SQL,
    HTML,
    CSS,
    JSON,
    YAML,
    TOML,
    Markdown,
    Shell,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisProject {
    pub id: Uuid,
    pub name: String,
    pub root_path: PathBuf,
    pub language: Language,
    pub files: Vec<SourceFile>,
    pub dependencies: Vec<Dependency>,
    pub configuration: ProjectConfiguration,
    pub metadata: ProjectMetadata,
    pub created_at: DateTime<Utc>,
    pub last_analyzed: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    pub id: Uuid,
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub language: Language,
    pub content: String,
    pub size_bytes: u64,
    pub line_count: u32,
    pub hash: String,
    pub last_modified: DateTime<Utc>,
    pub analysis_results: Option<FileAnalysisResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub dependency_type: DependencyType,
    pub source: DependencySource,
    pub vulnerabilities: Vec<SecurityVulnerability>,
    pub license: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Production,
    Development,
    Optional,
    Peer,
    Build,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencySource {
    Registry,
    Git,
    Local,
    URL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfiguration {
    pub target_language_version: Option<String>,
    pub build_system: Option<String>,
    pub test_framework: Option<String>,
    pub linting_rules: HashMap<String, serde_json::Value>,
    pub formatting_config: HashMap<String, serde_json::Value>,
    pub analysis_config: AnalysisConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfiguration {
    pub enabled_analyzers: Vec<String>,
    pub disabled_rules: Vec<String>,
    pub severity_levels: HashMap<String, Severity>,
    pub custom_rules: Vec<CustomRule>,
    pub ai_analysis_enabled: bool,
    pub security_analysis_enabled: bool,
    pub performance_analysis_enabled: bool,
    pub refactoring_suggestions_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub pattern: String,
    pub severity: Severity,
    pub category: RuleCategory,
    pub auto_fix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub total_files: u32,
    pub total_lines: u32,
    pub total_size_bytes: u64,
    pub language_distribution: HashMap<Language, u32>,
    pub complexity_metrics: ComplexityMetrics,
    pub test_coverage: Option<TestCoverage>,
    pub documentation_coverage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysisResult {
    pub file_id: Uuid,
    pub issues: Vec<AnalysisIssue>,
    pub metrics: FileMetrics,
    pub security_findings: Vec<SecurityFinding>,
    pub performance_insights: Vec<PerformanceInsight>,
    pub refactoring_opportunities: Vec<RefactoringOpportunity>,
    pub ai_suggestions: Vec<AISuggestion>,
    pub dependencies: Vec<FileDependency>,
    pub exports: Vec<Export>,
    pub analysis_duration_ms: u64,
    pub analyzed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisIssue {
    pub id: Uuid,
    pub rule_id: String,
    pub rule_name: String,
    pub severity: Severity,
    pub category: RuleCategory,
    pub message: String,
    pub description: Option<String>,
    pub location: CodeLocation,
    pub suggested_fix: Option<SuggestedFix>,
    pub related_issues: Vec<Uuid>,
    pub external_references: Vec<ExternalReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCategory {
    Syntax,
    Style,
    Performance,
    Security,
    Maintainability,
    Correctness,
    Compatibility,
    Documentation,
    Testing,
    Architecture,
    Accessibility,
    SEO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLocation {
    pub file_path: PathBuf,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
    pub start_byte: u32,
    pub end_byte: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedFix {
    pub description: String,
    pub changes: Vec<TextChange>,
    pub confidence: f64,
    pub auto_applicable: bool,
    pub requires_user_input: bool,
    pub side_effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextChange {
    pub location: CodeLocation,
    pub new_text: String,
    pub change_type: ChangeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Insert,
    Replace,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalReference {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetrics {
    pub lines_of_code: u32,
    pub comment_lines: u32,
    pub blank_lines: u32,
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: u32,
    pub maintainability_index: f64,
    pub technical_debt_minutes: u32,
    pub duplication_percentage: f64,
    pub function_count: u32,
    pub class_count: u32,
    pub interface_count: u32,
    pub variable_count: u32,
    pub import_count: u32,
    pub export_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    pub average_cyclomatic_complexity: f64,
    pub average_cognitive_complexity: f64,
    pub average_maintainability_index: f64,
    pub total_technical_debt_hours: f64,
    pub hotspots: Vec<ComplexityHotspot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityHotspot {
    pub file_path: PathBuf,
    pub function_name: Option<String>,
    pub complexity_score: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCoverage {
    pub line_coverage: f64,
    pub branch_coverage: f64,
    pub function_coverage: f64,
    pub statement_coverage: f64,
    pub uncovered_lines: Vec<u32>,
    pub test_files: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFinding {
    pub id: Uuid,
    pub vulnerability_type: SecurityVulnerabilityType,
    pub severity: SecuritySeverity,
    pub title: String,
    pub description: String,
    pub location: CodeLocation,
    pub cwe_id: Option<String>,
    pub owasp_category: Option<String>,
    pub remediation: Vec<String>,
    pub false_positive_likelihood: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityVulnerabilityType {
    SQLInjection,
    XSS,
    CSRF,
    PathTraversal,
    CommandInjection,
    BufferOverflow,
    UseAfterFree,
    DataExposure,
    WeakCryptography,
    InsecureDeserialization,
    BrokenAuthentication,
    BrokenAccessControl,
    SecurityMisconfiguration,
    VulnerableComponents,
    InsufficientLogging,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityVulnerability {
    pub cve_id: Option<String>,
    pub severity: SecuritySeverity,
    pub title: String,
    pub description: String,
    pub affected_versions: Vec<String>,
    pub fixed_versions: Vec<String>,
    pub references: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceInsight {
    pub id: Uuid,
    pub insight_type: PerformanceInsightType,
    pub severity: Severity,
    pub title: String,
    pub description: String,
    pub location: CodeLocation,
    pub estimated_impact: PerformanceImpact,
    pub recommendations: Vec<String>,
    pub benchmarks: Option<PerformanceBenchmark>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceInsightType {
    SlowAlgorithm,
    MemoryLeak,
    IneffecientDataStructure,
    UnnecessaryComputation,
    BlockingOperation,
    LargeObjectAllocation,
    FrequentGarbageCollection,
    DatabaseNPlusOne,
    UnoptimizedQuery,
    CacheMiss,
    NetworkLatency,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceImpact {
    pub cpu_impact: Option<f64>,
    pub memory_impact: Option<f64>,
    pub latency_impact: Option<f64>,
    pub throughput_impact: Option<f64>,
    pub user_experience_impact: UserExperienceImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserExperienceImpact {
    Critical,
    High,
    Medium,
    Low,
    Negligible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBenchmark {
    pub baseline_duration_ms: f64,
    pub optimized_duration_ms: f64,
    pub improvement_percentage: f64,
    pub test_scenario: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringOpportunity {
    pub id: Uuid,
    pub opportunity_type: RefactoringType,
    pub priority: RefactoringPriority,
    pub title: String,
    pub description: String,
    pub location: CodeLocation,
    pub affected_files: Vec<PathBuf>,
    pub estimated_effort_hours: f64,
    pub benefits: Vec<String>,
    pub risks: Vec<String>,
    pub suggested_approach: Vec<RefactoringStep>,
    pub automation_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefactoringType {
    ExtractMethod,
    ExtractClass,
    ExtractInterface,
    RenameVariable,
    RenameFunction,
    RenameClass,
    MoveMethod,
    MoveClass,
    InlineMethod,
    ReplaceConditionalWithPolymorphism,
    RemoveDuplication,
    SimplifyConditional,
    ReduceComplexity,
    ImproveNaming,
    ModernizeCode,
    OptimizePerformance,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefactoringPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringStep {
    pub step_number: u32,
    pub description: String,
    pub automated: bool,
    pub estimated_duration_minutes: u32,
    pub dependencies: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISuggestion {
    pub id: Uuid,
    pub suggestion_type: AISuggestionType,
    pub confidence: f64,
    pub title: String,
    pub description: String,
    pub location: CodeLocation,
    pub suggested_code: Option<String>,
    pub explanation: String,
    pub learning_resources: Vec<ExternalReference>,
    pub model_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AISuggestionType {
    CodeCompletion,
    BugFix,
    PerformanceOptimization,
    SecurityImprovement,
    BestPractice,
    Modernization,
    Documentation,
    Testing,
    ErrorHandling,
    APIUsage,
    DesignPattern,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDependency {
    pub path: PathBuf,
    pub dependency_type: FileDependencyType,
    pub imported_symbols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileDependencyType {
    Import,
    Include,
    Require,
    Use,
    From,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub name: String,
    pub export_type: ExportType,
    pub visibility: Visibility,
    pub location: CodeLocation,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportType {
    Function,
    Class,
    Interface,
    Variable,
    Constant,
    Type,
    Module,
    Namespace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
    Package,
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[async_trait::async_trait]
pub trait CodeAnalyzer {
    async fn analyze_project(&self, project: &AnalysisProject) -> Result<ProjectAnalysisResult>;
    async fn analyze_file(&self, file: &SourceFile) -> Result<FileAnalysisResult>;
    async fn analyze_code_snippet(&self, code: &str, language: Language) -> Result<Vec<AnalysisIssue>>;
    async fn get_metrics(&self, project: &AnalysisProject) -> Result<ProjectMetrics>;
    async fn find_security_issues(&self, project: &AnalysisProject) -> Result<Vec<SecurityFinding>>;
    async fn suggest_refactorings(&self, file: &SourceFile) -> Result<Vec<RefactoringOpportunity>>;
    async fn get_ai_suggestions(&self, file: &SourceFile) -> Result<Vec<AISuggestion>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAnalysisResult {
    pub project_id: Uuid,
    pub overall_health_score: f64,
    pub total_issues: u32,
    pub critical_issues: u32,
    pub security_score: f64,
    pub maintainability_score: f64,
    pub performance_score: f64,
    pub test_coverage_score: f64,
    pub file_results: HashMap<PathBuf, FileAnalysisResult>,
    pub project_level_insights: Vec<ProjectInsight>,
    pub recommendations: Vec<ProjectRecommendation>,
    pub trends: Option<AnalysisTrends>,
    pub analysis_duration_ms: u64,
    pub analyzed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetrics {
    pub total_lines_of_code: u32,
    pub total_files: u32,
    pub language_distribution: HashMap<Language, LanguageStats>,
    pub complexity_distribution: ComplexityDistribution,
    pub dependency_metrics: DependencyMetrics,
    pub quality_metrics: QualityMetrics,
    pub technical_debt: TechnicalDebtMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageStats {
    pub file_count: u32,
    pub line_count: u32,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityDistribution {
    pub low_complexity_files: u32,
    pub medium_complexity_files: u32,
    pub high_complexity_files: u32,
    pub average_complexity: f64,
    pub complexity_histogram: Vec<ComplexityBucket>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityBucket {
    pub range_min: u32,
    pub range_max: u32,
    pub file_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyMetrics {
    pub total_dependencies: u32,
    pub direct_dependencies: u32,
    pub transitive_dependencies: u32,
    pub outdated_dependencies: u32,
    pub vulnerable_dependencies: u32,
    pub license_distribution: HashMap<String, u32>,
    pub dependency_tree_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub duplication_percentage: f64,
    pub test_coverage_percentage: f64,
    pub documentation_percentage: f64,
    pub code_smells: u32,
    pub bugs: u32,
    pub vulnerabilities: u32,
    pub maintainability_rating: QualityRating,
    pub reliability_rating: QualityRating,
    pub security_rating: QualityRating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityRating {
    A, // Best
    B,
    C,
    D,
    E, // Worst
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalDebtMetrics {
    pub total_debt_hours: f64,
    pub debt_ratio_percentage: f64,
    pub sqale_rating: QualityRating,
    pub remediation_cost: f64,
    pub debt_by_category: HashMap<RuleCategory, f64>,
    pub debt_trends: Option<Vec<DebtTrendPoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtTrendPoint {
    pub date: DateTime<Utc>,
    pub debt_hours: f64,
    pub debt_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInsight {
    pub id: Uuid,
    pub insight_type: InsightType,
    pub priority: InsightPriority,
    pub title: String,
    pub description: String,
    pub affected_files: Vec<PathBuf>,
    pub metrics: HashMap<String, f64>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightType {
    ArchitecturalIssue,
    CodeQualityTrend,
    PerformanceBottleneck,
    SecurityRisk,
    MaintenanceRisk,
    TestingGap,
    DependencyRisk,
    TechnicalDebtAccumulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRecommendation {
    pub id: Uuid,
    pub category: RecommendationCategory,
    pub priority: InsightPriority,
    pub title: String,
    pub description: String,
    pub estimated_effort_hours: f64,
    pub expected_benefits: Vec<String>,
    pub implementation_steps: Vec<String>,
    pub resources: Vec<ExternalReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationCategory {
    Architecture,
    Performance,
    Security,
    Maintainability,
    Testing,
    Documentation,
    Dependencies,
    Tooling,
    ProcessImprovement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisTrends {
    pub quality_trend: TrendDirection,
    pub complexity_trend: TrendDirection,
    pub test_coverage_trend: TrendDirection,
    pub security_trend: TrendDirection,
    pub performance_trend: TrendDirection,
    pub debt_trend: TrendDirection,
    pub historical_data: Vec<HistoricalDataPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Stable,
    Declining,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalDataPoint {
    pub date: DateTime<Utc>,
    pub quality_score: f64,
    pub complexity_score: f64,
    pub security_score: f64,
    pub test_coverage: f64,
    pub technical_debt_hours: f64,
    pub commit_hash: Option<String>,
}