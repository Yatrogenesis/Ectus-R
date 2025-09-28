//! # Model Manager
//!
//! Manages AI model loading, caching, and lifecycle.

use crate::errors::{AIEngineError, AIResult};
use anyhow::{Context, Result};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Model information and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Unique model identifier
    pub id: String,
    /// Human-readable model name
    pub name: String,
    /// Model version
    pub version: String,
    /// Model description
    pub description: String,
    /// Model type (text, image, audio, multimodal)
    pub model_type: ModelType,
    /// Supported tasks
    pub tasks: Vec<String>,
    /// Model size in bytes
    pub size_bytes: u64,
    /// Memory requirements in bytes
    pub memory_requirements: u64,
    /// Local file path (if downloaded)
    pub local_path: Option<PathBuf>,
    /// Remote URL for downloading
    pub remote_url: Option<String>,
    /// Model format (ONNX, PyTorch, TensorFlow, etc.)
    pub format: ModelFormat,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

/// Model types supported by the engine
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModelType {
    /// Text processing models (LLMs, embeddings, etc.)
    Text,
    /// Image processing models (classification, detection, etc.)
    Image,
    /// Audio processing models (ASR, TTS, etc.)
    Audio,
    /// Multimodal models (vision-language, etc.)
    Multimodal,
    /// Traditional ML models
    Traditional,
}

/// Supported model formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModelFormat {
    /// ONNX format
    ONNX,
    /// PyTorch format (.pt, .pth)
    PyTorch,
    /// TensorFlow SavedModel
    TensorFlow,
    /// Hugging Face format
    HuggingFace,
    /// Candle format
    Candle,
    /// Custom format
    Custom(String),
}

/// Model state in the manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelState {
    /// Model is not loaded
    NotLoaded,
    /// Model is currently being loaded
    Loading,
    /// Model is loaded and ready for inference
    Loaded,
    /// Model failed to load
    Failed { error: String },
    /// Model is being unloaded
    Unloading,
}

/// Loaded model instance
#[derive(Debug)]
pub struct LoadedModel {
    /// Model information
    pub info: ModelInfo,
    /// Model state
    pub state: ModelState,
    /// Last access time
    pub last_accessed: std::time::Instant,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Reference count
    pub ref_count: u32,
    /// Model-specific data (opaque to manager)
    pub model_data: Option<Arc<dyn Send + Sync>>,
}

/// Model manager for loading and caching models
pub struct ModelManager {
    /// Cache directory for downloaded models
    cache_dir: PathBuf,
    /// Maximum memory usage for cached models
    max_memory: u64,
    /// Currently loaded models
    loaded_models: Arc<DashMap<String, Arc<RwLock<LoadedModel>>>>,
    /// Available models catalog
    model_catalog: Arc<RwLock<std::collections::HashMap<String, ModelInfo>>>,
    /// Current memory usage
    current_memory_usage: Arc<std::sync::atomic::AtomicU64>,
    /// Download client
    http_client: reqwest::Client,
}

