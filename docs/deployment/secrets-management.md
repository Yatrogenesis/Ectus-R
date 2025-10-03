# Secrets Management - AION/Ectus-R

## Overview

AION/Ectus-R uses a secure secrets management system to handle sensitive configuration data like API keys, database credentials, and encryption keys. This document describes the architecture and best practices.

## Architecture

### Secrets Manager

The `SecretsManager` (located in `crates/aion-core/src/secrets_manager.rs`) provides a unified interface for loading and accessing secrets from multiple sources:

1. **Environment Variables** (Default for development)
2. **JSON Files** (For local testing)
3. **AWS Secrets Manager** (Production - requires `aws` feature)
4. **HashiCorp Vault** (Enterprise - requires `vault` feature)

### Secret Categories

#### Required Secrets
- `JWT_SECRET`: 256-bit secret for JWT token signing
- `ENCRYPTION_KEY`: 256-bit key for data encryption
- `DATABASE_URL`: PostgreSQL connection string
- `REDIS_URL`: Redis connection string

#### LLM Provider Secrets (At least one required)
- `GROQ_API_KEY`: Groq API key (fastest inference)
- `OPENAI_API_KEY`: OpenAI API key (highest quality)
- `GITHUB_TOKEN`: GitHub Personal Access Token (free tier)
- `HUGGINGFACE_API_KEY`: Hugging Face API token
- `CLOUDFLARE_API_KEY`: Cloudflare Workers AI key

#### Optional Secrets
- `STRIPE_SECRET_KEY`: Stripe payment processing
- `MINIO_ACCESS_KEY`/`MINIO_SECRET_KEY`: Object storage
- `DOCKER_REGISTRY_*`: Container registry credentials

## Security Features

### 1. Validation
- **Length validation**: JWT and encryption keys must be ≥32 characters
- **Format validation**: URLs must have proper schemes
- **Required secrets check**: Ensures all mandatory secrets are present
- **Provider validation**: At least one LLM provider must be configured

### 2. Git Protection
- `.env` files are in `.gitignore`
- Pre-commit hooks prevent credential commits
- Secrets are never logged or exposed in error messages

### 3. Runtime Security
- Secrets are loaded once at startup
- No secrets in memory dumps or core files
- Secrets masked in debug output

## Setup Instructions

### Development Environment

1. **Copy environment template:**
   ```bash
   cp .env.example .env
   ```

2. **Generate secure secrets:**
   ```bash
   # JWT Secret
   openssl rand -hex 32

   # Encryption Key
   openssl rand -hex 32
   ```

3. **Configure API keys:**
   - Get Groq API key: https://console.groq.com/keys
   - Get OpenAI API key: https://platform.openai.com/api-keys
   - Configure at least one LLM provider

4. **Set database credentials:**
   ```bash
   # PostgreSQL
   DATABASE_URL=postgresql://user:password@localhost:5432/database

   # Redis
   REDIS_URL=redis://:redis_password@localhost:6379
   ```

### Production Environment

#### Option 1: Environment Variables
Set secrets as environment variables on your production server:

```bash
export JWT_SECRET="your_generated_jwt_secret"
export ENCRYPTION_KEY="your_generated_encryption_key"
export GROQ_API_KEY="gsk_your_groq_key"
export DATABASE_URL="postgresql://user:password@host:5432/database"
```

#### Option 2: AWS Secrets Manager (Recommended)
1. **Enable AWS feature:**
   ```toml
   # Cargo.toml
   aion-core = { path = "../aion-core", features = ["aws"] }
   ```

2. **Create secrets in AWS:**
   ```bash
   aws secretsmanager create-secret \
     --name "aion/production/jwt" \
     --secret-string "your_jwt_secret"
   ```

3. **Configure secrets manager:**
   ```rust
   use aion_core::{SecretsManager, SecretProvider};

   let mut secrets = SecretsManager::with_provider(
       SecretProvider::AwsSecretsManager
   );
   secrets.initialize().await?;
   ```

#### Option 3: HashiCorp Vault
1. **Enable vault feature:**
   ```toml
   aion-core = { path = "../aion-core", features = ["vault"] }
   ```

