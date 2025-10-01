//! Payment and Subscription Management
//!
//! Handles Stripe integration for Ectus-R subscriptions

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionPlan {
    Free,
    Pro,
    Enterprise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingInfo {
    pub plan: SubscriptionPlan,
    pub monthly_price: f64,
    pub annual_price: f64,
    pub features: Vec<String>,
    pub limits: SubscriptionLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionLimits {
    pub projects_per_month: Option<u32>,
    pub api_calls_per_month: Option<u32>,
    pub team_members: Option<u32>,
    pub storage_gb: Option<u32>,
}

impl PricingInfo {
    pub fn free() -> Self {
        Self {
            plan: SubscriptionPlan::Free,
            monthly_price: 0.0,
            annual_price: 0.0,
            features: vec![
                "10 projects per month".to_string(),
                "All core features".to_string(),
                "Community support".to_string(),
                "Public projects only".to_string(),
            ],
            limits: SubscriptionLimits {
                projects_per_month: Some(10),
                api_calls_per_month: Some(1000),
                team_members: Some(1),
                storage_gb: Some(1),
            },
        }
    }

    pub fn pro() -> Self {
        Self {
            plan: SubscriptionPlan::Pro,
            monthly_price: 49.0,
            annual_price: 490.0, // 2 months free
            features: vec![
                "Unlimited projects".to_string(),
                "Advanced AI models".to_string(),
                "Private repositories".to_string(),
                "Priority support".to_string(),
                "Custom templates".to_string(),
                "Team collaboration (5 members)".to_string(),
            ],
            limits: SubscriptionLimits {
                projects_per_month: None,
                api_calls_per_month: Some(100000),
                team_members: Some(5),
                storage_gb: Some(100),
            },
        }
    }

    pub fn enterprise() -> Self {
        Self {
            plan: SubscriptionPlan::Enterprise,
            monthly_price: 0.0, // Custom pricing
            annual_price: 0.0,
            features: vec![
                "Everything in Pro".to_string(),
                "On-premise deployment".to_string(),
                "Custom AI training".to_string(),
                "SLA guarantees (99.9%)".to_string(),
                "Dedicated support".to_string(),
                "SOC2/HIPAA compliance".to_string(),
                "Unlimited team members".to_string(),
            ],
            limits: SubscriptionLimits {
                projects_per_month: None,
                api_calls_per_month: None,
                team_members: None,
                storage_gb: None,
            },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CheckoutRequest {
    pub plan: SubscriptionPlan,
    pub billing_period: BillingPeriod,
    pub email: String,
    pub success_url: String,
    pub cancel_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingPeriod {
    Monthly,
    Annual,
}

#[derive(Debug, Serialize)]
pub struct CheckoutResponse {
    pub session_id: String,
    pub checkout_url: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhookEvent {
    pub event_type: String,
    pub data: serde_json::Value,
}

/// Get all pricing plans
pub async fn get_pricing_plans() -> impl IntoResponse {
    let plans = vec![
        PricingInfo::free(),
        PricingInfo::pro(),
        PricingInfo::enterprise(),
    ];

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "plans": plans
    })))
}

/// Create Stripe checkout session
pub async fn create_checkout_session(
    Json(request): Json<CheckoutRequest>,
) -> impl IntoResponse {
    // Get pricing for selected plan
    let pricing = match request.plan {
        SubscriptionPlan::Free => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "success": false,
                    "error": "Free plan does not require checkout"
                }))
            );
        }
        SubscriptionPlan::Pro => PricingInfo::pro(),
        SubscriptionPlan::Enterprise => {
            return (
                StatusCode::OK,
                Json(serde_json::json!({
                    "success": true,
                    "message": "Enterprise plan requires custom quote",
                    "contact_url": "/contact-sales"
                }))
            );
        }
    };

    // Calculate price based on billing period
    let amount = match request.billing_period {
        BillingPeriod::Monthly => pricing.monthly_price,
        BillingPeriod::Annual => pricing.annual_price,
    };

    // In production, integrate with Stripe API
    // For now, return mock checkout session
    let session_id = Uuid::new_v4().to_string();
    let checkout_url = format!(
        "https://checkout.stripe.com/pay/{}?plan={:?}&amount={}",
        session_id,
        request.plan,
        amount
    );

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "session_id": session_id,
        "checkout_url": checkout_url,
        "amount": amount,
        "currency": "USD"
    })))
}

