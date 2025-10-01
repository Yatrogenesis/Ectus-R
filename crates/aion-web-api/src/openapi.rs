//! OpenAPI Specification Generator
//!
//! Implements automated OpenAPI 3.1 specification generation
//! Addresses Audit Recommendation #3: API Documentation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAPISpec {
    pub openapi: String,
    pub info: Info,
    pub servers: Vec<Server>,
    pub paths: HashMap<String, PathItem>,
    pub components: Components,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub title: String,
    pub version: String,
    pub description: String,
    pub contact: Contact,
    pub license: License,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub summary: String,
    pub description: String,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RequestBody>,
    pub responses: HashMap<String, Response>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: String,
    pub description: String,
    pub required: bool,
    pub schema: Schema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    pub description: String,
    pub required: bool,
    pub content: HashMap<String, MediaType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<HashMap<String, MediaType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    pub schema: Schema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Schema {
    Ref {
        #[serde(rename = "$ref")]
        reference: String,
    },
    Object {
        #[serde(rename = "type")]
        schema_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        properties: Option<HashMap<String, Box<Schema>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        items: Option<Box<Schema>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    pub schemas: HashMap<String, ComponentSchema>,
    pub security_schemes: HashMap<String, SecurityScheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSchema {
    #[serde(rename = "type")]
    pub schema_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, PropertySchema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertySchema {
    #[serde(rename = "type")]
    pub schema_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScheme {
    #[serde(rename = "type")]
    pub scheme_type: String,
    pub scheme: String,
    #[serde(rename = "bearerFormat")]
    pub bearer_format: String,
}

pub type SecurityRequirement = HashMap<String, Vec<String>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub description: String,
}

/// Generate Ectus-R API OpenAPI specification
pub fn generate_openapi_spec() -> OpenAPISpec {
    OpenAPISpec {
        openapi: "3.1.0".to_string(),
        info: Info {
            title: "Ectus-R API".to_string(),
            version: "1.0.0".to_string(),
            description: "Autonomous Software Engineering Platform API - Powered by AION-R Engine".to_string(),
            contact: Contact {
                name: "Yatrogenesis".to_string(),
                email: "info@yatrogenesis.com".to_string(),
                url: "https://github.com/Yatrogenesis/Ectus-R".to_string(),
            },
            license: License {
                name: "MIT".to_string(),
                url: "https://opensource.org/licenses/MIT".to_string(),
            },
        },
        servers: vec![
            Server {
                url: "http://localhost:8080".to_string(),
                description: "Local development server".to_string(),
            },
            Server {
                url: "https://api.ectus-r.dev".to_string(),
                description: "Production server".to_string(),
            },
        ],
        paths: generate_paths(),
        components: generate_components(),
        tags: vec![
            Tag {
                name: "Health".to_string(),
                description: "Health check and system status endpoints".to_string(),
            },
            Tag {
                name: "Authentication".to_string(),
                description: "User authentication and authorization".to_string(),
            },
            Tag {
                name: "AI".to_string(),
                description: "AI inference and code generation endpoints".to_string(),
            },
            Tag {
                name: "Projects".to_string(),
                description: "Project management and generation".to_string(),
            },
        ],
    }
}

fn generate_paths() -> HashMap<String, PathItem> {
    let mut paths = HashMap::new();

    // Health endpoint
    paths.insert(
        "/health".to_string(),
        PathItem {
            get: Some(Operation {
                summary: "Health check".to_string(),
                description: "Check system health and service status".to_string(),
                tags: vec!["Health".to_string()],
                parameters: None,
                request_body: None,
                responses: {
                    let mut responses = HashMap::new();
                    responses.insert(
                        "200".to_string(),
                        Response {
                            description: "System is healthy".to_string(),
                            content: Some({
                                let mut content = HashMap::new();
                                content.insert(
                                    "application/json".to_string(),
                                    MediaType {
                                        schema: Schema::Ref {
                                            reference: "#/components/schemas/HealthResponse".to_string(),
                                        },
                                    },
                                );
                                content
                            }),
                        },
                    );
                    responses
                },
                security: None,
            }),
            post: None,
            put: None,
            delete: None,
            patch: None,
        },
    );

    // Auth endpoints
    paths.insert(
        "/api/v1/auth/register".to_string(),
        PathItem {
            get: None,
            post: Some(Operation {
                summary: "Register new user".to_string(),
                description: "Create a new user account".to_string(),
                tags: vec!["Authentication".to_string()],
                parameters: None,
                request_body: Some(RequestBody {
                    description: "User registration data".to_string(),
                    required: true,
                    content: {
                        let mut content = HashMap::new();
                        content.insert(
                            "application/json".to_string(),
                            MediaType {
                                schema: Schema::Ref {
                                    reference: "#/components/schemas/RegisterRequest".to_string(),
                                },
                            },
                        );
                        content
                    },
                }),
                responses: {
                    let mut responses = HashMap::new();
                    responses.insert(
                        "201".to_string(),
                        Response {
                            description: "User created successfully".to_string(),
                            content: Some({
                                let mut content = HashMap::new();
                                content.insert(
                                    "application/json".to_string(),
                                    MediaType {
                                        schema: Schema::Ref {
                                            reference: "#/components/schemas/AuthResponse".to_string(),
                                        },
                                    },
                                );
                                content
                            }),
                        },
                    );
                    responses
                },
                security: None,
            }),
            put: None,
            delete: None,
            patch: None,
        },
    );

    paths.insert(
        "/api/v1/auth/login".to_string(),
        PathItem {
            get: None,
            post: Some(Operation {
                summary: "User login".to_string(),
                description: "Authenticate user and receive JWT token".to_string(),
                tags: vec!["Authentication".to_string()],
                parameters: None,
                request_body: Some(RequestBody {
                    description: "Login credentials".to_string(),
                    required: true,
                    content: {
                        let mut content = HashMap::new();
                        content.insert(
                            "application/json".to_string(),
                            MediaType {
                                schema: Schema::Ref {
                                    reference: "#/components/schemas/LoginRequest".to_string(),
                                },
                            },
                        );
                        content
                    },
                }),
                responses: {
                    let mut responses = HashMap::new();
                    responses.insert(
                        "200".to_string(),
                        Response {
                            description: "Login successful".to_string(),
                            content: Some({
                                let mut content = HashMap::new();
                                content.insert(
                                    "application/json".to_string(),
                                    MediaType {
                                        schema: Schema::Ref {
                                            reference: "#/components/schemas/AuthResponse".to_string(),
                                        },
                                    },
                                );
                                content
                            }),
                        },
                    );
                    responses
                },
                security: None,
            }),
            put: None,
            delete: None,
            patch: None,
        },
    );

    // AI endpoints
    paths.insert(
        "/api/v1/ai/generate".to_string(),
        PathItem {
            get: None,
            post: Some(Operation {
                summary: "Generate code from requirements".to_string(),
                description: "Generate production-ready code using AION-R AI engine".to_string(),
                tags: vec!["AI".to_string()],
                parameters: None,
                request_body: Some(RequestBody {
                    description: "Code generation request".to_string(),
                    required: true,
                    content: {
                        let mut content = HashMap::new();
                        content.insert(
                            "application/json".to_string(),
                            MediaType {
                                schema: Schema::Ref {
                                    reference: "#/components/schemas/CodeGenerationRequest".to_string(),
                                },
                            },
                        );
                        content
                    },
                }),
                responses: {
                    let mut responses = HashMap::new();
                    responses.insert(
                        "200".to_string(),
                        Response {
                            description: "Code generated successfully".to_string(),
                            content: Some({
                                let mut content = HashMap::new();
                                content.insert(
                                    "application/json".to_string(),
                                    MediaType {
                                        schema: Schema::Ref {
                                            reference: "#/components/schemas/CodeGenerationResponse".to_string(),
                                        },
                                    },
                                );
                                content
                            }),
                        },
                    );
                    responses
                },
                security: Some(vec![{
                    let mut req = HashMap::new();
                    req.insert("bearerAuth".to_string(), vec![]);
                    req
                }]),
            }),
            put: None,
            delete: None,
            patch: None,
        },
    );

    paths
}

fn generate_components() -> Components {
    let mut schemas = HashMap::new();

    // HealthResponse
    schemas.insert(
        "HealthResponse".to_string(),
        ComponentSchema {
            schema_type: "object".to_string(),
            properties: Some({
                let mut props = HashMap::new();
                props.insert(
                    "success".to_string(),
                    PropertySchema {
                        schema_type: "boolean".to_string(),
                        description: Some("Health check success status".to_string()),
                        format: None,
                    },
                );
                props.insert(
                    "data".to_string(),
                    PropertySchema {
                        schema_type: "object".to_string(),
                        description: Some("Health check data".to_string()),
                        format: None,
                    },
                );
                props
            }),
            required: Some(vec!["success".to_string()]),
        },
    );

    // RegisterRequest
    schemas.insert(
        "RegisterRequest".to_string(),
        ComponentSchema {
            schema_type: "object".to_string(),
            properties: Some({
                let mut props = HashMap::new();
                props.insert(
                    "username".to_string(),
                    PropertySchema {
                        schema_type: "string".to_string(),
                        description: Some("Username (3-32 characters)".to_string()),
                        format: None,
                    },
                );
                props.insert(
                    "email".to_string(),
                    PropertySchema {
                        schema_type: "string".to_string(),
                        description: Some("Email address".to_string()),
                        format: Some("email".to_string()),
                    },
                );
                props.insert(
                    "password".to_string(),
                    PropertySchema {
                        schema_type: "string".to_string(),
                        description: Some("Password (minimum 8 characters)".to_string()),
                        format: Some("password".to_string()),
                    },
                );
                props
            }),
            required: Some(vec!["username".to_string(), "email".to_string(), "password".to_string()]),
        },
    );

    // LoginRequest
    schemas.insert(
        "LoginRequest".to_string(),
        ComponentSchema {
            schema_type: "object".to_string(),
            properties: Some({
                let mut props = HashMap::new();
                props.insert(
                    "username".to_string(),
                    PropertySchema {
                        schema_type: "string".to_string(),
                        description: Some("Username or email".to_string()),
                        format: None,
                    },
                );
                props.insert(
                    "password".to_string(),
                    PropertySchema {
                        schema_type: "string".to_string(),
                        description: Some("User password".to_string()),
                        format: Some("password".to_string()),
                    },
                );
                props
            }),
            required: Some(vec!["username".to_string(), "password".to_string()]),
        },
    );

    // AuthResponse
    schemas.insert(
        "AuthResponse".to_string(),
        ComponentSchema {
            schema_type: "object".to_string(),
            properties: Some({
                let mut props = HashMap::new();
                props.insert(
                    "token".to_string(),
                    PropertySchema {
                        schema_type: "string".to_string(),
                        description: Some("JWT access token".to_string()),
                        format: None,
                    },
                );
                props.insert(
                    "user_id".to_string(),
                    PropertySchema {
                        schema_type: "string".to_string(),
                        description: Some("User ID (UUID)".to_string()),
                        format: Some("uuid".to_string()),
                    },
                );
                props
            }),
            required: Some(vec!["token".to_string(), "user_id".to_string()]),
        },
    );

    // CodeGenerationRequest
    schemas.insert(
        "CodeGenerationRequest".to_string(),
        ComponentSchema {
            schema_type: "object".to_string(),
            properties: Some({
                let mut props = HashMap::new();
                props.insert(
                    "requirements".to_string(),
                    PropertySchema {
                        schema_type: "string".to_string(),
                        description: Some("Project requirements description".to_string()),
                        format: None,
                    },
                );
                props.insert(
                    "language".to_string(),
                    PropertySchema {
                        schema_type: "string".to_string(),
                        description: Some("Target programming language".to_string()),
                        format: None,
                    },
                );
                props.insert(
                    "framework".to_string(),
                    PropertySchema {
                        schema_type: "string".to_string(),
                        description: Some("Target framework (optional)".to_string()),
                        format: None,
                    },
                );
                props
            }),
            required: Some(vec!["requirements".to_string(), "language".to_string()]),
        },
    );

    // CodeGenerationResponse
    schemas.insert(
        "CodeGenerationResponse".to_string(),
        ComponentSchema {
            schema_type: "object".to_string(),
            properties: Some({
                let mut props = HashMap::new();
                props.insert(
                    "project_id".to_string(),
                    PropertySchema {
                        schema_type: "string".to_string(),
                        description: Some("Generated project ID".to_string()),
                        format: Some("uuid".to_string()),
                    },
                );
                props.insert(
                    "files".to_string(),
                    PropertySchema {
                        schema_type: "array".to_string(),
                        description: Some("Generated files".to_string()),
                        format: None,
                    },
                );
                props
            }),
            required: Some(vec!["project_id".to_string(), "files".to_string()]),
        },
    );

    let mut security_schemes = HashMap::new();
    security_schemes.insert(
        "bearerAuth".to_string(),
        SecurityScheme {
            scheme_type: "http".to_string(),
            scheme: "bearer".to_string(),
            bearer_format: "JWT".to_string(),
        },
    );

    Components {
        schemas,
        security_schemes,
    }
}

/// Export OpenAPI spec as JSON
pub fn export_json() -> anyhow::Result<String> {
    let spec = generate_openapi_spec();
    serde_json::to_string_pretty(&spec)
        .map_err(|e| anyhow::anyhow!("Failed to serialize OpenAPI spec: {}", e))
}

/// Export OpenAPI spec as YAML
pub fn export_yaml() -> anyhow::Result<String> {
    let spec = generate_openapi_spec();
    serde_yaml::to_string(&spec)
        .map_err(|e| anyhow::anyhow!("Failed to serialize OpenAPI spec: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_openapi_spec() {
        let spec = generate_openapi_spec();
        assert_eq!(spec.openapi, "3.1.0");
        assert_eq!(spec.info.title, "Ectus-R API");
        assert!(!spec.paths.is_empty());
        assert!(!spec.components.schemas.is_empty());
    }

    #[test]
    fn test_export_json() {
        let json = export_json().unwrap();
        assert!(json.contains("\"openapi\": \"3.1.0\""));
        assert!(json.contains("Ectus-R API"));
    }
}
