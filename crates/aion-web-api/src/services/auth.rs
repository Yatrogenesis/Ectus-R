//! Authentication and authorization service

use anyhow::Result;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use crate::models::*;

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub role: String,
    pub exp: usize, // Expiration time
    pub iat: usize, // Issued at
}

/// Authentication service with secure database integration
pub struct AuthService {
    jwt_secret: String,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    db_pool: Arc<PgPool>,
    argon2: Argon2<'static>,
}

impl AuthService {
    pub async fn new(jwt_secret: &str, db_pool: Arc<PgPool>) -> Result<Self> {
        println!("🔐 Initializing secure Authentication Service...");

        // Validate JWT secret strength
        if jwt_secret.len() < 32 {
            return Err(anyhow::anyhow!("JWT secret must be at least 32 characters long for security"));
        }

        // Initialize Argon2 with secure parameters
        let argon2 = Argon2::default();

        // Initialize database tables if they don't exist
        Self::initialize_database_tables(&db_pool).await?;

        println!("✅ Secure Authentication Service initialized");

        Ok(Self {
            jwt_secret: jwt_secret.to_string(),
            encoding_key: EncodingKey::from_secret(jwt_secret.as_ref()),
            decoding_key: DecodingKey::from_secret(jwt_secret.as_ref()),
            db_pool,
            argon2,
        })
    }

    /// Initialize authentication-related database tables
    async fn initialize_database_tables(pool: &PgPool) -> Result<()> {
        // Create users table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                name TEXT NOT NULL,
                role TEXT NOT NULL DEFAULT 'user',
                created_at TIMESTAMPTZ DEFAULT NOW(),
                last_login TIMESTAMPTZ,
                is_active BOOLEAN DEFAULT true,
                failed_login_attempts INTEGER DEFAULT 0,
                locked_until TIMESTAMPTZ,
                email_verified BOOLEAN DEFAULT false,
                two_factor_enabled BOOLEAN DEFAULT false,
                two_factor_secret TEXT
            )
        "#)
        .execute(pool)
        .await?;

        // Create user sessions table for refresh token management
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS user_sessions (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                refresh_token_hash TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                expires_at TIMESTAMPTZ NOT NULL,
                last_used TIMESTAMPTZ DEFAULT NOW(),
                user_agent TEXT,
                ip_address INET,
                is_active BOOLEAN DEFAULT true
            )
        "#)
        .execute(pool)
        .await?;

        // Create indexes for performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)")
            .execute(pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_user_sessions_user_id ON user_sessions(user_id)")
            .execute(pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_user_sessions_refresh_token ON user_sessions(refresh_token_hash)")
            .execute(pool)
            .await?;

        println!("✅ Authentication database tables initialized");
        Ok(())
    }

    /// Authenticate user and generate JWT token
    pub async fn authenticate(&self, email: &str, password: &str) -> Result<LoginResponse> {
        println!("🔑 Authenticating user: {}", email);

        // Input validation
        if email.is_empty() || password.is_empty() {
            return Err(anyhow::anyhow!("Email and password are required"));
        }

        if !email.contains('@') || email.len() > 254 {
            return Err(anyhow::anyhow!("Invalid email format"));
        }

        // Check for account lockout
        let user_data = sqlx::query!(
            r#"
            SELECT id, email, password_hash, name, role, created_at, last_login,
                   is_active, failed_login_attempts, locked_until,
                   email_verified, two_factor_enabled
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&*self.db_pool)
        .await?;

        let user_data = match user_data {
            Some(user) => user,
            None => {
                // Prevent user enumeration by taking the same time as a real hash verification
                let dummy_salt = SaltString::generate(&mut OsRng);
                let dummy_hash = self.argon2.hash_password(b"dummy", &dummy_salt);
                return Err(anyhow::anyhow!("Invalid credentials"));
            }
        };

        // Check if account is active
        if !user_data.is_active {
            return Err(anyhow::anyhow!("Account is deactivated"));
        }

        // Check account lockout
        if let Some(locked_until) = user_data.locked_until {
            if locked_until > chrono::Utc::now() {
                return Err(anyhow::anyhow!("Account is temporarily locked due to too many failed login attempts"));
            }
        }

        // Verify password
        let password_hash = PasswordHash::new(&user_data.password_hash)
            .map_err(|_| anyhow::anyhow!("Invalid password hash in database"))?;

        let is_valid = self.argon2
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok();

        if !is_valid {
            // Increment failed login attempts
            self.increment_failed_login_attempts(user_data.id).await?;
            return Err(anyhow::anyhow!("Invalid credentials"));
        }

        // Check email verification
        if !user_data.email_verified {
            return Err(anyhow::anyhow!("Email address must be verified before login"));
        }

        // Reset failed login attempts on successful login
        self.reset_failed_login_attempts(user_data.id).await?;

        // Update last login timestamp
        sqlx::query!(
            "UPDATE users SET last_login = NOW() WHERE id = $1",
            user_data.id
        )
        .execute(&*self.db_pool)
        .await?;

        let user = User {
            id: user_data.id,
            email: user_data.email.clone(),
            name: user_data.name,
            role: user_data.role.clone(),
            created_at: user_data.created_at.unwrap_or_else(|| chrono::Utc::now()),
            last_login: chrono::Utc::now(),
        };

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: user.role.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
        };

        let access_token = encode(&Header::default(), &claims, &self.encoding_key)?;

        // Generate secure refresh token and store session
        let refresh_token_raw = format!("{}{}", Uuid::new_v4(), Uuid::new_v4());
        let refresh_token_salt = SaltString::generate(&mut OsRng);
        let refresh_token_hash = self.argon2
            .hash_password(refresh_token_raw.as_bytes(), &refresh_token_salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash refresh token: {}", e))?
            .to_string();

        // Store session in database
        let session_expires = chrono::Utc::now() + chrono::Duration::days(30);
        sqlx::query!(
            r#"
            INSERT INTO user_sessions (user_id, refresh_token_hash, expires_at)
            VALUES ($1, $2, $3)
            "#,
            user.id,
            refresh_token_hash,
            session_expires
        )
        .execute(&*self.db_pool)
        .await?;

        Ok(LoginResponse {
            access_token,
            refresh_token: refresh_token_raw,
            expires_in: 24 * 3600, // 24 hours
            user,
        })
    }

    /// Validate JWT token
    pub async fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::new(Algorithm::HS256),
        )?;

