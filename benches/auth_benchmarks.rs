use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use aion_auth::*;
use chrono::Utc;
use std::collections::HashMap;
use std::time::Duration;
use tokio::runtime::Runtime;
use uuid::Uuid;

// Authentication and Authorization Performance Benchmarks

fn benchmark_password_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("password_hashing");

    let passwords = vec![
        "password123",
        "SecurePassword!@#",
        "VeryLongPasswordWithManyCharacters123!@#$%^&*()",
        "🔐🛡️🔑🛠️🔒", // Unicode characters
    ];

    for password in passwords {
        group.bench_with_input(
            BenchmarkId::new("hash_password", password.len()),
            password,
            |b, password| {
                b.iter(|| {
                    let salt = generate_salt();
                    let hashed = hash_password(black_box(password), black_box(&salt));
                    black_box(hashed)
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("verify_password", password.len()),
            password,
            |b, password| {
                b.iter_setup(|| {
                    let salt = generate_salt();
                    let hashed = hash_password(password, &salt);
                    (password, hashed, salt)
                }, |(password, hashed, salt)| {
                    let result = verify_password(black_box(password), black_box(&hashed), black_box(&salt));
                    black_box(result)
                });
            },
        );
    }

    group.finish();
}

fn benchmark_jwt_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("jwt_operations");

    let claims = JwtClaims {
        sub: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        iat: Utc::now().timestamp(),
        exp: (Utc::now() + chrono::Duration::hours(1)).timestamp(),
        aud: "test-audience".to_string(),
        iss: "test-issuer".to_string(),
        roles: vec!["user".to_string(), "admin".to_string()],
        permissions: vec!["read".to_string(), "write".to_string(), "delete".to_string()],
        session_id: Uuid::new_v4(),
    };

    let secret = "test-secret-key-for-benchmarking";

    group.bench_function("jwt_generation", |b| {
        b.iter(|| {
            let token = generate_jwt_token(black_box(&claims), black_box(secret)).unwrap();
            black_box(token)
        });
    });

    group.bench_function("jwt_validation", |b| {
        b.iter_setup(|| {
            generate_jwt_token(&claims, secret).unwrap()
        }, |token| {
            let decoded = validate_jwt_token(
                black_box(&token),
                black_box(secret),
                black_box("test-issuer"),
                black_box("test-audience")
            ).unwrap();
            black_box(decoded)
        });
    });

    // Test with different payload sizes
    for role_count in [1, 5, 10, 20, 50].iter() {
        let large_claims = JwtClaims {
            roles: (0..*role_count).map(|i| format!("role_{}", i)).collect(),
            permissions: (0..*role_count * 3).map(|i| format!("permission_{}", i)).collect(),
            ..claims.clone()
        };

        group.bench_with_input(
            BenchmarkId::new("jwt_generation_large", role_count),
            &large_claims,
            |b, claims| {
                b.iter(|| {
                    let token = generate_jwt_token(black_box(claims), black_box(secret)).unwrap();
                    black_box(token)
                });
            },
        );
    }

    group.finish();
}

fn benchmark_authorization_checks(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("authorization_checks");

    // Setup authorization service with test data
    let auth_service = rt.block_on(async {
        let mut service = AuthorizationService::new();

        // Load test permissions
        let mut role_permissions = HashMap::new();
        role_permissions.insert(
            "admin".to_string(),
            vec![
                Permission {
                    id: Uuid::new_v4(),
                    name: "admin_read".to_string(),
                    description: "Admin read access".to_string(),
                    resource: "users".to_string(),
                    action: "read".to_string(),
                    conditions: None,
                    created_at: Utc::now(),
                },
                Permission {
                    id: Uuid::new_v4(),
                    name: "admin_write".to_string(),
                    description: "Admin write access".to_string(),
                    resource: "users".to_string(),
                    action: "write".to_string(),
                    conditions: None,
                    created_at: Utc::now(),
                },
            ],
        );

        role_permissions.insert(
            "user".to_string(),
            vec![
                Permission {
                    id: Uuid::new_v4(),
                    name: "user_read".to_string(),
                    description: "User read access".to_string(),
                    resource: "documents".to_string(),
                    action: "read".to_string(),
                    conditions: None,
                    created_at: Utc::now(),
                },
            ],
        );

        service.load_role_permissions(role_permissions).await;
        service
    });

    let base_context = AuthorizationContext {
        user_id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        session_id: Uuid::new_v4(),
        roles: vec!["admin".to_string(), "user".to_string()],
        permissions: vec![],
        ip_address: Some("192.168.1.1".to_string()),
        user_agent: Some("Benchmark".to_string()),
        timestamp: Utc::now(),
        additional_attributes: HashMap::new(),
    };

    group.bench_function("simple_permission_check", |b| {
        b.to_async(&rt).iter(|| async {
            let result = auth_service
                .check_permission(
                    black_box(&base_context),
                    black_box("users"),
                    None,
                    black_box("read")
                )
                .await
                .unwrap();
            black_box(result)
        });
    });

    // Benchmark with different numbers of roles
    for role_count in [1, 5, 10, 20].iter() {
        let multi_role_context = AuthorizationContext {
            roles: (0..*role_count).map(|i| format!("role_{}", i)).collect(),
            ..base_context.clone()
        };

        group.bench_with_input(
            BenchmarkId::new("multi_role_check", role_count),
            &multi_role_context,
            |b, context| {
                b.to_async(&rt).iter(|| async {
                    let result = auth_service
                        .check_permission(
                            black_box(context),
                            black_box("documents"),
                            None,
                            black_box("read")
                        )
                        .await
                        .unwrap();
                    black_box(result)
                });
            },
        );
    }

    group.finish();
}

fn benchmark_policy_evaluation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("policy_evaluation");

    let auth_service = rt.block_on(async {
        let mut service = AuthorizationService::new();

        // Add multiple policies
        for i in 0..50 {
            let policy = helpers::create_role_policy(
                &format!("policy_{}", i),
                vec![format!("role_{}", i % 5)],
                vec!["documents".to_string()],
                vec!["read".to_string()],
                if i % 2 == 0 { PermissionEffect::Allow } else { PermissionEffect::Deny },
            );
            service.add_policy(policy).await.unwrap();
        }

        service
    });

    let context = AuthorizationContext {
        user_id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        session_id: Uuid::new_v4(),
        roles: vec!["role_1".to_string()],
        permissions: vec![],
        ip_address: Some("192.168.1.1".to_string()),
        user_agent: Some("Benchmark".to_string()),
        timestamp: Utc::now(),
        additional_attributes: HashMap::new(),
    };

    group.bench_function("policy_evaluation_50_policies", |b| {
        b.to_async(&rt).iter(|| async {
            let result = auth_service
                .check_permission(
                    black_box(&context),
                    black_box("documents"),
                    None,
                    black_box("read")
                )
                .await
                .unwrap();
            black_box(result)
        });
    });

    group.finish();
}

