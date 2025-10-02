//! # Ectus-R Plugin SDK
//!
//! Official Rust SDK for building Ectus-R plugins.
//!
//! This SDK provides the core types and traits needed to create:
//! - Code generators for new languages
//! - Custom analyzers for security and quality
//! - Code transformers for refactoring
//! - Project templates
//!
//! ## Example
//!
//! ```rust
//! use ectus_plugin_sdk::*;
//!
//! pub struct MyGenerator;
//!
//! #[async_trait::async_trait]
//! impl CodeGenerator for MyGenerator {
//!     fn name(&self) -> &str {
//!         "my-generator"
//!     }
//!
//!     fn supported_languages(&self) -> Vec<String> {
//!         vec!["mylang".to_string()]
//!     }
//!
//!     async fn generate(&self, context: &PluginContext) -> Result<Vec<GeneratedFile>> {
//!         let requirements = context.get_requirements();
//!         // Generate code...
//!         Ok(vec![])
//!     }
//! }
//! ```

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

pub type Result<T> = std::result::Result<T, PluginError>;

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Generation error: {0}")]
    GenerationError(String),

    #[error("Analysis error: {0}")]
    AnalysisError(String),

    #[error("Transformation error: {0}")]
    TransformationError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    Other(String),
}

// ============================================================================
// Core Plugin Traits
// ============================================================================

/// Main plugin trait that all plugins must implement
pub trait Plugin: Send + Sync {
    /// Plugin name
    fn name(&self) -> &str;

    /// Plugin version
    fn version(&self) -> &str;

    /// Plugin description
    fn description(&self) -> Option<&str> {
        None
    }

    /// Initialize the plugin
    fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// Shutdown the plugin
    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Code generator trait for creating source code
#[async_trait]
pub trait CodeGenerator: Plugin {
    /// Supported programming languages
    fn supported_languages(&self) -> Vec<String>;

    /// Generate code from requirements
    async fn generate(&self, context: &PluginContext) -> Result<Vec<GeneratedFile>>;

    /// Validate generated code
    async fn validate(&self, files: &[GeneratedFile]) -> Result<ValidationResult> {
        Ok(ValidationResult {
            is_valid: true,
            errors: vec![],
            warnings: vec![],
        })
    }
}

/// Analyzer trait for code analysis
#[async_trait]
pub trait Analyzer: Plugin {
    /// Analysis category
    fn category(&self) -> AnalyzerCategory;

    /// Analyze code
    async fn analyze(&self, context: &PluginContext) -> Result<AnalysisResult>;
}

/// Transformer trait for code transformation
#[async_trait]
pub trait Transformer: Plugin {
    /// Transform code
    async fn transform(&self, context: &PluginContext) -> Result<TransformResult>;
}

/// Template generator trait
#[async_trait]
pub trait TemplateGenerator: Plugin {
    /// Generate project template
    async fn generate_template(&self, context: &PluginContext) -> Result<Vec<GeneratedFile>>;
}

// ============================================================================
// Plugin Context
// ============================================================================

/// Context provided to plugins during execution
pub trait PluginContext: Send + Sync {
    /// Get project path
    fn get_project_path(&self) -> &PathBuf;

    /// Get requirements
    fn get_requirements(&self) -> &Requirements;

    /// Get system architecture
    fn get_architecture(&self) -> &SystemArchitecture;

    /// Get plugin configuration
    fn get_configuration(&self) -> &PluginConfig;

    /// Get generated files
    fn get_generated_files(&self) -> &[GeneratedFile];

    /// Read file content
    fn read_file(&self, path: &PathBuf) -> Result<String>;

    /// Check if file exists
    fn file_exists(&self, path: &PathBuf) -> bool;

    /// Invoke AI model
    fn invoke_ai(&self, prompt: &str, options: Option<AIOptions>) -> Result<String>;

    /// Analyze code
    fn analyze_code(&self, code: &str, language: &str) -> Result<CodeAnalysis>;

    /// Log message
    fn log(&self, level: LogLevel, message: &str, metadata: Option<HashMap<String, String>>);

    /// Resolve template
    fn resolve_template(&self, name: &str) -> Result<String>;

