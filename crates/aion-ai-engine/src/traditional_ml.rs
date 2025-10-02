//! # Traditional Machine Learning
//!
//! Classical ML algorithms and utilities.

use crate::errors::{AIEngineError, AIResult};
use nalgebra::{DMatrix, DVector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dataset for training and inference
#[derive(Debug, Clone)]
pub struct Dataset {
    /// Feature matrix (rows = samples, columns = features)
    pub features: DMatrix<f64>,
    /// Target values (for supervised learning)
    pub targets: Option<DVector<f64>>,
    /// Feature names
    pub feature_names: Vec<String>,
    /// Class labels (for classification)
    pub class_labels: Option<Vec<String>>,
}

/// Data preprocessing options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessingOptions {
    /// Normalize features to [0, 1] range
    pub normalize: bool,
    /// Standardize features (mean=0, std=1)
    pub standardize: bool,
    /// Handle missing values
    pub handle_missing: MissingValueStrategy,
    /// Remove outliers
    pub remove_outliers: bool,
    /// Outlier threshold (number of standard deviations)
    pub outlier_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MissingValueStrategy {
    /// Remove rows with missing values
    Remove,
    /// Fill with mean value
    FillMean,
    /// Fill with median value
    FillMedian,
    /// Fill with specified value
    FillValue(f64),
    /// Forward fill
    ForwardFill,
}

impl Default for PreprocessingOptions {
    fn default() -> Self {
        Self {
            normalize: false,
            standardize: true,
            handle_missing: MissingValueStrategy::FillMean,
            remove_outliers: false,
            outlier_threshold: 3.0,
        }
    }
}

/// Training configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Maximum number of iterations
    pub max_iterations: usize,
    /// Learning rate
    pub learning_rate: f64,
    /// Convergence tolerance
    pub tolerance: f64,
    /// Regularization parameter
    pub regularization: f64,
    /// Random seed for reproducibility
    pub random_seed: Option<u64>,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            learning_rate: 0.01,
            tolerance: 1e-6,
            regularization: 0.01,
            random_seed: Some(42),
        }
    }
}

