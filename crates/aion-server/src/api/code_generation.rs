// AION-R Code Generation API Endpoints
// REST API endpoints for code generation functionality

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use aion_ai_engine::{
    CodeGenerationEngine, CodeGenerationRequest, CodeGenerationResult,
    ProgrammingLanguage, ArchitecturePattern, GenerationConstraints,
    OptimizationLevel, ProjectContext,
};
use aion_auth::models::AuthenticatedUser;
use crate::{AppState, errors::AppError};

/// API request for code generation
#[derive(Debug, Deserialize)]
pub struct GenerateCodeRequest {
    pub requirements: String,
    pub language: String,
    pub framework: Option<String>,
    pub architecture: Option<String>,
    pub optimization_level: Option<String>,
    pub constraints: Option<ApiGenerationConstraints>,
    pub context: Option<ApiProjectContext>,
}

/// API generation constraints
#[derive(Debug, Deserialize)]
pub struct ApiGenerationConstraints {
    pub max_file_size: Option<usize>,
    pub performance_requirements: Option<PerformanceRequirements>,
    pub security_requirements: Option<SecurityRequirements>,
    pub coding_standards: Option<CodingStandards>,
}

#[derive(Debug, Deserialize)]
pub struct PerformanceRequirements {
    pub max_latency_ms: Option<u64>,
    pub min_throughput_rps: Option<u64>,
    pub max_memory_mb: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct SecurityRequirements {
    pub encryption_required: bool,
    pub authentication_type: Option<String>,
    pub compliance_standards: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CodingStandards {
    pub style_guide: Option<String>,
    pub max_line_length: Option<usize>,
    pub naming_convention: Option<String>,
}

/// API project context
#[derive(Debug, Deserialize)]
pub struct ApiProjectContext {
    pub existing_codebase: Option<String>,
    pub dependencies: Vec<String>,
    pub domain_knowledge: Option<String>,
}

/// API response for code generation
#[derive(Debug, Serialize)]
pub struct GenerateCodeResponse {
    pub id: Uuid,
    pub status: String,
    pub generated_files_count: usize,
    pub total_lines_of_code: usize,
    pub documentation_url: String,
    pub download_url: String,
    pub preview: CodePreview,
    pub suggestions: Vec<String>,
    pub estimated_time_saved_hours: f32,
}

/// Code preview in response
#[derive(Debug, Serialize)]
pub struct CodePreview {
    pub main_file: String,
    pub structure: Vec<FileInfo>,
}

#[derive(Debug, Serialize)]
pub struct FileInfo {
    pub path: String,
    pub language: String,
    pub size_bytes: usize,
    pub purpose: String,
}

/// Generate code from requirements
pub async fn generate_code(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<GenerateCodeRequest>,
) -> Result<Json<GenerateCodeResponse>, AppError> {
    // Validate user permissions
    if !user.has_permission("ai.code_generation") {
        return Err(AppError::Forbidden("Insufficient permissions for code generation".into()));
    }

    // Parse language
    let language = parse_language(&request.language)?;

    // Parse architecture pattern
    let architecture = parse_architecture(request.architecture.as_deref())?;

    // Parse optimization level
    let optimization = parse_optimization_level(request.optimization_level.as_deref())?;

    // Build constraints
    let constraints = build_constraints(request.constraints);

    // Build project context
    let context = build_project_context(request.context);

    // Create generation request
    let gen_request = CodeGenerationRequest {
        id: Uuid::new_v4(),
        requirements: request.requirements.clone(),
        language,
        framework: request.framework,
        architecture,
        constraints,
        context,
        optimization_level: optimization,
    };

    // Get code generation engine from state
    let engine = state.code_generation_engine.clone();

    // Generate code
    let result = engine.generate_code(gen_request).await
        .map_err(|e| AppError::Internal(format!("Code generation failed: {}", e)))?;

    // Store generated code for download
    let download_url = store_generated_code(&state, &user, &result).await?;

    // Create response
    let response = GenerateCodeResponse {
        id: result.id,
        status: "completed".to_string(),
        generated_files_count: result.generated_files.len(),
        total_lines_of_code: result.metrics.total_lines_of_code,
        documentation_url: format!("/api/v1/code/{}/docs", result.id),
        download_url,
        preview: create_preview(&result),
        suggestions: result.suggestions.iter()
            .map(|s| s.description.clone())
            .collect(),
        estimated_time_saved_hours: estimate_time_saved(&result),
    };

    // Track usage metrics
    track_usage_metrics(&state, &user, &result).await;

    Ok(Json(response))
}

/// Get code generation status
pub async fn get_generation_status(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<GenerationStatus>, AppError> {
    // Check if generation exists for user
    let status = state.generation_tracker
        .get_status(&user.id, &id)
        .await
        .ok_or(AppError::NotFound("Generation not found".into()))?;

    Ok(Json(status))
}

/// Download generated code
pub async fn download_generated_code(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
) -> Result<Vec<u8>, AppError> {
    // Verify user owns this generation
    if !state.generation_tracker.user_owns_generation(&user.id, &id).await {
        return Err(AppError::Forbidden("Access denied to this generation".into()));
    }

    // Get generated code archive
    let archive = state.storage
        .get_generation_archive(&id)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to retrieve archive: {}", e)))?;

    Ok(archive)
}

/// Get generation documentation
pub async fn get_generation_documentation(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<DocumentationResponse>, AppError> {
    // Verify access
    if !state.generation_tracker.user_owns_generation(&user.id, &id).await {
        return Err(AppError::Forbidden("Access denied".into()));
    }

    // Get documentation
    let docs = state.storage
        .get_generation_docs(&id)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to retrieve docs: {}", e)))?;

    Ok(Json(DocumentationResponse {
        readme: docs.readme,
        api_docs: docs.api_docs,
        architecture_docs: docs.architecture_docs,
        setup_guide: docs.setup_guide,
    }))
}

/// List user's generations
pub async fn list_generations(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthenticatedUser>,
    Query(params): Query<ListParams>,
) -> Result<Json<ListGenerationsResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100);

    let generations = state.generation_tracker
        .list_user_generations(&user.id, page, per_page)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list generations: {}", e)))?;

    Ok(Json(ListGenerationsResponse {
        generations,
        page,
        per_page,
        total: state.generation_tracker.count_user_generations(&user.id).await?,
    }))
}

/// Delete a generation
pub async fn delete_generation(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // Verify ownership
    if !state.generation_tracker.user_owns_generation(&user.id, &id).await {
        return Err(AppError::Forbidden("Cannot delete this generation".into()));
    }

    // Delete generation
    state.generation_tracker
        .delete_generation(&id)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to delete: {}", e)))?;

    // Clean up storage
    state.storage
        .delete_generation_files(&id)
        .await
        .map_err(|e| AppError::Internal(format!("Storage cleanup failed: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Analyze requirements before generation
pub async fn analyze_requirements(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<AnalyzeRequirementsRequest>,
) -> Result<Json<AnalyzeRequirementsResponse>, AppError> {
    // Check permissions
    if !user.has_permission("ai.requirements_analysis") {
        return Err(AppError::Forbidden("Insufficient permissions".into()));
    }

    // Get requirements analyzer
    let analyzer = state.requirements_analyzer.clone();

    // Analyze requirements
    let analysis = analyzer
        .analyze_requirements(&request.requirements)
        .await
        .map_err(|e| AppError::Internal(format!("Analysis failed: {}", e)))?;

    Ok(Json(AnalyzeRequirementsResponse {
        id: analysis.id,
        confidence_score: analysis.confidence_score,
        user_stories_count: analysis.user_stories.len(),
        risks_identified: analysis.risk_assessment.identified_risks.len(),
        implementation_phases: analysis.implementation_plan.phases.len(),
        optimization_suggestions: analysis.optimization_suggestions.len(),
        technical_summary: create_technical_summary(&analysis),
    }))
}

// Helper functions

fn parse_language(lang: &str) -> Result<ProgrammingLanguage, AppError> {
    match lang.to_lowercase().as_str() {
        "rust" => Ok(ProgrammingLanguage::Rust),
        "python" => Ok(ProgrammingLanguage::Python),
        "javascript" | "js" => Ok(ProgrammingLanguage::JavaScript),
        "typescript" | "ts" => Ok(ProgrammingLanguage::TypeScript),
        "go" | "golang" => Ok(ProgrammingLanguage::Go),
        "java" => Ok(ProgrammingLanguage::Java),
        "c#" | "csharp" => Ok(ProgrammingLanguage::CSharp),
        "c++" | "cpp" => Ok(ProgrammingLanguage::Cpp),
        "swift" => Ok(ProgrammingLanguage::Swift),
        "kotlin" => Ok(ProgrammingLanguage::Kotlin),
        _ => Err(AppError::BadRequest(format!("Unsupported language: {}", lang))),
    }
}

fn parse_architecture(arch: Option<&str>) -> Result<ArchitecturePattern, AppError> {
    match arch.map(|a| a.to_lowercase()).as_deref() {
        Some("microservices") => Ok(ArchitecturePattern::Microservices),
        Some("monolithic") => Ok(ArchitecturePattern::Monolithic),
        Some("serverless") => Ok(ArchitecturePattern::Serverless),
        Some("event-driven") => Ok(ArchitecturePattern::EventDriven),
        Some("layered") => Ok(ArchitecturePattern::Layered),
        Some("hexagonal") => Ok(ArchitecturePattern::Hexagonal),
        Some("mvc") => Ok(ArchitecturePattern::MVC),
        Some("mvvm") => Ok(ArchitecturePattern::MVVM),
        Some("clean") => Ok(ArchitecturePattern::Clean),
        None => Ok(ArchitecturePattern::Layered), // Default
        Some(other) => Err(AppError::BadRequest(format!("Unknown architecture: {}", other))),
    }
}

fn parse_optimization_level(level: Option<&str>) -> Result<OptimizationLevel, AppError> {
    match level.map(|l| l.to_lowercase()).as_deref() {
        Some("none") => Ok(OptimizationLevel::None),
        Some("basic") => Ok(OptimizationLevel::Basic),
        Some("balanced") => Ok(OptimizationLevel::Balanced),
        Some("performance") => Ok(OptimizationLevel::Performance),
        Some("size") => Ok(OptimizationLevel::Size),
        Some("maximum") => Ok(OptimizationLevel::Maximum),
        None => Ok(OptimizationLevel::Balanced), // Default
        Some(other) => Err(AppError::BadRequest(format!("Unknown optimization level: {}", other))),
    }
}

fn build_constraints(api_constraints: Option<ApiGenerationConstraints>) -> GenerationConstraints {
    if let Some(c) = api_constraints {
        GenerationConstraints {
            max_file_size: c.max_file_size,
            performance_requirements: c.performance_requirements.map(|pr| {
                aion_ai_engine::PerformanceRequirements {
                    max_latency_ms: pr.max_latency_ms,
                    min_throughput_rps: pr.min_throughput_rps,
                    max_memory_mb: pr.max_memory_mb,
                    max_cpu_percent: None,
                }
            }),
            security_requirements: c.security_requirements.map(|sr| {
                aion_ai_engine::SecurityRequirements {
                    encryption_required: sr.encryption_required,
                    authentication_type: sr.authentication_type,
                    authorization_model: None,
                    compliance_standards: sr.compliance_standards,
                }
            }),
            compatibility_requirements: None,
            coding_standards: c.coding_standards.map(|cs| {
                aion_ai_engine::CodingStandards {
                    style_guide: cs.style_guide,
                    max_line_length: cs.max_line_length,
                    indent_size: None,
                    naming_convention: cs.naming_convention,
                }
            }),
        }
    } else {
        GenerationConstraints {
            max_file_size: None,
            performance_requirements: None,
            security_requirements: None,
            compatibility_requirements: None,
            coding_standards: None,
        }
    }
}

fn build_project_context(api_context: Option<ApiProjectContext>) -> Option<ProjectContext> {
    api_context.map(|c| ProjectContext {
        existing_codebase: c.existing_codebase,
        dependencies: c.dependencies,
        project_structure: None,
        domain_knowledge: c.domain_knowledge,
        team_preferences: None,
    })
}

async fn store_generated_code(
    state: &AppState,
    user: &AuthenticatedUser,
    result: &CodeGenerationResult,
) -> Result<String, AppError> {
    // Create archive of generated files
    let archive = create_archive(result)?;

    // Store in storage system
    let url = state.storage
        .store_generation(&result.id, &user.id, archive)
        .await
        .map_err(|e| AppError::Internal(format!("Storage failed: {}", e)))?;

    Ok(url)
}

fn create_archive(result: &CodeGenerationResult) -> Result<Vec<u8>, AppError> {
    // Create ZIP archive of generated files
    use zip::write::FileOptions;
    use std::io::Write;

    let mut buffer = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buffer));
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        for file in &result.generated_files {
            let path = file.path.to_string_lossy();
            zip.start_file(path.as_ref(), options)
                .map_err(|e| AppError::Internal(format!("Archive creation failed: {}", e)))?;
            zip.write_all(file.content.as_bytes())
                .map_err(|e| AppError::Internal(format!("Write failed: {}", e)))?;
        }

        // Add documentation
        zip.start_file("README.md", options)
            .map_err(|e| AppError::Internal(format!("Archive failed: {}", e)))?;
        zip.write_all(result.documentation.readme.as_bytes())
            .map_err(|e| AppError::Internal(format!("Write failed: {}", e)))?;

        zip.finish()
            .map_err(|e| AppError::Internal(format!("Archive finalization failed: {}", e)))?;
    }

    Ok(buffer)
}

fn create_preview(result: &CodeGenerationResult) -> CodePreview {
    let main_file = result.generated_files
        .first()
        .map(|f| f.content.lines().take(50).collect::<Vec<_>>().join("\n"))
        .unwrap_or_else(|| "No files generated".to_string());

    let structure = result.generated_files
        .iter()
        .map(|f| FileInfo {
            path: f.path.to_string_lossy().to_string(),
            language: format!("{:?}", f.language),
            size_bytes: f.content.len(),
            purpose: f.purpose.clone(),
        })
        .collect();

    CodePreview {
        main_file,
        structure,
    }
}

fn estimate_time_saved(result: &CodeGenerationResult) -> f32 {
    // Estimate based on lines of code and complexity
    let base_time = result.metrics.total_lines_of_code as f32 / 50.0; // 50 lines per hour
    let complexity_multiplier = result.metrics.estimated_complexity.maintainability_index / 100.0;
    base_time * (2.0 - complexity_multiplier) // Adjust for complexity
}

async fn track_usage_metrics(
    state: &AppState,
    user: &AuthenticatedUser,
    result: &CodeGenerationResult,
) {
    // Track metrics for billing and analytics
    let _ = state.metrics_tracker
        .track_code_generation(
            &user.id,
            result.metrics.total_lines_of_code,
            result.generated_files.len(),
            result.metrics.generation_time_ms,
        )
        .await;
}

fn create_technical_summary(analysis: &aion_ai_engine::OptimizedRequirements) -> String {
    format!(
        "Analysis identified {} functional requirements, {} user stories, and {} risks. \
         Recommended architecture includes {} components with {} integration points.",
        analysis.parsed_requirements.len(),
        analysis.user_stories.len(),
        analysis.risk_assessment.identified_risks.len(),
        analysis.technical_specifications.architecture_requirements.components.len(),
        analysis.technical_specifications.integration_requirements.external_systems.len()
    )
}

// Request/Response types

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct GenerationStatus {
    pub id: Uuid,
    pub status: String,
    pub progress: f32,
    pub message: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct DocumentationResponse {
    pub readme: String,
    pub api_docs: String,
    pub architecture_docs: String,
    pub setup_guide: String,
}

#[derive(Debug, Serialize)]
pub struct ListGenerationsResponse {
    pub generations: Vec<GenerationSummary>,
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct GenerationSummary {
    pub id: Uuid,
    pub requirements_preview: String,
    pub language: String,
    pub files_count: usize,
    pub lines_of_code: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AnalyzeRequirementsRequest {
    pub requirements: String,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeRequirementsResponse {
    pub id: Uuid,
    pub confidence_score: f32,
    pub user_stories_count: usize,
    pub risks_identified: usize,
    pub implementation_phases: usize,
    pub optimization_suggestions: usize,
    pub technical_summary: String,
}