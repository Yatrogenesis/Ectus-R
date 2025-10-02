//! Optimization Engine Benchmarks
//! Performance benchmarks for the optimization engine components

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;
use tokio::runtime::Runtime;

use aion_optimization_engine::{
    OptimizationEngine, OptimizationConfig,
    CurrentMetrics, BaselineMetrics, RecommendationRequest
};

fn benchmark_optimization_engine_initialization(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("optimization_engine_init", |b| {
        b.to_async(&rt).iter(|| async {
            let config = OptimizationConfig::default();
            let engine = OptimizationEngine::new(&config).await.unwrap();
            black_box(engine)
        })
    });
}

fn benchmark_performance_score_calculation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let engine = rt.block_on(async {
        let config = OptimizationConfig::default();
        let mut engine = OptimizationEngine::new(&config).await.unwrap();

        let baseline = BaselineMetrics {
            response_time: 200.0,
            throughput: 1000.0,
            error_rate: 0.02,
            cpu_usage: 70.0,
            memory_usage: 60.0,
            availability: 99.5,
            established_at: chrono::Utc::now(),
        };

        engine.set_baseline_metrics(baseline).await.unwrap();
        engine
    });

    c.bench_function("performance_score_calculation", |b| {
        b.to_async(&rt).iter(|| async {
            let current = CurrentMetrics {
                response_time: 150.0,
                throughput: 1200.0,
                error_rate: 0.01,
                cpu_usage: 65.0,
                memory_usage: 55.0,
                availability: 99.8,
                measured_at: chrono::Utc::now(),
            };

            engine.update_current_metrics(current).await.unwrap();
            let score = engine.get_performance_score().await.unwrap();
            black_box(score)
        })
    });
}

fn benchmark_metrics_update_throughput(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let engine = rt.block_on(async {
        let config = OptimizationConfig::default();
        OptimizationEngine::new(&config).await.unwrap()
    });

    let mut group = c.benchmark_group("metrics_update_throughput");

    for batch_size in [1, 10, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("batch_size", batch_size), batch_size, |b, &batch_size| {
            b.to_async(&rt).iter(|| async {
                for i in 0..batch_size {
                    let metrics = CurrentMetrics {
                        response_time: 150.0 + (i as f64),
                        throughput: 1000.0 + (i as f64 * 10.0),
                        error_rate: 0.01,
                        cpu_usage: 60.0 + (i as f64 % 30.0),
                        memory_usage: 50.0 + (i as f64 % 20.0),
                        availability: 99.5,
                        measured_at: chrono::Utc::now(),
                    };

                    engine.update_current_metrics(metrics).await.unwrap();
                }
            })
        });
    }

    group.finish();
}

fn benchmark_ml_predictions(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let engine = rt.block_on(async {
        let mut config = OptimizationConfig::default();
        config.ml_enabled = true;

        let mut engine = OptimizationEngine::new(&config).await.unwrap();
        engine.start().await.unwrap();

        // Add some training data
        for i in 0..20 {
            let metrics = CurrentMetrics {
                response_time: 200.0 - (i as f64 * 5.0),
                throughput: 1000.0 + (i as f64 * 50.0),
                error_rate: 0.02 - (i as f64 * 0.001),
                cpu_usage: 70.0 - (i as f64 * 2.0),
                memory_usage: 60.0 - (i as f64 * 1.5),
                availability: 99.0 + (i as f64 * 0.05),
                measured_at: chrono::Utc::now(),
            };

            engine.update_current_metrics(metrics).await.unwrap();
        }

        engine
    });

    c.bench_function("ml_performance_predictions", |b| {
        b.to_async(&rt).iter(|| async {
            let predictions = engine.predict_performance_trends(Duration::from_hours(1)).await.unwrap();
            black_box(predictions)
        })
    });
}

fn benchmark_recommendation_generation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let engine = rt.block_on(async {
        let mut config = OptimizationConfig::default();
        config.auto_tuning_enabled = true;

        let mut engine = OptimizationEngine::new(&config).await.unwrap();
        engine.start().await.unwrap();

        // Add poor performance metrics to trigger recommendations
        let poor_metrics = CurrentMetrics {
            response_time: 500.0,
            throughput: 500.0,
            error_rate: 0.05,
            cpu_usage: 90.0,
            memory_usage: 85.0,
            availability: 95.0,
            measured_at: chrono::Utc::now(),
        };

        engine.update_current_metrics(poor_metrics).await.unwrap();
        engine
    });

    let mut group = c.benchmark_group("recommendation_generation");

    for max_recommendations in [5, 10, 20, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("max_recommendations", max_recommendations),
            max_recommendations,
            |b, &max_recommendations| {
                b.to_async(&rt).iter(|| async {
                    let request = RecommendationRequest {
                        system_context: "benchmark_test".to_string(),
                        performance_data: std::collections::HashMap::new(),
                        constraints: vec!["max_cpu_80".to_string()],
                        max_recommendations,
                    };

                    let recommendations = engine.get_recommendations(request).await.unwrap();
                    black_box(recommendations)
                })
            }
        );
    }

    group.finish();
}

