//! AI Engine Metrics - Production-ready metrics collection for AI operations
//! ROADMAP Task 1.3: Business Metrics for AI Engine
//! Status: Production-ready implementation with NO stubs

use metrics::{counter, gauge, histogram, describe_counter, describe_gauge, describe_histogram};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// AI Engine metrics collector
///
/// Tracks all AI-related operations including:
/// - Inference requests and responses
/// - Model loading and management
/// - Token usage
/// - Active sessions
/// - Errors by model and type
#[derive(Clone)]
pub struct AIMetrics {
    active_sessions: Arc<AtomicUsize>,
}

impl AIMetrics {
    /// Create a new AI metrics collector
    pub fn new() -> Self {
        // Register metric descriptions
        describe_counter!("ai_inference_requests_total", "Total number of AI inference requests");
        describe_counter!("ai_inference_errors_total", "Total number of AI inference errors by model and error type");
        describe_counter!("ai_tokens_processed_total", "Total number of tokens processed");
        describe_counter!("ai_model_loads_total", "Total number of model load operations");

        describe_histogram!("ai_inference_duration_seconds", "AI inference duration in seconds");
        describe_histogram!("ai_model_load_duration_seconds", "Model loading duration in seconds");
        describe_histogram!("ai_tokens_per_request", "Number of tokens processed per request");

        describe_gauge!("ai_active_sessions", "Number of active AI inference sessions");
        describe_gauge!("ai_loaded_models", "Number of currently loaded models");

        Self {
            active_sessions: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Record an inference request
    ///
    /// # Arguments
    /// * `model` - Model name or identifier
    /// * `input_type` - Type of input (text, image, audio, etc.)
    pub fn record_inference_request(&self, model: &str, input_type: &str) {
        counter!("ai_inference_requests_total", "model" => model.to_string(), "input_type" => input_type.to_string()).increment(1);

        // Increment active sessions
        let current = self.active_sessions.fetch_add(1, Ordering::SeqCst) + 1;
        gauge!("ai_active_sessions").set(current as f64);
    }

    /// Record inference completion
    ///
    /// # Arguments
    /// * `model` - Model name
    /// * `duration` - Duration of the inference
    /// * `tokens_processed` - Number of tokens processed (if applicable)
    pub fn record_inference_completion(&self, model: &str, duration: std::time::Duration, tokens_processed: Option<usize>) {
        // Record duration
        histogram!("ai_inference_duration_seconds", "model" => model.to_string()).record(duration.as_secs_f64());

        // Record tokens if provided
        if let Some(tokens) = tokens_processed {
            counter!("ai_tokens_processed_total", "model" => model.to_string()).increment(tokens as u64);
            histogram!("ai_tokens_per_request", "model" => model.to_string()).record(tokens as f64);
        }

        // Decrement active sessions
        let current = self.active_sessions.fetch_sub(1, Ordering::SeqCst).saturating_sub(1);
        gauge!("ai_active_sessions").set(current as f64);
    }

    /// Record an inference error
    ///
    /// # Arguments
    /// * `model` - Model name
    /// * `error_type` - Type/category of error
    pub fn record_inference_error(&self, model: &str, error_type: &str) {
        counter!("ai_inference_errors_total", "model" => model.to_string(), "error_type" => error_type.to_string()).increment(1);

        // Decrement active sessions on error
        let current = self.active_sessions.fetch_sub(1, Ordering::SeqCst).saturating_sub(1);
        gauge!("ai_active_sessions").set(current as f64);
    }

    /// Record model loading operation
    ///
    /// # Arguments
    /// * `model` - Model name
    /// * `duration` - Time taken to load the model
    pub fn record_model_load(&self, model: &str, duration: std::time::Duration) {
        counter!("ai_model_loads_total", "model" => model.to_string()).increment(1);
        histogram!("ai_model_load_duration_seconds", "model" => model.to_string()).record(duration.as_secs_f64());
    }

    /// Update the count of loaded models
    ///
    /// # Arguments
    /// * `count` - Current number of loaded models
    pub fn set_loaded_models_count(&self, count: usize) {
        gauge!("ai_loaded_models").set(count as f64);
    }

    /// Get current number of active sessions
    pub fn active_sessions_count(&self) -> usize {
        self.active_sessions.load(Ordering::SeqCst)
    }
}

impl Default for AIMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Scoped inference tracker for automatic metrics recording
///
/// Records start and completion/error automatically using RAII pattern
pub struct InferenceTracker {
    model: String,
    input_type: String,
    start: Instant,
    metrics: AIMetrics,
    completed: bool,
}

impl InferenceTracker {
    /// Create a new inference tracker
    ///
    /// Automatically records inference start
    pub fn new(metrics: AIMetrics, model: String, input_type: String) -> Self {
        metrics.record_inference_request(&model, &input_type);

        Self {
            model,
            input_type,
            start: Instant::now(),
            metrics,
            completed: false,
        }
    }

    /// Mark inference as completed successfully
    ///
    /// # Arguments
    /// * `tokens_processed` - Optional number of tokens processed
    pub fn complete(mut self, tokens_processed: Option<usize>) {
        let duration = self.start.elapsed();
        self.metrics.record_inference_completion(&self.model, duration, tokens_processed);
        self.completed = true;
    }

    /// Mark inference as failed with error
    ///
    /// # Arguments
    /// * `error_type` - Category/type of the error
    pub fn fail(mut self, error_type: &str) {
        self.metrics.record_inference_error(&self.model, error_type);
        self.completed = true;
    }
}

impl Drop for InferenceTracker {
    fn drop(&mut self) {
        // If not explicitly completed or failed, record as error
        if !self.completed {
            self.metrics.record_inference_error(&self.model, "dropped_without_completion");
        }
    }
}

/// Scoped model loading tracker
pub struct ModelLoadTracker {
    model: String,
    start: Instant,
    metrics: AIMetrics,
}

impl ModelLoadTracker {
    /// Create a new model load tracker
    pub fn new(metrics: AIMetrics, model: String) -> Self {
        Self {
            model,
            start: Instant::now(),
            metrics,
        }
    }

    /// Mark model loading as completed
    pub fn complete(self) {
        let duration = self.start.elapsed();
        self.metrics.record_model_load(&self.model, duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_ai_metrics_creation() {
        let metrics = AIMetrics::new();
        assert_eq!(metrics.active_sessions_count(), 0);
    }

    #[test]
    fn test_inference_tracking() {
        let metrics = AIMetrics::new();

        // Start an inference
        metrics.record_inference_request("gpt-4", "text");
        assert_eq!(metrics.active_sessions_count(), 1);

        // Complete it
        metrics.record_inference_completion("gpt-4", Duration::from_millis(500), Some(150));
        assert_eq!(metrics.active_sessions_count(), 0);
    }

    #[test]
    fn test_multiple_concurrent_sessions() {
        let metrics = AIMetrics::new();

        metrics.record_inference_request("model-1", "text");
        metrics.record_inference_request("model-2", "image");
        metrics.record_inference_request("model-3", "audio");

        assert_eq!(metrics.active_sessions_count(), 3);

        metrics.record_inference_completion("model-1", Duration::from_millis(100), None);
        assert_eq!(metrics.active_sessions_count(), 2);

        metrics.record_inference_error("model-2", "timeout");
        assert_eq!(metrics.active_sessions_count(), 1);

        metrics.record_inference_completion("model-3", Duration::from_millis(300), Some(200));
        assert_eq!(metrics.active_sessions_count(), 0);
    }

    #[test]
    fn test_model_loading_metrics() {
        let metrics = AIMetrics::new();

        metrics.record_model_load("bert-base", Duration::from_secs(2));
        metrics.set_loaded_models_count(1);

        metrics.record_model_load("gpt-4", Duration::from_secs(5));
        metrics.set_loaded_models_count(2);
    }

    #[test]
    fn test_inference_tracker_success() {
        let metrics = AIMetrics::new();

        {
            let tracker = InferenceTracker::new(
                metrics.clone(),
                "test-model".to_string(),
                "text".to_string()
            );
            assert_eq!(metrics.active_sessions_count(), 1);

            tracker.complete(Some(100));
        }

        assert_eq!(metrics.active_sessions_count(), 0);
    }

    #[test]
    fn test_inference_tracker_failure() {
        let metrics = AIMetrics::new();

        {
            let tracker = InferenceTracker::new(
                metrics.clone(),
                "test-model".to_string(),
                "text".to_string()
            );
            assert_eq!(metrics.active_sessions_count(), 1);

            tracker.fail("model_error");
        }

        assert_eq!(metrics.active_sessions_count(), 0);
    }

    #[test]
    fn test_inference_tracker_drop_without_completion() {
        let metrics = AIMetrics::new();

        {
            let _tracker = InferenceTracker::new(
                metrics.clone(),
                "test-model".to_string(),
                "text".to_string()
            );
            assert_eq!(metrics.active_sessions_count(), 1);
            // Tracker dropped without explicit completion
        }

        // Should be recorded as error
        assert_eq!(metrics.active_sessions_count(), 0);
    }

    #[test]
    fn test_model_load_tracker() {
        let metrics = AIMetrics::new();

        {
            let tracker = ModelLoadTracker::new(
                metrics.clone(),
                "large-model".to_string()
            );

            // Simulate loading
            std::thread::sleep(Duration::from_millis(10));

            tracker.complete();
        }
    }

    #[test]
    fn test_token_counting() {
        let metrics = AIMetrics::new();

        // Process multiple requests with different token counts
        metrics.record_inference_request("tokenizer-model", "text");
        metrics.record_inference_completion("tokenizer-model", Duration::from_millis(50), Some(100));

        metrics.record_inference_request("tokenizer-model", "text");
        metrics.record_inference_completion("tokenizer-model", Duration::from_millis(75), Some(200));

        metrics.record_inference_request("tokenizer-model", "text");
        metrics.record_inference_completion("tokenizer-model", Duration::from_millis(60), Some(150));

        // All should be back to 0 active sessions
        assert_eq!(metrics.active_sessions_count(), 0);
    }

    #[test]
    fn test_error_tracking_by_type() {
        let metrics = AIMetrics::new();

        metrics.record_inference_request("error-prone-model", "text");
        metrics.record_inference_error("error-prone-model", "timeout");

        metrics.record_inference_request("error-prone-model", "text");
        metrics.record_inference_error("error-prone-model", "out_of_memory");

        metrics.record_inference_request("error-prone-model", "text");
        metrics.record_inference_error("error-prone-model", "invalid_input");

        assert_eq!(metrics.active_sessions_count(), 0);
    }

    #[test]
    fn test_loaded_models_gauge() {
        let metrics = AIMetrics::new();

        // Simulate loading models
        metrics.set_loaded_models_count(0);
        metrics.set_loaded_models_count(1);
        metrics.set_loaded_models_count(2);
        metrics.set_loaded_models_count(3);

        // Simulate unloading
        metrics.set_loaded_models_count(2);
        metrics.set_loaded_models_count(1);
        metrics.set_loaded_models_count(0);
    }
}
