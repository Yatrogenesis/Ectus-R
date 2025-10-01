//! # AION-R AI Engine
//!
//! Comprehensive AI/ML inference engine supporting multiple backends and modalities.
//!
//! ## Features
//! - Multi-backend support (Candle, PyTorch, TensorFlow, ONNX)
//! - Text processing and NLP
//! - Computer vision and image processing
//! - Audio processing and speech recognition
//! - Traditional ML algorithms
//! - Model management and caching
//! - Performance monitoring and benchmarking
//! - Code generation from requirements
//! - Requirements analysis and optimization

pub mod inference;
pub mod models;
pub mod nlp;
pub mod vision;
pub mod audio;
pub mod traditional_ml;
pub mod model_manager;
pub mod performance;
pub mod errors;
pub mod code_generation;
pub mod requirements_analyzer;
pub mod autonomous_qa;
pub mod template_engine;
pub mod project_scaffolding;
pub mod progress_tracking;
pub mod bug_prediction;
pub mod vulnerability_scanner;
pub mod refactoring_engine;
pub mod advanced_ai_assistant;
pub mod ai_providers;
pub mod test_integration;
pub mod autocorrection_cycle;

pub use inference::*;
pub use models::*;
pub use nlp::*;
pub use vision::*;
pub use audio::*;
pub use traditional_ml::*;
pub use model_manager::*;
pub use performance::*;
pub use errors::*;
pub use code_generation::*;
pub use requirements_analyzer::*;
pub use autonomous_qa::*;
pub use template_engine::*;
pub use project_scaffolding::*;

/// AI Engine configuration
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AIEngineConfig {
    /// Maximum memory usage in bytes
    pub max_memory: usize,
    /// Default inference backend
    pub default_backend: InferenceBackend,
    /// Model cache directory
    pub cache_dir: std::path::PathBuf,
    /// Maximum concurrent inferences
    pub max_concurrent_inferences: usize,
    /// Enable performance monitoring
    pub enable_monitoring: bool,
}

impl Default for AIEngineConfig {
    fn default() -> Self {
        Self {
            max_memory: 8 * 1024 * 1024 * 1024, // 8GB
            default_backend: InferenceBackend::Candle,
            cache_dir: std::path::PathBuf::from("./models"),
            max_concurrent_inferences: 10,
            enable_monitoring: true,
        }
    }
}

/// Supported inference backends
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum InferenceBackend {
    Candle,
    #[cfg(feature = "torch")]
    PyTorch,
    #[cfg(feature = "tensorflow")]
    TensorFlow,
    #[cfg(feature = "onnx")]
    ONNX,
}

/// Initialize the AI engine with configuration
pub async fn initialize_ai_engine(config: AIEngineConfig) -> anyhow::Result<()> {
    tracing::info!("Initializing AION-R AI Engine");

    // Create cache directory if it doesn't exist
    tokio::fs::create_dir_all(&config.cache_dir).await?;

    // Initialize model manager
    let _model_manager = ModelManager::new(config.cache_dir.clone(), config.max_memory).await?;

    tracing::info!("AI Engine initialized successfully");
    Ok(())
}