fn benchmark_anomaly_detection(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let engine = rt.block_on(async {
        let config = OptimizationConfig::default();
        let mut engine = OptimizationEngine::new(&config).await.unwrap();
        engine.start().await.unwrap();

        // Generate time series data with some anomalies
        for hour in 0..100 {
            let base_response_time = 150.0;
            let response_time = if hour == 50 || hour == 75 {
                base_response_time * 3.0 // Anomaly
            } else {
                base_response_time + (hour as f64 * 2.0 * (hour as f64 / 12.0).sin())
            };

            let metrics = CurrentMetrics {
                response_time,
                throughput: 1000.0 + (hour as f64 * 50.0 * (hour as f64 / 8.0).cos()),
                error_rate: 0.01,
                cpu_usage: 60.0 + (hour as f64 * 2.0),
                memory_usage: 50.0 + (hour as f64 * 1.5),
                availability: 99.5,
                measured_at: chrono::Utc::now() - chrono::Duration::hours(100 - hour),
            };

            engine.update_current_metrics(metrics).await.unwrap();
        }

        engine
    });

    c.bench_function("anomaly_detection", |b| {
        b.to_async(&rt).iter(|| async {
            let anomalies = engine.detect_anomalies().await.unwrap();
            black_box(anomalies)
        })
    });
}

fn benchmark_telemetry_collection(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let engine = rt.block_on(async {
        let config = OptimizationConfig::default();
        let mut engine = OptimizationEngine::new(&config).await.unwrap();
        engine.start().await.unwrap();
        engine
    });

    c.bench_function("telemetry_collection", |b| {
        b.to_async(&rt).iter(|| async {
            // Simulate collecting various metrics
            for _ in 0..10 {
                let metrics = CurrentMetrics {
                    response_time: fastrand::f64() * 100.0 + 100.0,
                    throughput: fastrand::f64() * 500.0 + 800.0,
                    error_rate: fastrand::f64() * 0.02,
                    cpu_usage: fastrand::f64() * 40.0 + 40.0,
                    memory_usage: fastrand::f64() * 30.0 + 40.0,
                    availability: 99.0 + fastrand::f64(),
                    measured_at: chrono::Utc::now(),
                };

                engine.update_current_metrics(metrics).await.unwrap();
            }

            let telemetry = engine.get_telemetry_summary().await.unwrap();
            black_box(telemetry)
        })
    });
}

fn benchmark_config_updates(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("config_updates", |b| {
        b.to_async(&rt).iter(|| async {
            let config = OptimizationConfig::default();
            let mut engine = OptimizationEngine::new(&config).await.unwrap();

            let mut new_config = engine.get_config().await.unwrap();
            new_config.ml_enabled = !new_config.ml_enabled;
            new_config.metrics_collection_interval = fastrand::u64(10..120);

            engine.update_config(new_config).await.unwrap();

            let updated_config = engine.get_config().await.unwrap();
            black_box(updated_config)
        })
    });
}

fn benchmark_concurrent_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let engine = rt.block_on(async {
        let config = OptimizationConfig::default();
        let mut engine = OptimizationEngine::new(&config).await.unwrap();
        engine.start().await.unwrap();
        std::sync::Arc::new(engine)
    });

    c.bench_function("concurrent_operations", |b| {
        b.to_async(&rt).iter(|| async {
            let engine = engine.clone();

            // Simulate concurrent operations
            let tasks = (0..10).map(|i| {
                let engine = engine.clone();
                tokio::spawn(async move {
                    let metrics = CurrentMetrics {
                        response_time: 150.0 + (i as f64 * 10.0),
                        throughput: 1000.0 + (i as f64 * 50.0),
                        error_rate: 0.01,
                        cpu_usage: 60.0 + (i as f64 * 2.0),
                        memory_usage: 50.0 + (i as f64 * 1.5),
                        availability: 99.5,
                        measured_at: chrono::Utc::now(),
                    };

                    engine.update_current_metrics(metrics).await.unwrap();
                    engine.get_performance_score().await.unwrap()
                })
            });

            let results = futures::future::join_all(tasks).await;
            black_box(results)
        })
    });
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("memory_usage_under_load", |b| {
        b.to_async(&rt).iter(|| async {
            let config = OptimizationConfig::default();
            let mut engine = OptimizationEngine::new(&config).await.unwrap();
            engine.start().await.unwrap();

            // Generate large amount of data to test memory management
            for i in 0..1000 {
                let metrics = CurrentMetrics {
                    response_time: 150.0 + (i as f64 % 100.0),
                    throughput: 1000.0 + (i as f64 % 500.0),
                    error_rate: 0.01 + (i as f64 % 50.0) * 0.001,
                    cpu_usage: 60.0 + (i as f64 % 30.0),
                    memory_usage: 50.0 + (i as f64 % 20.0),
                    availability: 99.0 + (i as f64 % 10.0) * 0.1,
                    measured_at: chrono::Utc::now(),
                };

                engine.update_current_metrics(metrics).await.unwrap();
            }

            let score = engine.get_performance_score().await.unwrap();
            engine.stop().await.unwrap();
            black_box(score)
        })
    });
}

criterion_group!(
    optimization_benchmarks,
    benchmark_optimization_engine_initialization,
    benchmark_performance_score_calculation,
    benchmark_metrics_update_throughput,
    benchmark_ml_predictions,
    benchmark_recommendation_generation,
    benchmark_anomaly_detection,
    benchmark_telemetry_collection,
    benchmark_config_updates,
    benchmark_concurrent_operations,
    benchmark_memory_usage
);

criterion_main!(optimization_benchmarks);