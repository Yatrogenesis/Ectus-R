use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use aion_ai_engine::*;
use aion_core::config::AIEngineConfig;
use std::time::Duration;
use tokio::runtime::Runtime;

// AI Engine Performance Benchmarks

fn benchmark_inference_engine_initialization(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("ai_engine_initialization", |b| {
        b.to_async(&rt).iter(|| async {
            let config = black_box(AIEngineConfig::default());
            let engine = InferenceEngine::new(config).await.unwrap();
            black_box(engine)
        });
    });
}

fn benchmark_session_creation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let engine = rt.block_on(async {
        let config = AIEngineConfig::default();
        InferenceEngine::new(config).await.unwrap()
    });

    c.bench_function("session_creation", |b| {
        b.to_async(&rt).iter(|| async {
            let config = black_box(InferenceConfig {
                model_id: "test-model".to_string(),
                max_tokens: Some(100),
                temperature: Some(0.7),
                top_p: Some(0.9),
                timeout_seconds: Some(30),
                batch_size: Some(1),
                use_cache: true,
                stream_response: false,
                priority: InferencePriority::Normal,
            });

            let session_id = engine.create_session(config).await.unwrap();
            black_box(session_id)
        });
    });
}

fn benchmark_text_processing_throughput(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let processor = NLPProcessor::new();

    let mut group = c.benchmark_group("text_processing_throughput");

    // Test with different text sizes
    for text_size in [100, 500, 1000, 5000].iter() {
        let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(*text_size / 50);

        group.throughput(Throughput::Bytes(text.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("preprocess_text", text_size),
            &text,
            |b, text| {
                b.to_async(&rt).iter(|| async {
                    let options = black_box(TextProcessingOptions {
                        lowercase: true,
                        remove_punctuation: true,
                        remove_stopwords: false,
                        stem: false,
                        lemmatize: false,
                        min_word_length: 1,
                    });

                    let result = processor.preprocess_text(text, &options).await.unwrap();
                    black_box(result)
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("tokenize_text", text_size),
            &text,
            |b, text| {
                b.to_async(&rt).iter(|| async {
                    let options = black_box(TokenizationOptions {
                        method: TokenizationMethod::Whitespace,
                        max_tokens: None,
                        preserve_special_tokens: false,
                    });

                    let result = processor.tokenize_text(text, &options).await.unwrap();
                    black_box(result)
                });
            },
        );
    }

    group.finish();
}

fn benchmark_sentiment_analysis_batch(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let processor = NLPProcessor::new();

    let test_texts = vec![
        "I love this product! It's amazing!",
        "This is terrible and I hate it.",
        "The weather is nice today.",
        "I'm feeling neutral about this.",
        "Absolutely fantastic experience!",
        "Could be better, not satisfied.",
        "Perfect solution for my needs!",
        "Disappointing results overall.",
    ];

    let mut group = c.benchmark_group("sentiment_analysis");

    for batch_size in [1, 4, 8, 16].iter() {
        let texts: Vec<&str> = test_texts.iter().cycle().take(*batch_size).copied().collect();

        group.throughput(Throughput::Elements(*batch_size as u64));
        group.bench_with_input(
            BenchmarkId::new("batch_sentiment", batch_size),
            &texts,
            |b, texts| {
                b.to_async(&rt).iter(|| async {
                    let mut results = Vec::new();
                    for text in texts {
                        let result = processor.analyze_sentiment(black_box(text)).await.unwrap();
                        results.push(result);
                    }
                    black_box(results)
                });
            },
        );
    }

    group.finish();
}

fn benchmark_model_manager_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("model_manager");

    group.bench_function("model_registration", |b| {
        b.to_async(&rt).iter(|| async {
            let mut manager = ModelManager::new();

            let model_info = black_box(ModelInfo {
                id: "benchmark-model".to_string(),
                name: "Benchmark Model".to_string(),
                description: "A model for benchmarking".to_string(),
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
            });

            manager.register_model(model_info).await.unwrap();
            black_box(manager)
        });
    });

    group.bench_function("model_lookup", |b| {
        b.to_async(&rt).iter_setup(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let mut manager = ModelManager::new();
                let model_info = ModelInfo {
                    id: "lookup-model".to_string(),
                    name: "Lookup Model".to_string(),
                    description: "A model for lookup benchmarking".to_string(),
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
                manager.register_model(model_info).await.unwrap();
                manager
            })
        }, |manager| async move {
            let result = manager.get_model_info(black_box("lookup-model")).await.unwrap();
            black_box(result)
        });
    });

    group.finish();
}

fn benchmark_performance_metrics_collection(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("performance_metrics");

    group.bench_function("metric_recording", |b| {
        b.to_async(&rt).iter(|| async {
            let metrics = PerformanceMetrics::new();

            // Record various metrics
            metrics.record_inference_latency(black_box("test-model"), black_box(150)).await;
            metrics.record_throughput(black_box("test-model"), black_box(10.5)).await;
            metrics.record_memory_usage(black_box("test-model"), black_box(1500)).await;
            metrics.record_gpu_utilization(black_box("test-model"), black_box(75.0)).await;

            black_box(metrics)
        });
    });

    group.bench_function("metrics_aggregation", |b| {
        b.to_async(&rt).iter_setup(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let metrics = PerformanceMetrics::new();

                // Pre-populate with data
                for i in 0..1000 {
                    metrics.record_inference_latency("test-model", 100 + i % 100).await;
                    metrics.record_throughput("test-model", 10.0 + (i as f64 % 5.0)).await;
                }

                metrics
            })
        }, |metrics| async move {
            let summary = metrics.get_summary(black_box("test-model")).await.unwrap();
            black_box(summary)
        });
    });

    group.finish();
}

