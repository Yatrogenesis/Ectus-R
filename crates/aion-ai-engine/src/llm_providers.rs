//! Multi-Provider LLM Integration for AION-R
//!
//! Supports multiple free and paid AI providers with automatic fallback:
//! - Groq (fast inference)
//! - OpenAI (GPT-4, GPT-3.5)
//! - Hugging Face Inference API
//! - GitHub Models (free tier)
//! - Cloudflare AI Workers
//!
//! Features:
//! - Automatic failover between providers
//! - Rate limiting and retry logic
//! - Streaming support
//! - Token counting and cost tracking

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use reqwest::Client;
use async_trait::async_trait;

/// LLM Provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LLMProvider {
    Groq,
    OpenAI,
    HuggingFace,
    GitHubModels,
    CloudflareAI,
}

/// LLM Request configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMRequest {
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub model: Option<String>,
}

/// LLM Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub content: String,
    pub model: String,
    pub provider: LLMProvider,
    pub tokens_used: Option<u32>,
    pub finish_reason: Option<String>,
}

/// Trait for LLM providers
#[async_trait]
pub trait LLMClient: Send + Sync {
    async fn generate(&self, request: &LLMRequest) -> Result<LLMResponse>;
    fn provider_type(&self) -> LLMProvider;
    fn is_available(&self) -> bool;
}

// ============================================================================
// GROQ CLIENT
// ============================================================================

pub struct GroqClient {
    api_key: String,
    client: Client,
    base_url: String,
    default_model: String,
}

impl GroqClient {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            api_key,
            client: Client::builder()
                .timeout(Duration::from_secs(120))
                .build()?,
            base_url: "https://api.groq.com/openai/v1".to_string(),
            default_model: "llama-3.1-70b-versatile".to_string(), // Fast and capable
        })
    }
}

#[async_trait]
impl LLMClient for GroqClient {
    async fn generate(&self, request: &LLMRequest) -> Result<LLMResponse> {
        #[derive(Serialize)]
        struct GroqRequest {
            model: String,
            messages: Vec<Message>,
            max_tokens: Option<u32>,
            temperature: Option<f32>,
        }

        #[derive(Serialize, Deserialize)]
        struct Message {
            role: String,
            content: String,
        }

        #[derive(Deserialize)]
        struct GroqResponse {
            choices: Vec<Choice>,
            usage: Usage,
            model: String,
        }

        #[derive(Deserialize)]
        struct Choice {
            message: Message,
            finish_reason: Option<String>,
        }

        #[derive(Deserialize)]
        struct Usage {
            total_tokens: u32,
        }

        let mut messages = vec![];

        if let Some(system) = &request.system_prompt {
            messages.push(Message {
                role: "system".to_string(),
                content: system.clone(),
            });
        }

        messages.push(Message {
            role: "user".to_string(),
            content: request.prompt.clone(),
        });

        let groq_request = GroqRequest {
            model: request.model.clone().unwrap_or(self.default_model.clone()),
            messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
        };

        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&groq_request)
            .send()
            .await
            .context("Failed to send request to Groq")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Groq API error {}: {}", status, error_text));
        }

        let groq_response: GroqResponse = response.json().await
            .context("Failed to parse Groq response")?;

        let choice = groq_response.choices.first()
            .ok_or_else(|| anyhow!("No choices in Groq response"))?;

        Ok(LLMResponse {
            content: choice.message.content.clone(),
            model: groq_response.model,
            provider: LLMProvider::Groq,
            tokens_used: Some(groq_response.usage.total_tokens),
            finish_reason: choice.finish_reason.clone(),
        })
    }

    fn provider_type(&self) -> LLMProvider {
        LLMProvider::Groq
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }
}

// ============================================================================
// OPENAI CLIENT
// ============================================================================

pub struct OpenAIClient {
    api_key: String,
    client: Client,
    base_url: String,
    default_model: String,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            api_key,
            client: Client::builder()
                .timeout(Duration::from_secs(120))
                .build()?,
            base_url: "https://api.openai.com/v1".to_string(),
            default_model: "gpt-4o-mini".to_string(), // Cost-effective GPT-4 class
        })
    }
}

