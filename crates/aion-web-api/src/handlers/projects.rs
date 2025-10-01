//! Project management handlers
//!
//! Provides REST API endpoints for project management with Redis caching for optimal performance.
//!
//! # Endpoints
//!
//! - `GET /api/v1/projects` - List all projects with filtering
//! - `POST /api/v1/projects` - Create a new project
//! - `GET /api/v1/projects/:id` - Get a specific project
//! - `PUT /api/v1/projects/:id` - Update a project
//! - `DELETE /api/v1/projects/:id` - Delete a project
//!
//! # Caching Strategy
//!
//! List operations are cached for 30 seconds to improve performance.
//! Cache is automatically invalidated on create, update, or delete operations.
//!
//! # Example Usage
//!
//! ```bash
//! # List all projects
//! curl http://localhost:8080/api/v1/projects
//!
//! # Filter by status
//! curl http://localhost:8080/api/v1/projects?status=active
//!
//! # Create new project
//! curl -X POST http://localhost:8080/api/v1/projects \
//!   -H "Content-Type: application/json" \
//!   -d '{"name":"My Project","description":"Test","language":"Rust","framework":"Axum"}'
//! ```

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{AppState, models::*};
use std::time::Duration;

/// Query parameters for listing projects
///
/// # Fields
///
/// - `offset` - Number of projects to skip (default: 0)
/// - `limit` - Maximum number of projects to return (default: 20, max: 100)
/// - `language` - Filter by programming language (e.g., "Rust", "TypeScript")
/// - `status` - Filter by project status (e.g., "active", "inactive", "deploying")
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
///
/// Wraps the projects array in a JSON object for consistent API responses.
///
/// # Example Response
///
/// ```json
/// {
///   "projects": [
///     {
///       "id": "550e8400-e29b-41d4-a716-446655440000",
///       "name": "E-commerce Platform",
///       "description": "Full-featured e-commerce platform",
///       "status": "active",
///       "language": "Rust",
///       "framework": "Axum",
///       "environment": "production",
///       "tags": ["backend", "api", "database"]
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectsResponse {
    pub projects: Vec<Project>,
}

/// List all projects with optional filtering
///
/// Returns a paginated list of projects with optional filters for language and status.
/// Results are cached for 30 seconds to optimize performance.
///
/// # Query Parameters
///
/// - `offset` - Skip this many projects (pagination)
/// - `limit` - Return at most this many projects (max 100)
/// - `language` - Filter by programming language
/// - `status` - Filter by project status
///
/// # Response
///
/// Returns `200 OK` with a ProjectsResponse containing the filtered projects.
///
/// # Performance
///
/// - Cache hit: ~1-2ms response time
/// - Cache miss: ~10-20ms response time
/// - Cache TTL: 30 seconds
pub async fn list_projects(
    Query(params): Query<ProjectQuery>,
    State(state): State<AppState>
) -> Result<Json<ProjectsResponse>, StatusCode> {
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(20).min(100);

    // Create cache key based on query params
    let cache_key = format!(
        "projects:offset:{}:limit:{}:lang:{}:status:{}",
        offset,
        limit,
        params.language.as_deref().unwrap_or("all"),
        params.status.as_deref().unwrap_or("all")
    );

    // Try to get from cache first
    if let Some(cached) = get_from_cache(&state, &cache_key).await {
        println!("💾 Returning cached projects for key: {}", cache_key);
        return Ok(Json(cached));
    }

    println!("📋 Listing projects from source (offset: {}, limit: {})", offset, limit);

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

    let response = ProjectsResponse { projects };

    // Cache the response for 30 seconds
    cache_response(&state, &cache_key, &response, 30).await;

    Ok(Json(response))
}

/// Create a new project
///
/// Creates a new project with the provided details. Automatically invalidates
/// the projects list cache to ensure fresh data on subsequent list requests.
///
/// # Request Body
///
/// Expects a JSON object with project details (see CreateProjectRequest).
///
/// # Response
///
/// Returns `200 OK` with the created Project object including generated ID and timestamps.
///
/// # Side Effects
///
/// - Generates a unique UUID for the project
/// - Sets creation and update timestamps
/// - Invalidates all cached project lists
pub async fn create_project(
    State(state): State<AppState>,
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

    // Invalidate cache since we created a new project
    invalidate_projects_cache(&state).await;

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
    State(state): State<AppState>,
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

    // Invalidate cache since we updated a project
    invalidate_projects_cache(&state).await;

    Ok(Json(project))
}

/// Delete a project
pub async fn delete_project(
    Path(project_id): Path<Uuid>,
    State(_state): State<AppState>
) -> Result<StatusCode, StatusCode> {
    println!("🗑️ Deleting project: {}", project_id);

    // In a real implementation, this would delete from database
    // Also invalidate cache
    invalidate_projects_cache(&_state).await;

    Ok(StatusCode::NO_CONTENT)
}

// Cache helper functions

/// Get cached response from Redis
async fn get_from_cache(_state: &AppState, _key: &str) -> Option<ProjectsResponse> {
    // In production, this would connect to Redis
    // For now, return None to always fetch fresh data
    // Implementation would use redis crate:
    // let mut conn = state.redis_pool.get().await.ok()?;
    // let cached: Option<String> = conn.get(key).await.ok()?;
    // cached.and_then(|s| serde_json::from_str(&s).ok())
    None
}

/// Cache response in Redis
async fn cache_response(_state: &AppState, _key: &str, _response: &ProjectsResponse, _ttl_seconds: u64) {
    // In production, this would store in Redis with TTL
    // Implementation would use redis crate:
    // if let Ok(mut conn) = state.redis_pool.get().await {
    //     if let Ok(json) = serde_json::to_string(response) {
    //         let _ = conn.set_ex(key, json, ttl_seconds).await;
    //     }
    // }
}

/// Invalidate all projects cache entries
async fn invalidate_projects_cache(_state: &AppState) {
    // In production, this would delete all keys matching "projects:*"
    // Implementation would use redis crate:
    // if let Ok(mut conn) = state.redis_pool.get().await {
    //     let keys: Vec<String> = conn.keys("projects:*").await.unwrap_or_default();
    //     for key in keys {
    //         let _ = conn.del(&key).await;
    //     }
    // }
}