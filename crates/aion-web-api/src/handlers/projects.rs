//! Project management handlers

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{AppState, models::*};

/// Query parameters for listing projects
#[derive(Deserialize)]
pub struct ProjectQuery {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub language: Option<String>,
    pub status: Option<String>,
}

/// Request for creating a project
#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: String,
    pub language: String,
    pub framework: String,
    pub repository_url: Option<String>,
}

/// Request for updating a project
#[derive(Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub repository_url: Option<String>,
}

/// Response wrapper for projects list
#[derive(Serialize)]
pub struct ProjectsResponse {
    pub projects: Vec<Project>,
}

/// List all projects with filtering
pub async fn list_projects(
    Query(params): Query<ProjectQuery>,
    State(_state): State<AppState>
) -> Result<Json<ProjectsResponse>, StatusCode> {
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(20).min(100);

    println!("📋 Listing projects (offset: {}, limit: {})", offset, limit);

    // Generate sample projects
    let mut projects = Vec::new();
    let project_names = vec![
        ("E-commerce Platform", "Full-featured e-commerce platform with AI recommendations"),
        ("Task Management System", "Collaborative task management with real-time updates"),
        ("Analytics Dashboard", "Real-time analytics dashboard with custom visualizations"),
        ("Social Media API", "RESTful API for social media platform with OAuth integration"),
        ("IoT Data Processor", "Real-time IoT data processing and analytics pipeline"),
        ("Chat Application", "Real-time chat application with file sharing"),
        ("CRM System", "Customer relationship management with automation"),
        ("Blog Engine", "Modern blog engine with SEO optimization"),
        ("Inventory Manager", "Warehouse inventory management system"),
        ("Learning Platform", "Online learning platform with video streaming"),
    ];

    let languages = vec!["Rust", "TypeScript", "Python", "Go", "Java"];
    let frameworks = vec!["Axum", "React", "FastAPI", "Gin", "Spring"];
    let statuses = vec!["active", "inactive", "deploying", "error", "building"];
    let environments = vec!["production", "staging", "development"];
    let visibilities = vec!["public", "private"];
    let tags_list = vec![
        vec!["backend", "api", "database"],
        vec!["frontend", "ui", "react"],
        vec!["analytics", "data", "visualization"],
        vec!["api", "social", "oauth"],
        vec!["iot", "real-time", "data"],
        vec!["chat", "websocket", "real-time"],
        vec!["crm", "automation", "sales"],
        vec!["cms", "seo", "blog"],
        vec!["inventory", "warehouse", "management"],
        vec!["education", "video", "streaming"],
    ];

    for (i, (name, description)) in project_names.iter().enumerate() {
        if i >= offset && projects.len() < limit {
            let created_at = chrono::Utc::now() - chrono::Duration::days((i * 7) as i64);
            let updated_at = chrono::Utc::now() - chrono::Duration::hours((i * 6) as i64);
            let repo_name = name.to_lowercase().replace(' ', "-");

            let project = Project {
                id: Uuid::new_v4(),
                name: name.to_string(),
                description: description.to_string(),
                language: languages[i % languages.len()].to_string(),
                framework: frameworks[i % frameworks.len()].to_string(),
                status: statuses[i % statuses.len()].to_string(),
                created_at,
                updated_at,
                repository_url: Some(format!("https://github.com/ectus-r/{}", repo_name)),
                deployment_count: fastrand::u32(1..10),
                last_deployment: updated_at.to_rfc3339(),
                created_at_iso: created_at.to_rfc3339(),
                repository: format!("ectus-r/{}", repo_name),
                environment: environments[i % environments.len()].to_string(),
                team: vec!["team-member-1".to_string(), "team-member-2".to_string()],
                deployment_url: if i % 2 == 0 {
                    Some(format!("https://{}.ectus.app", repo_name))
                } else {
                    None
                },
                visibility: visibilities[i % visibilities.len()].to_string(),
                tags: tags_list[i % tags_list.len()].iter().map(|s| s.to_string()).collect(),
            };
            projects.push(project);
        }
    }

    // Apply filters
    if let Some(language) = params.language {
        projects.retain(|p| p.language.to_lowercase() == language.to_lowercase());
    }

    if let Some(status) = params.status {
        projects.retain(|p| p.status == status);
    }

    Ok(Json(ProjectsResponse { projects }))
}

