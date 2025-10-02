use crate::models::*;
use anyhow::Result;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait AuthenticationProvider: Send + Sync {
    async fn authenticate(&self, request: &AuthenticationRequest) -> Result<AuthenticationResponse>;
    async fn validate_password(&self, password: &str, hash: &str) -> Result<bool>;
    async fn hash_password(&self, password: &str) -> Result<String>;
    async fn create_session(&self, user: &User) -> Result<Session>;
    async fn invalidate_session(&self, session_id: Uuid) -> Result<()>;
    async fn validate_session(&self, session_id: Uuid) -> Result<Option<Session>>;
}

pub struct EnterpriseAuthProvider {
    argon2: Argon2<'static>,
    // In a real implementation, this would have database connections
}

impl EnterpriseAuthProvider {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    async fn get_user_by_username(&self, username: &str, tenant_domain: Option<&str>) -> Result<Option<User>> {
        // In a real implementation, this would query the database
        // For now, return a mock user
        if username == "admin" {
            Ok(Some(User {
                id: Uuid::new_v4(),
                username: username.to_string(),
                email: "admin@example.com".to_string(),
                password_hash: self.hash_password("admin123").await?,
                first_name: "Admin".to_string(),
                last_name: "User".to_string(),
                is_active: true,
                is_verified: true,
                tenant_id: Uuid::new_v4(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                last_login: None,
                failed_login_attempts: 0,
                locked_until: None,
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<Role>> {
        // Mock implementation
        Ok(vec![
            Role {
                id: Uuid::new_v4(),
                name: "admin".to_string(),
                description: "Administrator role".to_string(),
                tenant_id: Uuid::new_v4(),
                is_system_role: true,
                permissions: vec![
                    Permission {
                        id: Uuid::new_v4(),
                        name: "read".to_string(),
                        description: "Read permission".to_string(),
                        resource: "*".to_string(),
                        action: "read".to_string(),
                        conditions: None,
                        created_at: Utc::now(),
                    },
                    Permission {
                        id: Uuid::new_v4(),
                        name: "write".to_string(),
                        description: "Write permission".to_string(),
                        resource: "*".to_string(),
                        action: "write".to_string(),
                        conditions: None,
                        created_at: Utc::now(),
                    },
                ],
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }
        ])
    }

    async fn update_last_login(&self, user_id: Uuid) -> Result<()> {
        // In a real implementation, this would update the database
        tracing::info!("Updated last login for user: {}", user_id);
        Ok(())
    }

    async fn increment_failed_attempts(&self, user_id: Uuid) -> Result<()> {
        // In a real implementation, this would update the database
        tracing::warn!("Incremented failed login attempts for user: {}", user_id);
        Ok(())
    }

    fn is_user_locked(&self, user: &User) -> bool {
        if let Some(locked_until) = user.locked_until {
            locked_until > Utc::now()
        } else {
            false
        }
    }

    fn check_password_policy(&self, password: &str, policy: &PasswordPolicy) -> Result<()> {
        if password.len() < policy.min_length as usize {
            return Err(anyhow::anyhow!("Password too short"));
        }

        if policy.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            return Err(anyhow::anyhow!("Password must contain uppercase letters"));
        }

        if policy.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            return Err(anyhow::anyhow!("Password must contain lowercase letters"));
        }

        if policy.require_numbers && !password.chars().any(|c| c.is_numeric()) {
            return Err(anyhow::anyhow!("Password must contain numbers"));
        }

        if policy.require_special_chars && !password.chars().any(|c| !c.is_alphanumeric()) {
            return Err(anyhow::anyhow!("Password must contain special characters"));
        }

        Ok(())
    }
}

#[async_trait]
impl AuthenticationProvider for EnterpriseAuthProvider {
    async fn authenticate(&self, request: &AuthenticationRequest) -> Result<AuthenticationResponse> {
        // Get user from database
        let user = match self.get_user_by_username(&request.username, request.tenant_domain.as_deref()).await? {
            Some(user) => user,
            None => {
                return Ok(AuthenticationResponse {
                    success: false,
                    access_token: None,
                    refresh_token: None,
                    user: None,
                    error: Some("Invalid credentials".to_string()),
                    requires_mfa: false,
                    mfa_methods: vec![],
                });
            }
        };

        // Check if user is locked
        if self.is_user_locked(&user) {
            return Ok(AuthenticationResponse {
                success: false,
                access_token: None,
                refresh_token: None,
                user: None,
                error: Some("Account is temporarily locked".to_string()),
                requires_mfa: false,
                mfa_methods: vec![],
            });
        }

        // Check if user is active
        if !user.is_active {
            return Ok(AuthenticationResponse {
                success: false,
                access_token: None,
                refresh_token: None,
                user: None,
                error: Some("Account is disabled".to_string()),
                requires_mfa: false,
                mfa_methods: vec![],
            });
        }

        // Validate password
        if !self.validate_password(&request.password, &user.password_hash).await? {
            self.increment_failed_attempts(user.id).await?;
            return Ok(AuthenticationResponse {
                success: false,
                access_token: None,
                refresh_token: None,
                user: None,
                error: Some("Invalid credentials".to_string()),
                requires_mfa: false,
                mfa_methods: vec![],
            });
        }

        // Get user roles and permissions
        let roles = self.get_user_roles(user.id).await?;
        let role_names: Vec<String> = roles.iter().map(|r| r.name.clone()).collect();
        let permissions: Vec<String> = roles
            .iter()
            .flat_map(|r| r.permissions.iter().map(|p| format!("{}:{}", p.resource, p.action)))
            .collect();

        // Check if MFA is required
        // In a real implementation, this would check tenant settings and user preferences
        let requires_mfa = false; // Simplified for now

        if requires_mfa && request.mfa_code.is_none() {
            return Ok(AuthenticationResponse {
                success: false,
                access_token: None,
                refresh_token: None,
                user: None,
                error: None,
                requires_mfa: true,
                mfa_methods: vec![MfaMethod::Totp, MfaMethod::Email],
            });
        }

        // Create session
        let session = self.create_session(&user).await?;

        // Update last login
        self.update_last_login(user.id).await?;

        // Generate JWT tokens (simplified)
        let access_token = format!("access_token_{}", session.id);
        let refresh_token = format!("refresh_token_{}", session.id);

        Ok(AuthenticationResponse {
            success: true,
            access_token: Some(access_token),
            refresh_token: Some(refresh_token),
            user: Some(UserInfo {
                id: user.id,
                username: user.username,
                email: user.email,
                first_name: user.first_name,
                last_name: user.last_name,
                tenant_id: user.tenant_id,
                roles: role_names,
                permissions,
            }),
            error: None,
            requires_mfa: false,
            mfa_methods: vec![],
        })
    }

    async fn validate_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;

        Ok(self.argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }

    async fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;

        Ok(password_hash.to_string())
    }

    async fn create_session(&self, user: &User) -> Result<Session> {
        let session_id = Uuid::new_v4();
        let now = Utc::now();
        let expires_at = now + Duration::hours(8); // 8 hour session

        let session = Session {
            id: session_id,
            user_id: user.id,
            tenant_id: user.tenant_id,
            token_hash: format!("session_{}", session_id), // In production, this would be a proper hash
            ip_address: None, // Would be filled from request
            user_agent: None, // Would be filled from request
            created_at: now,
            expires_at,
            last_activity: now,
            is_active: true,
        };

        // In a real implementation, this would save to database
        tracing::info!("Created session {} for user {}", session_id, user.id);

        Ok(session)
    }

    async fn invalidate_session(&self, session_id: Uuid) -> Result<()> {
        // In a real implementation, this would update the database
        tracing::info!("Invalidated session: {}", session_id);
        Ok(())
    }

    async fn validate_session(&self, session_id: Uuid) -> Result<Option<Session>> {
        // In a real implementation, this would query the database
        // For now, return None (session not found)
        tracing::debug!("Validating session: {}", session_id);
        Ok(None)
    }
}

pub struct AuthenticationService {
    provider: Arc<dyn AuthenticationProvider>,
}

impl AuthenticationService {
    pub fn new(provider: Arc<dyn AuthenticationProvider>) -> Self {
        Self { provider }
    }

    pub async fn authenticate(&self, request: AuthenticationRequest) -> Result<AuthenticationResponse> {
        self.provider.authenticate(&request).await
    }

    pub async fn logout(&self, session_id: Uuid) -> Result<()> {
        self.provider.invalidate_session(session_id).await
    }

    pub async fn validate_session(&self, session_id: Uuid) -> Result<Option<Session>> {
        self.provider.validate_session(session_id).await
    }

    pub async fn change_password(&self, user_id: Uuid, old_password: &str, new_password: &str) -> Result<()> {
        // In a real implementation, this would:
        // 1. Validate the old password
        // 2. Check password policy
        // 3. Hash the new password
        // 4. Update the database
        // 5. Invalidate all existing sessions

        tracing::info!("Password changed for user: {}", user_id);
        Ok(())
    }
}