# Contributing to AION-R Platform

Thank you for your interest in contributing to AION-R! This document provides guidelines and information for contributors.

## How to Contribute

### Reporting Issues

Before creating an issue, please:

1. **Search existing issues** to avoid duplicates
2. **Use the issue templates** provided
3. **Provide detailed information** including:
   - AION-R version
   - Operating system and version
   - Rust version (`rustc --version`)
   - Steps to reproduce
   - Expected vs actual behavior
   - Relevant logs or error messages

### Feature Requests

We welcome feature requests! Please:

1. **Check existing feature requests** first
2. **Describe the use case** and problem you're solving
3. **Explain why this feature belongs in AION-R core** vs a plugin
4. **Consider the impact** on performance, security, and complexity

### Pull Requests

1. Fork the repository and create a feature branch
2. Follow our coding standards (see below)
3. Write tests for your changes
4. Update documentation as needed
5. Ensure all checks pass before submitting

## Development Setup

### Prerequisites

- Rust 1.75+ with `cargo`
- PostgreSQL 14+
- Redis 6+
- Git 2.28+

### Local Development

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/AION-R.git
cd AION-R

# Set up upstream remote
git remote add upstream https://github.com/yatrogenesis/AION-R.git

# Install dependencies
cargo check

# Set up development database
createdb aion_dev
psql -d aion_dev -c "CREATE USER aion_user WITH PASSWORD 'dev_password';"

# Copy development config
cp config/development.toml.example config/development.toml

# Run migrations
cargo run --bin aion-migration migrate

# Start development server
./scripts/start-dev.sh
```

### Development Workflow

```bash
# Create feature branch
git checkout -b feature/my-awesome-feature

# Make your changes
# ...

# Run tests
cargo test

# Run linting
cargo clippy -- -D warnings

# Format code
cargo fmt

# Run security audit
cargo audit

# Commit changes
git add .
git commit -m "feat: add awesome feature

- Add new functionality X
- Improve performance of Y
- Fix edge case in Z

Closes #123"

# Push to your fork
git push origin feature/my-awesome-feature

# Create pull request
gh pr create --title "feat: add awesome feature" --body "Description of changes..."
```

## Coding Standards

### Rust Style Guide

We follow the official Rust style guide with these additions:

#### Code Organization

```rust
// 1. Standard library imports
use std::collections::HashMap;
use std::time::Duration;

// 2. External crate imports
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

// 3. Internal crate imports
use crate::error::Result;
use crate::models::User;

// 4. Local module imports
use super::utils;
```

#### Error Handling

```rust
//  Good: Use custom error types
pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {message}")]
    Validation { message: String },
}

//  Good: Propagate errors with context
pub async fn create_user(data: UserData) -> Result<User> {
    let user = validate_user_data(data)
        .with_context("Invalid user data")?;

    database::insert_user(user).await
        .with_context("Failed to create user")
}

//  Bad: Don't panic or unwrap in production code
let user = database::get_user(id).unwrap(); // Never do this!
```

#### Documentation

```rust
/// Creates a new user in the system
///
/// # Arguments
///
/// * `data` - User creation data containing email, name, etc.
///
/// # Returns
///
/// Returns the created user with generated ID and timestamps
///
/// # Errors
///
/// This function will return an error if:
/// * The email is already taken
/// * The data fails validation
/// * Database operation fails
///
/// # Examples
///
/// ```rust
/// use aion_core::users::{create_user, UserData};
///
/// let data = UserData {
///     email: "user@example.com".to_string(),
///     name: "John Doe".to_string(),
/// };
///
/// let user = create_user(data).await?;
/// println!("Created user: {}", user.id);
/// ```
pub async fn create_user(data: UserData) -> Result<User> {
    // Implementation...
}
```

#### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_create_user_success() {
        // Arrange
        let data = UserData {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        };

        // Act
        let result = create_user(data).await;

        // Assert
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test User");
    }

    #[tokio::test]
    async fn test_create_user_duplicate_email() {
        // Test error cases
        let data = UserData {
            email: "duplicate@example.com".to_string(),
            name: "Test User".to_string(),
        };

        // Create first user
        create_user(data.clone()).await.unwrap();

        // Try to create duplicate
        let result = create_user(data).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Validation { .. }));
    }
}
```

### Security Guidelines

#### Input Validation

```rust
//  Good: Validate all inputs
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct UserInput {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(custom = "validate_password")]
    pub password: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("Password too short"));
    }
    // Additional password validation...
    Ok(())
}
```

#### SQL Injection Prevention

```rust
//  Good: Use parameterized queries
pub async fn get_user_by_email(email: &str) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, name FROM users WHERE email = $1",
        email
    )
    .fetch_optional(&pool)
    .await?;

    Ok(user)
}

//  Bad: String concatenation
let query = format!("SELECT * FROM users WHERE email = '{}'", email); // NEVER!
```

#### Authentication & Authorization

```rust
//  Good: Always check permissions
pub async fn update_user(
    user_id: Uuid,
    current_user: &AuthenticatedUser,
    updates: UserUpdates,
) -> Result<User> {
    // Check authorization
    if !current_user.can_edit_user(user_id) {
        return Err(AppError::Forbidden);
    }

    // Proceed with update
    database::update_user(user_id, updates).await
}
```

