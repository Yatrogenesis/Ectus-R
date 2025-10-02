//! Security Middleware and Rate Limiting
//! Production-grade security features for API protection

use axum::{
    extract::{Request, ConnectInfo},
    http::{StatusCode, HeaderValue, HeaderMap},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::time::sleep;
use tower::ServiceBuilder;
use tracing::{warn, info, error};
use uuid::Uuid;

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_capacity: u32,
    pub cleanup_interval: Duration,
    pub ban_duration: Duration,
    pub suspicious_threshold: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 100,
            burst_capacity: 20,
            cleanup_interval: Duration::from_secs(60),
            ban_duration: Duration::from_secs(300), // 5 minutes
            suspicious_threshold: 1000, // Requests per minute that trigger suspicion
        }
    }
}

/// Rate limit bucket for token bucket algorithm
#[derive(Debug, Clone)]
pub struct RateLimitBucket {
    pub tokens: u32,
    pub last_refill: Instant,
    pub request_count: u32,
    pub first_request: Instant,
    pub banned_until: Option<Instant>,
    pub suspicious_activity: bool,
}

impl RateLimitBucket {
    pub fn new(capacity: u32) -> Self {
        Self {
            tokens: capacity,
            last_refill: Instant::now(),
            request_count: 0,
            first_request: Instant::now(),
            banned_until: None,
            suspicious_activity: false,
        }
    }

    pub fn is_banned(&self) -> bool {
        if let Some(banned_until) = self.banned_until {
            Instant::now() < banned_until
        } else {
            false
        }
    }

    pub fn refill(&mut self, config: &RateLimitConfig) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);

        if elapsed >= Duration::from_secs(1) {
            let tokens_to_add = (elapsed.as_secs() * config.requests_per_minute as u64 / 60) as u32;
            self.tokens = (self.tokens + tokens_to_add).min(config.burst_capacity);
            self.last_refill = now;
        }

        // Reset request count every minute
        if now.duration_since(self.first_request) >= Duration::from_secs(60) {
            if self.request_count > config.suspicious_threshold {
                self.suspicious_activity = true;
                self.banned_until = Some(now + config.ban_duration);
                warn!("IP banned due to suspicious activity: {} requests in 1 minute", self.request_count);
            }

            self.request_count = 0;
            self.first_request = now;
        }
    }

    pub fn consume(&mut self) -> bool {
        if self.tokens > 0 {
            self.tokens -= 1;
            self.request_count += 1;
            true
        } else {
            false
        }
    }
}

/// Global rate limiter with IP-based tracking
pub struct RateLimiter {
    buckets: Arc<Mutex<HashMap<IpAddr, RateLimitBucket>>>,
    config: RateLimitConfig,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    pub fn check_rate_limit(&self, ip: IpAddr) -> bool {
        let mut buckets = self.buckets.lock().unwrap();

        let bucket = buckets
            .entry(ip)
            .or_insert_with(|| RateLimitBucket::new(self.config.burst_capacity));

        bucket.refill(&self.config);

        if bucket.is_banned() {
            return false;
        }

        bucket.consume()
    }

    pub fn cleanup_old_buckets(&self) {
        let mut buckets = self.buckets.lock().unwrap();
        let now = Instant::now();

        buckets.retain(|_, bucket| {
            now.duration_since(bucket.last_refill) < self.config.cleanup_interval * 2
        });
    }
}

/// Security headers middleware
pub async fn security_headers_middleware(
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    // Security headers
    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        "X-Frame-Options",
        HeaderValue::from_static("DENY"),
    );
    headers.insert(
        "X-XSS-Protection",
        HeaderValue::from_static("1; mode=block"),
    );
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        "Permissions-Policy",
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );
    headers.insert(
        "Content-Security-Policy",
        HeaderValue::from_static("default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self'; connect-src 'self'; frame-ancestors 'none'"),
    );

    // HSTS for HTTPS
    if request.headers().get("x-forwarded-proto").map(|h| h.as_bytes()) == Some(b"https") {
        headers.insert(
            "Strict-Transport-Security",
            HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
        );
    }

    response
}