        Ok(token_data.claims)
    }

    /// Refresh JWT token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<String> {
        let claims = self.validate_token(refresh_token).await?;

        // Generate new access token with updated expiration
        let new_claims = Claims {
            sub: claims.sub,
            email: claims.email,
            role: claims.role,
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
        };

        let new_token = encode(&Header::default(), &new_claims, &self.encoding_key)?;
        Ok(new_token)
    }

    /// Get user from token
    pub async fn get_user_from_token(&self, token: &str) -> Result<User> {
        let claims = self.validate_token(token).await?;

        Ok(User {
            id: Uuid::parse_str(&claims.sub)?,
            email: claims.email.clone(),
            name: claims.email.split('@').next().unwrap_or("User").to_string(),
            role: claims.role,
            created_at: chrono::Utc::now() - chrono::Duration::days(30),
            last_login: chrono::Utc::now(),
        })
    }

    /// Check if user has required role
    pub async fn check_role(&self, token: &str, required_role: &str) -> Result<bool> {
        let claims = self.validate_token(token).await?;

        let has_permission = match required_role {
            "admin" => claims.role == "admin",
            "user" => claims.role == "user" || claims.role == "admin",
            _ => false,
        };

        Ok(has_permission)
    }

    /// Increment failed login attempts and lock account if necessary
    async fn increment_failed_login_attempts(&self, user_id: Uuid) -> Result<()> {
        let result = sqlx::query!(
            r#"
            UPDATE users
            SET failed_login_attempts = failed_login_attempts + 1,
                locked_until = CASE
                    WHEN failed_login_attempts + 1 >= 5 THEN NOW() + INTERVAL '15 minutes'
                    ELSE locked_until
                END
            WHERE id = $1
            RETURNING failed_login_attempts
            "#,
            user_id
        )
        .fetch_one(&*self.db_pool)
        .await?;

        if result.failed_login_attempts.unwrap_or(0) >= 5 {
            println!("🔒 Account {} locked due to too many failed login attempts", user_id);
        }

        Ok(())
    }

    /// Reset failed login attempts on successful login
    async fn reset_failed_login_attempts(&self, user_id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET failed_login_attempts = 0, locked_until = NULL WHERE id = $1",
            user_id
        )
        .execute(&*self.db_pool)
        .await?;

        Ok(())
    }

    /// Register a new user with secure password hashing
    pub async fn register_user(&self, email: &str, password: &str, name: &str) -> Result<User> {
        // Validate input
        if email.is_empty() || password.is_empty() || name.is_empty() {
            return Err(anyhow::anyhow!("All fields are required"));
        }

        if !email.contains('@') || email.len() > 254 {
            return Err(anyhow::anyhow!("Invalid email format"));
        }

        if password.len() < 8 {
            return Err(anyhow::anyhow!("Password must be at least 8 characters long"));
        }

        // Check if user already exists
        let existing_user = sqlx::query!(
            "SELECT id FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&*self.db_pool)
        .await?;

        if existing_user.is_some() {
            return Err(anyhow::anyhow!("User with this email already exists"));
        }

        // Hash password securely
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
            .to_string();

        // Insert user into database
        let user_id = sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash, name, role, email_verified)
            VALUES ($1, $2, $3, 'user', false)
            RETURNING id, created_at
            "#,
            email,
            password_hash,
            name
        )
        .fetch_one(&*self.db_pool)
        .await?;

        println!("✅ New user registered: {}", email);

        Ok(User {
            id: user_id.id,
            email: email.to_string(),
            name: name.to_string(),
            role: "user".to_string(),
            created_at: user_id.created_at.unwrap_or_else(|| chrono::Utc::now()),
            last_login: chrono::Utc::now(),
        })
    }

    /// Verify user email
    pub async fn verify_email(&self, user_id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET email_verified = true WHERE id = $1",
            user_id
        )
        .execute(&*self.db_pool)
        .await?;

        println!("✅ Email verified for user: {}", user_id);
        Ok(())
    }

    /// Invalidate all user sessions (logout from all devices)
    pub async fn invalidate_all_sessions(&self, user_id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE user_sessions SET is_active = false WHERE user_id = $1",
            user_id
        )
        .execute(&*self.db_pool)
        .await?;

        println!("✅ All sessions invalidated for user: {}", user_id);
        Ok(())
    }
}