### Performance Guidelines

#### Async Best Practices

```rust
//  Good: Use concurrent operations when possible
pub async fn get_user_dashboard(user_id: Uuid) -> Result<Dashboard> {
    let (user, projects, notifications) = tokio::try_join!(
        get_user(user_id),
        get_user_projects(user_id),
        get_user_notifications(user_id)
    )?;

    Ok(Dashboard {
        user,
        projects,
        notifications,
    })
}

//  Bad: Sequential operations
pub async fn get_user_dashboard_slow(user_id: Uuid) -> Result<Dashboard> {
    let user = get_user(user_id).await?;
    let projects = get_user_projects(user_id).await?;
    let notifications = get_user_notifications(user_id).await?;

    Ok(Dashboard {
        user,
        projects,
        notifications,
    })
}
```

#### Memory Management

```rust
//  Good: Use references when possible
pub fn process_large_dataset(data: &[DataPoint]) -> ProcessingResult {
    data.iter()
        .filter(|point| point.is_valid())
        .map(|point| point.transform())
        .collect()
}

//  Good: Use streaming for large datasets
pub async fn process_large_file(
    file_path: &Path,
) -> Result<impl Stream<Item = Result<ProcessedData>>> {
    let file = File::open(file_path).await?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    Ok(lines.map(|line| {
        line.map_err(Into::into)
            .and_then(|line| process_line(&line))
    }))
}
```

## Testing Requirements

### Test Coverage

- Minimum 80% code coverage for all new code
- Unit tests for all public functions
- Integration tests for API endpoints
- Performance tests for critical paths

### Test Categories

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test integration

# Documentation tests
cargo test --doc

# Performance benchmarks
cargo bench

# Security tests
cargo test --test security
```

### Writing Good Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use tokio_test;

    // Test naming: test_[function]_[scenario]_[expected_result]
    #[tokio::test]
    async fn test_create_user_valid_input_returns_user() {
        // Arrange
        let user_data = UserData {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        };

        // Act
        let result = create_user(user_data).await;

        // Assert
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email, "test@example.com");
        assert!(!user.id.is_nil());
    }

    #[tokio::test]
    async fn test_create_user_invalid_email_returns_validation_error() {
        // Test error conditions
        let user_data = UserData {
            email: "invalid-email".to_string(),
            name: "Test User".to_string(),
        };

        let result = create_user(user_data).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation { message } => {
                assert!(message.contains("email"));
            }
            _ => panic!("Expected validation error"),
        }
    }
}
```

## Documentation Standards

### Code Documentation

- All public APIs must have comprehensive documentation
- Examples for complex functions
- Error conditions clearly documented
- Performance characteristics for critical functions

### Architectural Documentation

- Architecture Decision Records (ADRs) for significant changes
- API documentation using OpenAPI 3.0
- Database schema documentation
- Deployment guides for different environments

## Release Process

### Versioning

We follow [Semantic Versioning (SemVer)](https://semver.org/):

- MAJOR version for incompatible API changes
- MINOR version for backwards-compatible functionality
- PATCH version for backwards-compatible bug fixes

### Commit Message Format

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(auth): add OAuth2 integration

- Add OAuth2 provider support
- Implement token refresh mechanism
- Add user profile sync

Closes #123

fix(api): handle edge case in user validation

The validation was failing for users with unicode characters
in their names. This fix properly handles UTF-8 encoding.

Fixes #456

docs(readme): update installation instructions

Add Docker installation steps and troubleshooting section.
```

### Pull Request Process

1. Create feature branch from `develop`
2. Implement changes following coding standards
3. Add comprehensive tests with good coverage
4. Update documentation as needed
5. Run full test suite and ensure all checks pass
6. Create pull request with detailed description
7. Address review feedback promptly
8. Squash and merge after approval

### Review Checklist

Reviewers should verify:

- [ ] Code follows style guidelines
- [ ] Tests are comprehensive and pass
- [ ] Documentation is updated
- [ ] Security considerations addressed
- [ ] Performance impact acceptable
- [ ] Breaking changes documented
- [ ] Backwards compatibility maintained

## Contribution Areas

### High Priority

- Performance optimizations
- Security enhancements
- Enterprise integrations
- Documentation improvements
- Test coverage expansion

### Medium Priority

- New AI/ML algorithms
- Additional authentication providers
- Monitoring enhancements
- Developer tooling
- Example applications

### Low Priority

- UI/UX improvements
- Additional output formats
- Nice-to-have features
- Code cleanup
- Minor optimizations

## Getting Help

### Community

- GitHub Discussions: For questions and general discussion
- GitHub Issues: For bug reports and feature requests
- Documentation: Comprehensive guides and API references

### Enterprise Support

- Enterprise Slack: Priority support for enterprise customers
- Professional Services: Custom development and consulting
- Training Programs: Team training and best practices

## License

By contributing to AION-R, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for contributing to AION-R! Your efforts help improve this enterprise AI platform.