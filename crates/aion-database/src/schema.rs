//! # Database Schema
//!
//! Complete database schema for AION-R Enterprise Platform.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

/// Database schema version for migrations
pub const SCHEMA_VERSION: u32 = 1;

/// User account information
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: UserRole,
    pub status: UserStatus,
    pub email_verified: bool,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub tenant_id: Uuid,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
    Developer,
    Viewer,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Pending,
}

/// Tenant information for multi-tenancy
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub subscription_tier: SubscriptionTier,
    pub status: TenantStatus,
    pub settings: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "subscription_tier", rename_all = "lowercase")]
pub enum SubscriptionTier {
    Free,
    Pro,
    Enterprise,
    Custom,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "tenant_status", rename_all = "lowercase")]
pub enum TenantStatus {
    Active,
    Suspended,
    Trial,
    Expired,
}

/// API keys for authentication
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub key_hash: String,
    pub permissions: Vec<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// AI model information
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub model_type: ModelType,
    pub provider: String,
    pub version: String,
    pub size_bytes: Option<i64>,
    pub parameters: Option<i64>,
    pub supported_tasks: Vec<String>,
    pub input_modalities: Vec<String>,
    pub output_modalities: Vec<String>,
    pub config: serde_json::Value,
    pub is_active: bool,
    pub is_public: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "model_type", rename_all = "lowercase")]
pub enum ModelType {
    Text,
    Image,
    Audio,
    Multimodal,
    Traditional,
}

/// Inference requests and responses
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct InferenceRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub model_id: Uuid,
    pub request_data: serde_json::Value,
    pub response_data: Option<serde_json::Value>,
    pub status: RequestStatus,
    pub error_message: Option<String>,
    pub processing_time_ms: Option<i32>,
    pub tokens_used: Option<i32>,
    pub cost_cents: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "request_status", rename_all = "lowercase")]
pub enum RequestStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

/// Usage tracking and billing
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UsageRecord {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub model_id: Uuid,
    pub date: chrono::NaiveDate,
    pub request_count: i32,
    pub total_tokens: i32,
    pub total_processing_time_ms: i64,
    pub total_cost_cents: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Audit logs for security and compliance
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<Uuid>,
    pub details: serde_json::Value,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// System configuration and settings
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SystemConfig {
    pub key: String,
    pub value: serde_json::Value,
    pub description: Option<String>,
    pub is_sensitive: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Performance metrics and monitoring
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub id: Uuid,
    pub metric_name: String,
    pub metric_value: f64,
    pub dimensions: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub tenant_id: Option<Uuid>,
    pub model_id: Option<Uuid>,
}

/// Cached results for performance optimization
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub value: serde_json::Value,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub access_count: i32,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
}

/// Model training jobs
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TrainingJob {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub model_type: ModelType,
    pub training_config: serde_json::Value,
    pub dataset_config: serde_json::Value,
    pub status: JobStatus,
    pub progress_percentage: i16,
    pub logs: Option<String>,
    pub result_model_id: Option<Uuid>,
    pub error_message: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "job_status", rename_all = "lowercase")]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Data sources and datasets
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Dataset {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub dataset_type: DatasetType,
    pub format: String,
    pub size_bytes: i64,
    pub record_count: Option<i64>,
    pub schema_info: Option<serde_json::Value>,
    pub storage_path: String,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "dataset_type", rename_all = "lowercase")]
pub enum DatasetType {
    Text,
    Image,
    Audio,
    Video,
    Structured,
    Multimodal,
}

/// Feature flags for gradual rollouts
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FeatureFlag {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_enabled: bool,
    pub rules: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Notifications and alerts
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub is_read: bool,
    pub data: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub read_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "notification_type", rename_all = "lowercase")]
pub enum NotificationType {
    Info,
    Warning,
    Error,
    Success,
    System,
}

/// Database repository trait
#[async_trait::async_trait]
pub trait Repository<T> {
    async fn create(&self, entity: &T) -> Result<T, sqlx::Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<T>, sqlx::Error>;
    async fn update(&self, entity: &T) -> Result<T, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
    async fn list(&self, offset: i64, limit: i64) -> Result<Vec<T>, sqlx::Error>;
}

/// User repository implementation
pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, username, password_hash, first_name, last_name,
                   role as "role: UserRole", status as "status: UserStatus",
                   email_verified, last_login, created_at, updated_at, tenant_id
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, username, password_hash, first_name, last_name,
                   role as "role: UserRole", status as "status: UserStatus",
                   email_verified, last_login, created_at, updated_at, tenant_id
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn update_last_login(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET last_login = NOW() WHERE id = $1",
            user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_by_tenant(&self, tenant_id: Uuid, offset: i64, limit: i64) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, username, password_hash, first_name, last_name,
                   role as "role: UserRole", status as "status: UserStatus",
                   email_verified, last_login, created_at, updated_at, tenant_id
            FROM users
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            OFFSET $2 LIMIT $3
            "#,
            tenant_id,
            offset,
            limit
        )
        .fetch_all(&self.pool)
        .await
    }
}

