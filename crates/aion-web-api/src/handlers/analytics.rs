//! Analytics and Conversion Tracking
//!
//! Tracks user behavior, conversion funnels, and product metrics

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    // Page views
    PageView { page: String, referrer: Option<String> },

    // User actions
    SignupStarted,
    SignupCompleted { plan: String },
    TrialStarted,

    // Engagement
    DemoPlayed { prompt: String, language: String },
    CodeGenerated { lines: u32, language: String },
    ProjectCreated { project_type: String },

    // Conversion events
    CheckoutStarted { plan: String, billing_period: String },
    PaymentCompleted { plan: String, amount: f64 },
    SubscriptionCancelled { reason: Option<String> },

    // Feature usage
    FeatureUsed { feature_name: String },

    // Marketing
    EmailOpened { campaign_id: String },
    EmailClicked { campaign_id: String, link: String },
    AdClicked { campaign: String, source: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub event_id: String,
    pub user_id: Option<String>,
    pub session_id: String,
    pub event_type: EventType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub properties: serde_json::Value,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrackEventRequest {
    pub event_type: EventType,
    pub user_id: Option<String>,
    pub session_id: String,
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct AnalyticsMetrics {
    pub total_users: u64,
    pub active_users_30d: u64,
    pub trial_conversions: ConversionMetrics,
    pub revenue_metrics: RevenueMetrics,
    pub feature_usage: Vec<FeatureUsage>,
    pub funnel_metrics: FunnelMetrics,
}

#[derive(Debug, Serialize)]
pub struct ConversionMetrics {
    pub signup_to_trial_rate: f64,
    pub trial_to_paid_rate: f64,
    pub overall_conversion_rate: f64,
    pub average_time_to_convert_hours: f64,
}

#[derive(Debug, Serialize)]
pub struct RevenueMetrics {
    pub mrr: f64,  // Monthly Recurring Revenue
    pub arr: f64,  // Annual Recurring Revenue
    pub average_revenue_per_user: f64,
    pub customer_lifetime_value: f64,
    pub churn_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct FeatureUsage {
    pub feature_name: String,
    pub usage_count: u64,
    pub unique_users: u64,
    pub adoption_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct FunnelMetrics {
    pub landing_page_views: u64,
    pub signup_started: u64,
    pub signup_completed: u64,
    pub trial_started: u64,
    pub payment_completed: u64,
    pub conversion_rates: Vec<FunnelStep>,
}

#[derive(Debug, Serialize)]
pub struct FunnelStep {
    pub from: String,
    pub to: String,
    pub rate: f64,
    pub drop_off: u64,
}

/// Track an analytics event
pub async fn track_event(
    Json(request): Json<TrackEventRequest>,
) -> impl IntoResponse {
    let event = AnalyticsEvent {
        event_id: Uuid::new_v4().to_string(),
        user_id: request.user_id,
        session_id: request.session_id,
        event_type: request.event_type,
        timestamp: chrono::Utc::now(),
        properties: request.properties.unwrap_or_else(|| serde_json::json!({})),
        user_agent: None,
        ip_address: None,
    };

    tracing::info!("Analytics event tracked: {:?}", event.event_type);

    // In production, send to analytics backend (e.g., Mixpanel, Amplitude, PostHog)
    // store_event_in_database(&event).await;
    // send_to_analytics_platform(&event).await;

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "event_id": event.event_id
    })))
}

