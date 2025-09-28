//! # Natural Language Processing
//!
//! NLP utilities and text processing functions.

use crate::errors::{AIEngineError, AIResult};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

/// Text preprocessing options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPreprocessingOptions {
    /// Convert to lowercase
    pub lowercase: bool,
    /// Remove punctuation
    pub remove_punctuation: bool,
    /// Remove extra whitespace
    pub normalize_whitespace: bool,
    /// Apply Unicode normalization
    pub unicode_normalize: bool,
    /// Remove stop words (language-specific)
    pub remove_stop_words: Option<String>,
    /// Minimum token length
    pub min_token_length: Option<usize>,
    /// Maximum token length
    pub max_token_length: Option<usize>,
}

impl Default for TextPreprocessingOptions {
    fn default() -> Self {
        Self {
            lowercase: true,
            remove_punctuation: false,
            normalize_whitespace: true,
            unicode_normalize: true,
            remove_stop_words: None,
            min_token_length: Some(1),
            max_token_length: Some(100),
        }
    }
}

/// Tokenization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizationResult {
    /// Original text
    pub original_text: String,
    /// Tokenized text
    pub tokens: Vec<String>,
    /// Token positions in original text
    pub token_positions: Vec<(usize, usize)>,
    /// Preprocessing applied
    pub preprocessing: TextPreprocessingOptions,
}

/// Text similarity result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    /// Similarity score (0.0 to 1.0)
    pub score: f32,
    /// Similarity method used
    pub method: String,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Named Entity Recognition result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NERResult {
    /// Detected entities
    pub entities: Vec<Entity>,
    /// Original text
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Entity text
    pub text: String,
    /// Entity type (PERSON, ORG, LOC, etc.)
    pub entity_type: String,
    /// Confidence score
    pub confidence: f32,
    /// Start position in text
    pub start: usize,
    /// End position in text
    pub end: usize,
}

/// Sentiment analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentResult {
    /// Overall sentiment (positive, negative, neutral)
    pub sentiment: String,
    /// Confidence score
    pub confidence: f32,
    /// Detailed scores
    pub scores: HashMap<String, f32>,
}

/// Language detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDetectionResult {
    /// Detected language code (ISO 639-1)
    pub language: String,
    /// Confidence score
    pub confidence: f32,
    /// Alternative language candidates
    pub alternatives: Vec<(String, f32)>,
}

/// NLP processor for text analysis and preprocessing
pub struct NLPProcessor {
    /// Compiled regex patterns for optimization
    punctuation_regex: Regex,
    whitespace_regex: Regex,
    /// Stop words by language
    stop_words: HashMap<String, std::collections::HashSet<String>>,
}

