use crate::{types::*, error::Result, client::AionClient};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ProjectsApi {
    client: AionClient,
}

impl ProjectsApi {
    pub(crate) fn new(client: AionClient) -> Self {
        Self { client }
    }

    /// List all projects with optional pagination
    pub async fn list(&self, params: Option<PaginationParams>) -> Result<PaginatedResponse<Project>> {
        let mut path = "/api/v1/projects".to_string();

        if let Some(params) = params {
            let mut query_params = Vec::new();

            if let Some(page) = params.page {
                query_params.push(format!("page={}", page));
            }
            if let Some(per_page) = params.per_page {
                query_params.push(format!("per_page={}", per_page));
            }
            if let Some(sort_by) = params.sort_by {
                query_params.push(format!("sort_by={}", sort_by));
            }
            if let Some(sort_order) = params.sort_order {
                let order = match sort_order {
                    SortOrder::Asc => "asc",
                    SortOrder::Desc => "desc",
                };
                query_params.push(format!("sort_order={}", order));
            }

            if !query_params.is_empty() {
                path.push('?');
                path.push_str(&query_params.join("&"));
            }
        }

        self.client.get(&path).await
    }

    /// Get a specific project by ID
    pub async fn get(&self, project_id: Uuid) -> Result<Project> {
        let path = format!("/api/v1/projects/{}", project_id);
        self.client.get(&path).await
    }

    /// Create a new project
    pub async fn create(&self, request: ProjectRequest) -> Result<Project> {
        self.client.post("/api/v1/projects", &request).await
    }

    /// Update an existing project
    pub async fn update(&self, project_id: Uuid, request: ProjectRequest) -> Result<Project> {
        let path = format!("/api/v1/projects/{}", project_id);
        self.client.put(&path, &request).await
    }

    /// Delete a project
    pub async fn delete(&self, project_id: Uuid) -> Result<()> {
        let path = format!("/api/v1/projects/{}", project_id);
        self.client.delete::<serde_json::Value>(&path).await?;
        Ok(())
    }

    /// Generate project from template
    pub async fn from_template(&self, request: TemplateRequest) -> Result<Project> {
        self.client.post("/api/v1/projects/from-template", &request).await
    }

    /// Get project status
    pub async fn status(&self, project_id: Uuid) -> Result<ProjectStatus> {
        let path = format!("/api/v1/projects/{}/status", project_id);
        self.client.get(&path).await
    }

    /// Start project execution
    pub async fn start(&self, project_id: Uuid) -> Result<()> {
        let path = format!("/api/v1/projects/{}/start", project_id);
        self.client.post::<serde_json::Value, _>(&path, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Stop project execution
    pub async fn stop(&self, project_id: Uuid) -> Result<()> {
        let path = format!("/api/v1/projects/{}/stop", project_id);
        self.client.post::<serde_json::Value, _>(&path, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Pause project execution
    pub async fn pause(&self, project_id: Uuid) -> Result<()> {
        let path = format!("/api/v1/projects/{}/pause", project_id);
        self.client.post::<serde_json::Value, _>(&path, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Resume project execution
    pub async fn resume(&self, project_id: Uuid) -> Result<()> {
        let path = format!("/api/v1/projects/{}/resume", project_id);
        self.client.post::<serde_json::Value, _>(&path, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Get project logs
    pub async fn logs(&self, project_id: Uuid, limit: Option<u32>) -> Result<Vec<serde_json::Value>> {
        let mut path = format!("/api/v1/projects/{}/logs", project_id);
        if let Some(limit) = limit {
            path.push_str(&format!("?limit={}", limit));
        }
        self.client.get(&path).await
    }

    /// Get project metrics
    pub async fn metrics(&self, project_id: Uuid) -> Result<serde_json::Value> {
        let path = format!("/api/v1/projects/{}/metrics", project_id);
        self.client.get(&path).await
    }

    /// Download project files
    pub async fn download(&self, project_id: Uuid) -> Result<bytes::Bytes> {
        let path = format!("/api/v1/projects/{}/download", project_id);
        let url = self.client.base_url.join(&path)?;
        let response = self.client.client.get(url).send().await?;

        if response.status().is_success() {
            Ok(response.bytes().await?)
        } else {
            Err(crate::error::AionError::Api {
                status: response.status().as_u16(),
                message: "Failed to download project".to_string(),
            })
        }
    }

    /// Upload project files
    pub async fn upload(&self, project_id: Uuid, file_data: bytes::Bytes, filename: &str) -> Result<()> {
        let path = format!("/api/v1/projects/{}/upload", project_id);
        let url = self.client.base_url.join(&path)?;

        let form = reqwest::multipart::Form::new()
            .part("file", reqwest::multipart::Part::bytes(file_data.to_vec())
                .file_name(filename.to_string())
                .mime_str("application/octet-stream")?);

        let response = self.client.client.post(url).multipart(form).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::error::AionError::Api {
                status: response.status().as_u16(),
                message: "Failed to upload file".to_string(),
            })
        }
    }

    /// Search projects
    pub async fn search(&self, query: &str, params: Option<PaginationParams>) -> Result<PaginatedResponse<Project>> {
        let mut path = format!("/api/v1/projects/search?q={}", urlencoding::encode(query));

        if let Some(params) = params {
            if let Some(page) = params.page {
                path.push_str(&format!("&page={}", page));
            }
            if let Some(per_page) = params.per_page {
                path.push_str(&format!("&per_page={}", per_page));
            }
        }

        self.client.get(&path).await
    }

    /// Get project statistics
    pub async fn stats(&self) -> Result<serde_json::Value> {
        self.client.get("/api/v1/projects/stats").await
    }
}