2. **Store secrets in Vault:**
   ```bash
   vault kv put secret/aion/jwt value="your_jwt_secret"
   vault kv put secret/aion/encryption value="your_encryption_key"
   ```

## Usage Examples

### Basic Usage
```rust
use aion_core::SecretsManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut secrets = SecretsManager::new();
    secrets.initialize().await?;

    // Get required secrets
    let jwt_secret = secrets.get_jwt_secret()?;
    let db_url = secrets.get_database_url()?;

    // Get optional secrets
    if let Some(groq_key) = secrets.get_optional_secret("GROQ_API_KEY") {
        println!("Groq API configured");
    }

    // Get LLM providers in priority order
    let providers = secrets.get_llm_providers();
    for (provider, _key) in providers {
        println!("Available provider: {}", provider);
    }

    Ok(())
}
```

### Web Server Integration
```rust
use aion_core::SecretsManager;
use aion_server::ServerConfig;

async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let mut secrets = SecretsManager::new();
    secrets.initialize().await?;

    let config = ServerConfig {
        jwt_secret: secrets.get_jwt_secret()?,
        database_url: secrets.get_database_url()?,
        redis_url: secrets.get_redis_url()?,
        groq_api_key: secrets.get_optional_secret("GROQ_API_KEY"),
        openai_api_key: secrets.get_optional_secret("OPENAI_API_KEY"),
    };

    aion_server::run(config).await
}
```

## Security Best Practices

### 1. Secret Generation
```bash
# Always use cryptographically secure random generation
openssl rand -hex 32  # For 256-bit secrets
openssl rand -base64 48  # For base64 encoded secrets

# Never use:
# - Dictionary words
# - Personal information
# - Predictable patterns
# - Short secrets (<32 characters)
```

### 2. Secret Rotation
- Rotate secrets regularly (every 90 days minimum)
- Use different secrets for different environments
- Never reuse secrets across services
- Implement graceful secret rotation (overlap period)

### 3. Access Control
- Limit secret access to necessary personnel only
- Use role-based access control (RBAC)
- Log and monitor secret access
- Implement break-glass procedures

### 4. Monitoring
```rust
// Log secret loading (without exposing values)
info!("Loaded {} secrets", secrets.list_configured_secrets().len());

// Monitor for missing secrets
if secrets.get_optional_secret("BACKUP_API_KEY").is_none() {
    warn!("Backup API key not configured - degraded functionality");
}
```

## Troubleshooting

### Common Issues

1. **Missing required secrets:**
   ```
   Error: Missing required secrets: JWT_SECRET, DATABASE_URL
   ```
   **Solution:** Configure missing environment variables or add to secrets file.

2. **Invalid secret format:**
   ```
   Error: JWT_SECRET must be at least 32 characters long
   ```
   **Solution:** Generate proper-length secrets using `openssl rand -hex 32`.

3. **No LLM providers:**
   ```
   Warning: No LLM providers configured. AI features will be disabled.
   ```
   **Solution:** Configure at least one API key (GROQ_API_KEY, OPENAI_API_KEY, etc.).

### Debug Mode
```bash
# Enable debug logging for secrets
RUST_LOG=aion_core::secrets_manager=debug cargo run
```

### Validation
```bash
# Test secrets configuration
cargo test secrets_manager::tests
```

## Migration Guide

### From Hardcoded Secrets
1. Replace hardcoded values with `secrets.get_secret("KEY_NAME")?`
2. Add keys to `.env.example` with placeholder values
3. Update deployment scripts to set environment variables
4. Test with `cargo test` and `cargo run`

### From Direct Environment Access
```rust
// Before
let api_key = std::env::var("GROQ_API_KEY")?;

// After
let api_key = secrets.get_secret("GROQ_API_KEY")?;
```

## Compliance

This secrets management system helps meet:
- **SOC 2 Type II** - Secure configuration management
- **GDPR** - Data protection through encryption
- **HIPAA** - Secure handling of sensitive data
- **PCI DSS** - Secure key management

## Support

For secrets management issues:
1. Check logs with `RUST_LOG=debug`
2. Verify `.env.example` has all required keys
3. Test with minimal configuration first
4. Review this documentation
5. Contact DevOps team for production issues