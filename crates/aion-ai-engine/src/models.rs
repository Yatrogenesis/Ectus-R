//! # Model Definitions
//!
//! Common model structures and utilities.

use crate::errors::{AIEngineError, AIResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Model metadata and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Model unique identifier
    pub id: String,
    /// Model name
    pub name: String,
    /// Model version
    pub version: String,
    /// Model description
    pub description: String,
    /// Model author/organization
    pub author: String,
    /// License information
    pub license: String,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Model tags for categorization
    pub tags: Vec<String>,
    /// Model capabilities
    pub capabilities: ModelCapabilities,
    /// Hardware requirements
    pub requirements: HardwareRequirements,
    /// Model configuration
    pub config: ModelConfig,
}

/// Model capabilities and supported tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilities {
    /// Primary task type
    pub primary_task: TaskType,
    /// Additional supported tasks
    pub additional_tasks: Vec<TaskType>,
    /// Supported input modalities
    pub input_modalities: Vec<Modality>,
    /// Supported output modalities
    pub output_modalities: Vec<Modality>,
    /// Maximum input length (for text models)
    pub max_input_length: Option<usize>,
    /// Maximum output length (for generative models)
    pub max_output_length: Option<usize>,
    /// Supported languages (for NLP models)
    pub supported_languages: Vec<String>,
    /// Model performance metrics
    pub performance_metrics: Option<PerformanceMetrics>,
}

/// Task types supported by models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    // Text tasks
    TextGeneration,
    TextClassification,
    TextSummarization,
    TextTranslation,
    QuestionAnswering,
    NamedEntityRecognition,
    SentimentAnalysis,
    TextEmbedding,

    // Image tasks
    ImageClassification,
    ObjectDetection,
    ImageSegmentation,
    ImageGeneration,
    ImageCaptioning,
    FaceRecognition,
    OpticalCharacterRecognition,

    // Audio tasks
    SpeechRecognition,
    SpeechSynthesis,
    AudioClassification,
    MusicGeneration,
    SpeakerIdentification,
    EmotionRecognition,

    // Multimodal tasks
    VisionLanguageUnderstanding,
    ImageTextMatching,
    VideoAnalysis,

    // Traditional ML tasks
    Regression,
    Classification,
    Clustering,
    DimensionalityReduction,
    AnomalyDetection,

    // Custom task
    Custom(String),
}

/// Input/output modalities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Modality {
    Text,
    Image,
    Audio,
    Video,
    Structured,
    Embedding,
}