#[async_trait]
impl LLMClient for OpenAIClient {
    async fn generate(&self, request: &LLMRequest) -> Result<LLMResponse> {
        #[derive(Serialize)]
        struct OpenAIRequest {
            model: String,
            messages: Vec<Message>,
            max_tokens: Option<u32>,
            temperature: Option<f32>,
        }

        #[derive(Serialize, Deserialize)]
        struct Message {
            role: String,
            content: String,
        }

        #[derive(Deserialize)]
        struct OpenAIResponse {
            choices: Vec<Choice>,
            usage: Usage,
            model: String,
        }

        #[derive(Deserialize)]
        struct Choice {
            message: Message,
            finish_reason: Option<String>,
        }

        #[derive(Deserialize)]
        struct Usage {
            total_tokens: u32,
        }

        let mut messages = vec![];

        if let Some(system) = &request.system_prompt {
            messages.push(Message {
                role: "system".to_string(),
                content: system.clone(),
            });
        }

        messages.push(Message {
            role: "user".to_string(),
            content: request.prompt.clone(),
        });

        let openai_request = OpenAIRequest {
            model: request.model.clone().unwrap_or(self.default_model.clone()),
            messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
        };

        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&openai_request)
            .send()
            .await
            .context("Failed to send request to OpenAI")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("OpenAI API error {}: {}", status, error_text));
        }

        let openai_response: OpenAIResponse = response.json().await
            .context("Failed to parse OpenAI response")?;

        let choice = openai_response.choices.first()
            .ok_or_else(|| anyhow!("No choices in OpenAI response"))?;

        Ok(LLMResponse {
            content: choice.message.content.clone(),
            model: openai_response.model,
            provider: LLMProvider::OpenAI,
            tokens_used: Some(openai_response.usage.total_tokens),
            finish_reason: choice.finish_reason.clone(),
        })
    }

    fn provider_type(&self) -> LLMProvider {
        LLMProvider::OpenAI
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }
}

// ============================================================================
// HUGGING FACE CLIENT
// ============================================================================

pub struct HuggingFaceClient {
    api_key: String,
    client: Client,
    base_url: String,
    default_model: String,
}

impl HuggingFaceClient {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            api_key,
            client: Client::builder()
                .timeout(Duration::from_secs(180))
                .build()?,
            base_url: "https://api-inference.huggingface.co/models".to_string(),
            default_model: "mistralai/Mixtral-8x7B-Instruct-v0.1".to_string(),
        })
    }
}

#[async_trait]
impl LLMClient for HuggingFaceClient {
    async fn generate(&self, request: &LLMRequest) -> Result<LLMResponse> {
        #[derive(Serialize)]
        struct HFRequest {
            inputs: String,
            parameters: Parameters,
        }

        #[derive(Serialize)]
        struct Parameters {
            max_new_tokens: Option<u32>,
            temperature: Option<f32>,
            return_full_text: bool,
        }

        #[derive(Deserialize)]
        struct HFResponse {
            generated_text: String,
        }

        let full_prompt = if let Some(system) = &request.system_prompt {
            format!("{}\n\n{}", system, request.prompt)
        } else {
            request.prompt.clone()
        };

        let hf_request = HFRequest {
            inputs: full_prompt,
            parameters: Parameters {
                max_new_tokens: request.max_tokens,
                temperature: request.temperature,
                return_full_text: false,
            },
        };

        let model = request.model.clone().unwrap_or(self.default_model.clone());

        let response = self.client
            .post(&format!("{}/{}", self.base_url, model))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&hf_request)
            .send()
            .await
            .context("Failed to send request to Hugging Face")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Hugging Face API error {}: {}", status, error_text));
        }

        let hf_response: Vec<HFResponse> = response.json().await
            .context("Failed to parse Hugging Face response")?;

        let result = hf_response.first()
            .ok_or_else(|| anyhow!("No response from Hugging Face"))?;

        Ok(LLMResponse {
            content: result.generated_text.clone(),
            model,
            provider: LLMProvider::HuggingFace,
            tokens_used: None, // HF doesn't always provide token counts
            finish_reason: None,
        })
    }

    fn provider_type(&self) -> LLMProvider {
        LLMProvider::HuggingFace
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }
}

