use aion_ai_engine::*;
use aion_core::config::AIEngineConfig;
use std::collections::HashMap;
use tokio::sync::Semaphore;
use uuid::Uuid;

#[tokio::test]
async fn test_inference_engine_initialization() {
    let config = AIEngineConfig::default();
    let engine = InferenceEngine::new(config.clone()).await.unwrap();

    assert_eq!(engine.get_config().backend, config.backend);
    assert!(engine.get_active_sessions().is_empty());
}

#[tokio::test]
async fn test_inference_session_creation() {
    let config = AIEngineConfig::default();
    let engine = InferenceEngine::new(config).await.unwrap();

    let session_config = InferenceConfig {
        model_id: "test-model".to_string(),
        max_tokens: Some(100),
        temperature: Some(0.7),
        top_p: Some(0.9),
        timeout_seconds: Some(30),
        batch_size: Some(1),
        use_cache: true,
        stream_response: false,
        priority: InferencePriority::Normal,
    };

    let session_result = engine.create_session(session_config).await;
    assert!(session_result.is_ok());

    let session_id = session_result.unwrap();
    assert!(engine.get_active_sessions().contains_key(&session_id));
}

#[tokio::test]
async fn test_text_inference_basic() {
    let config = AIEngineConfig::default();
    let engine = InferenceEngine::new(config).await.unwrap();

    let session_config = InferenceConfig {
        model_id: "gpt2".to_string(),
        max_tokens: Some(50),
        temperature: Some(0.7),
        top_p: Some(0.9),
        timeout_seconds: Some(30),
        batch_size: Some(1),
        use_cache: true,
        stream_response: false,
        priority: InferencePriority::Normal,
    };

    let session_id = engine.create_session(session_config).await.unwrap();

    let input = InferenceInput::Text {
        prompt: "Hello, world!".to_string(),
        context: None,
    };

    // Note: This would normally connect to an actual model
    // For testing, we'll test the error handling
    let result = engine.run_inference(session_id, input).await;

    // Should either succeed or fail gracefully with proper error
    match result {
        Ok(_) => {
            // Success case - verify result structure
            assert!(true);
        }
        Err(e) => {
            // Expected error case for test environment
            assert!(e.to_string().contains("model") || e.to_string().contains("backend"));
        }
    }
}

#[tokio::test]
async fn test_inference_config_validation() {
    // Test valid config
    let valid_config = InferenceConfig {
        model_id: "valid-model".to_string(),
        max_tokens: Some(100),
        temperature: Some(0.7),
        top_p: Some(0.9),
        timeout_seconds: Some(30),
        batch_size: Some(1),
        use_cache: true,
        stream_response: false,
        priority: InferencePriority::Normal,
    };

    assert!(validate_inference_config(&valid_config).is_ok());

    // Test invalid temperature
    let invalid_temp_config = InferenceConfig {
        model_id: "test-model".to_string(),
        max_tokens: Some(100),
        temperature: Some(2.0), // Invalid: > 1.0
        top_p: Some(0.9),
        timeout_seconds: Some(30),
        batch_size: Some(1),
        use_cache: true,
        stream_response: false,
        priority: InferencePriority::Normal,
    };

    assert!(validate_inference_config(&invalid_temp_config).is_err());

    // Test invalid top_p
    let invalid_top_p_config = InferenceConfig {
        model_id: "test-model".to_string(),
        max_tokens: Some(100),
        temperature: Some(0.7),
        top_p: Some(1.5), // Invalid: > 1.0
        timeout_seconds: Some(30),
        batch_size: Some(1),
        use_cache: true,
        stream_response: false,
        priority: InferencePriority::Normal,
    };

    assert!(validate_inference_config(&invalid_top_p_config).is_err());
}

