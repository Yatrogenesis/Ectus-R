use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Authentication result type
pub type AuthResult<T> = Result<T, AuthError>;

/// Authentication errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,

    #[error("Account locked")]
    AccountLocked,

    #[error("Account inactive")]
    AccountInactive,

    #[error("Rate limited")]
    RateLimited,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("MFA required")]
    MfaRequired,

    #[error("Invalid MFA code")]
    InvalidMfaCode,

    #[error("Password policy violation: {0}")]
    PasswordPolicyViolation(String),

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Hashing error: {0}")]
    HashingError(String),

    #[error("Session expired")]
    SessionExpired,

    #[error("Session not found")]
    SessionNotFound,

    #[error("Concurrent session limit exceeded")]
    ConcurrentSessionLimit,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// JWT signing key
    pub jwt_secret: String,
    /// JWT issuer
    pub jwt_issuer: String,
    /// JWT audience
    pub jwt_audience: String,
    /// Access token lifetime in seconds
    pub access_token_lifetime: u64,
    /// Refresh token lifetime in seconds
    pub refresh_token_lifetime: u64,
    /// Enable CSRF protection
    pub csrf_protection: bool,
    /// Enable rate limiting
    pub rate_limiting: bool,
    /// Enable audit logging
    pub audit_logging: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "your-secret-key".to_string(),
            jwt_issuer: "aion-r".to_string(),
            jwt_audience: "aion-r-api".to_string(),
            access_token_lifetime: 3600,    // 1 hour
            refresh_token_lifetime: 604800, // 1 week
            csrf_protection: true,
            rate_limiting: true,
            audit_logging: true,
        }
    }
}

/// User credentials for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCredentials {
    /// Username or email
    pub identifier: String,
    /// Password
    pub password: String,
    /// Optional MFA code
    pub mfa_code: Option<String>,
    /// Remember session
    pub remember_me: bool,
}

/// Enhanced user model with security features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSecurityProfile {
    pub user_id: Uuid,
    pub two_factor_enabled: bool,
    pub backup_codes: Vec<String>,
    pub security_questions: Vec<SecurityQuestion>,
    pub trusted_devices: Vec<TrustedDevice>,
    pub login_history: Vec<LoginAttempt>,
    pub password_history: Vec<PasswordHistoryEntry>,
    pub security_preferences: SecurityPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityQuestion {
    pub id: Uuid,
    pub question: String,
    pub answer_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedDevice {
    pub id: Uuid,
    pub device_fingerprint: String,
    pub device_name: String,
    pub user_agent: String,
    pub ip_address: String,
    pub location: Option<String>,
    pub trusted_until: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginAttempt {
    pub id: Uuid,
    pub ip_address: String,
    pub user_agent: String,
    pub success: bool,
    pub failure_reason: Option<String>,
    pub location: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHistoryEntry {
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPreferences {
    pub login_notifications: bool,
    pub unusual_activity_alerts: bool,
    pub session_timeout_minutes: u32,
    pub require_device_confirmation: bool,
}

/// OAuth2 and OIDC models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClient {
    pub id: Uuid,
    pub client_id: String,
    pub client_secret_hash: String,
    pub name: String,
    pub description: Option<String>,
    pub redirect_uris: Vec<String>,
    pub allowed_scopes: Vec<String>,
    pub is_confidential: bool,
    pub is_active: bool,
    pub tenant_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthAuthorizationCode {
    pub code: String,
    pub client_id: String,
    pub user_id: Uuid,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    pub token_hash: String,
    pub user_id: Uuid,
    pub client_id: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// RBAC (Role-Based Access Control) models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleHierarchy {
    pub parent_role_id: Uuid,
    pub child_role_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePermission {
    pub id: Uuid,
    pub resource_type: String,
    pub resource_id: Option<Uuid>,
    pub action: String,
    pub effect: PermissionEffect,
    pub conditions: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionEffect {
    Allow,
    Deny,
}

/// Audit and compliance models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: Uuid,
    pub event_type: SecurityEventType,
    pub severity: SecurityEventSeverity,
    pub user_id: Option<Uuid>,
    pub tenant_id: Uuid,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub details: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    LoginSuccess,
    LoginFailure,
    LogoutSuccess,
    PasswordChange,
    PasswordReset,
    MfaEnabled,
    MfaDisabled,
    MfaChallenge,
    MfaSuccess,
    MfaFailure,
    AccountLocked,
    AccountUnlocked,
    PermissionGranted,
    PermissionDenied,
    TokenIssued,
    TokenRevoked,
    SuspiciousActivity,
    DataAccess,
    DataModification,
    ConfigurationChange,
    SystemError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Compliance and data protection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProcessingConsent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub purpose: String,
    pub legal_basis: String,
    pub consented: bool,
    pub consent_date: DateTime<Utc>,
    pub withdrawal_date: Option<DateTime<Utc>>,
    pub retention_period_days: Option<u32>,
}

/// Encryption and key management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionKey {
    pub id: Uuid,
    pub key_type: EncryptionKeyType,
    pub algorithm: String,
    pub key_size: u32,
    pub usage: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub rotated_from: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionKeyType {
    Symmetric,
    Asymmetric,
    Signing,
    Encryption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub tenant_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub failed_login_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub tenant_id: Uuid,
    pub is_system_role: bool,
    pub permissions: Vec<Permission>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub resource: String,
    pub action: String,
    pub conditions: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub domain: String,
    pub is_active: bool,
    pub subscription_tier: SubscriptionTier,
    pub settings: TenantSettings,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionTier {
    Free,
    Professional,
    Enterprise,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantSettings {
    pub max_users: Option<u32>,
    pub session_timeout_minutes: u32,
    pub password_policy: PasswordPolicy,
    pub mfa_required: bool,
    pub sso_enabled: bool,
    pub audit_retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    pub min_length: u8,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_special_chars: bool,
    pub max_age_days: Option<u32>,
    pub history_count: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub token_hash: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationRequest {
    pub username: String,
    pub password: String,
    pub tenant_domain: Option<String>,
    pub mfa_code: Option<String>,
    pub remember_me: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    pub success: bool,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub user: Option<UserInfo>,
    pub error: Option<String>,
    pub requires_mfa: bool,
    pub mfa_methods: Vec<MfaMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub tenant_id: Uuid,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {
    Totp,
    Sms,
    Email,
    BackupCodes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid,        // Subject (user ID)
    pub tenant_id: Uuid,  // Tenant ID
    pub iat: i64,         // Issued at
    pub exp: i64,         // Expiration time
    pub aud: String,      // Audience
    pub iss: String,      // Issuer
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub session_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub key_hash: String,
    pub prefix: String,
    pub scopes: Vec<String>,
    pub is_active: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoProvider {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub provider_type: SsoProviderType,
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub discovery_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SsoProviderType {
    Oidc,
    Saml,
    Oauth2,
    Ldap,
    ActiveDirectory,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special_chars: true,
            max_age_days: Some(90),
            history_count: 5,
        }
    }
}

impl Default for TenantSettings {
    fn default() -> Self {
        Self {
            max_users: None,
            session_timeout_minutes: 480, // 8 hours
            password_policy: PasswordPolicy::default(),
            mfa_required: false,
            sso_enabled: false,
            audit_retention_days: 365,
        }
    }
}