/// Hardware requirements for model execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareRequirements {
    /// Minimum RAM required (in bytes)
    pub min_memory: u64,
    /// Recommended RAM (in bytes)
    pub recommended_memory: u64,
    /// Minimum VRAM for GPU acceleration (in bytes)
    pub min_gpu_memory: Option<u64>,
    /// Recommended VRAM (in bytes)
    pub recommended_gpu_memory: Option<u64>,
    /// Required compute units
    pub compute_requirements: ComputeRequirements,
    /// Storage requirements (in bytes)
    pub storage_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeRequirements {
    /// Minimum CPU cores
    pub min_cpu_cores: u32,
    /// GPU acceleration support
    pub gpu_support: GPUSupport,
    /// Special hardware requirements
    pub special_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GPUSupport {
    /// No GPU required
    None,
    /// GPU recommended but not required
    Optional,
    /// GPU required
    Required,
    /// Specific GPU architectures required
    Specific(Vec<String>),
}

/// Model configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Model architecture
    pub architecture: String,
    /// Model size category
    pub size_category: ModelSize,
    /// Precision (fp16, fp32, int8, etc.)
    pub precision: Precision,
    /// Quantization settings
    pub quantization: Option<QuantizationConfig>,
    /// Optimization settings
    pub optimization: OptimizationConfig,
    /// Custom configuration parameters
    pub custom_params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelSize {
    Tiny,    // < 100M parameters
    Small,   // 100M - 1B parameters
    Medium,  // 1B - 10B parameters
    Large,   // 10B - 100B parameters
    XLarge,  // > 100B parameters
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Precision {
    FP32,  // 32-bit floating point
    FP16,  // 16-bit floating point
    BF16,  // bfloat16
    INT8,  // 8-bit integer
    INT4,  // 4-bit integer
    Mixed, // Mixed precision
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizationConfig {
    /// Quantization method
    pub method: QuantizationMethod,
    /// Target bits per weight
    pub bits_per_weight: u8,
    /// Calibration dataset size
    pub calibration_samples: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuantizationMethod {
    PostTrainingQuantization,
    QuantizationAwareTraining,
    DynamicQuantization,
    StaticQuantization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Enable graph optimization
    pub graph_optimization: bool,
    /// Enable operator fusion
    pub operator_fusion: bool,
    /// Memory optimization level
    pub memory_optimization: MemoryOptimization,
    /// Batch size optimization
    pub batch_optimization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryOptimization {
    None,
    Conservative,
    Aggressive,
}

/// Model performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Benchmark results
    pub benchmarks: HashMap<String, BenchmarkResult>,
    /// Inference speed metrics
    pub speed_metrics: SpeedMetrics,
    /// Quality metrics
    pub quality_metrics: QualityMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Benchmark name
    pub benchmark: String,
    /// Score achieved
    pub score: f64,
    /// Metric type (accuracy, F1, BLEU, etc.)
    pub metric: String,
    /// Comparison to baseline
    pub baseline_comparison: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedMetrics {
    /// Tokens per second (for text models)
    pub tokens_per_second: Option<f64>,
    /// Images per second (for vision models)
    pub images_per_second: Option<f64>,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// 95th percentile latency
    pub p95_latency_ms: f64,
    /// Memory usage during inference (bytes)
    pub memory_usage_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Accuracy score (0.0 to 1.0)
    pub accuracy: Option<f64>,
    /// F1 score (0.0 to 1.0)
    pub f1_score: Option<f64>,
    /// BLEU score (for translation tasks)
    pub bleu_score: Option<f64>,
    /// ROUGE score (for summarization tasks)
    pub rouge_score: Option<f64>,
    /// Perplexity (for language models)
    pub perplexity: Option<f64>,
    /// Custom quality metrics
    pub custom_metrics: HashMap<String, f64>,
}

/// Model registry for managing available models
#[derive(Debug)]
pub struct ModelRegistry {
    /// Available models
    models: HashMap<String, ModelMetadata>,
    /// Model categories
    categories: HashMap<TaskType, Vec<String>>,
}

impl ModelRegistry {
    /// Create a new model registry
    pub fn new() -> Self {
        let mut registry = Self {
            models: HashMap::new(),
            categories: HashMap::new(),
        };

        // Initialize with some default models
        registry.add_default_models();
        registry
    }

    /// Add a model to the registry
    pub fn register_model(&mut self, metadata: ModelMetadata) -> AIResult<()> {
        let model_id = metadata.id.clone();

        // Add to models
        self.models.insert(model_id.clone(), metadata.clone());

        // Add to categories
        let primary_task = metadata.capabilities.primary_task.clone();
        self.categories
            .entry(primary_task)
            .or_insert_with(Vec::new)
            .push(model_id.clone());

        // Add to additional task categories
        for task in &metadata.capabilities.additional_tasks {
            self.categories
                .entry(task.clone())
                .or_insert_with(Vec::new)
                .push(model_id.clone());
        }

        Ok(())
    }

    /// Get model metadata by ID
    pub fn get_model(&self, model_id: &str) -> Option<&ModelMetadata> {
        self.models.get(model_id)
    }

    /// List all available models
    pub fn list_models(&self) -> Vec<&ModelMetadata> {
        self.models.values().collect()
    }

    /// Find models by task type
    pub fn find_models_by_task(&self, task: &TaskType) -> Vec<&ModelMetadata> {
        if let Some(model_ids) = self.categories.get(task) {
            model_ids
                .iter()
                .filter_map(|id| self.models.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Find models by capability
    pub fn find_models_by_capabilities(
        &self,
        input_modality: &Modality,
        output_modality: &Modality,
    ) -> Vec<&ModelMetadata> {
        self.models
            .values()
            .filter(|model| {
                model.capabilities.input_modalities.contains(input_modality)
                    && model.capabilities.output_modalities.contains(output_modality)
            })
            .collect()
    }

    /// Find models by size category
    pub fn find_models_by_size(&self, size: &ModelSize) -> Vec<&ModelMetadata> {
        self.models
            .values()
            .filter(|model| &model.config.size_category == size)
            .collect()
    }

    /// Search models by tags
    pub fn search_by_tags(&self, tags: &[String]) -> Vec<&ModelMetadata> {
        self.models
            .values()
            .filter(|model| {
                tags.iter().any(|tag| model.tags.contains(tag))
            })
            .collect()
    }

    /// Get recommended model for a task
    pub fn get_recommended_model(&self, task: &TaskType) -> Option<&ModelMetadata> {
        let candidates = self.find_models_by_task(task);

        // Simple recommendation: choose the model with best performance metrics
        candidates
            .into_iter()
            .max_by(|a, b| {
                let score_a = a.capabilities.performance_metrics
                    .as_ref()
                    .and_then(|p| p.quality_metrics.accuracy)
                    .unwrap_or(0.0);
                let score_b = b.capabilities.performance_metrics
                    .as_ref()
                    .and_then(|p| p.quality_metrics.accuracy)
                    .unwrap_or(0.0);
                score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Add default models to the registry
    fn add_default_models(&mut self) {
        // Text generation model
        let gpt2_metadata = ModelMetadata {
            id: "gpt2-small".to_string(),
            name: "GPT-2 Small".to_string(),
            version: "1.0.0".to_string(),
            description: "Small GPT-2 model for text generation and completion".to_string(),
            author: "OpenAI".to_string(),
            license: "MIT".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: vec!["nlp".to_string(), "generation".to_string(), "gpt".to_string()],
            capabilities: ModelCapabilities {
                primary_task: TaskType::TextGeneration,
                additional_tasks: vec![TaskType::TextClassification],
                input_modalities: vec![Modality::Text],
                output_modalities: vec![Modality::Text],
                max_input_length: Some(1024),
                max_output_length: Some(1024),
                supported_languages: vec!["en".to_string()],
                performance_metrics: Some(PerformanceMetrics {
                    benchmarks: HashMap::new(),
                    speed_metrics: SpeedMetrics {
                        tokens_per_second: Some(50.0),
                        images_per_second: None,
                        avg_latency_ms: 100.0,
                        p95_latency_ms: 200.0,
                        memory_usage_bytes: 1_000_000_000,
                    },
                    quality_metrics: QualityMetrics {
                        accuracy: Some(0.85),
                        f1_score: None,
                        bleu_score: None,
                        rouge_score: None,
                        perplexity: Some(15.0),
                        custom_metrics: HashMap::new(),
                    },
                }),
            },
            requirements: HardwareRequirements {
                min_memory: 2_000_000_000,
                recommended_memory: 4_000_000_000,
                min_gpu_memory: None,
                recommended_gpu_memory: Some(2_000_000_000),
                compute_requirements: ComputeRequirements {
                    min_cpu_cores: 2,
                    gpu_support: GPUSupport::Optional,
                    special_requirements: vec![],
                },
                storage_size: 500_000_000,
            },
            config: ModelConfig {
                architecture: "transformer".to_string(),
                size_category: ModelSize::Small,
                precision: Precision::FP32,
                quantization: None,
                optimization: OptimizationConfig {
                    graph_optimization: true,
                    operator_fusion: true,
                    memory_optimization: MemoryOptimization::Conservative,
                    batch_optimization: true,
                },
                custom_params: HashMap::new(),
            },
        };

        self.register_model(gpt2_metadata).unwrap();

        // Image classification model
        let resnet_metadata = ModelMetadata {
            id: "resnet50".to_string(),
            name: "ResNet-50".to_string(),
            version: "1.0.0".to_string(),
            description: "ResNet-50 for image classification".to_string(),
            author: "Microsoft Research".to_string(),
            license: "Apache-2.0".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: vec!["vision".to_string(), "classification".to_string(), "resnet".to_string()],
            capabilities: ModelCapabilities {
                primary_task: TaskType::ImageClassification,
                additional_tasks: vec![],
                input_modalities: vec![Modality::Image],
                output_modalities: vec![Modality::Structured],
                max_input_length: None,
                max_output_length: None,
                supported_languages: vec![],
                performance_metrics: Some(PerformanceMetrics {
                    benchmarks: HashMap::new(),
                    speed_metrics: SpeedMetrics {
                        tokens_per_second: None,
                        images_per_second: Some(100.0),
                        avg_latency_ms: 50.0,
                        p95_latency_ms: 100.0,
                        memory_usage_bytes: 500_000_000,
                    },
                    quality_metrics: QualityMetrics {
                        accuracy: Some(0.76),
                        f1_score: Some(0.75),
                        bleu_score: None,
                        rouge_score: None,
                        perplexity: None,
                        custom_metrics: HashMap::new(),
                    },
                }),
            },
            requirements: HardwareRequirements {
                min_memory: 1_000_000_000,
                recommended_memory: 2_000_000_000,
                min_gpu_memory: Some(1_000_000_000),
                recommended_gpu_memory: Some(2_000_000_000),
                compute_requirements: ComputeRequirements {
                    min_cpu_cores: 2,
                    gpu_support: GPUSupport::Recommended,
                    special_requirements: vec![],
                },
                storage_size: 100_000_000,
            },
            config: ModelConfig {
                architecture: "resnet".to_string(),
                size_category: ModelSize::Small,
                precision: Precision::FP32,
                quantization: None,
                optimization: OptimizationConfig {
                    graph_optimization: true,
                    operator_fusion: true,
                    memory_optimization: MemoryOptimization::Conservative,
                    batch_optimization: true,
                },
                custom_params: HashMap::new(),
            },
        };

        self.register_model(resnet_metadata).unwrap();
    }

    /// Remove a model from the registry
    pub fn unregister_model(&mut self, model_id: &str) -> AIResult<()> {
        if let Some(metadata) = self.models.remove(model_id) {
            // Remove from categories
            for (_, model_list) in self.categories.iter_mut() {
                model_list.retain(|id| id != model_id);
            }

            Ok(())
        } else {
            Err(AIEngineError::ModelNotFound {
                model: model_id.to_string(),
            })
        }
    }

    /// Update model metadata
    pub fn update_model(&mut self, model_id: &str, metadata: ModelMetadata) -> AIResult<()> {
        if self.models.contains_key(model_id) {
            self.unregister_model(model_id)?;
            self.register_model(metadata)?;
            Ok(())
        } else {
            Err(AIEngineError::ModelNotFound {
                model: model_id.to_string(),
            })
        }
    }

    /// Get statistics about the registry
    pub fn get_registry_stats(&self) -> RegistryStats {
        let mut task_distribution = HashMap::new();
        let mut size_distribution = HashMap::new();

        for model in self.models.values() {
            // Count primary tasks
            *task_distribution
                .entry(model.capabilities.primary_task.clone())
                .or_insert(0) += 1;

            // Count size categories
            *size_distribution
                .entry(model.config.size_category.clone())
                .or_insert(0) += 1;
        }

        RegistryStats {
            total_models: self.models.len(),
            task_distribution,
            size_distribution,
            total_categories: self.categories.len(),
        }
    }
}

/// Registry statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStats {
    /// Total number of registered models
    pub total_models: usize,
    /// Distribution of models by task type
    pub task_distribution: HashMap<TaskType, usize>,
    /// Distribution of models by size category
    pub size_distribution: HashMap<ModelSize, usize>,
    /// Total number of task categories
    pub total_categories: usize,
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}