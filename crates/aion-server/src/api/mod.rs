// AION-R API Module
// Central API routing and configuration

use axum::{
    Router,
    routing::{get, post, put, delete},
    middleware,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    compression::CompressionLayer,
};

use crate::{AppState, middleware::*};

pub mod auth;
pub mod health;
pub mod code_generation;
pub mod admin;
// requirements, models, and users are defined inline below

/// Build the main API router
pub fn build_api_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Health and status endpoints (no auth required)
        .route("/health", get(health::health_check))
        .route("/ready", get(health::readiness_check))
        .route("/metrics", get(health::metrics))

        // Public auth endpoints
        .route("/api/v1/auth/login", post(auth::login))
        .route("/api/v1/auth/register", post(auth::register))
        .route("/api/v1/auth/refresh", post(auth::refresh_token))
        .route("/api/v1/auth/forgot-password", post(auth::forgot_password))
        .route("/api/v1/auth/reset-password", post(auth::reset_password))

        // Protected API v1 routes
        .nest("/api/v1", api_v1_routes(state.clone()))

        // Admin routes
        .nest("/api/v1/admin", admin_routes(state.clone()))

        // Apply middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(CorsLayer::permissive())
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    rate_limiting_middleware,
                ))
        )
        .with_state(state)
}

/// API v1 routes (require authentication)
fn api_v1_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        // User profile
        .route("/users/profile", get(users::get_profile))
        .route("/users/profile", put(users::update_profile))
        .route("/users/change-password", post(users::change_password))
        .route("/users/enable-mfa", post(users::enable_mfa))
        .route("/users/disable-mfa", post(users::disable_mfa))

        // Code generation endpoints
        .route("/code/generate", post(code_generation::generate_code))
        .route("/code/status/:id", get(code_generation::get_generation_status))
        .route("/code/download/:id", get(code_generation::download_generated_code))
        .route("/code/:id/docs", get(code_generation::get_generation_documentation))
        .route("/code/list", get(code_generation::list_generations))
        .route("/code/:id", delete(code_generation::delete_generation))

        // Requirements analysis
        .route("/requirements/analyze", post(code_generation::analyze_requirements))
        .route("/requirements/optimize", post(requirements::optimize_requirements))
        .route("/requirements/validate", post(requirements::validate_requirements))

        // AI Models management
        .route("/models", get(models::list_models))
        .route("/models/:id", get(models::get_model))
        .route("/models/:id/info", get(models::get_model_info))
        .route("/models/download/:id", post(models::download_model))

        // Text processing
        .route("/ai/text/analyze", post(ai::text::analyze_text))
        .route("/ai/text/generate", post(ai::text::generate_text))
        .route("/ai/text/summarize", post(ai::text::summarize_text))
        .route("/ai/text/translate", post(ai::text::translate_text))

        // Image processing
        .route("/ai/vision/analyze", post(ai::vision::analyze_image))
        .route("/ai/vision/generate", post(ai::vision::generate_image))
        .route("/ai/vision/classify", post(ai::vision::classify_image))
        .route("/ai/vision/detect-objects", post(ai::vision::detect_objects))

        // Audio processing
        .route("/ai/audio/transcribe", post(ai::audio::transcribe_audio))
        .route("/ai/audio/generate", post(ai::audio::generate_audio))
        .route("/ai/audio/analyze", post(ai::audio::analyze_audio))

        // Project management
        .route("/projects", get(projects::list_projects))
        .route("/projects", post(projects::create_project))
        .route("/projects/:id", get(projects::get_project))
        .route("/projects/:id", put(projects::update_project))
        .route("/projects/:id", delete(projects::delete_project))
        .route("/projects/:id/deploy", post(projects::deploy_project))

        // Usage and billing
        .route("/usage", get(usage::get_usage))
        .route("/usage/history", get(usage::get_usage_history))
        .route("/billing/current", get(billing::get_current_billing))
        .route("/billing/history", get(billing::get_billing_history))

        // Apply authentication middleware to all routes
        .layer(middleware::from_fn_with_state(
            state.clone(),
            authenticate_middleware,
        ))
}

/// Admin routes (require admin role)
fn admin_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(admin::list_users))
        .route("/users/:id", get(admin::get_user))
        .route("/users/:id", put(admin::update_user))
        .route("/users/:id", delete(admin::delete_user))
        .route("/users/:id/suspend", post(admin::suspend_user))
        .route("/users/:id/activate", post(admin::activate_user))

        .route("/system/info", get(admin::system_info))
        .route("/system/config", get(admin::get_config))
        .route("/system/config", put(admin::update_config))
        .route("/system/maintenance", post(admin::toggle_maintenance))

        .route("/metrics/overview", get(admin::metrics_overview))
        .route("/metrics/performance", get(admin::performance_metrics))
        .route("/metrics/usage", get(admin::usage_metrics))

        .route("/logs", get(admin::view_logs))
        .route("/logs/export", post(admin::export_logs))

        .route("/tenants", get(admin::list_tenants))
        .route("/tenants/:id", get(admin::get_tenant))
        .route("/tenants", post(admin::create_tenant))
        .route("/tenants/:id", put(admin::update_tenant))

        // Apply admin authorization middleware
        .layer(middleware::from_fn_with_state(
            state.clone(),
            admin_authorization_middleware,
        ))
        // Apply authentication middleware
        .layer(middleware::from_fn_with_state(
            state.clone(),
            authenticate_middleware,
        ))
}