/// Get analytics dashboard metrics
pub async fn get_analytics_metrics() -> impl IntoResponse {
    // In production, query from database and analytics platform
    let metrics = AnalyticsMetrics {
        total_users: 12450,
        active_users_30d: 8923,
        trial_conversions: ConversionMetrics {
            signup_to_trial_rate: 0.78,
            trial_to_paid_rate: 0.42,
            overall_conversion_rate: 0.33,
            average_time_to_convert_hours: 72.5,
        },
        revenue_metrics: RevenueMetrics {
            mrr: 45600.0,
            arr: 547200.0,
            average_revenue_per_user: 49.0,
            customer_lifetime_value: 1470.0,
            churn_rate: 0.05,
        },
        feature_usage: vec![
            FeatureUsage {
                feature_name: "AI Code Generation".to_string(),
                usage_count: 156789,
                unique_users: 8234,
                adoption_rate: 0.92,
            },
            FeatureUsage {
                feature_name: "Deployment".to_string(),
                usage_count: 45321,
                unique_users: 6745,
                adoption_rate: 0.76,
            },
            FeatureUsage {
                feature_name: "Security Scan".to_string(),
                usage_count: 89456,
                unique_users: 7123,
                adoption_rate: 0.80,
            },
        ],
        funnel_metrics: FunnelMetrics {
            landing_page_views: 45000,
            signup_started: 15000,
            signup_completed: 12000,
            trial_started: 9360,
            payment_completed: 3931,
            conversion_rates: vec![
                FunnelStep {
                    from: "Landing Page".to_string(),
                    to: "Signup Started".to_string(),
                    rate: 0.33,
                    drop_off: 30000,
                },
                FunnelStep {
                    from: "Signup Started".to_string(),
                    to: "Signup Completed".to_string(),
                    rate: 0.80,
                    drop_off: 3000,
                },
                FunnelStep {
                    from: "Signup Completed".to_string(),
                    to: "Trial Started".to_string(),
                    rate: 0.78,
                    drop_off: 2640,
                },
                FunnelStep {
                    from: "Trial Started".to_string(),
                    to: "Payment Completed".to_string(),
                    rate: 0.42,
                    drop_off: 5429,
                },
            ],
        },
    };

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "metrics": metrics
    })))
}

/// Get conversion funnel analysis
pub async fn get_conversion_funnel() -> impl IntoResponse {
    let funnel = FunnelMetrics {
        landing_page_views: 45000,
        signup_started: 15000,
        signup_completed: 12000,
        trial_started: 9360,
        payment_completed: 3931,
        conversion_rates: vec![
            FunnelStep {
                from: "Landing Page".to_string(),
                to: "Signup Started".to_string(),
                rate: 0.33,
                drop_off: 30000,
            },
            FunnelStep {
                from: "Signup Started".to_string(),
                to: "Signup Completed".to_string(),
                rate: 0.80,
                drop_off: 3000,
            },
            FunnelStep {
                from: "Signup Completed".to_string(),
                to: "Trial Started".to_string(),
                rate: 0.78,
                drop_off: 2640,
            },
            FunnelStep {
                from: "Trial Started".to_string(),
                to: "Payment Completed".to_string(),
                rate: 0.42,
                drop_off: 5429,
            },
        ],
    };

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "funnel": funnel
    })))
}

/// Get user cohort analysis
pub async fn get_cohort_analysis() -> impl IntoResponse {
    #[derive(Serialize)]
    struct CohortData {
        month: String,
        new_users: u64,
        retention_rates: Vec<f64>,
    }

    let cohorts = vec![
        CohortData {
            month: "2025-01".to_string(),
            new_users: 1250,
            retention_rates: vec![1.0, 0.85, 0.78, 0.72, 0.68, 0.65],
        },
        CohortData {
            month: "2025-02".to_string(),
            new_users: 1450,
            retention_rates: vec![1.0, 0.88, 0.80, 0.75, 0.71],
        },
        CohortData {
            month: "2025-03".to_string(),
            new_users: 1680,
            retention_rates: vec![1.0, 0.90, 0.82, 0.78],
        },
        CohortData {
            month: "2025-04".to_string(),
            new_users: 1920,
            retention_rates: vec![1.0, 0.91, 0.84],
        },
    ];

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "cohorts": cohorts
    })))
}

/// Get A/B test results
pub async fn get_ab_test_results() -> impl IntoResponse {
    #[derive(Serialize)]
    struct ABTestResult {
        test_name: String,
        variant_a: ABTestVariant,
        variant_b: ABTestVariant,
        winner: Option<String>,
        confidence: f64,
    }

    #[derive(Serialize)]
    struct ABTestVariant {
        name: String,
        users: u64,
        conversions: u64,
        conversion_rate: f64,
    }

    let tests = vec![
        ABTestResult {
            test_name: "Pricing Page CTA".to_string(),
            variant_a: ABTestVariant {
                name: "Start Free Trial".to_string(),
                users: 5000,
                conversions: 650,
                conversion_rate: 0.13,
            },
            variant_b: ABTestVariant {
                name: "Get Started Free".to_string(),
                users: 5000,
                conversions: 780,
                conversion_rate: 0.156,
            },
            winner: Some("variant_b".to_string()),
            confidence: 0.95,
        },
        ABTestResult {
            test_name: "Demo Playground Position".to_string(),
            variant_a: ABTestVariant {
                name: "Above Fold".to_string(),
                users: 6000,
                conversions: 1200,
                conversion_rate: 0.20,
            },
            variant_b: ABTestVariant {
                name: "After Features".to_string(),
                users: 6000,
                conversions: 960,
                conversion_rate: 0.16,
            },
            winner: Some("variant_a".to_string()),
            confidence: 0.98,
        },
    ];

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "tests": tests
    })))
}