impl NLPProcessor {
    /// Create a new NLP processor
    pub fn new() -> Self {
        let punctuation_regex = Regex::new(r"[[:punct:]]+").unwrap();
        let whitespace_regex = Regex::new(r"\s+").unwrap();
        let mut stop_words = HashMap::new();

        // Add English stop words
        let english_stop_words: std::collections::HashSet<String> = vec![
            "a", "an", "and", "are", "as", "at", "be", "by", "for", "from", "has", "he", "in",
            "is", "it", "its", "of", "on", "that", "the", "to", "was", "will", "with",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        stop_words.insert("en".to_string(), english_stop_words);

        Self {
            punctuation_regex,
            whitespace_regex,
            stop_words,
        }
    }

    /// Preprocess text according to options
    pub fn preprocess_text(
        &self,
        text: &str,
        options: &TextPreprocessingOptions,
    ) -> AIResult<String> {
        let mut processed = text.to_string();

        // Unicode normalization
        if options.unicode_normalize {
            processed = processed.nfc().collect::<String>();
        }

        // Convert to lowercase
        if options.lowercase {
            processed = processed.to_lowercase();
        }

        // Remove punctuation
        if options.remove_punctuation {
            processed = self.punctuation_regex.replace_all(&processed, " ").to_string();
        }

        // Normalize whitespace
        if options.normalize_whitespace {
            processed = self.whitespace_regex.replace_all(&processed, " ").trim().to_string();
        }

        Ok(processed)
    }

    /// Tokenize text
    pub fn tokenize(
        &self,
        text: &str,
        options: &TextPreprocessingOptions,
    ) -> AIResult<TokenizationResult> {
        // Preprocess text
        let preprocessed = self.preprocess_text(text, options)?;

        // Simple whitespace tokenization
        let tokens: Vec<String> = preprocessed
            .split_whitespace()
            .filter_map(|token| {
                let token = token.trim();
                if token.is_empty() {
                    return None;
                }

                // Apply length filters
                if let Some(min_len) = options.min_token_length {
                    if token.len() < min_len {
                        return None;
                    }
                }

                if let Some(max_len) = options.max_token_length {
                    if token.len() > max_len {
                        return None;
                    }
                }

                // Remove stop words if specified
                if let Some(lang) = &options.remove_stop_words {
                    if let Some(stop_words) = self.stop_words.get(lang) {
                        if stop_words.contains(token) {
                            return None;
                        }
                    }
                }

                Some(token.to_string())
            })
            .collect();

        // Calculate token positions (simplified)
        let token_positions: Vec<(usize, usize)> = tokens
            .iter()
            .enumerate()
            .map(|(i, token)| {
                // This is a simplified position calculation
                let start = i * (token.len() + 1);
                let end = start + token.len();
                (start, end)
            })
            .collect();

        Ok(TokenizationResult {
            original_text: text.to_string(),
            tokens,
            token_positions,
            preprocessing: options.clone(),
        })
    }

    /// Calculate text similarity using Jaccard similarity
    pub fn calculate_similarity(
        &self,
        text1: &str,
        text2: &str,
        options: &TextPreprocessingOptions,
    ) -> AIResult<SimilarityResult> {
        let tokens1 = self.tokenize(text1, options)?;
        let tokens2 = self.tokenize(text2, options)?;

        let set1: std::collections::HashSet<_> = tokens1.tokens.into_iter().collect();
        let set2: std::collections::HashSet<_> = tokens2.tokens.into_iter().collect();

        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();

        let jaccard_score = if union > 0 {
            intersection as f32 / union as f32
        } else {
            0.0
        };

        let mut metadata = HashMap::new();
        metadata.insert("intersection_size".to_string(), serde_json::Value::from(intersection));
        metadata.insert("union_size".to_string(), serde_json::Value::from(union));

        Ok(SimilarityResult {
            score: jaccard_score,
            method: "jaccard".to_string(),
            metadata,
        })
    }

    /// Perform Named Entity Recognition (mock implementation)
    pub fn extract_entities(&self, text: &str) -> AIResult<NERResult> {
        let mut entities = Vec::new();

        // Simple pattern-based NER (mock implementation)
        // In a real implementation, this would use a trained NER model

        // Find email addresses
        let email_regex = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
        for mat in email_regex.find_iter(text) {
            entities.push(Entity {
                text: mat.as_str().to_string(),
                entity_type: "EMAIL".to_string(),
                confidence: 0.95,
                start: mat.start(),
                end: mat.end(),
            });
        }

        // Find URLs
        let url_regex = Regex::new(r"https?://[^\s]+").unwrap();
        for mat in url_regex.find_iter(text) {
            entities.push(Entity {
                text: mat.as_str().to_string(),
                entity_type: "URL".to_string(),
                confidence: 0.9,
                start: mat.start(),
                end: mat.end(),
            });
        }

        // Find potential person names (capitalized words)
        let name_regex = Regex::new(r"\b[A-Z][a-z]+ [A-Z][a-z]+\b").unwrap();
        for mat in name_regex.find_iter(text) {
            entities.push(Entity {
                text: mat.as_str().to_string(),
                entity_type: "PERSON".to_string(),
                confidence: 0.7,
                start: mat.start(),
                end: mat.end(),
            });
        }

        Ok(NERResult {
            entities,
            text: text.to_string(),
        })
    }

    /// Analyze sentiment (mock implementation)
    pub fn analyze_sentiment(&self, text: &str) -> AIResult<SentimentResult> {
        // Simple rule-based sentiment analysis (mock implementation)
        let positive_words = ["good", "great", "excellent", "amazing", "wonderful", "fantastic"];
        let negative_words = ["bad", "terrible", "awful", "horrible", "worst", "hate"];

        let preprocessed = self.preprocess_text(text, &TextPreprocessingOptions::default())?;
        let words: Vec<&str> = preprocessed.split_whitespace().collect();

        let mut positive_count = 0;
        let mut negative_count = 0;

        for word in &words {
            if positive_words.contains(word) {
                positive_count += 1;
            } else if negative_words.contains(word) {
                negative_count += 1;
            }
        }

        let total_sentiment_words = positive_count + negative_count;
        let sentiment = if total_sentiment_words == 0 {
            "neutral".to_string()
        } else if positive_count > negative_count {
            "positive".to_string()
        } else if negative_count > positive_count {
            "negative".to_string()
        } else {
            "neutral".to_string()
        };

        let confidence = if total_sentiment_words == 0 {
            0.5
        } else {
            let max_count = std::cmp::max(positive_count, negative_count);
            max_count as f32 / words.len() as f32
        };

        let mut scores = HashMap::new();
        scores.insert("positive".to_string(), positive_count as f32 / words.len() as f32);
        scores.insert("negative".to_string(), negative_count as f32 / words.len() as f32);
        scores.insert("neutral".to_string(), 1.0 - scores["positive"] - scores["negative"]);

        Ok(SentimentResult {
            sentiment,
            confidence,
            scores,
        })
    }

    /// Detect language (mock implementation)
    pub fn detect_language(&self, text: &str) -> AIResult<LanguageDetectionResult> {
        // Simple character-based language detection (mock implementation)

        let char_counts: HashMap<char, usize> = text.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        // Simple heuristics for common languages
        let english_score = char_counts.get(&'e').unwrap_or(&0) +
                           char_counts.get(&'t').unwrap_or(&0) +
                           char_counts.get(&'a').unwrap_or(&0);

        let spanish_score = char_counts.get(&'a').unwrap_or(&0) +
                           char_counts.get(&'e').unwrap_or(&0) +
                           char_counts.get(&'í').unwrap_or(&0) +
                           char_counts.get(&'ó').unwrap_or(&0);

        let french_score = char_counts.get(&'e').unwrap_or(&0) +
                          char_counts.get(&'é').unwrap_or(&0) +
                          char_counts.get(&'è').unwrap_or(&0) +
                          char_counts.get(&'ç').unwrap_or(&0);

        let mut scores = vec![
            ("en".to_string(), english_score as f32),
            ("es".to_string(), spanish_score as f32),
            ("fr".to_string(), french_score as f32),
        ];

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let total_score: f32 = scores.iter().map(|(_, score)| score).sum();
        let normalized_scores: Vec<(String, f32)> = scores
            .into_iter()
            .map(|(lang, score)| (lang, if total_score > 0.0 { score / total_score } else { 0.0 }))
            .collect();

        let (primary_lang, confidence) = normalized_scores[0].clone();
        let alternatives = normalized_scores[1..].to_vec();

        Ok(LanguageDetectionResult {
            language: primary_lang,
            confidence,
            alternatives,
        })
    }

    /// Extract keywords from text
    pub fn extract_keywords(
        &self,
        text: &str,
        max_keywords: usize,
    ) -> AIResult<Vec<(String, f32)>> {
        let options = TextPreprocessingOptions {
            remove_stop_words: Some("en".to_string()),
            min_token_length: Some(3),
            ..Default::default()
        };

        let tokenization = self.tokenize(text, &options)?;

        // Count token frequencies
        let mut word_counts: HashMap<String, usize> = HashMap::new();
        for token in &tokenization.tokens {
            *word_counts.entry(token.clone()).or_insert(0) += 1;
        }

        // Sort by frequency and take top keywords
        let mut keywords: Vec<(String, f32)> = word_counts
            .into_iter()
            .map(|(word, count)| (word, count as f32))
            .collect();

        keywords.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        keywords.truncate(max_keywords);

        // Normalize scores
        if let Some(max_score) = keywords.first().map(|(_, score)| *score) {
            if max_score > 0.0 {
                for (_, score) in &mut keywords {
                    *score /= max_score;
                }
            }
        }

        Ok(keywords)
    }
}

impl Default for NLPProcessor {
    fn default() -> Self {
        Self::new()
    }
}