/// Rate limiting middleware
pub async fn rate_limiting_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // Extract real IP from headers (for reverse proxy scenarios)
    let ip = extract_real_ip(&request, addr.ip());

    // Check rate limit
    let rate_limiter = request
        .extensions()
        .get::<Arc<RateLimiter>>()
        .expect("RateLimiter not found in request extensions");

    if !rate_limiter.check_rate_limit(ip) {
        warn!("Rate limit exceeded for IP: {}", ip);

        let error_response = serde_json::json!({
            "error": "RATE_LIMIT_EXCEEDED",
            "message": "Too many requests. Please try again later.",
            "retry_after": 60,
            "ip": ip.to_string(),
            "timestamp": chrono::Utc::now()
        });

        return Err((StatusCode::TOO_MANY_REQUESTS, Json(error_response)));
    }

    Ok(next.run(request).await)
}

/// Extract real IP address from request headers
fn extract_real_ip(request: &Request, fallback_ip: IpAddr) -> IpAddr {
    let headers = request.headers();

    // Check X-Forwarded-For header
    if let Some(xff) = headers.get("x-forwarded-for") {
        if let Ok(xff_str) = xff.to_str() {
            if let Some(first_ip) = xff_str.split(',').next() {
                if let Ok(ip) = first_ip.trim().parse::<IpAddr>() {
                    return ip;
                }
            }
        }
    }

    // Check X-Real-IP header
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                return ip;
            }
        }
    }

    // Check CF-Connecting-IP header (Cloudflare)
    if let Some(cf_ip) = headers.get("cf-connecting-ip") {
        if let Ok(ip_str) = cf_ip.to_str() {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                return ip;
            }
        }
    }

    fallback_ip
}

/// Input validation middleware
pub async fn input_validation_middleware(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // Validate request size
    if let Some(content_length) = request.headers().get("content-length") {
        if let Ok(length_str) = content_length.to_str() {
            if let Ok(length) = length_str.parse::<usize>() {
                if length > 50 * 1024 * 1024 { // 50MB limit
                    let error_response = serde_json::json!({
                        "error": "REQUEST_TOO_LARGE",
                        "message": "Request body too large. Maximum size is 50MB.",
                        "max_size": "50MB"
                    });

                    return Err((StatusCode::PAYLOAD_TOO_LARGE, Json(error_response)));
                }
            }
        }
    }

    // Validate content type for POST/PUT requests
    let method = request.method();
    if method == "POST" || method == "PUT" || method == "PATCH" {
        if let Some(content_type) = request.headers().get("content-type") {
            if let Ok(ct_str) = content_type.to_str() {
                if !ct_str.starts_with("application/json") &&
                   !ct_str.starts_with("application/x-www-form-urlencoded") &&
                   !ct_str.starts_with("multipart/form-data") {
                    let error_response = serde_json::json!({
                        "error": "INVALID_CONTENT_TYPE",
                        "message": "Unsupported content type. Use application/json, application/x-www-form-urlencoded, or multipart/form-data.",
                        "received": ct_str
                    });

                    return Err((StatusCode::UNSUPPORTED_MEDIA_TYPE, Json(error_response)));
                }
            }
        }
    }

    Ok(next.run(request).await)
}

