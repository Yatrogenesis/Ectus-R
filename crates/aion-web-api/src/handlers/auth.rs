//! Authentication and authorization handlers

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde_json::Value;
use crate::{AppState, models::*};

/// User login
pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>
) -> Result<Json<LoginResponse>, StatusCode> {
    println!("🔑 Login attempt for user: {}", request.email);

    match state.auth_service.authenticate(&request.email, &request.password).await {
        Ok(response) => {
            println!("✅ Login successful for user: {}", request.email);
            Ok(Json(response))
        },
        Err(e) => {
            println!("❌ Login failed for user: {} - {}", request.email, e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

/// Refresh authentication token
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(request): Json<Value>
) -> Result<Json<Value>, StatusCode> {
    let refresh_token = request.get("refresh_token")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    println!("🔄 Refreshing token");

    match state.auth_service.refresh_token(refresh_token).await {
        Ok(new_token) => Ok(Json(serde_json::json!({
            "access_token": new_token,
            "expires_in": 24 * 3600,
            "timestamp": chrono::Utc::now()
        }))),
        Err(e) => {
            println!("❌ Token refresh failed: {}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

/// User logout
pub async fn logout(
    State(_state): State<AppState>,
    Json(_request): Json<Value>
) -> Result<Json<Value>, StatusCode> {
    println!("👋 User logout");

    // In a real implementation, this would invalidate the token
    Ok(Json(serde_json::json!({
        "message": "Logout successful",
        "timestamp": chrono::Utc::now()
    })))
}