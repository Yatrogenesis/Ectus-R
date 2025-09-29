//! AI-related request handlers

use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::Value;
use crate::{AppState, models::*};

/// Generate code from natural language prompt
pub async fn generate_code(
    State(state): State<AppState>,
    Json(request): Json<GenerateRequest>
) -> Result<Json<GenerateResponse>, StatusCode> {
    println!("🧠 Processing code generation request for: {}", request.prompt);

    match state.ai_service.generate_code(request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            eprintln!("❌ Code generation failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Analyze existing code
pub async fn analyze_code(
    State(state): State<AppState>,
    Json(code): Json<Value>
) -> Result<Json<Value>, StatusCode> {
    let code_str = code.get("code")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    println!("🔍 Processing code analysis request");

    match state.ai_service.analyze_code(code_str).await {
        Ok(analysis) => Ok(Json(analysis)),
        Err(e) => {
            eprintln!("❌ Code analysis failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Fix code issues automatically
pub async fn fix_code(
    State(state): State<AppState>,
    Json(request): Json<Value>
) -> Result<Json<Value>, StatusCode> {
    let code = request.get("code")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let issues: Vec<String> = request.get("issues")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_else(Vec::new);

    println!("🔧 Processing code fix request");

    match state.ai_service.fix_code(code, issues).await {
        Ok(fix_result) => Ok(Json(fix_result)),
        Err(e) => {
            eprintln!("❌ Code fixing failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Refactor code for better structure
pub async fn refactor_code(
    State(state): State<AppState>,
    Json(request): Json<Value>
) -> Result<Json<Value>, StatusCode> {
    let code = request.get("code")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let objectives: Vec<String> = request.get("objectives")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_else(|| vec!["improve_readability".to_string()]);

    println!("🏗️ Processing code refactoring request");

    match state.ai_service.refactor_code(code, objectives).await {
        Ok(refactor_result) => Ok(Json(refactor_result)),
        Err(e) => {
            eprintln!("❌ Code refactoring failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Run autonomous quality assurance
pub async fn run_autonomous_qa(
    State(state): State<AppState>,
    Json(request): Json<Value>
) -> Result<Json<Value>, StatusCode> {
    let code = request.get("code")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    println!("🧪 Running autonomous QA");

    match state.ai_service.run_autonomous_qa(code).await {
        Ok(qa_result) => Ok(Json(qa_result)),
        Err(e) => {
            eprintln!("❌ Autonomous QA failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}