impl ModelManager {
    /// Create a new model manager
    pub async fn new(cache_dir: PathBuf, max_memory: usize) -> Result<Self> {
        // Create cache directory if it doesn't exist
        tokio::fs::create_dir_all(&cache_dir).await?;

        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300)) // 5 minute timeout for downloads
            .build()?;

        let manager = Self {
            cache_dir,
            max_memory: max_memory as u64,
            loaded_models: Arc::new(DashMap::new()),
            model_catalog: Arc::new(RwLock::new(std::collections::HashMap::new())),
            current_memory_usage: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            http_client,
        };

        // Load model catalog
        manager.load_model_catalog().await?;

        info!("Model manager initialized with cache dir: {:?}", cache_dir);
        Ok(manager)
    }

    /// Load model catalog from local or remote source
    async fn load_model_catalog(&self) -> Result<()> {
        // Try to load from local catalog file first
        let catalog_path = self.cache_dir.join("model_catalog.json");

        if catalog_path.exists() {
            match self.load_local_catalog(&catalog_path).await {
                Ok(_) => {
                    info!("Loaded model catalog from local file");
                    return Ok(());
                }
                Err(e) => {
                    warn!("Failed to load local catalog: {}, will create default", e);
                }
            }
        }

        // Create default catalog with some popular models
        self.create_default_catalog().await;

        // Save the default catalog
        self.save_catalog(&catalog_path).await?;

        Ok(())
    }

    /// Load catalog from local file
    async fn load_local_catalog(&self, path: &PathBuf) -> Result<()> {
        let content = tokio::fs::read_to_string(path).await?;
        let catalog: std::collections::HashMap<String, ModelInfo> = serde_json::from_str(&content)?;

        let mut catalog_guard = self.model_catalog.write().await;
        *catalog_guard = catalog;

        Ok(())
    }

    /// Create default model catalog
    async fn create_default_catalog(&self) {
        let mut catalog = std::collections::HashMap::new();

        // Add some example models
        catalog.insert(
            "gpt2-small".to_string(),
            ModelInfo {
                id: "gpt2-small".to_string(),
                name: "GPT-2 Small".to_string(),
                version: "1.0".to_string(),
                description: "Small GPT-2 model for text generation".to_string(),
                model_type: ModelType::Text,
                tasks: vec!["text-generation".to_string(), "text-completion".to_string()],
                size_bytes: 500_000_000, // ~500MB
                memory_requirements: 1_000_000_000, // ~1GB
                local_path: None,
                remote_url: Some("https://huggingface.co/gpt2".to_string()),
                format: ModelFormat::HuggingFace,
                metadata: std::collections::HashMap::new(),
            },
        );

        catalog.insert(
            "bert-base".to_string(),
            ModelInfo {
                id: "bert-base".to_string(),
                name: "BERT Base".to_string(),
                version: "1.0".to_string(),
                description: "BERT base model for text understanding".to_string(),
                model_type: ModelType::Text,
                tasks: vec!["text-classification".to_string(), "embeddings".to_string()],
                size_bytes: 400_000_000, // ~400MB
                memory_requirements: 800_000_000, // ~800MB
                local_path: None,
                remote_url: Some("https://huggingface.co/bert-base-uncased".to_string()),
                format: ModelFormat::HuggingFace,
                metadata: std::collections::HashMap::new(),
            },
        );

        catalog.insert(
            "resnet50".to_string(),
            ModelInfo {
                id: "resnet50".to_string(),
                name: "ResNet-50".to_string(),
                version: "1.0".to_string(),
                description: "ResNet-50 for image classification".to_string(),
                model_type: ModelType::Image,
                tasks: vec!["image-classification".to_string()],
                size_bytes: 100_000_000, // ~100MB
                memory_requirements: 500_000_000, // ~500MB
                local_path: None,
                remote_url: Some("https://download.pytorch.org/models/resnet50-19c8e357.pth".to_string()),
                format: ModelFormat::PyTorch,
                metadata: std::collections::HashMap::new(),
            },
        );

        let mut catalog_guard = self.model_catalog.write().await;
        *catalog_guard = catalog;
    }

    /// Save catalog to file
    async fn save_catalog(&self, path: &PathBuf) -> Result<()> {
        let catalog_guard = self.model_catalog.read().await;
        let content = serde_json::to_string_pretty(&*catalog_guard)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }

    /// Get available models
    pub async fn list_models(&self) -> Vec<ModelInfo> {
        let catalog_guard = self.model_catalog.read().await;
        catalog_guard.values().cloned().collect()
    }

    /// Get model information
    pub async fn get_model_info(&self, model_id: &str) -> Option<ModelInfo> {
        let catalog_guard = self.model_catalog.read().await;
        catalog_guard.get(model_id).cloned()
    }

    /// Load a model
    pub async fn load_model(&self, model_id: &str) -> AIResult<Arc<RwLock<LoadedModel>>> {
        // Check if model is already loaded
        if let Some(loaded_model) = self.loaded_models.get(model_id) {
            let mut model_guard = loaded_model.write().await;
            model_guard.last_accessed = std::time::Instant::now();
            model_guard.ref_count += 1;
            return Ok(loaded_model.clone());
        }

        // Get model info from catalog
        let model_info = {
            let catalog_guard = self.model_catalog.read().await;
            catalog_guard.get(model_id).cloned()
        };

        let model_info = model_info.ok_or_else(|| AIEngineError::ModelNotFound {
            model: model_id.to_string(),
        })?;

        // Check memory requirements
        self.ensure_memory_available(model_info.memory_requirements)?;

        // Create loaded model entry
        let loaded_model = Arc::new(RwLock::new(LoadedModel {
            info: model_info.clone(),
            state: ModelState::Loading,
            last_accessed: std::time::Instant::now(),
            memory_usage: model_info.memory_requirements,
            ref_count: 1,
            model_data: None,
        }));

        self.loaded_models
            .insert(model_id.to_string(), loaded_model.clone());

        // Download model if not available locally
        if model_info.local_path.is_none() || !model_info.local_path.as_ref().unwrap().exists() {
            if let Some(remote_url) = &model_info.remote_url {
                self.download_model(model_id, remote_url).await?;
            }
        }

        // Simulate model loading
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Update model state
        {
            let mut model_guard = loaded_model.write().await;
            model_guard.state = ModelState::Loaded;
        }

        // Update memory usage
        self.current_memory_usage.fetch_add(
            model_info.memory_requirements,
            std::sync::atomic::Ordering::Relaxed,
        );

        info!("Model {} loaded successfully", model_id);
        Ok(loaded_model)
    }

    /// Unload a model
    pub async fn unload_model(&self, model_id: &str) -> AIResult<()> {
        if let Some((_, loaded_model)) = self.loaded_models.remove(model_id) {
            let mut model_guard = loaded_model.write().await;
            model_guard.state = ModelState::Unloading;

            // Update memory usage
            self.current_memory_usage.fetch_sub(
                model_guard.memory_usage,
                std::sync::atomic::Ordering::Relaxed,
            );

            model_guard.state = ModelState::NotLoaded;
            info!("Model {} unloaded", model_id);
        }

        Ok(())
    }

    /// Download a model from remote URL
    async fn download_model(&self, model_id: &str, url: &str) -> AIResult<()> {
        info!("Downloading model {} from {}", model_id, url);

        let model_path = self.cache_dir.join(format!("{}.model", model_id));

        // Create a simple mock download - in reality, this would download the actual model
        let mock_content = format!("Mock model data for {}", model_id);
        tokio::fs::write(&model_path, mock_content).await?;

        // Update model info with local path
        {
            let mut catalog_guard = self.model_catalog.write().await;
            if let Some(model_info) = catalog_guard.get_mut(model_id) {
                model_info.local_path = Some(model_path);
            }
        }

        info!("Model {} downloaded successfully", model_id);
        Ok(())
    }

    /// Ensure sufficient memory is available
    fn ensure_memory_available(&self, required_memory: u64) -> AIResult<()> {
        let current_usage = self.current_memory_usage.load(std::sync::atomic::Ordering::Relaxed);

        if current_usage + required_memory > self.max_memory {
            // Try to free up memory by unloading least recently used models
            self.cleanup_unused_models();

            let updated_usage = self.current_memory_usage.load(std::sync::atomic::Ordering::Relaxed);
            if updated_usage + required_memory > self.max_memory {
                return Err(AIEngineError::MemoryLimitExceeded {
                    requested: required_memory as usize,
                    limit: self.max_memory as usize,
                });
            }
        }

        Ok(())
    }

    /// Clean up unused models to free memory
    fn cleanup_unused_models(&self) {
        // This would implement LRU eviction policy
        // For now, it's a placeholder
        debug!("Cleaning up unused models");
    }

    /// Get current memory usage
    pub fn get_memory_usage(&self) -> u64 {
        self.current_memory_usage.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Get number of loaded models
    pub fn get_loaded_model_count(&self) -> usize {
        self.loaded_models.len()
    }

    /// Add model to catalog
    pub async fn add_model_to_catalog(&self, model_info: ModelInfo) -> AIResult<()> {
        let mut catalog_guard = self.model_catalog.write().await;
        catalog_guard.insert(model_info.id.clone(), model_info);
        Ok(())
    }

    /// Remove model from catalog
    pub async fn remove_model_from_catalog(&self, model_id: &str) -> AIResult<()> {
        // Unload if currently loaded
        if self.loaded_models.contains_key(model_id) {
            self.unload_model(model_id).await?;
        }

        // Remove from catalog
        let mut catalog_guard = self.model_catalog.write().await;
        catalog_guard.remove(model_id);

        Ok(())
    }
}

impl Default for ModelManager {
    fn default() -> Self {
        // This will panic in async context, so it's mainly for testing
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(Self::new(PathBuf::from("./models"), 8 * 1024 * 1024 * 1024))
            .unwrap()
    }
}