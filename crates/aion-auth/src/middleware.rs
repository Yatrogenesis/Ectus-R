use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

pub struct AuthMiddleware {
    // JWT service would be injected here
}

impl AuthMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Validate Bearer token
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix

    // In production, validate JWT token here
    if token.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Continue to next middleware/handler
    Ok(next.run(request).await)
}

pub async fn cors_middleware(
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;

    // Add CORS headers
    let mut response = response;
    let headers = response.headers_mut();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "Content-Type, Authorization".parse().unwrap());

    response
}