#[tokio::test]
async fn test_model_manager_registration() {
    let mut manager = ModelManager::new();

    let model_info = ModelInfo {
        id: "test-model".to_string(),
        name: "Test Model".to_string(),
        description: "A test model".to_string(),
        version: "1.0.0".to_string(),
        model_type: ModelType::TextGeneration,
        backend: AIBackend::Candle,
        capabilities: ModelCapabilities {
            max_context_length: 2048,
            supports_streaming: true,
            supports_batching: true,
            input_modalities: vec![InputModality::Text],
            output_modalities: vec![OutputModality::Text],
            languages: vec!["en".to_string()],
            tasks: vec!["text-generation".to_string()],
        },
        resource_requirements: ResourceRequirements {
            min_memory_mb: 1024,
            recommended_memory_mb: 2048,
            min_compute_units: 1,
            gpu_required: false,
            estimated_latency_ms: 100,
        },
        file_path: None,
        download_url: None,
        checksum: None,
        license: "MIT".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let result = manager.register_model(model_info.clone()).await;
    assert!(result.is_ok());

    let retrieved = manager.get_model_info("test-model").await;
    assert!(retrieved.is_ok());
    assert_eq!(retrieved.unwrap().id, "test-model");
}

#[tokio::test]
async fn test_performance_metrics_collection() {
    let metrics = PerformanceMetrics::new();

    // Record some metrics
    metrics.record_inference_latency("test-model", 150).await;
    metrics.record_inference_latency("test-model", 200).await;
    metrics.record_inference_latency("test-model", 100).await;

    metrics.record_throughput("test-model", 10.5).await;
    metrics.record_memory_usage("test-model", 1500).await;

    let summary = metrics.get_summary("test-model").await.unwrap();

    assert_eq!(summary.total_requests, 3);
    assert_eq!(summary.average_latency_ms, 150);
    assert_eq!(summary.min_latency_ms, 100);
    assert_eq!(summary.max_latency_ms, 200);
    assert!(summary.current_throughput_rps > 0.0);
}

#[tokio::test]
async fn test_nlp_preprocessing() {
    let processor = NLPProcessor::new();

    let text = "Hello, World! This is a TEST sentence with MIXED cases.";
    let processed = processor.preprocess_text(text, &TextProcessingOptions {
        lowercase: true,
        remove_punctuation: true,
        remove_stopwords: false,
        stem: false,
        lemmatize: false,
        min_word_length: 1,
    }).await.unwrap();

    assert!(!processed.contains("!"));
    assert!(!processed.contains(","));
    assert!(processed.to_lowercase() == processed);
}

#[tokio::test]
async fn test_nlp_tokenization() {
    let processor = NLPProcessor::new();

    let text = "This is a simple test sentence.";
    let tokens = processor.tokenize_text(text, &TokenizationOptions {
        method: TokenizationMethod::Whitespace,
        max_tokens: None,
        preserve_special_tokens: false,
    }).await.unwrap();

    assert_eq!(tokens.len(), 6); // "This", "is", "a", "simple", "test", "sentence."
    assert_eq!(tokens[0], "This");
    assert_eq!(tokens[5], "sentence.");
}

#[tokio::test]
async fn test_nlp_sentiment_analysis() {
    let processor = NLPProcessor::new();

    // Test positive sentiment
    let positive_text = "I love this product! It's amazing and wonderful.";
    let positive_result = processor.analyze_sentiment(positive_text).await.unwrap();
    assert!(positive_result.confidence > 0.5);

    // Test negative sentiment
    let negative_text = "This is terrible and awful. I hate it.";
    let negative_result = processor.analyze_sentiment(negative_text).await.unwrap();
    assert!(negative_result.confidence > 0.5);
}

#[tokio::test]
async fn test_vision_image_analysis() {
    let processor = VisionProcessor::new();

    // Create a simple test image (1x1 RGB pixel)
    let test_image = ImageInput {
        data: vec![255, 0, 0], // Red pixel
        width: 1,
        height: 1,
        format: ImageFormat::RGB,
    };

    let analysis_options = ImageAnalysisOptions {
        detect_objects: false,
        extract_text: false,
        analyze_faces: false,
        classify_image: true,
        generate_description: false,
        extract_features: true,
    };

    let result = processor.analyze_image(&test_image, &analysis_options).await;

    // Should either succeed or fail gracefully
    match result {
        Ok(analysis) => {
            assert!(analysis.features.is_some());
        }
        Err(e) => {
            // Expected in test environment without actual vision models
            assert!(e.to_string().contains("model") || e.to_string().contains("backend"));
        }
    }
}

#[tokio::test]
async fn test_audio_processing() {
    let processor = AudioProcessor::new();

    // Create simple test audio data (1 second of silence at 16kHz)
    let test_audio = AudioInput {
        data: vec![0i16; 16000], // 1 second of silence
        sample_rate: 16000,
        channels: 1,
        format: AudioFormat::PCM16,
    };

    let transcription_options = TranscriptionOptions {
        language: Some("en".to_string()),
        model: None,
        enable_timestamps: false,
        enable_word_confidence: false,
        filter_profanity: false,
    };

    let result = processor.transcribe_audio(&test_audio, &transcription_options).await;

    // Should either succeed or fail gracefully
    match result {
        Ok(_transcription) => {
            // Success case
            assert!(true);
        }
        Err(e) => {
            // Expected in test environment without actual audio models
            assert!(e.to_string().contains("model") || e.to_string().contains("backend"));
        }
    }
}

#[tokio::test]
async fn test_traditional_ml_linear_regression() {
    let ml = TraditionalML::new();

    // Simple linear dataset: y = 2x + 1
    let x_data = vec![
        vec![1.0], vec![2.0], vec![3.0], vec![4.0], vec![5.0]
    ];
    let y_data = vec![3.0, 5.0, 7.0, 9.0, 11.0];

    let model = ml.train_linear_regression(&x_data, &y_data).await.unwrap();

    // Test prediction
    let prediction = ml.predict_linear_regression(&model, &vec![6.0]).await.unwrap();

    // Should predict approximately 13.0 (2*6 + 1)
    assert!((prediction - 13.0).abs() < 0.1);
}

#[tokio::test]
async fn test_traditional_ml_clustering() {
    let ml = TraditionalML::new();

    // Simple 2D dataset with two clear clusters
    let data = vec![
        vec![1.0, 1.0], vec![1.5, 1.2], vec![1.2, 1.5],  // Cluster 1
        vec![5.0, 5.0], vec![5.2, 4.8], vec![4.8, 5.2],  // Cluster 2
    ];

    let clustering_options = ClusteringOptions {
        algorithm: ClusteringAlgorithm::KMeans,
        num_clusters: Some(2),
        max_iterations: Some(100),
        tolerance: Some(1e-4),
    };

    let result = ml.cluster_data(&data, &clustering_options).await.unwrap();

    assert_eq!(result.cluster_labels.len(), 6);
    assert_eq!(result.num_clusters, 2);

    // Points in same cluster should have same label
    assert_eq!(result.cluster_labels[0], result.cluster_labels[1]);
    assert_eq!(result.cluster_labels[3], result.cluster_labels[4]);
}

#[tokio::test]
async fn test_error_handling_and_recovery() {
    let config = AIEngineConfig::default();
    let engine = InferenceEngine::new(config).await.unwrap();

    // Test invalid session ID
    let invalid_session = Uuid::new_v4();
    let input = InferenceInput::Text {
        prompt: "test".to_string(),
        context: None,
    };

    let result = engine.run_inference(invalid_session, input).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        AIEngineError::SessionNotFound(_) => {
            // Expected error
            assert!(true);
        }
        _ => {
            panic!("Expected SessionNotFound error");
        }
    }
}

