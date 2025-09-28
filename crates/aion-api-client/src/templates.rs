use crate::{types::*, error::Result, client::AionClient};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TemplatesApi {
    client: AionClient,
}

impl TemplatesApi {
    pub(crate) fn new(client: AionClient) -> Self {
        Self { client }
    }

    /// List all available templates with optional filtering and pagination
    pub async fn list(&self, params: Option<TemplateListParams>) -> Result<PaginatedResponse<Template>> {
        let mut path = "/api/v1/templates".to_string();
        let mut query_params = Vec::new();

        if let Some(params) = params {
            if let Some(category) = params.category {
                let category_str = match category {
                    TemplateCategory::Web => "web",
                    TemplateCategory::Mobile => "mobile",
                    TemplateCategory::Desktop => "desktop",
                    TemplateCategory::Api => "api",
                    TemplateCategory::Microservice => "microservice",
                    TemplateCategory::Library => "library",
                    TemplateCategory::Cli => "cli",
                    TemplateCategory::Game => "game",
                    TemplateCategory::AiMl => "ai_ml",
                    TemplateCategory::Blockchain => "blockchain",
                };
                query_params.push(format!("category={}", category_str));
            }

            if let Some(tech_stack) = params.tech_stack {
                query_params.push(format!("tech_stack={}", urlencoding::encode(&tech_stack)));
            }

            if let Some(architecture) = params.architecture {
                query_params.push(format!("architecture={}", urlencoding::encode(&architecture)));
            }

            if !params.tags.is_empty() {
                let tags_str = params.tags.join(",");
                query_params.push(format!("tags={}", urlencoding::encode(&tags_str)));
            }

            if let Some(page) = params.pagination.page {
                query_params.push(format!("page={}", page));
            }

            if let Some(per_page) = params.pagination.per_page {
                query_params.push(format!("per_page={}", per_page));
            }

            if let Some(sort_by) = params.pagination.sort_by {
                query_params.push(format!("sort_by={}", sort_by));
            }

            if let Some(sort_order) = params.pagination.sort_order {
                let order = match sort_order {
                    SortOrder::Asc => "asc",
                    SortOrder::Desc => "desc",
                };
                query_params.push(format!("sort_order={}", order));
            }
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.client.get(&path).await
    }

    /// Get a specific template by ID
    pub async fn get(&self, template_id: Uuid) -> Result<Template> {
        let path = format!("/api/v1/templates/{}", template_id);
        self.client.get(&path).await
    }

    /// Get template content/structure
    pub async fn content(&self, template_id: Uuid) -> Result<TemplateContent> {
        let path = format!("/api/v1/templates/{}/content", template_id);
        self.client.get(&path).await
    }

    /// Download template as archive
    pub async fn download(&self, template_id: Uuid) -> Result<bytes::Bytes> {
        let path = format!("/api/v1/templates/{}/download", template_id);
        let url = self.client.base_url.join(&path)?;
        let response = self.client.client.get(url).send().await?;

        if response.status().is_success() {
            Ok(response.bytes().await?)
        } else {
            Err(crate::error::AionError::Api {
                status: response.status().as_u16(),
                message: "Failed to download template".to_string(),
            })
        }
    }

    /// Generate project from template
    pub async fn generate(&self, request: TemplateGenerateRequest) -> Result<TemplateGenerateResponse> {
        self.client.post("/api/v1/templates/generate", &request).await
    }

    /// Preview template generation (dry run)
    pub async fn preview(&self, request: TemplateGenerateRequest) -> Result<TemplatePreview> {
        self.client.post("/api/v1/templates/preview", &request).await
    }

    /// Search templates
    pub async fn search(&self, query: &str, params: Option<TemplateSearchParams>) -> Result<PaginatedResponse<Template>> {
        let mut path = format!("/api/v1/templates/search?q={}", urlencoding::encode(query));

        if let Some(params) = params {
            if let Some(category) = params.category {
                let category_str = match category {
                    TemplateCategory::Web => "web",
                    TemplateCategory::Mobile => "mobile",
                    TemplateCategory::Desktop => "desktop",
                    TemplateCategory::Api => "api",
                    TemplateCategory::Microservice => "microservice",
                    TemplateCategory::Library => "library",
                    TemplateCategory::Cli => "cli",
                    TemplateCategory::Game => "game",
                    TemplateCategory::AiMl => "ai_ml",
                    TemplateCategory::Blockchain => "blockchain",
                };
                path.push_str(&format!("&category={}", category_str));
            }

            if let Some(min_rating) = params.min_rating {
                path.push_str(&format!("&min_rating={}", min_rating));
            }

            if let Some(page) = params.pagination.page {
                path.push_str(&format!("&page={}", page));
            }

            if let Some(per_page) = params.pagination.per_page {
                path.push_str(&format!("&per_page={}", per_page));
            }
        }

        self.client.get(&path).await
    }

    /// Get popular templates
    pub async fn popular(&self, limit: Option<u32>) -> Result<Vec<Template>> {
        let mut path = "/api/v1/templates/popular".to_string();
        if let Some(limit) = limit {
            path.push_str(&format!("?limit={}", limit));
        }
        self.client.get(&path).await
    }

    /// Get recent templates
    pub async fn recent(&self, limit: Option<u32>) -> Result<Vec<Template>> {
        let mut path = "/api/v1/templates/recent".to_string();
        if let Some(limit) = limit {
            path.push_str(&format!("?limit={}", limit));
        }
        self.client.get(&path).await
    }

    /// Get templates by category
    pub async fn by_category(&self, category: TemplateCategory, params: Option<PaginationParams>) -> Result<PaginatedResponse<Template>> {
        let category_str = match category {
            TemplateCategory::Web => "web",
            TemplateCategory::Mobile => "mobile",
            TemplateCategory::Desktop => "desktop",
            TemplateCategory::Api => "api",
            TemplateCategory::Microservice => "microservice",
            TemplateCategory::Library => "library",
            TemplateCategory::Cli => "cli",
            TemplateCategory::Game => "game",
            TemplateCategory::AiMl => "ai_ml",
            TemplateCategory::Blockchain => "blockchain",
        };

        let mut path = format!("/api/v1/templates/category/{}", category_str);

        if let Some(params) = params {
            let mut query_params = Vec::new();

            if let Some(page) = params.page {
                query_params.push(format!("page={}", page));
            }
            if let Some(per_page) = params.per_page {
                query_params.push(format!("per_page={}", per_page));
            }

            if !query_params.is_empty() {
                path.push('?');
                path.push_str(&query_params.join("&"));
            }
        }

        self.client.get(&path).await
    }

    /// Get template statistics
    pub async fn stats(&self) -> Result<TemplateStats> {
        self.client.get("/api/v1/templates/stats").await
    }

    /// Rate a template
    pub async fn rate(&self, template_id: Uuid, rating: f32) -> Result<()> {
        let path = format!("/api/v1/templates/{}/rate", template_id);
        let request = serde_json::json!({ "rating": rating });
        self.client.post::<serde_json::Value, _>(&path, &request).await?;
        Ok(())
    }

    /// Get template reviews
    pub async fn reviews(&self, template_id: Uuid, params: Option<PaginationParams>) -> Result<PaginatedResponse<TemplateReview>> {
        let mut path = format!("/api/v1/templates/{}/reviews", template_id);

        if let Some(params) = params {
            let mut query_params = Vec::new();

            if let Some(page) = params.page {
                query_params.push(format!("page={}", page));
            }
            if let Some(per_page) = params.per_page {
                query_params.push(format!("per_page={}", per_page));
            }

            if !query_params.is_empty() {
                path.push('?');
                path.push_str(&query_params.join("&"));
            }
        }

        self.client.get(&path).await
    }
}

#[derive(Debug, Clone, Default)]
pub struct TemplateListParams {
    pub category: Option<TemplateCategory>,
    pub tech_stack: Option<String>,
    pub architecture: Option<String>,
    pub tags: Vec<String>,
    pub pagination: PaginationParams,
}

#[derive(Debug, Clone, Default)]
pub struct TemplateSearchParams {
    pub category: Option<TemplateCategory>,
    pub min_rating: Option<f32>,
    pub pagination: PaginationParams,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplateContent {
    pub structure: TemplateStructure,
    pub files: Vec<TemplateFile>,
    pub variables: Vec<TemplateVariable>,
    pub hooks: Option<TemplateHooks>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplateStructure {
    pub directories: Vec<String>,
    pub root_files: Vec<String>,
    pub package_files: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplateFile {
    pub path: String,
    pub content: String,
    pub is_binary: bool,
    pub executable: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: String,
    pub default_value: Option<String>,
    pub required: bool,
    pub variable_type: TemplateVariableType,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TemplateVariableType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "choice")]
    Choice { options: Vec<String> },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplateHooks {
    pub pre_generate: Option<Vec<String>>,
    pub post_generate: Option<Vec<String>>,
    pub pre_install: Option<Vec<String>>,
    pub post_install: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplateGenerateRequest {
    pub template_id: Uuid,
    pub project_name: String,
    pub target_directory: Option<String>,
    pub variables: std::collections::HashMap<String, serde_json::Value>,
    pub options: Option<TemplateGenerateOptions>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplateGenerateOptions {
    pub skip_git_init: Option<bool>,
    pub skip_install: Option<bool>,
    pub skip_hooks: Option<bool>,
    pub overwrite_existing: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplateGenerateResponse {
    pub project_id: Uuid,
    pub generated_files: Vec<String>,
    pub skipped_files: Vec<String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplatePreview {
    pub files_to_create: Vec<String>,
    pub files_to_modify: Vec<String>,
    pub files_to_skip: Vec<String>,
    pub variables_resolved: std::collections::HashMap<String, serde_json::Value>,
    pub estimated_size: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplateStats {
    pub total_templates: u64,
    pub total_downloads: u64,
    pub popular_categories: Vec<CategoryStat>,
    pub popular_tech_stacks: Vec<TechStackStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryStat {
    pub category: TemplateCategory,
    pub count: u64,
    pub percentage: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TechStackStat {
    pub tech_stack: String,
    pub count: u64,
    pub percentage: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplateReview {
    pub id: Uuid,
    pub template_id: Uuid,
    pub user_id: String,
    pub rating: f32,
    pub comment: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub helpful_votes: u32,
}