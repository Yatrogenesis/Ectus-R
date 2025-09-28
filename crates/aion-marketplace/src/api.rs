use crate::{
    core::Marketplace,
    models::*,
    errors::*,
    PackageType,
};
use axum::{
    extract::{Path, Query, State, Multipart},
    http::{StatusCode, HeaderMap},
    response::{Json, IntoResponse},
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    compression::CompressionLayer,
    limit::RequestBodyLimitLayer,
};

/// API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// API state shared across handlers
#[derive(Clone)]
pub struct ApiState {
    pub marketplace: Arc<Marketplace>,
}

/// Create the marketplace API router
pub fn create_api_router(marketplace: Arc<Marketplace>) -> Router {
    let state = ApiState { marketplace };

    Router::new()
        // Package routes
        .route("/packages", get(list_packages).post(publish_package))
        .route("/packages/:id", get(get_package).put(update_package).delete(delete_package))
        .route("/packages/:id/download", get(download_package))
        .route("/packages/:id/install", post(install_package))
        .route("/packages/:id/versions", get(get_package_versions))
        .route("/packages/:id/reviews", get(get_package_reviews).post(submit_review))
        .route("/packages/search", get(search_packages))
        .route("/packages/featured", get(get_featured_packages))
        .route("/packages/popular", get(get_popular_packages))
        .route("/packages/recent", get(get_recent_packages))

        // User routes
        .route("/users/:id", get(get_user).put(update_user))
        .route("/users/:id/packages", get(get_user_packages))
        .route("/users/:id/reviews", get(get_user_reviews))
        .route("/users/:id/installations", get(get_user_installations))
        .route("/users/me", get(get_current_user))

        // Category and tag routes
        .route("/categories", get(list_categories))
        .route("/tags", get(list_tags))
        .route("/categories/:category/packages", get(get_packages_by_category))

        // Statistics routes
        .route("/stats", get(get_marketplace_stats))
        .route("/stats/trending", get(get_trending_packages))

        // Payment routes
        .route("/payments/process", post(process_payment))
        .route("/payments/webhooks/stripe", post(stripe_webhook))

        // Admin routes
        .route("/admin/packages/:id/moderate", post(moderate_package))
        .route("/admin/users/:id/verify", post(verify_user))
        .route("/admin/reviews/:id/moderate", post(moderate_review))

        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(CompressionLayer::new())
                .layer(RequestBodyLimitLayer::new(100 * 1024 * 1024)) // 100MB limit
        )
        .with_state(state)
}

/// List packages with filtering and pagination
async fn list_packages(
    State(state): State<ApiState>,
    Query(params): Query<ListPackagesParams>,
) -> impl IntoResponse {
    let filters = SearchFilters {
        package_type: params.package_type,
        tags: params.tags.unwrap_or_default(),
        categories: params.categories.unwrap_or_default(),
        min_rating: params.min_rating,
        verified_only: params.verified_only.unwrap_or(false),
        free_only: params.free_only.unwrap_or(false),
        ..Default::default()
    };

    let pagination = PaginationParams {
        page: params.page.unwrap_or(1),
        per_page: params.per_page.unwrap_or(20).min(100), // Cap at 100
        sort_by: params.sort_by.unwrap_or(SortField::Downloads),
        sort_order: params.sort_order.unwrap_or(SortOrder::Descending),
    };

    match state.marketplace.search_packages("", Some(filters), Some(pagination)).await {
        Ok(results) => (StatusCode::OK, Json(ApiResponse::success(results))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(e.to_string()))),
    }
}

