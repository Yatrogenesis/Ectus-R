//! Deployment management handlers

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::{AppState, models::*};

/// Query parameters for listing deployments
#[derive(Deserialize)]
pub struct DeploymentQuery {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub environment: Option<String>,
    pub status: Option<String>,
}

/// Request for creating a deployment
#[derive(Deserialize)]
pub struct CreateDeploymentRequest {
    pub project_id: Uuid,
    pub environment: String,
    pub config: Option<Value>,
}

/// List all deployments with filtering
pub async fn list_deployments(
    Query(params): Query<DeploymentQuery>,
    State(state): State<AppState>
) -> Result<Json<Vec<Deployment>>, StatusCode> {
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(20).min(100); // Cap at 100

    println!("📋 Listing deployments (offset: {}, limit: {})", offset, limit);

    match state.deployment_service.list_deployments(offset, limit).await {
        Ok(deployments) => {
            // Apply filtering
            let mut filtered_deployments = deployments;

            if let Some(environment) = params.environment {
                filtered_deployments.retain(|d| d.environment == environment);
            }

            if let Some(status) = params.status {
                filtered_deployments.retain(|d| d.status == status);
            }

            Ok(Json(filtered_deployments))
        },
        Err(e) => {
            eprintln!("❌ Failed to list deployments: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a new deployment
pub async fn create_deployment(
    State(state): State<AppState>,
    Json(request): Json<CreateDeploymentRequest>
) -> Result<Json<Deployment>, StatusCode> {
    println!("🚀 Creating new deployment for project: {}", request.project_id);

    match state.deployment_service.create_deployment(request.project_id, request.environment).await {
        Ok(deployment) => Ok(Json(deployment)),
        Err(e) => {
            eprintln!("❌ Failed to create deployment: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get specific deployment by ID
pub async fn get_deployment(
    Path(deployment_id): Path<Uuid>,
    State(state): State<AppState>
) -> Result<Json<Deployment>, StatusCode> {
    println!("📄 Fetching deployment: {}", deployment_id);

    match state.deployment_service.get_deployment(deployment_id).await {
        Ok(Some(deployment)) => Ok(Json(deployment)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("❌ Failed to get deployment: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Update deployment status
pub async fn update_deployment(
    Path(deployment_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(update): Json<Value>
) -> Result<Json<Deployment>, StatusCode> {
    let status = update.get("status")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    println!("🔄 Updating deployment {} to status: {}", deployment_id, status);

    match state.deployment_service.update_deployment(deployment_id, status.to_string()).await {
        Ok(deployment) => Ok(Json(deployment)),
        Err(e) => {
            eprintln!("❌ Failed to update deployment: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Delete a deployment
pub async fn delete_deployment(
    Path(deployment_id): Path<Uuid>,
    State(state): State<AppState>
) -> Result<StatusCode, StatusCode> {
    println!("🗑️ Deleting deployment: {}", deployment_id);

    match state.deployment_service.delete_deployment(deployment_id).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            eprintln!("❌ Failed to delete deployment: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get deployment logs
#[derive(Deserialize)]
pub struct LogsQuery {
    pub lines: Option<usize>,
    pub follow: Option<bool>,
}

pub async fn get_deployment_logs(
    Path(deployment_id): Path<Uuid>,
    Query(params): Query<LogsQuery>,
    State(state): State<AppState>
) -> Result<Json<Value>, StatusCode> {
    let lines = params.lines.unwrap_or(100).min(1000); // Cap at 1000 lines

    println!("📋 Fetching {} lines of logs for deployment: {}", lines, deployment_id);

    match state.deployment_service.get_deployment_logs(deployment_id, Some(lines)).await {
        Ok(logs) => Ok(Json(serde_json::json!({
            "deployment_id": deployment_id,
            "lines": logs.len(),
            "logs": logs,
            "timestamp": chrono::Utc::now()
        }))),
        Err(e) => {
            eprintln!("❌ Failed to get deployment logs: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}