fn benchmark_session_management(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("session_management");

    group.bench_function("session_creation", |b| {
        b.to_async(&rt).iter(|| async {
            let session_manager = SessionManager::new();
            let user_id = Uuid::new_v4();
            let tenant_id = Uuid::new_v4();

            let session = session_manager
                .create_session(
                    black_box(user_id),
                    black_box(tenant_id),
                    Some("192.168.1.1".to_string())
                )
                .await
                .unwrap();

            black_box(session)
        });
    });

    group.bench_function("session_validation", |b| {
        b.to_async(&rt).iter_setup(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let session_manager = SessionManager::new();
                let session = session_manager
                    .create_session(Uuid::new_v4(), Uuid::new_v4(), None)
                    .await
                    .unwrap();
                (session_manager, session.id)
            })
        }, |(session_manager, session_id)| async move {
            let is_valid = session_manager
                .validate_session(black_box(session_id))
                .await
                .unwrap();
            black_box(is_valid)
        });
    });

    // Benchmark concurrent session operations
    for concurrency in [1, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_session_creation", concurrency),
            concurrency,
            |b, &concurrency| {
                b.to_async(&rt).iter(|| async {
                    let session_manager = SessionManager::new();

                    let mut handles = Vec::new();
                    for i in 0..concurrency {
                        let manager = session_manager.clone();
                        let handle = tokio::spawn(async move {
                            manager
                                .create_session(
                                    Uuid::new_v4(),
                                    Uuid::new_v4(),
                                    Some(format!("192.168.1.{}", i + 1))
                                )
                                .await
                                .unwrap()
                        });
                        handles.push(handle);
                    }

                    let sessions: Vec<_> = futures::future::join_all(handles)
                        .await
                        .into_iter()
                        .map(|r| r.unwrap())
                        .collect();

                    black_box(sessions)
                });
            },
        );
    }

    group.finish();
}