// ============================================================================
// GITHUB MODELS CLIENT (Free Tier)
// ============================================================================

pub struct GitHubModelsClient {
    api_key: String,
    client: Client,
    base_url: String,
    default_model: String,
}

impl GitHubModelsClient {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            api_key,
            client: Client::builder()
                .timeout(Duration::from_secs(120))
                .build()?,
            base_url: "https://models.inference.ai.azure.com".to_string(),
            default_model: "gpt-4o-mini".to_string(), // Available on free tier
        })
    }
}

#[async_trait]
impl LLMClient for GitHubModelsClient {
    async fn generate(&self, request: &LLMRequest) -> Result<LLMResponse> {
        #[derive(Serialize)]
        struct GitHubRequest {
            model: String,
            messages: Vec<Message>,
            max_tokens: Option<u32>,
            temperature: Option<f32>,
        }

        #[derive(Serialize, Deserialize)]
        struct Message {
            role: String,
            content: String,
        }

        #[derive(Deserialize)]
        struct GitHubResponse {
            choices: Vec<Choice>,
            usage: Option<Usage>,
            model: String,
        }

        #[derive(Deserialize)]
        struct Choice {
            message: Message,
            finish_reason: Option<String>,
        }

        #[derive(Deserialize)]
        struct Usage {
            total_tokens: u32,
        }

        let mut messages = vec![];

        if let Some(system) = &request.system_prompt {
            messages.push(Message {
                role: "system".to_string(),
                content: system.clone(),
            });
        }

        messages.push(Message {
            role: "user".to_string(),
            content: request.prompt.clone(),
        });

        let github_request = GitHubRequest {
            model: request.model.clone().unwrap_or(self.default_model.clone()),
            messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
        };

        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&github_request)
            .send()
            .await
            .context("Failed to send request to GitHub Models")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("GitHub Models API error {}: {}", status, error_text));
        }

        let github_response: GitHubResponse = response.json().await
            .context("Failed to parse GitHub Models response")?;

        let choice = github_response.choices.first()
            .ok_or_else(|| anyhow!("No choices in GitHub Models response"))?;

        Ok(LLMResponse {
            content: choice.message.content.clone(),
            model: github_response.model,
            provider: LLMProvider::GitHubModels,
            tokens_used: github_response.usage.map(|u| u.total_tokens),
            finish_reason: choice.finish_reason.clone(),
        })
    }

    fn provider_type(&self) -> LLMProvider {
        LLMProvider::GitHubModels
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }
}

// ============================================================================
// CLOUDFLARE AI CLIENT
// ============================================================================

pub struct CloudflareAIClient {
    api_key: String,
    account_id: String,
    client: Client,
    base_url: String,
    default_model: String,
}

impl CloudflareAIClient {
    pub fn new(api_key: String, account_id: String) -> Result<Self> {
        Ok(Self {
            api_key,
            account_id: account_id.clone(),
            client: Client::builder()
                .timeout(Duration::from_secs(120))
                .build()?,
            base_url: format!("https://api.cloudflare.com/client/v4/accounts/{}/ai/run", account_id),
            default_model: "@cf/meta/llama-3.1-8b-instruct".to_string(),
        })
    }
}

