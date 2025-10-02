use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::Instant;

pub async fn request_logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start_time = Instant::now();
    let method = request.method().clone();
    let path = request.uri().path().to_string();

    tracing::info!("Request started: {} {}", method, path);

    let response = next.run(request).await;

    let duration = start_time.elapsed();
    let status = response.status();

    tracing::info!(
        "Request completed: {} {} - {} ({:.2}ms)",
        method,
        path,
        status,
        duration.as_secs_f64() * 1000.0
    );

    response
}

pub async fn security_headers_middleware(
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;

    let mut response = response;
    let headers = response.headers_mut();

    // Add security headers
    headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    headers.insert("X-Frame-Options", "DENY".parse().unwrap());
    headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
    headers.insert("Strict-Transport-Security", "max-age=31536000; includeSubDomains".parse().unwrap());
    headers.insert("Referrer-Policy", "strict-origin-when-cross-origin".parse().unwrap());

    response
}

pub async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    // Generate or extract request ID
    let request_id_header = request
        .headers()
        .get("X-Request-ID")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    
    let request_id = request_id_header.unwrap_or_else(|| {
        let id = uuid::Uuid::new_v4().to_string();
        request.headers_mut().insert("X-Request-ID", id.parse().unwrap());
        request.headers().get("X-Request-ID").unwrap().to_str().unwrap().to_string()
    });

    // Add to response headers
    let response = next.run(request).await;
    let mut response = response;
    response.headers_mut().insert("X-Request-ID", request_id.parse().unwrap());

    response
}


pub async fn compression_middleware(
    request: Request,
    next: Next,
) -> Response {
    // In a real implementation, this would handle compression
    // For now, just pass through
    next.run(request).await
}