fn benchmark_oauth_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("oauth_operations");

    let oauth_client = OAuthClient {
        id: Uuid::new_v4(),
        client_id: "benchmark-client".to_string(),
        client_secret_hash: "hashed_secret".to_string(),
        name: "Benchmark Client".to_string(),
        description: Some("OAuth client for benchmarking".to_string()),
        redirect_uris: vec!["https://example.com/callback".to_string()],
        allowed_scopes: vec!["read".to_string(), "write".to_string()],
        is_confidential: true,
        is_active: true,
        tenant_id: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    group.bench_function("oauth_client_validation", |b| {
        b.iter(|| {
            let result = validate_oauth_client(black_box(&oauth_client));
            black_box(result)
        });
    });

    group.bench_function("authorization_code_generation", |b| {
        b.to_async(&rt).iter(|| async {
            let auth_code = OAuthAuthorizationCode {
                code: generate_auth_code(),
                client_id: oauth_client.client_id.clone(),
                user_id: Uuid::new_v4(),
                redirect_uri: oauth_client.redirect_uris[0].clone(),
                scopes: oauth_client.allowed_scopes.clone(),
                expires_at: Utc::now() + chrono::Duration::minutes(10),
                used: false,
                created_at: Utc::now(),
            };

            black_box(auth_code)
        });
    });

    group.finish();
}

fn benchmark_access_validation_with_audit(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("access_validation_audit");

    let auth_service = AuthorizationService::new();

    let context = AuthorizationContext {
        user_id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        session_id: Uuid::new_v4(),
        roles: vec!["user".to_string()],
        permissions: vec![],
        ip_address: Some("192.168.1.1".to_string()),
        user_agent: Some("Benchmark Browser".to_string()),
        timestamp: Utc::now(),
        additional_attributes: HashMap::new(),
    };

    group.bench_function("full_access_validation", |b| {
        b.to_async(&rt).iter(|| async {
            let validation_result = auth_service
                .validate_access(
                    black_box(&context),
                    black_box("documents"),
                    Some(Uuid::new_v4()),
                    black_box("read")
                )
                .await
                .unwrap();

            black_box(validation_result)
        });
    });

    group.finish();
}