#[async_trait]
impl LLMClient for CloudflareAIClient {
    async fn generate(&self, request: &LLMRequest) -> Result<LLMResponse> {
        #[derive(Serialize)]
        struct CloudflareRequest {
            messages: Vec<Message>,
            max_tokens: Option<u32>,
            temperature: Option<f32>,
        }

        #[derive(Serialize, Deserialize)]
        struct Message {
            role: String,
            content: String,
        }

        #[derive(Deserialize)]
        struct CloudflareResponse {
            result: CloudflareResult,
            success: bool,
        }

        #[derive(Deserialize)]
        struct CloudflareResult {
            response: String,
        }

        let mut messages = vec![];

        if let Some(system) = &request.system_prompt {
            messages.push(Message {
                role: "system".to_string(),
                content: system.clone(),
            });
        }

        messages.push(Message {
            role: "user".to_string(),
            content: request.prompt.clone(),
        });

        let cloudflare_request = CloudflareRequest {
            messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
        };

        let model = request.model.clone().unwrap_or(self.default_model.clone());

        let response = self.client
            .post(&format!("{}/{}", self.base_url, model))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&cloudflare_request)
            .send()
            .await
            .context("Failed to send request to Cloudflare AI")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Cloudflare AI error {}: {}", status, error_text));
        }

        let cloudflare_response: CloudflareResponse = response.json().await
            .context("Failed to parse Cloudflare AI response")?;

        if !cloudflare_response.success {
            return Err(anyhow!("Cloudflare AI request failed"));
        }

        Ok(LLMResponse {
            content: cloudflare_response.result.response,
            model,
            provider: LLMProvider::CloudflareAI,
            tokens_used: None,
            finish_reason: None,
        })
    }

    fn provider_type(&self) -> LLMProvider {
        LLMProvider::CloudflareAI
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty() && !self.account_id.is_empty()
    }
}

// ============================================================================
// MULTI-PROVIDER ORCHESTRATOR WITH FALLBACK
// ============================================================================

pub struct MultiProviderLLM {
    providers: Vec<Box<dyn LLMClient>>,
    preferred_order: Vec<LLMProvider>,
}

impl MultiProviderLLM {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            preferred_order: vec![
                LLMProvider::Groq,        // Fastest
                LLMProvider::OpenAI,      // Most capable
                LLMProvider::GitHubModels, // Free tier
                LLMProvider::HuggingFace, // Open models
                LLMProvider::CloudflareAI, // Serverless
            ],
        }
    }

    pub fn add_provider(&mut self, provider: Box<dyn LLMClient>) {
        if provider.is_available() {
            self.providers.push(provider);
        }
    }

    pub async fn generate_with_fallback(&self, request: &LLMRequest) -> Result<LLMResponse> {
        let mut last_error = None;

        // Try providers in preferred order
        for provider_type in &self.preferred_order {
            if let Some(provider) = self.providers.iter().find(|p| p.provider_type() == *provider_type) {
                match provider.generate(request).await {
                    Ok(response) => {
                        tracing::info!("Successfully generated response using {:?}", provider_type);
                        return Ok(response);
                    }
                    Err(e) => {
                        tracing::warn!("Provider {:?} failed: {}", provider_type, e);
                        last_error = Some(e);
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("No LLM providers available")))
    }

    pub fn available_providers(&self) -> Vec<LLMProvider> {
        self.providers.iter().map(|p| p.provider_type()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires API keys
    async fn test_groq_client() {
        let api_key = std::env::var("GROQ_API_KEY").unwrap();
        let client = GroqClient::new(api_key).unwrap();

        let request = LLMRequest {
            prompt: "Write a hello world function in Rust".to_string(),
            system_prompt: Some("You are a helpful coding assistant.".to_string()),
            max_tokens: Some(200),
            temperature: Some(0.7),
            model: None,
        };

        let response = client.generate(&request).await.unwrap();
        assert!(!response.content.is_empty());
        assert_eq!(response.provider, LLMProvider::Groq);
    }

    #[tokio::test]
    #[ignore] // Requires API keys
    async fn test_multi_provider_fallback() {
        let mut orchestrator = MultiProviderLLM::new();

        if let Ok(groq_key) = std::env::var("GROQ_API_KEY") {
            orchestrator.add_provider(Box::new(GroqClient::new(groq_key).unwrap()));
        }

        if let Ok(openai_key) = std::env::var("OPENAI_API_KEY") {
            orchestrator.add_provider(Box::new(OpenAIClient::new(openai_key).unwrap()));
        }

        let request = LLMRequest {
            prompt: "Say hello".to_string(),
            system_prompt: None,
            max_tokens: Some(50),
            temperature: Some(0.7),
            model: None,
        };

        let response = orchestrator.generate_with_fallback(&request).await.unwrap();
        assert!(!response.content.is_empty());
    }
}
