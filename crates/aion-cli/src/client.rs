// AION-R CLI Client
// HTTP client for interacting with AION-R API

use anyhow::{anyhow, Result};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

use crate::config::CliConfig;

/// AION-R API client
pub struct AionClient {
    client: Client,
    base_url: String,
    config: CliConfig,
}

/// API error response
#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub error: ErrorDetails,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

/// Authentication response
#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

/// Code generation request
#[derive(Debug, Serialize)]
pub struct GenerateCodeRequest {
    pub requirements: String,
    pub language: String,
    pub framework: Option<String>,
    pub architecture: Option<String>,
    pub optimization_level: Option<String>,
    pub constraints: Option<serde_json::Value>,
    pub context: Option<serde_json::Value>,
}

/// Code generation response
#[derive(Debug, Deserialize)]
pub struct GenerateCodeResponse {
    pub id: String,
    pub status: String,
    pub generated_files_count: usize,
    pub total_lines_of_code: usize,
    pub documentation_url: String,
    pub download_url: String,
    pub preview: CodePreview,
    pub suggestions: Vec<String>,
    pub estimated_time_saved_hours: f32,
}

#[derive(Debug, Deserialize)]
pub struct CodePreview {
    pub main_file: String,
    pub structure: Vec<FileInfo>,
}

#[derive(Debug, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub language: String,
    pub size_bytes: usize,
    pub purpose: String,
}

/// Requirements analysis response
#[derive(Debug, Deserialize)]
pub struct AnalyzeRequirementsResponse {
    pub id: String,
    pub confidence_score: f32,
    pub user_stories_count: usize,
    pub risks_identified: usize,
    pub implementation_phases: usize,
    pub optimization_suggestions: usize,
    pub technical_summary: String,
}

/// Text analysis response
#[derive(Debug, Deserialize)]
pub struct TextAnalysisResponse {
    pub sentiment: Option<SentimentAnalysis>,
    pub entities: Option<Vec<EntityAnalysis>>,
    pub language: Option<LanguageDetection>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Deserialize)]
pub struct SentimentAnalysis {
    pub label: String,
    pub confidence: f32,
    pub scores: HashMap<String, f32>,
}

#[derive(Debug, Deserialize)]
pub struct EntityAnalysis {
    pub text: String,
    pub label: String,
    pub confidence: f32,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Deserialize)]
pub struct LanguageDetection {
    pub detected: String,
    pub confidence: f32,
}