/// Search packages
async fn search_packages(
    State(state): State<ApiState>,
    Query(params): Query<SearchPackagesParams>,
) -> impl IntoResponse {
    let query = params.q.unwrap_or_default();

    let filters = SearchFilters {
        package_type: params.package_type,
        tags: params.tags.unwrap_or_default(),
        categories: params.categories.unwrap_or_default(),
        min_rating: params.min_rating,
        verified_only: params.verified_only.unwrap_or(false),
        free_only: params.free_only.unwrap_or(false),
        ..Default::default()
    };

    let pagination = PaginationParams {
        page: params.page.unwrap_or(1),
        per_page: params.per_page.unwrap_or(20).min(100),
        sort_by: params.sort_by.unwrap_or(SortField::Relevance),
        sort_order: params.sort_order.unwrap_or(SortOrder::Descending),
    };

    match state.marketplace.search_packages(&query, Some(filters), Some(pagination)).await {
        Ok(results) => (StatusCode::OK, Json(ApiResponse::success(results))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(e.to_string()))),
    }
}

/// Get package details
async fn get_package(
    State(state): State<ApiState>,
    Path(package_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.marketplace.get_package(package_id).await {
        Ok(package) => (StatusCode::OK, Json(ApiResponse::success(package))),
        Err(MarketplaceError::PackageNotFound(_)) => {
            (StatusCode::NOT_FOUND, Json(ApiResponse::error("Package not found".to_string())))
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(e.to_string()))),
    }
}

/// Publish a new package
async fn publish_package(
    State(state): State<ApiState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // Extract user ID from authorization header (simplified)
    let publisher_id = match extract_user_id_from_headers(&headers) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(ApiResponse::error("Unauthorized".to_string()))),
    };

    let mut package_data: Option<CreatePackageRequest> = None;
    let mut package_files: Option<Vec<u8>> = None;

    // Process multipart form data
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "metadata" => {
                let data = field.bytes().await.unwrap_or_default();
                package_data = serde_json::from_slice(&data).ok();
            },
            "package" => {
                package_files = Some(field.bytes().await.unwrap_or_default().to_vec());
            },
            _ => {
                // Ignore unknown fields
            }
        }
    }

    let package_data = match package_data {
        Some(data) => data,
        None => return (StatusCode::BAD_REQUEST, Json(ApiResponse::error("Missing package metadata".to_string()))),
    };

    let package_files = match package_files {
        Some(files) => files,
        None => return (StatusCode::BAD_REQUEST, Json(ApiResponse::error("Missing package files".to_string()))),
    };

    match state.marketplace.publish_package(publisher_id, package_data, package_files).await {
        Ok(package) => (StatusCode::CREATED, Json(ApiResponse::success(package))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::error(e.to_string()))),
    }
}

/// Download a package
async fn download_package(
    State(state): State<ApiState>,
    Path(package_id): Path<Uuid>,
    Query(params): Query<DownloadParams>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let downloader_id = extract_user_id_from_headers(&headers);

    match state.marketplace.download_package(package_id, params.version.as_deref(), downloader_id).await {
        Ok(download) => (StatusCode::OK, Json(ApiResponse::success(download))),
        Err(MarketplaceError::PackageNotFound(_)) => {
            (StatusCode::NOT_FOUND, Json(ApiResponse::error("Package not found".to_string())))
        },
        Err(MarketplaceError::AccessDenied) => {
            (StatusCode::FORBIDDEN, Json(ApiResponse::error("Access denied".to_string())))
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(e.to_string()))),
    }
}

/// Install a package
async fn install_package(
    State(state): State<ApiState>,
    Path(package_id): Path<Uuid>,
    Query(params): Query<InstallParams>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let installer_id = match extract_user_id_from_headers(&headers) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(ApiResponse::error("Unauthorized".to_string()))),
    };

    // Get package to extract name
    let package = match state.marketplace.get_package(package_id).await {
        Ok(p) => p,
        Err(_) => return (StatusCode::NOT_FOUND, Json(ApiResponse::error("Package not found".to_string()))),
    };

    match state.marketplace.install_package(&package.name, params.version.as_deref(), installer_id).await {
        Ok(result) => (StatusCode::OK, Json(ApiResponse::success(result))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::error(e.to_string()))),
    }
}