/// Tenant repository implementation
pub struct TenantRepository {
    pool: Pool<Postgres>,
}

impl TenantRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_by_slug(&self, slug: &str) -> Result<Option<Tenant>, sqlx::Error> {
        sqlx::query_as!(
            Tenant,
            r#"
            SELECT id, name, slug, description,
                   subscription_tier as "subscription_tier: SubscriptionTier",
                   status as "status: TenantStatus",
                   settings, created_at, updated_at
            FROM tenants
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await
    }
}

/// Model repository implementation
pub struct ModelRepository {
    pool: Pool<Postgres>,
}

impl ModelRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn list_active(&self, offset: i64, limit: i64) -> Result<Vec<Model>, sqlx::Error> {
        sqlx::query_as!(
            Model,
            r#"
            SELECT id, name, display_name, description,
                   model_type as "model_type: ModelType",
                   provider, version, size_bytes, parameters,
                   supported_tasks, input_modalities, output_modalities,
                   config, is_active, is_public, created_at, updated_at
            FROM models
            WHERE is_active = true
            ORDER BY created_at DESC
            OFFSET $1 LIMIT $2
            "#,
            offset,
            limit
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Option<Model>, sqlx::Error> {
        sqlx::query_as!(
            Model,
            r#"
            SELECT id, name, display_name, description,
                   model_type as "model_type: ModelType",
                   provider, version, size_bytes, parameters,
                   supported_tasks, input_modalities, output_modalities,
                   config, is_active, is_public, created_at, updated_at
            FROM models
            WHERE name = $1 AND is_active = true
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await
    }
}

/// Inference request repository implementation
pub struct InferenceRequestRepository {
    pool: Pool<Postgres>,
}

impl InferenceRequestRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_request(&self, request: &InferenceRequest) -> Result<InferenceRequest, sqlx::Error> {
        sqlx::query_as!(
            InferenceRequest,
            r#"
            INSERT INTO inference_requests
            (id, user_id, tenant_id, model_id, request_data, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, tenant_id, model_id, request_data, response_data,
                      status as "status: RequestStatus", error_message, processing_time_ms,
                      tokens_used, cost_cents, created_at, completed_at
            "#,
            request.id,
            request.user_id,
            request.tenant_id,
            request.model_id,
            request.request_data,
            request.status as RequestStatus,
            request.created_at
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_response(
        &self,
        request_id: Uuid,
        response_data: serde_json::Value,
        processing_time_ms: i32,
        tokens_used: Option<i32>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE inference_requests
            SET response_data = $1, status = 'completed', processing_time_ms = $2,
                tokens_used = $3, completed_at = NOW()
            WHERE id = $4
            "#,
            response_data,
            processing_time_ms,
            tokens_used,
            request_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn mark_failed(&self, request_id: Uuid, error_message: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE inference_requests
            SET status = 'failed', error_message = $1, completed_at = NOW()
            WHERE id = $2
            "#,
            error_message,
            request_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_by_user(
        &self,
        user_id: Uuid,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<InferenceRequest>, sqlx::Error> {
        sqlx::query_as!(
            InferenceRequest,
            r#"
            SELECT id, user_id, tenant_id, model_id, request_data, response_data,
                   status as "status: RequestStatus", error_message, processing_time_ms,
                   tokens_used, cost_cents, created_at, completed_at
            FROM inference_requests
            WHERE user_id = $1
            ORDER BY created_at DESC
            OFFSET $2 LIMIT $3
            "#,
            user_id,
            offset,
            limit
        )
        .fetch_all(&self.pool)
        .await
    }
}

/// Database service that aggregates all repositories
pub struct DatabaseService {
    pub users: UserRepository,
    pub tenants: TenantRepository,
    pub models: ModelRepository,
    pub inference_requests: InferenceRequestRepository,
    pool: Pool<Postgres>,
}

impl DatabaseService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            users: UserRepository::new(pool.clone()),
            tenants: TenantRepository::new(pool.clone()),
            models: ModelRepository::new(pool.clone()),
            inference_requests: InferenceRequestRepository::new(pool.clone()),
            pool,
        }
    }

    /// Run health check on database
    pub async fn health_check(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Get database statistics
    pub async fn get_stats(&self) -> Result<DatabaseStats, sqlx::Error> {
        let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;

        let tenant_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tenants")
            .fetch_one(&self.pool)
            .await?;

        let model_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM models WHERE is_active = true")
            .fetch_one(&self.pool)
            .await?;

        let request_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM inference_requests")
            .fetch_one(&self.pool)
            .await?;

        Ok(DatabaseStats {
            user_count,
            tenant_count,
            model_count,
            request_count,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub user_count: i64,
    pub tenant_count: i64,
    pub model_count: i64,
    pub request_count: i64,
}