// Re-export commonly used types
pub use auth::{LoginRequest, LoginResponse, RegisterRequest};
pub use code_generation::{GenerateCodeRequest, GenerateCodeResponse};
pub use requirements::{RequirementsRequest, RequirementsResponse};
pub use models::{ModelInfo, ModelList};

// Module placeholder implementations (these would be in separate files)
mod ai {
    pub mod text {
        use axum::{extract::State, response::Json, Extension};
        use std::sync::Arc;
        use crate::{AppState, errors::AppError, middleware::AuthenticatedUser};

        pub async fn analyze_text(
            State(_state): State<Arc<AppState>>,
            Extension(_user): Extension<AuthenticatedUser>,
            Json(_request): Json<serde_json::Value>,
        ) -> Result<Json<serde_json::Value>, AppError> {
            Ok(Json(serde_json::json!({"status": "ok"})))
        }

        pub async fn generate_text(
            State(_state): State<Arc<AppState>>,
            Extension(_user): Extension<AuthenticatedUser>,
            Json(_request): Json<serde_json::Value>,
        ) -> Result<Json<serde_json::Value>, AppError> {
            Ok(Json(serde_json::json!({"status": "ok"})))
        }

        pub async fn summarize_text(
            State(_state): State<Arc<AppState>>,
            Extension(_user): Extension<AuthenticatedUser>,
            Json(_request): Json<serde_json::Value>,
        ) -> Result<Json<serde_json::Value>, AppError> {
            Ok(Json(serde_json::json!({"status": "ok"})))
        }

        pub async fn translate_text(
            State(_state): State<Arc<AppState>>,
            Extension(_user): Extension<AuthenticatedUser>,
            Json(_request): Json<serde_json::Value>,
        ) -> Result<Json<serde_json::Value>, AppError> {
            Ok(Json(serde_json::json!({"status": "ok"})))
        }
    }

    pub mod vision {
        use axum::{extract::State, response::Json, Extension};
        use std::sync::Arc;
        use crate::{AppState, errors::AppError, middleware::AuthenticatedUser};

        pub async fn analyze_image(
            State(_state): State<Arc<AppState>>,
            Extension(_user): Extension<AuthenticatedUser>,
            Json(_request): Json<serde_json::Value>,
        ) -> Result<Json<serde_json::Value>, AppError> {
            Ok(Json(serde_json::json!({"status": "ok"})))
        }

        pub async fn generate_image(
            State(_state): State<Arc<AppState>>,
            Extension(_user): Extension<AuthenticatedUser>,
            Json(_request): Json<serde_json::Value>,
        ) -> Result<Json<serde_json::Value>, AppError> {
            Ok(Json(serde_json::json!({"status": "ok"})))
        }

        pub async fn classify_image(
            State(_state): State<Arc<AppState>>,
            Extension(_user): Extension<AuthenticatedUser>,
            Json(_request): Json<serde_json::Value>,
        ) -> Result<Json<serde_json::Value>, AppError> {
            Ok(Json(serde_json::json!({"status": "ok"})))
        }

        pub async fn detect_objects(
            State(_state): State<Arc<AppState>>,
            Extension(_user): Extension<AuthenticatedUser>,
            Json(_request): Json<serde_json::Value>,
        ) -> Result<Json<serde_json::Value>, AppError> {
            Ok(Json(serde_json::json!({"status": "ok"})))
        }
    }

    pub mod audio {
        use axum::{extract::State, response::Json, Extension};
        use std::sync::Arc;
        use crate::{AppState, errors::AppError, middleware::AuthenticatedUser};

        pub async fn transcribe_audio(
            State(_state): State<Arc<AppState>>,
            Extension(_user): Extension<AuthenticatedUser>,
            Json(_request): Json<serde_json::Value>,
        ) -> Result<Json<serde_json::Value>, AppError> {
            Ok(Json(serde_json::json!({"status": "ok"})))
        }

        pub async fn generate_audio(
            State(_state): State<Arc<AppState>>,
            Extension(_user): Extension<AuthenticatedUser>,
            Json(_request): Json<serde_json::Value>,
        ) -> Result<Json<serde_json::Value>, AppError> {
            Ok(Json(serde_json::json!({"status": "ok"})))
        }