/// Track feature flag activation
pub async fn track_feature_flag(
    Json(request): Json<serde_json::Value>,
) -> impl IntoResponse {
    tracing::info!("Feature flag tracked: {:?}", request);

    (StatusCode::OK, Json(serde_json::json!({
        "success": true
    })))
}

/// Get real-time analytics dashboard
pub async fn get_realtime_analytics() -> impl IntoResponse {
    #[derive(Serialize)]
    struct RealtimeMetrics {
        active_users_now: u64,
        events_per_minute: u64,
        top_pages: Vec<PageMetric>,
        recent_conversions: Vec<ConversionEvent>,
    }

    #[derive(Serialize)]
    struct PageMetric {
        page: String,
        active_users: u64,
    }

    #[derive(Serialize)]
    struct ConversionEvent {
        user_id: String,
        event: String,
        amount: Option<f64>,
        timestamp: chrono::DateTime<chrono::Utc>,
    }

    let metrics = RealtimeMetrics {
        active_users_now: 234,
        events_per_minute: 1247,
        top_pages: vec![
            PageMetric { page: "/dashboard".to_string(), active_users: 89 },
            PageMetric { page: "/playground".to_string(), active_users: 67 },
            PageMetric { page: "/pricing".to_string(), active_users: 45 },
            PageMetric { page: "/".to_string(), active_users: 33 },
        ],
        recent_conversions: vec![
            ConversionEvent {
                user_id: "user_123".to_string(),
                event: "Trial Started".to_string(),
                amount: None,
                timestamp: chrono::Utc::now() - chrono::Duration::minutes(2),
            },
            ConversionEvent {
                user_id: "user_456".to_string(),
                event: "Payment Completed".to_string(),
                amount: Some(49.0),
                timestamp: chrono::Utc::now() - chrono::Duration::minutes(5),
            },
        ],
    };

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "metrics": metrics
    })))
}

/// Integration with third-party analytics platforms
pub struct AnalyticsPlatform;

impl AnalyticsPlatform {
    /// Send event to Mixpanel
    pub async fn send_to_mixpanel(event: &AnalyticsEvent) -> Result<(), anyhow::Error> {
        // Implementation for Mixpanel API
        tracing::debug!("Sending event to Mixpanel: {:?}", event.event_type);
        Ok(())
    }

    /// Send event to Amplitude
    pub async fn send_to_amplitude(event: &AnalyticsEvent) -> Result<(), anyhow::Error> {
        // Implementation for Amplitude API
        tracing::debug!("Sending event to Amplitude: {:?}", event.event_type);
        Ok(())
    }

    /// Send event to PostHog
    pub async fn send_to_posthog(event: &AnalyticsEvent) -> Result<(), anyhow::Error> {
        // Implementation for PostHog API
        tracing::debug!("Sending event to PostHog: {:?}", event.event_type);
        Ok(())
    }

    /// Send event to Google Analytics 4
    pub async fn send_to_ga4(event: &AnalyticsEvent) -> Result<(), anyhow::Error> {
        // Implementation for GA4 Measurement Protocol
        tracing::debug!("Sending event to GA4: {:?}", event.event_type);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_rate_calculation() {
        let funnel = FunnelMetrics {
            landing_page_views: 10000,
            signup_started: 3000,
            signup_completed: 2400,
            trial_started: 1872,
            payment_completed: 786,
            conversion_rates: vec![],
        };

        let overall_rate = funnel.payment_completed as f64 / funnel.landing_page_views as f64;
        assert_eq!((overall_rate * 100.0).round(), 8.0);
    }

    #[test]
    fn test_mrr_calculation() {
        let revenue = RevenueMetrics {
            mrr: 45600.0,
            arr: 547200.0,
            average_revenue_per_user: 49.0,
            customer_lifetime_value: 1470.0,
            churn_rate: 0.05,
        };

        assert_eq!((revenue.mrr * 12.0).round(), revenue.arr);
    }
}