/// Submit a package review
async fn submit_review(
    State(state): State<ApiState>,
    Path(package_id): Path<Uuid>,
    headers: HeaderMap,
    Json(review_data): Json<CreateReviewRequest>,
) -> impl IntoResponse {
    let reviewer_id = match extract_user_id_from_headers(&headers) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(ApiResponse::error("Unauthorized".to_string()))),
    };

    match state.marketplace.submit_review(package_id, reviewer_id, review_data).await {
        Ok(review) => (StatusCode::CREATED, Json(ApiResponse::success(review))),
        Err(MarketplaceError::AlreadyReviewed) => {
            (StatusCode::CONFLICT, Json(ApiResponse::error("Already reviewed this package".to_string())))
        },
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::error(e.to_string()))),
    }
}

/// Get marketplace statistics
async fn get_marketplace_stats(
    State(state): State<ApiState>,
) -> impl IntoResponse {
    match state.marketplace.get_stats().await {
        Ok(stats) => (StatusCode::OK, Json(ApiResponse::success(stats))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(e.to_string()))),
    }
}

/// Process payment for premium package
async fn process_payment(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Json(payment_request): Json<ProcessPaymentRequest>,
) -> impl IntoResponse {
    let buyer_id = match extract_user_id_from_headers(&headers) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(ApiResponse::error("Unauthorized".to_string()))),
    };

    match state.marketplace.process_payment(payment_request.package_id, buyer_id, payment_request.payment_method).await {
        Ok(result) => (StatusCode::OK, Json(ApiResponse::success(result))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::error(e.to_string()))),
    }
}

// Placeholder implementations for remaining handlers
async fn update_package(State(_state): State<ApiState>, Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn delete_package(State(_state): State<ApiState>, Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn get_package_versions(State(_state): State<ApiState>, Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn get_package_reviews(State(_state): State<ApiState>, Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn get_featured_packages(State(_state): State<ApiState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn get_popular_packages(State(_state): State<ApiState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn get_recent_packages(State(_state): State<ApiState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn get_user(State(_state): State<ApiState>, Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn update_user(State(_state): State<ApiState>, Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn get_user_packages(State(_state): State<ApiState>, Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn get_user_reviews(State(_state): State<ApiState>, Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn get_user_installations(State(_state): State<ApiState>, Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn get_current_user(State(_state): State<ApiState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn list_categories(State(_state): State<ApiState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn list_tags(State(_state): State<ApiState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn get_packages_by_category(State(_state): State<ApiState>, Path(_category): Path<String>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn get_trending_packages(State(_state): State<ApiState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn stripe_webhook(State(_state): State<ApiState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn moderate_package(State(_state): State<ApiState>, Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn verify_user(State(_state): State<ApiState>, Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

async fn moderate_review(State(_state): State<ApiState>, Path(_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::error("Not implemented".to_string())))
}

/// Extract user ID from authorization headers
fn extract_user_id_from_headers(headers: &HeaderMap) -> Option<Uuid> {
    // Simplified - in production this would validate JWT tokens
    headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .and_then(|token| Uuid::parse_str(token).ok())
}

/// Query parameters for listing packages
#[derive(Debug, Deserialize)]
struct ListPackagesParams {
    pub package_type: Option<PackageType>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub min_rating: Option<f32>,
    pub verified_only: Option<bool>,
    pub free_only: Option<bool>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub sort_by: Option<SortField>,
    pub sort_order: Option<SortOrder>,
}

/// Query parameters for searching packages
#[derive(Debug, Deserialize)]
struct SearchPackagesParams {
    pub q: Option<String>,
    pub package_type: Option<PackageType>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub min_rating: Option<f32>,
    pub verified_only: Option<bool>,
    pub free_only: Option<bool>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub sort_by: Option<SortField>,
    pub sort_order: Option<SortOrder>,
}

/// Query parameters for downloading packages
#[derive(Debug, Deserialize)]
struct DownloadParams {
    pub version: Option<String>,
}

/// Query parameters for installing packages
#[derive(Debug, Deserialize)]
struct InstallParams {
    pub version: Option<String>,
}

/// Payment processing request
#[derive(Debug, Deserialize)]
struct ProcessPaymentRequest {
    pub package_id: Uuid,
    pub payment_method: PaymentMethod,
}