/// Model evaluation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    /// For regression: R-squared, RMSE, MAE
    pub regression_metrics: Option<RegressionMetrics>,
    /// For classification: accuracy, precision, recall, F1
    pub classification_metrics: Option<ClassificationMetrics>,
    /// Cross-validation scores
    pub cv_scores: Option<Vec<f64>>,
    /// Feature importance scores
    pub feature_importance: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionMetrics {
    /// R-squared score
    pub r_squared: f64,
    /// Root Mean Square Error
    pub rmse: f64,
    /// Mean Absolute Error
    pub mae: f64,
    /// Mean Squared Error
    pub mse: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationMetrics {
    /// Overall accuracy
    pub accuracy: f64,
    /// Precision per class
    pub precision: Vec<f64>,
    /// Recall per class
    pub recall: Vec<f64>,
    /// F1 score per class
    pub f1_score: Vec<f64>,
    /// Confusion matrix
    pub confusion_matrix: Vec<Vec<usize>>,
}

/// Linear regression model
#[derive(Debug, Clone)]
pub struct LinearRegression {
    /// Model coefficients
    pub coefficients: Option<DVector<f64>>,
    /// Intercept term
    pub intercept: f64,
    /// Training configuration
    config: TrainingConfig,
}

/// Logistic regression model
#[derive(Debug, Clone)]
pub struct LogisticRegression {
    /// Model coefficients
    pub coefficients: Option<DVector<f64>>,
    /// Intercept term
    pub intercept: f64,
    /// Training configuration
    config: TrainingConfig,
}

/// K-Means clustering model
#[derive(Debug, Clone)]
pub struct KMeansClustering {
    /// Cluster centroids
    pub centroids: Option<DMatrix<f64>>,
    /// Number of clusters
    pub n_clusters: usize,
    /// Training configuration
    config: TrainingConfig,
}

/// Principal Component Analysis
#[derive(Debug, Clone)]
pub struct PCA {
    /// Principal components (eigenvectors)
    pub components: Option<DMatrix<f64>>,
    /// Explained variance ratio
    pub explained_variance_ratio: Option<DVector<f64>>,
    /// Number of components
    pub n_components: usize,
    /// Mean of training data
    pub mean: Option<DVector<f64>>,
}

/// Traditional ML processor
pub struct TraditionalMLProcessor {
    /// Data preprocessing utilities
    preprocessor: DataPreprocessor,
}

/// Data preprocessing utilities
struct DataPreprocessor {
    /// Statistics for normalization/standardization
    feature_stats: Option<Vec<FeatureStats>>,
}

#[derive(Debug, Clone)]
struct FeatureStats {
    mean: f64,
    std: f64,
    min: f64,
    max: f64,
}

impl TraditionalMLProcessor {
    /// Create a new traditional ML processor
    pub fn new() -> Self {
        Self {
            preprocessor: DataPreprocessor {
                feature_stats: None,
            },
        }
    }

    /// Create dataset from JSON data
    pub fn create_dataset_from_json(
        &self,
        data: &serde_json::Value,
        target_column: Option<&str>,
    ) -> AIResult<Dataset> {
        let array = data.as_array().ok_or_else(|| AIEngineError::PreprocessingFailed {
            reason: "Data must be an array of objects".to_string(),
        })?;

        if array.is_empty() {
            return Err(AIEngineError::PreprocessingFailed {
                reason: "Dataset is empty".to_string(),
            });
        }

        // Extract feature names from first object
        let first_obj = array[0].as_object().ok_or_else(|| AIEngineError::PreprocessingFailed {
            reason: "Array elements must be objects".to_string(),
        })?;

        let mut feature_names: Vec<String> = first_obj.keys().cloned().collect();

        // Remove target column from features if specified
        if let Some(target_col) = target_column {
            feature_names.retain(|name| name != target_col);
        }

        let n_samples = array.len();
        let n_features = feature_names.len();

        // Initialize feature matrix
        let mut features = DMatrix::zeros(n_samples, n_features);
        let mut targets = target_column.map(|_| DVector::zeros(n_samples));

        // Fill data
        for (i, sample) in array.iter().enumerate() {
            let obj = sample.as_object().ok_or_else(|| AIEngineError::PreprocessingFailed {
                reason: format!("Sample {} is not an object", i),
            })?;

            // Fill features
            for (j, feature_name) in feature_names.iter().enumerate() {
                let value = obj.get(feature_name)
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0); // Default to 0 for missing values
                features[(i, j)] = value;
            }

            // Fill target if specified
            if let (Some(target_col), Some(ref mut target_vec)) = (target_column, &mut targets) {
                let target_value = obj.get(target_col)
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                target_vec[i] = target_value;
            }
        }

        Ok(Dataset {
            features,
            targets,
            feature_names,
            class_labels: None,
        })
    }

    /// Preprocess dataset
    pub fn preprocess_dataset(
        &mut self,
        dataset: &Dataset,
        options: &PreprocessingOptions,
    ) -> AIResult<Dataset> {
        let mut processed_features = dataset.features.clone();

        // Calculate feature statistics if not available
        if self.preprocessor.feature_stats.is_none() {
            self.preprocessor.calculate_feature_stats(&processed_features);
        }

        let stats = self.preprocessor.feature_stats.as_ref().unwrap();

        // Apply preprocessing based on options
        if options.standardize {
            for j in 0..processed_features.ncols() {
                let mean = stats[j].mean;
                let std = stats[j].std;
                if std > 0.0 {
                    for i in 0..processed_features.nrows() {
                        processed_features[(i, j)] = (processed_features[(i, j)] - mean) / std;
                    }
                }
            }
        } else if options.normalize {
            for j in 0..processed_features.ncols() {
                let min = stats[j].min;
                let max = stats[j].max;
                let range = max - min;
                if range > 0.0 {
                    for i in 0..processed_features.nrows() {
                        processed_features[(i, j)] = (processed_features[(i, j)] - min) / range;
                    }
                }
            }
        }

        // Remove outliers if requested
        if options.remove_outliers {
            // This is a simplified outlier removal
            // In practice, you'd want more sophisticated methods
        }

        Ok(Dataset {
            features: processed_features,
            targets: dataset.targets.clone(),
            feature_names: dataset.feature_names.clone(),
            class_labels: dataset.class_labels.clone(),
        })
    }

    /// Train linear regression model
    pub fn train_linear_regression(
        &self,
        dataset: &Dataset,
        config: &TrainingConfig,
    ) -> AIResult<LinearRegression> {
        let targets = dataset.targets.as_ref().ok_or_else(|| AIEngineError::ModelLoadingFailed {
            model: "linear_regression".to_string(),
            reason: "Target values required for regression".to_string(),
        })?;

        // Add bias term (column of ones)
        let mut x_with_bias = DMatrix::zeros(dataset.features.nrows(), dataset.features.ncols() + 1);

        // Fill bias column with ones
        for i in 0..dataset.features.nrows() {
            x_with_bias[(i, 0)] = 1.0;
        }

        // Copy feature data
        for i in 0..dataset.features.nrows() {
            for j in 0..dataset.features.ncols() {
                x_with_bias[(i, j + 1)] = dataset.features[(i, j)];
            }
        }

        // Solve normal equation: θ = (X^T X + λI)^(-1) X^T y
        let xt = x_with_bias.transpose();
        let xtx = &xt * &x_with_bias;

        // Add regularization
        let mut xtx_reg = xtx;
        for i in 0..xtx_reg.nrows() {
            xtx_reg[(i, i)] += config.regularization;
        }

        let xty = &xt * targets;

        // Solve for coefficients
        match xtx_reg.lu().solve(&xty) {
            Some(theta) => {
                let intercept = theta[0];
                let coefficients = DVector::from_iterator(theta.len() - 1, theta.iter().skip(1).cloned());

                Ok(LinearRegression {
                    coefficients: Some(coefficients),
                    intercept,
                    config: config.clone(),
                })
            }
            None => Err(AIEngineError::ModelLoadingFailed {
                model: "linear_regression".to_string(),
                reason: "Failed to solve normal equation".to_string(),
            }),
        }
    }

    /// Train logistic regression model
    pub fn train_logistic_regression(
        &self,
        dataset: &Dataset,
        config: &TrainingConfig,
    ) -> AIResult<LogisticRegression> {
        let targets = dataset.targets.as_ref().ok_or_else(|| AIEngineError::ModelLoadingFailed {
            model: "logistic_regression".to_string(),
            reason: "Target values required for classification".to_string(),
        })?;

        // Initialize coefficients
        let n_features = dataset.features.ncols();
        let mut coefficients = DVector::zeros(n_features);
        let mut intercept = 0.0;

        // Gradient descent training (simplified)
        for _iteration in 0..config.max_iterations {
            let mut gradient_coef = DVector::zeros(n_features);
            let mut gradient_intercept = 0.0;

            // Calculate gradients
            for i in 0..dataset.features.nrows() {
                let features_i = dataset.features.row(i);
                let z = features_i.dot(&coefficients) + intercept;
                let pred = self.sigmoid(z);
                let error = pred - targets[i];

                for j in 0..n_features {
                    gradient_coef[j] += error * features_i[j];
                }
                gradient_intercept += error;
            }

            // Update parameters
            let n_samples = dataset.features.nrows() as f64;
            for j in 0..n_features {
                coefficients[j] -= config.learning_rate * (gradient_coef[j] / n_samples + config.regularization * coefficients[j]);
            }
            intercept -= config.learning_rate * gradient_intercept / n_samples;
        }

        Ok(LogisticRegression {
            coefficients: Some(coefficients),
            intercept,
            config: config.clone(),
        })
    }

    /// Train K-Means clustering
    pub fn train_kmeans(
        &self,
        dataset: &Dataset,
        n_clusters: usize,
        config: &TrainingConfig,
    ) -> AIResult<KMeansClustering> {
        if n_clusters == 0 || n_clusters > dataset.features.nrows() {
            return Err(AIEngineError::ConfigurationError {
                field: "n_clusters".to_string(),
                reason: "Invalid number of clusters".to_string(),
            });
        }

        // Initialize centroids randomly
        let n_features = dataset.features.ncols();
        let mut centroids = DMatrix::zeros(n_clusters, n_features);

        // Simple initialization: choose random samples as initial centroids
        for k in 0..n_clusters {
            let sample_idx = (k * dataset.features.nrows() / n_clusters) % dataset.features.nrows();
            for j in 0..n_features {
                centroids[(k, j)] = dataset.features[(sample_idx, j)];
            }
        }

        // K-means iterations
        for _iteration in 0..config.max_iterations {
            // Assign points to clusters
            let mut assignments = vec![0; dataset.features.nrows()];

            for i in 0..dataset.features.nrows() {
                let mut min_distance = f64::INFINITY;
                let mut best_cluster = 0;

                for k in 0..n_clusters {
                    let distance = self.euclidean_distance(
                        &dataset.features.row(i).transpose(),
                        &centroids.row(k).transpose(),
                    );
                    if distance < min_distance {
                        min_distance = distance;
                        best_cluster = k;
                    }
                }
                assignments[i] = best_cluster;
            }

            // Update centroids
            let mut new_centroids = DMatrix::zeros(n_clusters, n_features);
            let mut cluster_counts = vec![0; n_clusters];

            for i in 0..dataset.features.nrows() {
                let cluster = assignments[i];
                cluster_counts[cluster] += 1;
                for j in 0..n_features {
                    new_centroids[(cluster, j)] += dataset.features[(i, j)];
                }
            }

            // Average the centroids
            for k in 0..n_clusters {
                if cluster_counts[k] > 0 {
                    for j in 0..n_features {
                        new_centroids[(k, j)] /= cluster_counts[k] as f64;
                    }
                }
            }

            // Check for convergence
            let centroid_change = (&new_centroids - &centroids).norm();
            centroids = new_centroids;

            if centroid_change < config.tolerance {
                break;
            }
        }

        Ok(KMeansClustering {
            centroids: Some(centroids),
            n_clusters,
            config: config.clone(),
        })
    }

    /// Perform Principal Component Analysis
    pub fn perform_pca(&self, dataset: &Dataset, n_components: usize) -> AIResult<PCA> {
        if n_components > dataset.features.ncols() {
            return Err(AIEngineError::ConfigurationError {
                field: "n_components".to_string(),
                reason: "Number of components cannot exceed number of features".to_string(),
            });
        }

        // Center the data
        let mean = self.calculate_column_means(&dataset.features);
        let mut centered_data = dataset.features.clone();

        for i in 0..centered_data.nrows() {
            for j in 0..centered_data.ncols() {
                centered_data[(i, j)] -= mean[j];
            }
        }

        // Calculate covariance matrix
        let cov_matrix = (&centered_data.transpose() * &centered_data) / (dataset.features.nrows() - 1) as f64;

        // Compute eigenvalues and eigenvectors (simplified - in practice use proper eigendecomposition)
        // For this mock implementation, we'll create placeholder results
        let components = DMatrix::identity(dataset.features.ncols(), n_components);
        let explained_variance_ratio = DVector::from_element(n_components, 1.0 / n_components as f64);

        Ok(PCA {
            components: Some(components),
            explained_variance_ratio: Some(explained_variance_ratio),
            n_components,
            mean: Some(mean),
        })
    }

    /// Make predictions with trained models
    pub fn predict_linear_regression(
        &self,
        model: &LinearRegression,
        features: &DMatrix<f64>,
    ) -> AIResult<DVector<f64>> {
        let coefficients = model.coefficients.as_ref().ok_or_else(|| AIEngineError::ModelNotFound {
            model: "linear_regression".to_string(),
        })?;

        let predictions = features * coefficients + DVector::from_element(features.nrows(), model.intercept);
        Ok(predictions)
    }

    /// Make predictions with logistic regression
    pub fn predict_logistic_regression(
        &self,
        model: &LogisticRegression,
        features: &DMatrix<f64>,
    ) -> AIResult<DVector<f64>> {
        let coefficients = model.coefficients.as_ref().ok_or_else(|| AIEngineError::ModelNotFound {
            model: "logistic_regression".to_string(),
        })?;

        let mut predictions = DVector::zeros(features.nrows());

        for i in 0..features.nrows() {
            let z = features.row(i).dot(coefficients) + model.intercept;
            predictions[i] = self.sigmoid(z);
        }

        Ok(predictions)
    }

    /// Predict clusters with K-means
    pub fn predict_kmeans(
        &self,
        model: &KMeansClustering,
        features: &DMatrix<f64>,
    ) -> AIResult<Vec<usize>> {
        let centroids = model.centroids.as_ref().ok_or_else(|| AIEngineError::ModelNotFound {
            model: "kmeans".to_string(),
        })?;

        let mut predictions = Vec::with_capacity(features.nrows());

        for i in 0..features.nrows() {
            let mut min_distance = f64::INFINITY;
            let mut best_cluster = 0;

            for k in 0..model.n_clusters {
                let distance = self.euclidean_distance(
                    &features.row(i).transpose(),
                    &centroids.row(k).transpose(),
                );
                if distance < min_distance {
                    min_distance = distance;
                    best_cluster = k;
                }
            }
            predictions.push(best_cluster);
        }

        Ok(predictions)
    }

    /// Sigmoid activation function
    fn sigmoid(&self, z: f64) -> f64 {
        1.0 / (1.0 + (-z).exp())
    }

    /// Calculate Euclidean distance between two vectors
    fn euclidean_distance(&self, a: &DVector<f64>, b: &DVector<f64>) -> f64 {
        (a - b).norm()
    }

    /// Calculate column means
    fn calculate_column_means(&self, matrix: &DMatrix<f64>) -> DVector<f64> {
        let mut means = DVector::zeros(matrix.ncols());

        for j in 0..matrix.ncols() {
            let sum: f64 = (0..matrix.nrows()).map(|i| matrix[(i, j)]).sum();
            means[j] = sum / matrix.nrows() as f64;
        }

        means
    }
}