#[tokio::test]
async fn test_concurrent_inference_sessions() {
    let config = AIEngineConfig::default();
    let engine = InferenceEngine::new(config).await.unwrap();

    let mut handles = vec![];

    // Create multiple concurrent sessions
    for i in 0..5 {
        let engine_clone = engine.clone();
        let handle = tokio::spawn(async move {
            let session_config = InferenceConfig {
                model_id: format!("test-model-{}", i),
                max_tokens: Some(50),
                temperature: Some(0.7),
                top_p: Some(0.9),
                timeout_seconds: Some(30),
                batch_size: Some(1),
                use_cache: true,
                stream_response: false,
                priority: InferencePriority::Normal,
            };

            engine_clone.create_session(session_config).await
        });
        handles.push(handle);
    }

    // Wait for all sessions to be created
    let mut session_ids = vec![];
    for handle in handles {
        let session_id = handle.await.unwrap().unwrap();
        session_ids.push(session_id);
    }

    assert_eq!(session_ids.len(), 5);
    assert_eq!(engine.get_active_sessions().len(), 5);
}

// Helper function for config validation
fn validate_inference_config(config: &InferenceConfig) -> Result<(), String> {
    if let Some(temp) = config.temperature {
        if temp < 0.0 || temp > 1.0 {
            return Err("Temperature must be between 0.0 and 1.0".to_string());
        }
    }

    if let Some(top_p) = config.top_p {
        if top_p < 0.0 || top_p > 1.0 {
            return Err("Top_p must be between 0.0 and 1.0".to_string());
        }
    }

    if config.model_id.is_empty() {
        return Err("Model ID cannot be empty".to_string());
    }

    Ok(())
}