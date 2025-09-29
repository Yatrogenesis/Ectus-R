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

/// List all projects with filtering
pub async fn list_projects(
    Query(params): Query<ProjectQuery>,
    State(_state): State<AppState>
) -> Result<Json<Vec<Project>>, StatusCode> {
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
    let statuses = vec!["active", "completed", "paused", "archived"];

    for (i, (name, description)) in project_names.iter().enumerate() {
        if i >= offset && projects.len() < limit {
            let project = Project {
                id: Uuid::new_v4(),
                name: name.to_string(),
                description: description.to_string(),
                language: languages[i % languages.len()].to_string(),
                framework: frameworks[i % frameworks.len()].to_string(),
                status: statuses[i % statuses.len()].to_string(),
                created_at: chrono::Utc::now() - chrono::Duration::days((i * 7) as i64),
                updated_at: chrono::Utc::now() - chrono::Duration::hours((i * 6) as i64),
                repository_url: Some(format!("https://github.com/ectus-r/{}", name.to_lowercase().replace(' ', "-"))),
                deployment_count: fastrand::u32(1..10),
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

    Ok(Json(projects))
}

/// Create a new project
pub async fn create_project(
    State(_state): State<AppState>,
    Json(request): Json<CreateProjectRequest>
) -> Result<Json<Project>, StatusCode> {
    println!("🆕 Creating new project: {}", request.name);

    let project = Project {
        id: Uuid::new_v4(),
        name: request.name,
        description: request.description,
        language: request.language,
        framework: request.framework,
        status: "active".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        repository_url: request.repository_url,
        deployment_count: 0,
    };

    Ok(Json(project))
}

/// Get specific project by ID
pub async fn get_project(
    Path(project_id): Path<Uuid>,
    State(_state): State<AppState>
) -> Result<Json<Project>, StatusCode> {
    println!("📄 Fetching project: {}", project_id);

    // Generate sample project data
    let project = Project {
        id: project_id,
        name: "Example Project".to_string(),
        description: "A sample project for demonstration".to_string(),
        language: "Rust".to_string(),
        framework: "Axum".to_string(),
        status: "active".to_string(),
        created_at: chrono::Utc::now() - chrono::Duration::days(30),
        updated_at: chrono::Utc::now() - chrono::Duration::hours(2),
        repository_url: Some("https://github.com/ectus-r/example-project".to_string()),
        deployment_count: 5,
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

    // In a real implementation, this would update the database
    let project = Project {
        id: project_id,
        name: update.name.unwrap_or_else(|| "Updated Project".to_string()),
        description: update.description.unwrap_or_else(|| "Updated description".to_string()),
        language: "Rust".to_string(),
        framework: "Axum".to_string(),
        status: update.status.unwrap_or_else(|| "active".to_string()),
        created_at: chrono::Utc::now() - chrono::Duration::days(30),
        updated_at: chrono::Utc::now(),
        repository_url: update.repository_url,
        deployment_count: 5,
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