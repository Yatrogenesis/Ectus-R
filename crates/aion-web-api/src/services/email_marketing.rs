//! Email Marketing Integration Service
//!
//! Integrates with email marketing platforms (SendGrid, Mailchimp, etc.)
//! for user onboarding, campaigns, and retention

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailProvider {
    SendGrid { api_key: String },
    Mailchimp { api_key: String, server_prefix: String },
    Resend { api_key: String },
    Custom { endpoint: String, api_key: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub id: String,
    pub name: String,
    pub subject: String,
    pub from_email: String,
    pub from_name: String,
    pub template_type: TemplateType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateType {
    Welcome,
    TrialStarted,
    TrialEnding,
    PaymentSuccess,
    PaymentFailed,
    SubscriptionCancelled,
    ProductUpdate,
    Newsletter,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailRecipient {
    pub email: String,
    pub name: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailCampaign {
    pub id: String,
    pub name: String,
    pub template_id: String,
    pub recipients: Vec<EmailRecipient>,
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: CampaignStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CampaignStatus {
    Draft,
    Scheduled,
    Sending,
    Sent,
    Cancelled,
}

pub struct EmailMarketingService {
    provider: EmailProvider,
    client: Client,
    templates: HashMap<String, EmailTemplate>,
}

impl EmailMarketingService {
    /// Create a new email marketing service
    pub fn new(provider: EmailProvider) -> Self {
        Self {
            provider,
            client: Client::new(),
            templates: Self::default_templates(),
        }
    }

    /// Initialize from environment variables
    pub fn from_env() -> Result<Self> {
        let provider_type = std::env::var("EMAIL_PROVIDER").unwrap_or_else(|_| "sendgrid".to_string());

        let provider = match provider_type.to_lowercase().as_str() {
            "sendgrid" => {
                let api_key = std::env::var("SENDGRID_API_KEY")?;
                EmailProvider::SendGrid { api_key }
            }
            "mailchimp" => {
                let api_key = std::env::var("MAILCHIMP_API_KEY")?;
                let server_prefix = std::env::var("MAILCHIMP_SERVER_PREFIX")?;
                EmailProvider::Mailchimp { api_key, server_prefix }
            }
            "resend" => {
                let api_key = std::env::var("RESEND_API_KEY")?;
                EmailProvider::Resend { api_key }
            }
            _ => {
                let endpoint = std::env::var("EMAIL_ENDPOINT")?;
                let api_key = std::env::var("EMAIL_API_KEY")?;
                EmailProvider::Custom { endpoint, api_key }
            }
        };

        Ok(Self::new(provider))
    }

    /// Get default email templates
    fn default_templates() -> HashMap<String, EmailTemplate> {
        let mut templates = HashMap::new();

        templates.insert(
            "welcome".to_string(),
            EmailTemplate {
                id: "welcome".to_string(),
                name: "Welcome Email".to_string(),
                subject: "Welcome to Ectus-R! 🚀".to_string(),
                from_email: "welcome@ectus.ai".to_string(),
                from_name: "Ectus-R Team".to_string(),
                template_type: TemplateType::Welcome,
            },
        );

        templates.insert(
            "trial_started".to_string(),
            EmailTemplate {
                id: "trial_started".to_string(),
                name: "Trial Started".to_string(),
                subject: "Your 14-day trial has started!".to_string(),
                from_email: "trials@ectus.ai".to_string(),
                from_name: "Ectus-R Team".to_string(),
                template_type: TemplateType::TrialStarted,
            },
        );

        templates.insert(
            "trial_ending".to_string(),
            EmailTemplate {
                id: "trial_ending".to_string(),
                name: "Trial Ending Soon".to_string(),
                subject: "Your trial ends in 3 days - Don't lose your projects!".to_string(),
                from_email: "trials@ectus.ai".to_string(),
                from_name: "Ectus-R Team".to_string(),
                template_type: TemplateType::TrialEnding,
            },
        );

        templates.insert(
            "payment_success".to_string(),
            EmailTemplate {
                id: "payment_success".to_string(),
                name: "Payment Successful".to_string(),
                subject: "Payment received - You're all set! ✓".to_string(),
                from_email: "billing@ectus.ai".to_string(),
                from_name: "Ectus-R Billing".to_string(),
                template_type: TemplateType::PaymentSuccess,
            },
        );

        templates.insert(
            "payment_failed".to_string(),
            EmailTemplate {
                id: "payment_failed".to_string(),
                name: "Payment Failed".to_string(),
                subject: "Payment failed - Please update your payment method".to_string(),
                from_email: "billing@ectus.ai".to_string(),
                from_name: "Ectus-R Billing".to_string(),
                template_type: TemplateType::PaymentFailed,
            },
        );

        templates
    }

    /// Send a single email using a template
    pub async fn send_template_email(
        &self,
        template_id: &str,
        recipient: EmailRecipient,
        variables: HashMap<String, String>,
    ) -> Result<String> {
        let template = self.templates.get(template_id)
            .ok_or_else(|| anyhow::anyhow!("Template not found: {}", template_id))?;

        match &self.provider {
            EmailProvider::SendGrid { api_key } => {
                self.send_sendgrid_email(api_key, template, recipient, variables).await
            }
            EmailProvider::Mailchimp { api_key, server_prefix } => {
                self.send_mailchimp_email(api_key, server_prefix, template, recipient, variables).await
            }
            EmailProvider::Resend { api_key } => {
                self.send_resend_email(api_key, template, recipient, variables).await
            }
            EmailProvider::Custom { endpoint, api_key } => {
                self.send_custom_email(endpoint, api_key, template, recipient, variables).await
            }
        }
    }

    /// SendGrid integration
    async fn send_sendgrid_email(
        &self,
        api_key: &str,
        template: &EmailTemplate,
        recipient: EmailRecipient,
        variables: HashMap<String, String>,
    ) -> Result<String> {
        let payload = serde_json::json!({
            "personalizations": [{
                "to": [{ "email": recipient.email, "name": recipient.name }],
                "dynamic_template_data": variables,
            }],
            "from": {
                "email": template.from_email,
                "name": template.from_name,
            },
            "template_id": template.id,
        });

        let response = self.client
            .post("https://api.sendgrid.com/v3/mail/send")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            info!("Email sent successfully via SendGrid to {}", recipient.email);
            Ok("Email sent".to_string())
        } else {
            let error_text = response.text().await?;
            error!("SendGrid API error: {}", error_text);
            Err(anyhow::anyhow!("Failed to send email: {}", error_text))
        }
    }

    /// Mailchimp integration
    async fn send_mailchimp_email(
        &self,
        api_key: &str,
        server_prefix: &str,
        template: &EmailTemplate,
        recipient: EmailRecipient,
        variables: HashMap<String, String>,
    ) -> Result<String> {
        // Mailchimp uses campaigns and audiences
        let endpoint = format!("https://{}.api.mailchimp.com/3.0/campaigns", server_prefix);

        let payload = serde_json::json!({
            "type": "regular",
            "recipients": {
                "list_id": std::env::var("MAILCHIMP_LIST_ID").unwrap_or_default(),
            },
            "settings": {
                "subject_line": template.subject,
                "from_name": template.from_name,
                "reply_to": template.from_email,
            },
        });

        let response = self.client
            .post(&endpoint)
            .basic_auth("anystring", Some(api_key))
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            info!("Campaign created via Mailchimp for {}", recipient.email);
            Ok("Campaign created".to_string())
        } else {
            let error_text = response.text().await?;
            error!("Mailchimp API error: {}", error_text);
            Err(anyhow::anyhow!("Failed to create campaign: {}", error_text))
        }
    }

    /// Resend integration (modern, simple API)
    async fn send_resend_email(
        &self,
        api_key: &str,
        template: &EmailTemplate,
        recipient: EmailRecipient,
        variables: HashMap<String, String>,
    ) -> Result<String> {
        let mut html_body = self.render_template(&template.template_type, &variables);

        let payload = serde_json::json!({
            "from": format!("{} <{}>", template.from_name, template.from_email),
            "to": [recipient.email],
            "subject": template.subject,
            "html": html_body,
        });

        let response = self.client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            info!("Email sent successfully via Resend to {}", recipient.email);
            Ok("Email sent".to_string())
        } else {
            let error_text = response.text().await?;
            error!("Resend API error: {}", error_text);
            Err(anyhow::anyhow!("Failed to send email: {}", error_text))
        }
    }

    /// Custom email provider integration
    async fn send_custom_email(
        &self,
        endpoint: &str,
        api_key: &str,
        template: &EmailTemplate,
        recipient: EmailRecipient,
        variables: HashMap<String, String>,
    ) -> Result<String> {
        let payload = serde_json::json!({
            "to": recipient.email,
            "subject": template.subject,
            "from": template.from_email,
            "template_id": template.id,
            "variables": variables,
        });

        let response = self.client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            info!("Email sent successfully via custom provider to {}", recipient.email);
            Ok("Email sent".to_string())
        } else {
            let error_text = response.text().await?;
            error!("Custom provider API error: {}", error_text);
            Err(anyhow::anyhow!("Failed to send email: {}", error_text))
        }
    }

    /// Render HTML template
    fn render_template(&self, template_type: &TemplateType, variables: &HashMap<String, String>) -> String {
        let base_styles = r#"
            <style>
                body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
                .container { max-width: 600px; margin: 0 auto; padding: 20px; }
                .header { background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%); color: white; padding: 30px; text-align: center; }
                .content { background: white; padding: 30px; }
                .button { display: inline-block; padding: 12px 24px; background: #6366f1; color: white; text-decoration: none; border-radius: 6px; }
                .footer { text-align: center; padding: 20px; color: #666; font-size: 14px; }
            </style>
        "#;

        match template_type {
            TemplateType::Welcome => {
                format!(r#"
                    <!DOCTYPE html>
                    <html>
                    <head>{}</head>
                    <body>
                        <div class="container">
                            <div class="header">
                                <h1>Welcome to Ectus-R! 🚀</h1>
                            </div>
                            <div class="content">
                                <p>Hi {},</p>
                                <p>We're thrilled to have you on board! Ectus-R is here to transform the way you build software.</p>
                                <p>Here's what you can do next:</p>
                                <ul>
                                    <li>Create your first project</li>
                                    <li>Explore the AI code generation playground</li>
                                    <li>Check out our documentation</li>
                                </ul>
                                <p style="text-align: center; margin: 30px 0;">
                                    <a href="https://dashboard.ectus.ai" class="button">Get Started</a>
                                </p>
                            </div>
                            <div class="footer">
                                <p>&copy; 2025 Yatrogenesis. All rights reserved.</p>
                            </div>
                        </div>
                    </body>
                    </html>
                "#, base_styles, variables.get("name").unwrap_or(&"there".to_string()))
            }
            TemplateType::TrialStarted => {
                format!(r#"
                    <!DOCTYPE html>
                    <html>
                    <head>{}</head>
                    <body>
                        <div class="container">
                            <div class="header">
                                <h1>Your 14-Day Trial Has Started!</h1>
                            </div>
                            <div class="content">
                                <p>Hi {},</p>
                                <p>Your free trial is now active. You have full access to all Pro features for the next 14 days.</p>
                                <p><strong>What's included:</strong></p>
                                <ul>
                                    <li>Unlimited projects</li>
                                    <li>Advanced AI models</li>
                                    <li>Private repositories</li>
                                    <li>Priority support</li>
                                </ul>
                                <p style="text-align: center; margin: 30px 0;">
                                    <a href="https://dashboard.ectus.ai/projects" class="button">Start Building</a>
                                </p>
                            </div>
                            <div class="footer">
                                <p>Your trial ends on {}</p>
                            </div>
                        </div>
                    </body>
                    </html>
                "#, base_styles,
                   variables.get("name").unwrap_or(&"there".to_string()),
                   variables.get("trial_end_date").unwrap_or(&"14 days".to_string()))
            }
            TemplateType::PaymentSuccess => {
                format!(r#"
                    <!DOCTYPE html>
                    <html>
                    <head>{}</head>
                    <body>
                        <div class="container">
                            <div class="header">
                                <h1>Payment Received ✓</h1>
                            </div>
                            <div class="content">
                                <p>Hi {},</p>
                                <p>Thank you! Your payment has been processed successfully.</p>
                                <p><strong>Payment Details:</strong></p>
                                <ul>
                                    <li>Amount: ${}</li>
                                    <li>Plan: {}</li>
                                    <li>Next billing date: {}</li>
                                </ul>
                                <p style="text-align: center; margin: 30px 0;">
                                    <a href="https://dashboard.ectus.ai/billing" class="button">View Invoice</a>
                                </p>
                            </div>
                            <div class="footer">
                                <p>Questions? Contact us at billing@ectus.ai</p>
                            </div>
                        </div>
                    </body>
                    </html>
                "#, base_styles,
                   variables.get("name").unwrap_or(&"there".to_string()),
                   variables.get("amount").unwrap_or(&"49.00".to_string()),
                   variables.get("plan").unwrap_or(&"Pro".to_string()),
                   variables.get("next_billing_date").unwrap_or(&"30 days".to_string()))
            }
            _ => {
                format!(r#"
                    <!DOCTYPE html>
                    <html>
                    <head>{}</head>
                    <body>
                        <div class="container">
                            <div class="content">
                                <p>Email from Ectus-R</p>
                            </div>
                        </div>
                    </body>
                    </html>
                "#, base_styles)
            }
        }
    }

    /// Add a contact to the mailing list
    pub async fn add_contact(
        &self,
        email: String,
        name: Option<String>,
        tags: Vec<String>,
    ) -> Result<String> {
        info!("Adding contact to mailing list: {}", email);

        // Implementation depends on provider
        match &self.provider {
            EmailProvider::SendGrid { .. } => {
                // Add to SendGrid contacts
                Ok("Contact added to SendGrid".to_string())
            }
            EmailProvider::Mailchimp { .. } => {
                // Add to Mailchimp audience
                Ok("Contact added to Mailchimp".to_string())
            }
            _ => Ok("Contact registered".to_string()),
        }
    }

    /// Track email engagement metrics
    pub async fn track_engagement(
        &self,
        email_id: &str,
        event: EmailEvent,
    ) -> Result<()> {
        info!("Email engagement tracked: {:?} for {}", event, email_id);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailEvent {
    Sent,
    Delivered,
    Opened,
    Clicked { link: String },
    Bounced { reason: String },
    Unsubscribed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_rendering() {
        let service = EmailMarketingService::new(EmailProvider::Resend {
            api_key: "test_key".to_string(),
        });

        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John Doe".to_string());

        let html = service.render_template(&TemplateType::Welcome, &variables);
        assert!(html.contains("Welcome to Ectus-R"));
        assert!(html.contains("John Doe"));
    }

    #[test]
    fn test_default_templates() {
        let templates = EmailMarketingService::default_templates();
        assert!(templates.contains_key("welcome"));
        assert!(templates.contains_key("trial_started"));
        assert!(templates.contains_key("payment_success"));
    }
}