fn benchmark_security_event_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("security_events");

    group.bench_function("security_event_creation", |b| {
        b.iter(|| {
            let event = SecurityEvent {
                id: Uuid::new_v4(),
                event_type: SecurityEventType::LoginSuccess,
                severity: SecurityEventSeverity::Low,
                user_id: Some(Uuid::new_v4()),
                tenant_id: Uuid::new_v4(),
                resource_type: Some("user".to_string()),
                resource_id: None,
                ip_address: Some("192.168.1.1".to_string()),
                user_agent: Some("Benchmark User Agent".to_string()),
                details: serde_json::json!({
                    "login_method": "password",
                    "success": true,
                    "duration_ms": 150
                }),
                timestamp: Utc::now(),
            };

            black_box(event)
        });
    });

    group.bench_function("security_event_serialization", |b| {
        b.iter_setup(|| {
            SecurityEvent {
                id: Uuid::new_v4(),
                event_type: SecurityEventType::LoginFailure,
                severity: SecurityEventSeverity::Medium,
                user_id: Some(Uuid::new_v4()),
                tenant_id: Uuid::new_v4(),
                resource_type: Some("user".to_string()),
                resource_id: None,
                ip_address: Some("192.168.1.1".to_string()),
                user_agent: Some("Benchmark User Agent".to_string()),
                details: serde_json::json!({
                    "login_method": "password",
                    "failure_reason": "invalid_credentials",
                    "attempt_count": 3
                }),
                timestamp: Utc::now(),
            }
        }, |event| {
            let serialized = serde_json::to_string(black_box(&event)).unwrap();
            black_box(serialized)
        });
    });

    group.finish();
}

// Helper functions for benchmarks

fn generate_salt() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let salt: String = (0..16).map(|_| rng.gen::<char>()).collect();
    salt
}

fn hash_password(password: &str, salt: &str) -> String {
    // Simplified hashing for benchmark (in production, use bcrypt/argon2)
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    salt.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn verify_password(password: &str, hashed: &str, salt: &str) -> bool {
    hash_password(password, salt) == hashed
}

fn generate_jwt_token(claims: &JwtClaims, secret: &str) -> Result<String, String> {
    // Simplified JWT generation for benchmark
    let header = r#"{"alg":"HS256","typ":"JWT"}"#;
    let payload = serde_json::to_string(claims).map_err(|e| e.to_string())?;

    let token = format!("{}.{}.signature",
        base64::encode(header),
        base64::encode(payload)
    );

    Ok(token)
}

fn validate_jwt_token(
    token: &str,
    _secret: &str,
    expected_issuer: &str,
    expected_audience: &str,
) -> Result<JwtClaims, String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid token format".to_string());
    }

    let payload = base64::decode(parts[1]).map_err(|_| "Invalid base64".to_string())?;
    let claims: JwtClaims = serde_json::from_slice(&payload).map_err(|e| e.to_string())?;

    if claims.exp < Utc::now().timestamp() {
        return Err("Token expired".to_string());
    }

    if claims.iss != expected_issuer || claims.aud != expected_audience {
        return Err("Invalid issuer or audience".to_string());
    }

    Ok(claims)
}

fn validate_oauth_client(client: &OAuthClient) -> Result<(), String> {
    if client.redirect_uris.is_empty() {
        return Err("No redirect URIs".to_string());
    }
    if client.client_id.is_empty() {
        return Err("No client ID".to_string());
    }
    Ok(())
}

fn generate_auth_code() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..32).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect()
}

// Mock session manager for benchmarks
#[derive(Clone)]
struct SessionManager {}

impl SessionManager {
    fn new() -> Self {
        Self {}
    }

    async fn create_session(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
        ip_address: Option<String>,
    ) -> Result<Session, String> {
        Ok(Session {
            id: Uuid::new_v4(),
            user_id,
            tenant_id,
            token_hash: "mock_token_hash".to_string(),
            ip_address,
            user_agent: None,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(8),
            last_activity: Utc::now(),
            is_active: true,
        })
    }

    async fn validate_session(&self, _session_id: Uuid) -> Result<bool, String> {
        Ok(true)
    }
}

criterion_group!(
    auth_benches,
    benchmark_password_hashing,
    benchmark_jwt_operations,
    benchmark_authorization_checks,
    benchmark_policy_evaluation,
    benchmark_session_management,
    benchmark_oauth_operations,
    benchmark_access_validation_with_audit,
    benchmark_security_event_processing
);

criterion_main!(auth_benches);