    /// Format code
    fn format_code(&self, code: &str, language: &str) -> Result<String>;
}

// ============================================================================
// Data Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirements {
    pub project_name: String,
    pub description: String,
    pub functional_requirements: Vec<String>,
    pub non_functional_requirements: Vec<String>,
    pub target_language: String,
    pub framework: Option<String>,
    pub constraints: Constraints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraints {
    pub max_file_size: Option<usize>,
    pub performance_requirements: Option<PerformanceRequirements>,
    pub security_requirements: Option<SecurityRequirements>,
    pub compatibility_requirements: Option<CompatibilityRequirements>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    pub max_latency_ms: Option<u64>,
    pub min_throughput_rps: Option<u64>,
    pub max_memory_mb: Option<u64>,
    pub max_cpu_percent: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    pub encryption_required: bool,
    pub authentication_type: Option<String>,
    pub authorization_model: Option<String>,
    pub compliance_standards: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityRequirements {
    pub min_runtime_version: Option<String>,
    pub target_platforms: Vec<String>,
    pub required_dependencies: Vec<String>,
    pub excluded_dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemArchitecture {
    pub pattern: ArchitecturePattern,
    pub components: Vec<Component>,
    pub connections: Vec<Connection>,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchitecturePattern {
    Microservices,
    Monolithic,
    Serverless,
    EventDriven,
    Layered,
    Hexagonal,
    MVC,
    MVVM,
    Clean,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub component_type: String,
    pub responsibilities: Vec<String>,
    pub dependencies: Vec<String>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from: String,
    pub to: String,
    pub protocol: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub components: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
    pub language: String,
    pub purpose: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub exports: Option<Vec<String>>,
}

impl GeneratedFile {
    pub fn new(path: PathBuf, content: String, language: String) -> Self {
        Self {
            path,
            content,
            language,
            purpose: None,
            dependencies: None,
            exports: None,
        }
    }

    pub fn with_purpose(mut self, purpose: String) -> Self {
        self.purpose = Some(purpose);
        self
    }

    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = Some(dependencies);
        self
    }

    pub fn with_exports(mut self, exports: Vec<String>) -> Self {
        self.exports = Some(exports);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    pub options: HashMap<String, serde_json::Value>,
    pub custom_settings: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIOptions {
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
    pub system_prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAnalysis {
    pub complexity: f32,
    pub maintainability: f32,
    pub security_score: f32,
    pub issues: Vec<Issue>,
    pub metrics: CodeMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetrics {
    pub lines_of_code: usize,
    pub cyclomatic_complexity: f32,
    pub cognitive_complexity: f32,
    pub maintainability_index: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub issues: Vec<Issue>,
    pub score: f32,
    pub recommendations: Option<Vec<Recommendation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub severity: Severity,
    pub category: String,
    pub file: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub description: String,
    pub suggested_fix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub title: String,
    pub description: String,
    pub impact: Impact,
    pub effort: Effort,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Impact {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effort {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformResult {
    pub modified_files: Vec<ModifiedFile>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifiedFile {
    pub path: PathBuf,
    pub original_content: String,
    pub new_content: String,
    pub changes: Vec<Change>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub line: usize,
    pub change_type: ChangeType,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Addition,
    Deletion,
    Modification,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnalyzerCategory {
    Security,
    Performance,
    Quality,
    Style,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

// ============================================================================
// Builder Patterns
// ============================================================================

pub struct IssueBuilder {
    severity: Option<Severity>,
    category: Option<String>,
    file: Option<String>,
    line: Option<usize>,
    column: Option<usize>,
    description: Option<String>,
    suggested_fix: Option<String>,
}

impl IssueBuilder {
    pub fn new() -> Self {
        Self {
            severity: None,
            category: None,
            file: None,
            line: None,
            column: None,
            description: None,
            suggested_fix: None,
        }
    }

    pub fn severity(mut self, severity: Severity) -> Self {
        self.severity = Some(severity);
        self
    }

    pub fn category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }

    pub fn file(mut self, file: String) -> Self {
        self.file = Some(file);
        self
    }

    pub fn line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    pub fn column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn suggested_fix(mut self, suggested_fix: String) -> Self {
        self.suggested_fix = Some(suggested_fix);
        self
    }

    pub fn build(self) -> Result<Issue> {
        Ok(Issue {
            severity: self.severity.ok_or_else(|| PluginError::ConfigurationError("Severity is required".to_string()))?,
            category: self.category.ok_or_else(|| PluginError::ConfigurationError("Category is required".to_string()))?,
            file: self.file.ok_or_else(|| PluginError::ConfigurationError("File is required".to_string()))?,
            line: self.line,
            column: self.column,
            description: self.description.ok_or_else(|| PluginError::ConfigurationError("Description is required".to_string()))?,
            suggested_fix: self.suggested_fix,
        })
    }
}

impl Default for IssueBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Create a new generated file
pub fn create_file(path: impl Into<PathBuf>, content: impl Into<String>, language: impl Into<String>) -> GeneratedFile {
    GeneratedFile::new(path.into(), content.into(), language.into())
}

/// Create an issue
pub fn create_issue(severity: Severity, category: impl Into<String>, file: impl Into<String>, description: impl Into<String>) -> Issue {
    Issue {
        severity,
        category: category.into(),
        file: file.into(),
        line: None,
        column: None,
        description: description.into(),
        suggested_fix: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generated_file_builder() {
        let file = GeneratedFile::new(
            PathBuf::from("src/main.rs"),
            "fn main() {}".to_string(),
            "rust".to_string(),
        )
        .with_purpose("Main entry point".to_string())
        .with_dependencies(vec!["tokio".to_string()]);

        assert_eq!(file.path, PathBuf::from("src/main.rs"));
        assert_eq!(file.language, "rust");
        assert_eq!(file.purpose, Some("Main entry point".to_string()));
    }

    #[test]
    fn test_issue_builder() {
        let issue = IssueBuilder::new()
            .severity(Severity::High)
            .category("Security".to_string())
            .file("src/main.rs".to_string())
            .line(42)
            .description("Potential SQL injection".to_string())
            .build()
            .unwrap();

        assert_eq!(issue.file, "src/main.rs");
        assert_eq!(issue.line, Some(42));
    }
}