/// Anti-CSRF middleware
pub async fn csrf_protection_middleware(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let method = request.method();

    // Only check CSRF for state-changing methods
    if method == "POST" || method == "PUT" || method == "PATCH" || method == "DELETE" {
        let headers = request.headers();

        // Check for CSRF token in header
        let csrf_token = headers
            .get("x-csrf-token")
            .or_else(|| headers.get("x-xsrf-token"));

        // For API endpoints, we can be more lenient with CSRF if proper authentication is present
        let has_auth = headers.get("authorization").is_some();

        // If no auth and no CSRF token, reject
        if !has_auth && csrf_token.is_none() {
            let error_response = serde_json::json!({
                "error": "CSRF_TOKEN_REQUIRED",
                "message": "CSRF token required for this request. Include X-CSRF-Token header.",
                "method": method.as_str()
            });

            return Err((StatusCode::FORBIDDEN, Json(error_response)));
        }

        // Validate CSRF token if present
        if let Some(token) = csrf_token {
            if let Ok(token_str) = token.to_str() {
                if !validate_csrf_token(token_str) {
                    let error_response = serde_json::json!({
                        "error": "INVALID_CSRF_TOKEN",
                        "message": "Invalid CSRF token provided."
                    });

                    return Err((StatusCode::FORBIDDEN, Json(error_response)));
                }
            }
        }
    }

    Ok(next.run(request).await)
}

/// Validate CSRF token (simplified version)
fn validate_csrf_token(token: &str) -> bool {
    // In a real implementation, this would:
    // 1. Decode the token
    // 2. Verify the signature
    // 3. Check expiration
    // 4. Validate against session

    // For now, just check basic format
    token.len() >= 32 && token.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

/// Request logging and monitoring middleware
pub async fn request_monitoring_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let user_agent = request
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    let ip = extract_real_ip(&request, addr.ip());
    let start_time = Instant::now();
    let request_id = Uuid::new_v4();

    // Log request start
    info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        ip = %ip,
        user_agent = %user_agent,
        "Request started"
    );

    let response = next.run(request).await;
    let duration = start_time.elapsed();

    // Log request completion
    info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        ip = %ip,
        status = %response.status(),
        duration_ms = duration.as_millis(),
        "Request completed"
    );

    // Add request ID to response headers
    let mut response = response;
    response.headers_mut().insert(
        "x-request-id",
        HeaderValue::from_str(&request_id.to_string()).unwrap_or(HeaderValue::from_static("invalid")),
    );

    response
}

/// Security configuration for the application
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub rate_limit: RateLimitConfig,
    pub enable_csrf: bool,
    pub enable_rate_limiting: bool,
    pub enable_request_logging: bool,
    pub enable_input_validation: bool,
    pub allowed_origins: Vec<String>,
    pub max_request_size: usize,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            rate_limit: RateLimitConfig::default(),
            enable_csrf: true,
            enable_rate_limiting: true,
            enable_request_logging: true,
            enable_input_validation: true,
            allowed_origins: vec![
                "https://dashboard.ectus.ai".to_string(),
                "https://ectus.ai".to_string(),
                "http://localhost:3000".to_string(), // Development
            ],
            max_request_size: 50 * 1024 * 1024, // 50MB
        }
    }
}

/// Build security middleware stack
pub fn build_security_middleware(config: SecurityConfig) -> ServiceBuilder<
    tower::layer::util::Stack<
        axum::middleware::FromFnLayer<fn(Request, Next) -> impl std::future::Future<Output = Response>>,
        tower::layer::util::Stack<
            axum::middleware::FromFnLayer<fn(ConnectInfo<SocketAddr>, Request, Next) -> impl std::future::Future<Output = Response>>,
            tower::layer::util::Identity
        >
    >
> {
    let mut builder = ServiceBuilder::new();

    if config.enable_request_logging {
        builder = builder.layer(axum::middleware::from_fn(request_monitoring_middleware));
    }

    builder = builder.layer(axum::middleware::from_fn(security_headers_middleware));

    if config.enable_input_validation {
        builder = builder.layer(axum::middleware::from_fn(input_validation_middleware));
    }

    if config.enable_csrf {
        builder = builder.layer(axum::middleware::from_fn(csrf_protection_middleware));
    }

    if config.enable_rate_limiting {
        builder = builder.layer(axum::middleware::from_fn(rate_limiting_middleware));
    }

    builder
}