impl AionClient {
    /// Create a new AION-R client
    pub fn new(base_url: String, config: CliConfig) -> Result<Self> {
        let client = Client::builder()
            .user_agent("aion-cli/1.0.0")
            .timeout(std::time::Duration::from_secs(300))
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            config,
        })
    }

    /// Check if user is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.config.get_auth_token().is_some()
    }

    /// Login to AION-R platform
    pub async fn login(&self, email: &str, password: &str, mfa_code: Option<&str>) -> Result<AuthResponse> {
        let mut body = serde_json::json!({
            "email": email,
            "password": password
        });

        if let Some(code) = mfa_code {
            body["mfa_code"] = serde_json::Value::String(code.to_string());
        }

        let response = self.client
            .post(&format!("{}/api/v1/auth/login", self.base_url))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Logout from platform
    pub async fn logout(&self) -> Result<()> {
        // Clear local authentication
        // In a real implementation, you might also call a logout endpoint
        Ok(())
    }

    /// Register new user
    pub async fn register(
        &self,
        email: &str,
        first_name: &str,
        last_name: &str,
        company: Option<&str>,
        password: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::json!({
            "email": email,
            "first_name": first_name,
            "last_name": last_name,
            "company": company,
            "password": password
        });

        let response = self.client
            .post(&format!("{}/api/v1/auth/register", self.base_url))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Generate code from requirements
    pub async fn generate_code(&self, request: GenerateCodeRequest) -> Result<GenerateCodeResponse> {
        let response = self.client
            .post(&format!("{}/api/v1/code/generate", self.base_url))
            .bearer_auth(self.get_auth_token()?)
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get generation status
    pub async fn get_generation_status(&self, id: &str) -> Result<serde_json::Value> {
        let response = self.client
            .get(&format!("{}/api/v1/code/status/{}", self.base_url, id))
            .bearer_auth(self.get_auth_token()?)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// List user's generations
    pub async fn list_generations(&self, page: u32, per_page: u32) -> Result<serde_json::Value> {
        let response = self.client
            .get(&format!("{}/api/v1/code/list", self.base_url))
            .bearer_auth(self.get_auth_token()?)
            .query(&[("page", page), ("per_page", per_page)])
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Download generated code
    pub async fn download_generated_code(&self, id: &str) -> Result<Vec<u8>> {
        let response = self.client
            .get(&format!("{}/api/v1/code/download/{}", self.base_url, id))
            .bearer_auth(self.get_auth_token()?)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.bytes().await?.to_vec())
        } else {
            let error: ApiError = response.json().await?;
            Err(anyhow!("API Error: {}", error.error.message))
        }
    }

    /// Delete generation
    pub async fn delete_generation(&self, id: &str) -> Result<()> {
        let response = self.client
            .delete(&format!("{}/api/v1/code/{}", self.base_url, id))
            .bearer_auth(self.get_auth_token()?)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error: ApiError = response.json().await?;
            Err(anyhow!("API Error: {}", error.error.message))
        }
    }

    /// Analyze requirements
    pub async fn analyze_requirements(&self, requirements: &str) -> Result<AnalyzeRequirementsResponse> {
        let body = serde_json::json!({
            "requirements": requirements
        });

        let response = self.client
            .post(&format!("{}/api/v1/requirements/analyze", self.base_url))
            .bearer_auth(self.get_auth_token()?)
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Analyze text
    pub async fn analyze_text(&self, text: &str, analysis_types: Vec<String>) -> Result<TextAnalysisResponse> {
        let body = serde_json::json!({
            "text": text,
            "analysis_types": analysis_types,
            "options": {
                "include_confidence": true
            }
        });

        let response = self.client
            .post(&format!("{}/api/v1/ai/text/analyze", self.base_url))
            .bearer_auth(self.get_auth_token()?)
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Generate text
    pub async fn generate_text(&self, prompt: &str, max_tokens: u32, temperature: f32) -> Result<serde_json::Value> {
        let body = serde_json::json!({
            "prompt": prompt,
            "max_tokens": max_tokens,
            "temperature": temperature
        });

        let response = self.client
            .post(&format!("{}/api/v1/ai/text/generate", self.base_url))
            .bearer_auth(self.get_auth_token()?)
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Analyze image
    pub async fn analyze_image(&self, image_path: &Path, analysis_types: Vec<String>) -> Result<serde_json::Value> {
        let image_data = fs::read(image_path).await?;
        let form = reqwest::multipart::Form::new()
            .part("image", reqwest::multipart::Part::bytes(image_data)
                .file_name(image_path.file_name().unwrap().to_string_lossy().to_string()))
            .text("analysis_types", analysis_types.join(","));

        let response = self.client
            .post(&format!("{}/api/v1/ai/vision/analyze", self.base_url))
            .bearer_auth(self.get_auth_token()?)
            .multipart(form)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Transcribe audio
    pub async fn transcribe_audio(&self, audio_path: &Path, language: Option<&str>) -> Result<serde_json::Value> {
        let audio_data = fs::read(audio_path).await?;
        let mut form = reqwest::multipart::Form::new()
            .part("audio", reqwest::multipart::Part::bytes(audio_data)
                .file_name(audio_path.file_name().unwrap().to_string_lossy().to_string()));

        if let Some(lang) = language {
            form = form.text("language", lang.to_string());
        }

        let response = self.client
            .post(&format!("{}/api/v1/ai/audio/transcribe", self.base_url))
            .bearer_auth(self.get_auth_token()?)
            .multipart(form)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get user profile
    pub async fn get_profile(&self) -> Result<serde_json::Value> {
        let response = self.client
            .get(&format!("{}/api/v1/users/profile", self.base_url))
            .bearer_auth(self.get_auth_token()?)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get usage statistics
    pub async fn get_usage(&self) -> Result<serde_json::Value> {
        let response = self.client
            .get(&format!("{}/api/v1/usage", self.base_url))
            .bearer_auth(self.get_auth_token()?)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get platform health
    pub async fn get_health(&self) -> Result<serde_json::Value> {
        let response = self.client
            .get(&format!("{}/health", self.base_url))
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get available models
    pub async fn get_models(&self) -> Result<serde_json::Value> {
        let response = self.client
            .get(&format!("{}/api/v1/models", self.base_url))
            .bearer_auth(self.get_auth_token()?)
            .send()
            .await?;

        self.handle_response(response).await
    }

    // Helper methods

    fn get_auth_token(&self) -> Result<String> {
        self.config.get_auth_token()
            .ok_or_else(|| anyhow!("Not authenticated. Please run 'aion auth login' first."))
    }

    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            match response.json::<ApiError>().await {
                Ok(error) => Err(anyhow!("API Error ({}): {}", status, error.error.message)),
                Err(_) => Err(anyhow!("HTTP Error: {}", status)),
            }
        }
    }
}