        pub async fn analyze_audio(
            State(_state): State<Arc<AppState>>,
            Extension(_user): Extension<AuthenticatedUser>,
            Json(_request): Json<serde_json::Value>,
        ) -> Result<Json<serde_json::Value>, AppError> {
            Ok(Json(serde_json::json!({"status": "ok"})))
        }
    }
}

mod projects {
    use axum::{extract::{State, Path}, response::Json, Extension};
    use std::sync::Arc;
    use uuid::Uuid;
    use crate::{AppState, errors::AppError, middleware::AuthenticatedUser};

    pub async fn list_projects(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"projects": []})))
    }

    pub async fn create_project(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Json(_request): Json<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"id": Uuid::new_v4()})))
    }

    pub async fn get_project(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Path(_id): Path<Uuid>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"project": {}})))
    }

    pub async fn update_project(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Path(_id): Path<Uuid>,
        Json(_request): Json<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"status": "updated"})))
    }

    pub async fn delete_project(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Path(_id): Path<Uuid>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"status": "deleted"})))
    }

    pub async fn deploy_project(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Path(_id): Path<Uuid>,
        Json(_request): Json<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"deployment_id": Uuid::new_v4()})))
    }
}

mod usage {
    use axum::{extract::State, response::Json, Extension};
    use std::sync::Arc;
    use crate::{AppState, errors::AppError, middleware::AuthenticatedUser};

    pub async fn get_usage(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"usage": {}})))
    }

    pub async fn get_usage_history(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"history": []})))
    }
}

mod billing {
    use axum::{extract::State, response::Json, Extension};
    use std::sync::Arc;
    use crate::{AppState, errors::AppError, middleware::AuthenticatedUser};

    pub async fn get_current_billing(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"billing": {}})))
    }

    pub async fn get_billing_history(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"history": []})))
    }
}

mod requirements {
    use axum::{extract::State, response::Json, Extension};
    use std::sync::Arc;
    use serde::{Deserialize, Serialize};
    use crate::{AppState, errors::AppError, middleware::AuthenticatedUser};

    #[derive(Debug, Deserialize)]
    pub struct RequirementsRequest {
        pub requirements: String,
    }

    #[derive(Debug, Serialize)]
    pub struct RequirementsResponse {
        pub optimized: String,
        pub suggestions: Vec<String>,
    }

    pub async fn optimize_requirements(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Json(_request): Json<RequirementsRequest>,
    ) -> Result<Json<RequirementsResponse>, AppError> {
        Ok(Json(RequirementsResponse {
            optimized: "Optimized requirements".to_string(),
            suggestions: vec![],
        }))
    }

    pub async fn validate_requirements(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Json(_request): Json<RequirementsRequest>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"valid": true})))
    }
}

mod models {
    use axum::{extract::{State, Path}, response::Json, Extension};
    use std::sync::Arc;
    use uuid::Uuid;
    use serde::Serialize;
    use crate::{AppState, errors::AppError, middleware::AuthenticatedUser};

    #[derive(Debug, Serialize)]
    pub struct ModelInfo {
        pub id: Uuid,
        pub name: String,
        pub version: String,
    }

    #[derive(Debug, Serialize)]
    pub struct ModelList {
        pub models: Vec<ModelInfo>,
    }

    pub async fn list_models(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
    ) -> Result<Json<ModelList>, AppError> {
        Ok(Json(ModelList { models: vec![] }))
    }

    pub async fn get_model(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Path(_id): Path<Uuid>,
    ) -> Result<Json<ModelInfo>, AppError> {
        Ok(Json(ModelInfo {
            id: Uuid::new_v4(),
            name: "Model".to_string(),
            version: "1.0.0".to_string(),
        }))
    }

    pub async fn get_model_info(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Path(_id): Path<Uuid>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"info": {}})))
    }

    pub async fn download_model(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Path(_id): Path<Uuid>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"download_url": ""})))
    }
}

mod users {
    use axum::{extract::State, response::Json, Extension};
    use std::sync::Arc;
    use crate::{AppState, errors::AppError, middleware::AuthenticatedUser};

    pub async fn get_profile(
        State(_state): State<Arc<AppState>>,
        Extension(user): Extension<AuthenticatedUser>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({
            "id": user.id,
            "email": user.email,
            "role": user.role,
        })))
    }

    pub async fn update_profile(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Json(_request): Json<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"status": "updated"})))
    }

    pub async fn change_password(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Json(_request): Json<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"status": "changed"})))
    }

    pub async fn enable_mfa(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Json(_request): Json<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"qr_code": "", "backup_codes": []})))
    }

    pub async fn disable_mfa(
        State(_state): State<Arc<AppState>>,
        Extension(_user): Extension<AuthenticatedUser>,
        Json(_request): Json<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, AppError> {
        Ok(Json(serde_json::json!({"status": "disabled"})))
    }
}