/// Handle Stripe webhooks
pub async fn handle_stripe_webhook(
    Json(event): Json<WebhookEvent>,
) -> impl IntoResponse {
    tracing::info!("Received Stripe webhook: {}", event.event_type);

    match event.event_type.as_str() {
        "checkout.session.completed" => {
            // Handle successful payment
            handle_successful_payment(&event.data).await;
        }
        "customer.subscription.created" => {
            // Handle new subscription
            handle_subscription_created(&event.data).await;
        }
        "customer.subscription.deleted" => {
            // Handle subscription cancellation
            handle_subscription_cancelled(&event.data).await;
        }
        "invoice.payment_succeeded" => {
            // Handle recurring payment
            handle_payment_succeeded(&event.data).await;
        }
        "invoice.payment_failed" => {
            // Handle failed payment
            handle_payment_failed(&event.data).await;
        }
        _ => {
            tracing::warn!("Unhandled webhook event: {}", event.event_type);
        }
    }

    (StatusCode::OK, Json(serde_json::json!({
        "received": true
    })))
}

async fn handle_successful_payment(data: &serde_json::Value) {
    tracing::info!("Processing successful payment: {:?}", data);
    // TODO: Activate user subscription in database
    // TODO: Send confirmation email
    // TODO: Update user permissions
}

async fn handle_subscription_created(data: &serde_json::Value) {
    tracing::info!("New subscription created: {:?}", data);
    // TODO: Initialize user workspace
    // TODO: Send welcome email with onboarding
}

async fn handle_subscription_cancelled(data: &serde_json::Value) {
    tracing::info!("Subscription cancelled: {:?}", data);
    // TODO: Downgrade user to free tier
    // TODO: Send cancellation survey
}

async fn handle_payment_succeeded(data: &serde_json::Value) {
    tracing::info!("Recurring payment succeeded: {:?}", data);
    // TODO: Extend subscription period
    // TODO: Send payment receipt
}

async fn handle_payment_failed(data: &serde_json::Value) {
    tracing::warn!("Payment failed: {:?}", data);
    // TODO: Send payment failure notification
    // TODO: Retry payment or suspend account
}

/// Get user's subscription status
pub async fn get_subscription_status(
    user_id: Uuid,
) -> impl IntoResponse {
    // TODO: Query database for user subscription
    // For now, return mock data

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "subscription": {
            "plan": "Pro",
            "status": "active",
            "current_period_end": "2025-11-30T23:59:59Z",
            "cancel_at_period_end": false,
            "usage": {
                "projects_this_month": 45,
                "api_calls_this_month": 15420,
                "storage_used_gb": 12.5
            }
        }
    })))
}

/// Cancel subscription
pub async fn cancel_subscription(
    user_id: Uuid,
) -> impl IntoResponse {
    tracing::info!("Cancelling subscription for user: {}", user_id);

    // TODO: Call Stripe API to cancel subscription
    // TODO: Update database

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "message": "Subscription will be cancelled at the end of the billing period",
        "cancellation_date": "2025-11-30T23:59:59Z"
    })))
}

/// Update payment method
pub async fn update_payment_method(
    user_id: Uuid,
) -> impl IntoResponse {
    // Generate Stripe setup intent for updating payment method
    let setup_intent_id = Uuid::new_v4().to_string();

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "client_secret": format!("seti_secret_{}", setup_intent_id),
        "setup_url": format!("https://checkout.stripe.com/setup/{}", setup_intent_id)
    })))
}

/// Generate customer portal URL
pub async fn get_customer_portal(
    user_id: Uuid,
) -> impl IntoResponse {
    // In production, use Stripe Customer Portal
    let portal_url = format!(
        "https://billing.stripe.com/session/{}",
        Uuid::new_v4()
    );

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "portal_url": portal_url
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pricing_plans() {
        let free = PricingInfo::free();
        assert_eq!(free.monthly_price, 0.0);
        assert!(free.limits.projects_per_month.is_some());

        let pro = PricingInfo::pro();
        assert_eq!(pro.monthly_price, 49.0);
        assert_eq!(pro.annual_price, 490.0);

        let enterprise = PricingInfo::enterprise();
        assert!(enterprise.limits.projects_per_month.is_none());
    }

    #[test]
    fn test_annual_discount() {
        let pro = PricingInfo::pro();
        let monthly_annual = pro.monthly_price * 12.0;
        let discount = monthly_annual - pro.annual_price;

        // Should get ~2 months free (17% discount)
        assert!(discount > pro.monthly_price);
    }
}