/// Create a new project
pub async fn create_project(
    State(_state): State<AppState>,
    Json(request): Json<CreateProjectRequest>
) -> Result<Json<Project>, StatusCode> {
    println!("🆕 Creating new project: {}", request.name);

    let now = chrono::Utc::now();
    let repo_name = request.name.to_lowercase().replace(' ', "-");

    let project = Project {
        id: Uuid::new_v4(),
        name: request.name,
        description: request.description,
        language: request.language.clone(),
        framework: request.framework.clone(),
        status: "active".to_string(),
        created_at: now,
        updated_at: now,
        repository_url: request.repository_url.clone(),
        deployment_count: 0,
        last_deployment: now.to_rfc3339(),
        created_at_iso: now.to_rfc3339(),
        repository: request.repository_url.unwrap_or_else(|| format!("ectus-r/{}", repo_name)),
        environment: "development".to_string(),
        team: vec![],
        deployment_url: None,
        visibility: "private".to_string(),
        tags: vec!["new".to_string()],
    };

    Ok(Json(project))
}

/// Get specific project by ID
pub async fn get_project(
    Path(project_id): Path<Uuid>,
    State(_state): State<AppState>
) -> Result<Json<Project>, StatusCode> {
    println!("📄 Fetching project: {}", project_id);

    let created_at = chrono::Utc::now() - chrono::Duration::days(30);
    let updated_at = chrono::Utc::now() - chrono::Duration::hours(2);

    // Generate sample project data
    let project = Project {
        id: project_id,
        name: "Example Project".to_string(),
        description: "A sample project for demonstration".to_string(),
        language: "Rust".to_string(),
        framework: "Axum".to_string(),
        status: "active".to_string(),
        created_at,
        updated_at,
        repository_url: Some("https://github.com/ectus-r/example-project".to_string()),
        deployment_count: 5,
        last_deployment: updated_at.to_rfc3339(),
        created_at_iso: created_at.to_rfc3339(),
        repository: "ectus-r/example-project".to_string(),
        environment: "production".to_string(),
        team: vec!["alice".to_string(), "bob".to_string()],
        deployment_url: Some("https://example-project.ectus.app".to_string()),
        visibility: "public".to_string(),
        tags: vec!["rust".to_string(), "backend".to_string(), "api".to_string()],
    };

    Ok(Json(project))
}

/// Update a project
pub async fn update_project(
    Path(project_id): Path<Uuid>,
    State(_state): State<AppState>,
    Json(update): Json<UpdateProjectRequest>
) -> Result<Json<Project>, StatusCode> {
    println!("🔄 Updating project: {}", project_id);

    let created_at = chrono::Utc::now() - chrono::Duration::days(30);
    let updated_at = chrono::Utc::now();

    // In a real implementation, this would update the database
    let project = Project {
        id: project_id,
        name: update.name.unwrap_or_else(|| "Updated Project".to_string()),
        description: update.description.unwrap_or_else(|| "Updated description".to_string()),
        language: "Rust".to_string(),
        framework: "Axum".to_string(),
        status: update.status.unwrap_or_else(|| "active".to_string()),
        created_at,
        updated_at,
        repository_url: update.repository_url.clone(),
        deployment_count: 5,
        last_deployment: updated_at.to_rfc3339(),
        created_at_iso: created_at.to_rfc3339(),
        repository: update.repository_url.unwrap_or_else(|| "ectus-r/updated-project".to_string()),
        environment: "production".to_string(),
        team: vec!["alice".to_string()],
        deployment_url: Some("https://updated-project.ectus.app".to_string()),
        visibility: "private".to_string(),
        tags: vec!["updated".to_string()],
    };

    Ok(Json(project))
}

/// Delete a project
pub async fn delete_project(
    Path(project_id): Path<Uuid>,
    State(_state): State<AppState>
) -> Result<StatusCode, StatusCode> {
    println!("🗑️ Deleting project: {}", project_id);

    // In a real implementation, this would delete from database
    Ok(StatusCode::NO_CONTENT)
}