use crate::{error::*, types::*, projects::*, templates::*, qa::*, progress::*, websocket::*};
use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT}};
use serde::de::DeserializeOwned;
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone)]
pub struct AionClient {
    client: Client,
    base_url: Url,
    api_key: String,
}

impl AionClient {
    /// Create a new AION API client
    pub fn new(base_url: &str, api_key: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))
                .map_err(|_| AionError::Auth("Invalid API key format".to_string()))?
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json")
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("aion-rust-sdk/0.1.0")
        );

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .default_headers(headers)
            .build()?;

        let base_url = Url::parse(base_url)?;

        Ok(Self {
            client,
            base_url,
            api_key: api_key.to_string(),
        })
    }

    /// Create a new client with custom configuration
    pub fn with_config(base_url: &str, api_key: &str, timeout: Duration) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))
                .map_err(|_| AionError::Auth("Invalid API key format".to_string()))?
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json")
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("aion-rust-sdk/0.1.0")
        );

        let client = Client::builder()
            .timeout(timeout)
            .default_headers(headers)
            .build()?;

        let base_url = Url::parse(base_url)?;

        Ok(Self {
            client,
            base_url,
            api_key: api_key.to_string(),
        })
    }

    /// Get projects API interface
    pub fn projects(&self) -> ProjectsApi {
        ProjectsApi::new(self.clone())
    }

    /// Get templates API interface
    pub fn templates(&self) -> TemplatesApi {
        TemplatesApi::new(self.clone())
    }

    /// Get QA API interface
    pub fn qa(&self) -> QaApi {
        QaApi::new(self.clone())
    }

    /// Get progress tracking API interface
    pub fn progress(&self) -> ProgressApi {
        ProgressApi::new(self.clone())
    }

    /// Create a WebSocket connection for real-time updates
    pub async fn websocket(&self) -> Result<WebSocketClient> {
        WebSocketClient::new(&self.base_url, &self.api_key).await
    }

    /// Internal method to make HTTP GET requests
    pub(crate) async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.base_url.join(path)?;
        let response = self.client.get(url).send().await?;
        self.handle_response(response).await
    }

    /// Internal method to make HTTP POST requests
    pub(crate) async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        let url = self.base_url.join(path)?;
        let response = self.client.post(url).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Internal method to make HTTP PUT requests
    pub(crate) async fn put<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        let url = self.base_url.join(path)?;
        let response = self.client.put(url).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Internal method to make HTTP DELETE requests
    pub(crate) async fn delete<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.base_url.join(path)?;
        let response = self.client.delete(url).send().await?;
        self.handle_response(response).await
    }

    /// Internal method to handle HTTP responses
    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            let api_response: ApiResponse<T> = response.json().await?;
            if api_response.success {
                Ok(api_response.data)
            } else {
                Err(AionError::Api {
                    status: status.as_u16(),
                    message: api_response.message.unwrap_or_else(|| "Unknown error".to_string()),
                })
            }
        } else {
            let error_text = response.text().await?;
            match serde_json::from_str::<ApiError>(&error_text) {
                Ok(api_error) => {
                    match status.as_u16() {
                        401 => Err(AionError::Auth(api_error.message)),
                        404 => Err(AionError::NotFound {
                            resource: api_error.message
                        }),
                        429 => {
                            let retry_after = None; // Could parse from headers
                            Err(AionError::RateLimit { retry_after })
                        },
                        400 => Err(AionError::Validation(api_error.message)),
                        _ => Err(AionError::Api {
                            status: status.as_u16(),
                            message: api_error.message,
                        }),
                    }
                },
                Err(_) => Err(AionError::Api {
                    status: status.as_u16(),
                    message: error_text,
                }),
            }
        }
    }

    /// Test the API connection
    pub async fn health_check(&self) -> Result<bool> {
        match self.get::<serde_json::Value>("/health").await {
            Ok(_) => Ok(true),
            Err(AionError::NotFound { .. }) => Ok(true), // Health endpoint might not exist
            Err(e) => Err(e),
        }
    }

    /// Get API information
    pub async fn info(&self) -> Result<serde_json::Value> {
        self.get("/info").await
    }

    /// Get current user information
    pub async fn user_info(&self) -> Result<serde_json::Value> {
        self.get("/user").await
    }

    /// Get API usage statistics
    pub async fn usage_stats(&self) -> Result<serde_json::Value> {
        self.get("/usage").await
    }
}