fn benchmark_traditional_ml_algorithms(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let ml = TraditionalML::new();

    let mut group = c.benchmark_group("traditional_ml");

    // Linear regression benchmark
    group.bench_function("linear_regression_training", |b| {
        b.to_async(&rt).iter(|| async {
            // Generate synthetic data
            let x_data: Vec<Vec<f64>> = (0..1000)
                .map(|i| vec![i as f64 / 100.0])
                .collect();
            let y_data: Vec<f64> = x_data.iter()
                .map(|x| 2.0 * x[0] + 1.0 + (rand::random::<f64>() - 0.5) * 0.1)
                .collect();

            let model = ml.train_linear_regression(black_box(&x_data), black_box(&y_data)).await.unwrap();
            black_box(model)
        });
    });

    // Clustering benchmark
    group.bench_function("kmeans_clustering", |b| {
        b.to_async(&rt).iter(|| async {
            // Generate synthetic 2D data with 3 clusters
            let mut data = Vec::new();
            for cluster in 0..3 {
                for _ in 0..100 {
                    let center_x = (cluster as f64) * 5.0;
                    let center_y = (cluster as f64) * 5.0;
                    data.push(vec![
                        center_x + (rand::random::<f64>() - 0.5) * 2.0,
                        center_y + (rand::random::<f64>() - 0.5) * 2.0,
                    ]);
                }
            }

            let options = ClusteringOptions {
                algorithm: ClusteringAlgorithm::KMeans,
                num_clusters: Some(3),
                max_iterations: Some(100),
                tolerance: Some(1e-4),
            };

            let result = ml.cluster_data(black_box(&data), black_box(&options)).await.unwrap();
            black_box(result)
        });
    });

    group.finish();
}

fn benchmark_concurrent_inference_sessions(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let engine = rt.block_on(async {
        let config = AIEngineConfig::default();
        InferenceEngine::new(config).await.unwrap()
    });

    let mut group = c.benchmark_group("concurrent_sessions");

    for concurrency in [1, 2, 4, 8, 16].iter() {
        group.bench_with_input(
            BenchmarkId::new("session_creation", concurrency),
            concurrency,
            |b, &concurrency| {
                b.to_async(&rt).iter(|| async {
                    let mut handles = Vec::new();

                    for i in 0..concurrency {
                        let engine = engine.clone();
                        let handle = tokio::spawn(async move {
                            let config = InferenceConfig {
                                model_id: format!("concurrent-model-{}", i),
                                max_tokens: Some(50),
                                temperature: Some(0.7),
                                top_p: Some(0.9),
                                timeout_seconds: Some(30),
                                batch_size: Some(1),
                                use_cache: true,
                                stream_response: false,
                                priority: InferencePriority::Normal,
                            };

                            engine.create_session(config).await.unwrap()
                        });
                        handles.push(handle);
                    }

                    let session_ids: Vec<_> = futures::future::join_all(handles)
                        .await
                        .into_iter()
                        .map(|r| r.unwrap())
                        .collect();

                    black_box(session_ids)
                });
            },
        );
    }

    group.finish();
}

fn benchmark_memory_efficiency(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("memory_efficiency");

    // Benchmark memory allocation patterns
    group.bench_function("large_text_processing", |b| {
        b.to_async(&rt).iter(|| async {
            let processor = NLPProcessor::new();

            // Process increasingly large texts
            let base_text = "This is a sample text for memory efficiency testing. ";
            let large_text = base_text.repeat(10000); // ~500KB of text

            let options = TextProcessingOptions {
                lowercase: true,
                remove_punctuation: true,
                remove_stopwords: true,
                stem: false,
                lemmatize: false,
                min_word_length: 3,
            };

            let result = processor.preprocess_text(black_box(&large_text), black_box(&options)).await.unwrap();
            black_box(result)
        });
    });

    group.bench_function("batch_vector_operations", |b| {
        b.to_async(&rt).iter(|| async {
            let ml = TraditionalML::new();

            // Large dataset for memory testing
            let data: Vec<Vec<f64>> = (0..10000)
                .map(|i| vec![
                    (i as f64).sin(),
                    (i as f64).cos(),
                    (i as f64 / 100.0).sqrt(),
                    i as f64 % 10.0,
                ])
                .collect();

            let normalized = ml.normalize_features(black_box(&data)).await.unwrap();
            black_box(normalized)
        });
    });

    group.finish();
}

// Criterion benchmark groups
criterion_group!(
    ai_engine_benches,
    benchmark_inference_engine_initialization,
    benchmark_session_creation,
    benchmark_text_processing_throughput,
    benchmark_sentiment_analysis_batch,
    benchmark_model_manager_operations,
    benchmark_performance_metrics_collection,
    benchmark_traditional_ml_algorithms,
    benchmark_concurrent_inference_sessions,
    benchmark_memory_efficiency
);

criterion_main!(ai_engine_benches);