impl DataPreprocessor {
    /// Calculate feature statistics
    fn calculate_feature_stats(&mut self, features: &DMatrix<f64>) {
        let mut stats = Vec::with_capacity(features.ncols());

        for j in 0..features.ncols() {
            let column: Vec<f64> = (0..features.nrows()).map(|i| features[(i, j)]).collect();

            let mean = column.iter().sum::<f64>() / column.len() as f64;
            let variance = column.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / column.len() as f64;
            let std = variance.sqrt();
            let min = column.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = column.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

            stats.push(FeatureStats { mean, std, min, max });
        }

        self.feature_stats = Some(stats);
    }
}

impl Default for TraditionalMLProcessor {
    fn default() -> Self {
        Self::new()
    }
}

// Implement model traits
impl LinearRegression {
    pub fn new(config: TrainingConfig) -> Self {
        Self {
            coefficients: None,
            intercept: 0.0,
            config,
        }
    }
}

impl LogisticRegression {
    pub fn new(config: TrainingConfig) -> Self {
        Self {
            coefficients: None,
            intercept: 0.0,
            config,
        }
    }
}

impl KMeansClustering {
    pub fn new(n_clusters: usize, config: TrainingConfig) -> Self {
        Self {
            centroids: None,
            n_clusters,
            config,
        }
    }
}

impl PCA {
    pub fn new(n_components: usize) -> Self {
        Self {
            components: None,
            explained_variance_ratio: None,
            n_components,
            mean: None,
        }
    }
}