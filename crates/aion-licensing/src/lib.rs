pub mod billing;
pub mod licensing;
pub mod payments;
pub mod subscriptions;
pub mod invoicing;
pub mod usage_tracking;
pub mod pricing;
pub mod entitlements;
pub mod compliance;
pub mod analytics;
pub mod notifications;
pub mod webhooks;
pub mod security;

pub use billing::*;
pub use licensing::*;
pub use payments::*;
pub use subscriptions::*;
pub use invoicing::*;
pub use usage_tracking::*;
pub use pricing::*;
pub use entitlements::*;
pub use compliance::*;
pub use analytics::*;
pub use notifications::*;
pub use webhooks::*;
pub use security::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensingSystem {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub configuration: SystemConfiguration,
    pub payment_providers: Vec<PaymentProvider>,
    pub pricing_models: Vec<PricingModel>,
    pub license_templates: Vec<LicenseTemplate>,
    pub compliance_settings: ComplianceSettings,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: SystemStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemStatus {
    Active,
    Inactive,
    Maintenance,
    Suspended,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfiguration {
    pub currency: Currency,
    pub timezone: String,
    pub tax_calculation: TaxConfiguration,
    pub billing_cycle_day: u8,
    pub grace_period_days: u32,
    pub dunning_settings: DunningSettings,
    pub proration_settings: ProrationSettings,
    pub refund_policy: RefundPolicy,
    pub trial_settings: TrialSettings,
    pub enterprise_features: EnterpriseFeatures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    CAD,
    AUD,
    JPY,
    CHF,
    SEK,
    NOK,
    DKK,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxConfiguration {
    pub enabled: bool,
    pub tax_provider: Option<TaxProvider>,
    pub default_tax_rate: Decimal,
    pub tax_inclusive_pricing: bool,
    pub eu_vat_handling: EuVatHandling,
    pub us_sales_tax_handling: UsSalesTaxHandling,
    pub reverse_charge_enabled: bool,
    pub tax_exemption_supported: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaxProvider {
    Avalara,
    TaxJar,
    Stripe,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EuVatHandling {
    Moss,          // Mini One Stop Shop
    Oss,           // One Stop Shop
    Domestic,      // Domestic registration in each country
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UsSalesTaxHandling {
    Economic,      // Economic nexus
    Physical,      // Physical presence
    Marketplace,   // Marketplace facilitator
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DunningSettings {
    pub enabled: bool,
    pub retry_schedule: Vec<DunningStep>,
    pub final_action: DunningFinalAction,
    pub email_templates: HashMap<String, String>,
    pub webhook_notifications: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DunningStep {
    pub days_after_failure: u32,
    pub action: DunningAction,
    pub email_template: String,
    pub suspend_access: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DunningAction {
    SendEmail,
    RetryPayment,
    SuspendAccess,
    ReduceFeatures,
    SendSMS,
    CreateTicket,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DunningFinalAction {
    CancelSubscription,
    SuspendIndefinitely,
    TransferToCollections,
    WriteOff,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProrationSettings {
    pub enabled: bool,
    pub proration_type: ProrationType,
    pub minimum_proration_amount: Decimal,
    pub credit_unused_time: bool,
    pub immediate_charge: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProrationType {
    Daily,
    Hourly,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundPolicy {
    pub auto_refund_enabled: bool,
    pub refund_window_days: u32,
    pub partial_refunds_allowed: bool,
    pub refund_method: RefundMethod,
    pub approval_required: bool,
    pub approval_threshold: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefundMethod {
    Original,      // Refund to original payment method
    StoreCredit,   // Issue as store credit
    BankTransfer,  // Manual bank transfer
    Check,         // Physical check
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialSettings {
    pub enabled: bool,
    pub default_trial_days: u32,
    pub require_payment_method: bool,
    pub auto_convert: bool,
    pub trial_extensions_allowed: bool,
    pub trial_feature_restrictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseFeatures {
    pub custom_contracts: bool,
    pub volume_discounts: bool,
    pub custom_billing_cycles: bool,
    pub purchase_orders: bool,
    pub multi_entity_billing: bool,
    pub white_label_billing: bool,
    pub advanced_analytics: bool,
    pub dedicated_support: bool,
    pub sla_agreements: bool,
    pub custom_integrations: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub id: Uuid,
    pub license_key: String,
    pub customer_id: Uuid,
    pub product_id: Uuid,
    pub subscription_id: Option<Uuid>,
    pub license_type: LicenseType,
    pub tier: LicenseTier,
    pub features: Vec<Feature>,
    pub limitations: LicenseLimitations,
    pub validity: LicenseValidity,
    pub metadata: LicenseMetadata,
    pub compliance_info: LicenseCompliance,
    pub created_at: DateTime<Utc>,
    pub activated_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_verified: Option<DateTime<Utc>>,
    pub status: LicenseStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LicenseType {
    Perpetual,
    Subscription,
    Trial,
    Evaluation,
    Educational,
    NonCommercial,
    Commercial,
    Enterprise,
    OEM,
    Reseller,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LicenseTier {
    Free,
    Starter,
    Professional,
    Business,
    Enterprise,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub limitations: Option<FeatureLimitations>,
    pub expires_at: Option<DateTime<Utc>>,
    pub usage_tracking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureLimitations {
    pub max_users: Option<u32>,
    pub max_api_calls: Option<u64>,
    pub max_storage_gb: Option<u32>,
    pub max_projects: Option<u32>,
    pub max_deployments: Option<u32>,
    pub max_bandwidth_gb: Option<u32>,
    pub rate_limits: HashMap<String, RateLimit>,
    pub custom_limits: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_second: Option<u32>,
    pub requests_per_minute: Option<u32>,
    pub requests_per_hour: Option<u32>,
    pub requests_per_day: Option<u32>,
    pub burst_limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseLimitations {
    pub max_installations: Option<u32>,
    pub hardware_fingerprinting: bool,
    pub ip_restrictions: Vec<String>,
    pub domain_restrictions: Vec<String>,
    pub geographic_restrictions: Vec<String>,
    pub concurrent_users: Option<u32>,
    pub offline_grace_period_hours: Option<u32>,
    pub transfer_restrictions: TransferRestrictions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRestrictions {
    pub transferable: bool,
    pub requires_approval: bool,
    pub transfer_fee: Option<Decimal>,
    pub max_transfers_per_year: Option<u32>,
    pub cooling_off_period_days: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseValidity {
    pub starts_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub auto_renewal: bool,
    pub grace_period_days: u32,
    pub heartbeat_required: bool,
    pub heartbeat_interval_hours: u32,
    pub offline_allowed: bool,
    pub offline_duration_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseMetadata {
    pub purchase_order: Option<String>,
    pub contract_reference: Option<String>,
    pub sales_person: Option<String>,
    pub partner_id: Option<Uuid>,
    pub reseller_id: Option<Uuid>,
    pub custom_fields: HashMap<String, String>,
    pub tags: Vec<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseCompliance {
    pub audit_required: bool,
    pub last_audit_date: Option<DateTime<Utc>>,
    pub next_audit_date: Option<DateTime<Utc>>,
    pub compliance_officer: Option<String>,
    pub regulatory_requirements: Vec<String>,
    pub export_restrictions: Vec<String>,
    pub privacy_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LicenseStatus {
    Active,
    Inactive,
    Suspended,
    Expired,
    Revoked,
    PendingActivation,
    Terminated,
    UnderReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: Uuid,
    pub external_id: Option<String>,
    pub organization_name: Option<String>,
    pub contact_info: ContactInformation,
    pub billing_info: BillingInformation,
    pub payment_methods: Vec<PaymentMethod>,
    pub tax_info: TaxInformation,
    pub account_settings: AccountSettings,
    pub subscription_history: Vec<SubscriptionHistory>,
    pub credit_balance: Decimal,
    pub account_status: AccountStatus,
    pub risk_assessment: RiskAssessment,
    pub compliance_status: CustomerCompliance,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub last_activity: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInformation {
    pub primary_contact: Contact,
    pub billing_contact: Option<Contact>,
    pub technical_contact: Option<Contact>,
    pub legal_contact: Option<Contact>,
    pub additional_contacts: Vec<Contact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub title: Option<String>,
    pub department: Option<String>,
    pub preferred_language: String,
    pub time_zone: String,
    pub communication_preferences: CommunicationPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationPreferences {
    pub email_notifications: bool,
    pub sms_notifications: bool,
    pub marketing_emails: bool,
    pub invoice_delivery: InvoiceDeliveryMethod,
    pub notification_frequency: NotificationFrequency,
    pub preferred_channels: Vec<CommunicationChannel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceDeliveryMethod {
    Email,
    Portal,
    Mail,
    EDI,
    API,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationFrequency {
    Immediate,
    Daily,
    Weekly,
    Monthly,
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationChannel {
    Email,
    SMS,
    Phone,
    Slack,
    Teams,
    Webhook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingInformation {
    pub billing_address: Address,
    pub shipping_address: Option<Address>,
    pub billing_cycle: BillingCycle,
    pub payment_terms: PaymentTerms,
    pub purchase_order_required: bool,
    pub invoice_consolidation: bool,
    pub auto_pay_enabled: bool,
    pub credit_limit: Option<Decimal>,
    pub payment_method_priority: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state_province: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
    Biennial,
    Custom(u32), // Days
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTerms {
    pub net_days: u32,
    pub early_payment_discount: Option<EarlyPaymentDiscount>,
    pub late_payment_fee: Option<LatePaymentFee>,
    pub currency: Currency,
    pub auto_collection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarlyPaymentDiscount {
    pub discount_percentage: Decimal,
    pub discount_days: u32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatePaymentFee {
    pub fee_type: LatePaymentFeeType,
    pub amount: Decimal,
    pub maximum_fee: Option<Decimal>,
    pub grace_period_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LatePaymentFeeType {
    Fixed,
    Percentage,
    Daily,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub id: Uuid,
    pub method_type: PaymentMethodType,
    pub is_default: bool,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub metadata: PaymentMethodMetadata,
    pub status: PaymentMethodStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethodType {
    CreditCard {
        last_four: String,
        brand: String,
        exp_month: u8,
        exp_year: u16,
        fingerprint: String,
    },
    BankAccount {
        last_four: String,
        routing_number: String,
        account_type: BankAccountType,
        bank_name: String,
    },
    DigitalWallet {
        provider: DigitalWalletProvider,
        account_id: String,
    },
    Cryptocurrency {
        currency: String,
        wallet_address: String,
    },
    WireTransfer {
        bank_details: WireBankDetails,
    },
    Check {
        check_number: Option<String>,
        routing_number: String,
        account_number: String,
    },
    StoreCredit {
        balance: Decimal,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BankAccountType {
    Checking,
    Savings,
    Business,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DigitalWalletProvider {
    PayPal,
    ApplePay,
    GooglePay,
    AmazonPay,
    Alipay,
    WeChat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireBankDetails {
    pub bank_name: String,
    pub swift_code: String,
    pub iban: Option<String>,
    pub routing_number: Option<String>,
    pub account_number: String,
    pub beneficiary_name: String,
    pub beneficiary_address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodMetadata {
    pub provider_id: String,
    pub provider_metadata: HashMap<String, serde_json::Value>,
    pub risk_score: Option<f64>,
    pub verification_data: Option<VerificationData>,
    pub failure_count: u32,
    pub last_failure_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationData {
    pub address_verification: Option<VerificationResult>,
    pub cvv_verification: Option<VerificationResult>,
    pub phone_verification: Option<VerificationResult>,
    pub identity_verification: Option<VerificationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationResult {
    Pass,
    Fail,
    Unavailable,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethodStatus {
    Active,
    Inactive,
    Expired,
    Failed,
    PendingVerification,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxInformation {
    pub tax_id: Option<String>,
    pub tax_exempt: bool,
    pub tax_exemption_certificate: Option<String>,
    pub vat_number: Option<String>,
    pub tax_classification: TaxClassification,
    pub tax_address: Option<Address>,
    pub reverse_charge_applicable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaxClassification {
    Individual,
    Business,
    NonProfit,
    Government,
    Educational,
    Reseller,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettings {
    pub auto_renewal: bool,
    pub usage_alerts: Vec<UsageAlert>,
    pub spending_limits: Vec<SpendingLimit>,
    pub notification_settings: NotificationSettings,
    pub integration_settings: HashMap<String, serde_json::Value>,
    pub security_settings: SecuritySettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageAlert {
    pub metric: String,
    pub threshold_percentage: u32,
    pub notification_channels: Vec<CommunicationChannel>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingLimit {
    pub limit_type: SpendingLimitType,
    pub amount: Decimal,
    pub period: SpendingLimitPeriod,
    pub action: SpendingLimitAction,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpendingLimitType {
    Total,
    Monthly,
    Feature(String),
    Overage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpendingLimitPeriod {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpendingLimitAction {
    Block,
    Alert,
    RequireApproval,
    ReduceService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub billing_notifications: bool,
    pub usage_notifications: bool,
    pub security_notifications: bool,
    pub marketing_notifications: bool,
    pub product_updates: bool,
    pub maintenance_notifications: bool,
    pub compliance_notifications: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub two_factor_enabled: bool,
    pub ip_whitelist: Vec<String>,
    pub api_key_restrictions: ApiKeyRestrictions,
    pub session_timeout_minutes: u32,
    pub password_policy: PasswordPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyRestrictions {
    pub allowed_ips: Vec<String>,
    pub allowed_domains: Vec<String>,
    pub rate_limits: HashMap<String, RateLimit>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    pub min_length: u8,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_symbols: bool,
    pub max_age_days: Option<u32>,
    pub history_count: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionHistory {
    pub subscription_id: Uuid,
    pub plan_name: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub reason: SubscriptionChangeReason,
    pub monthly_recurring_revenue: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionChangeReason {
    NewSubscription,
    Upgrade,
    Downgrade,
    Cancellation,
    NonPayment,
    Refund,
    Transfer,
    Churn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Inactive,
    Suspended,
    PendingActivation,
    Closed,
    UnderReview,
    Delinquent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub risk_score: f64,
    pub risk_factors: Vec<RiskFactor>,
    pub payment_behavior: PaymentBehavior,
    pub churn_probability: f64,
    pub fraud_indicators: Vec<FraudIndicator>,
    pub last_assessed: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor_type: RiskFactorType,
    pub severity: RiskSeverity,
    pub description: String,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskFactorType {
    PaymentFailure,
    ChargebackHistory,
    SuspiciousActivity,
    HighVelocity,
    GeographicRisk,
    IndustryRisk,
    ComplianceRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBehavior {
    pub average_payment_time_days: f64,
    pub payment_failure_rate: f64,
    pub chargeback_count: u32,
    pub dispute_count: u32,
    pub preferred_payment_method: Option<PaymentMethodType>,
    pub payment_patterns: Vec<PaymentPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentPattern {
    pub pattern_type: PaymentPatternType,
    pub confidence: f64,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentPatternType {
    OnTime,
    Late,
    Seasonal,
    Irregular,
    Declining,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudIndicator {
    pub indicator_type: FraudIndicatorType,
    pub confidence: f64,
    pub evidence: String,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FraudIndicatorType {
    VelocityAbuse,
    StolenCard,
    SyntheticIdentity,
    AccountTakeover,
    BotActivity,
    GeoAnomaly,
    DeviceFingerprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCompliance {
    pub kyc_status: KycStatus,
    pub aml_screening: AmlScreening,
    pub sanctions_screening: SanctionsScreening,
    pub pep_screening: PepScreening,
    pub compliance_documents: Vec<ComplianceDocument>,
    pub last_review_date: Option<DateTime<Utc>>,
    pub next_review_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KycStatus {
    NotRequired,
    Pending,
    Verified,
    Failed,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmlScreening {
    pub status: ScreeningStatus,
    pub risk_score: f64,
    pub last_screened: Option<DateTime<Utc>>,
    pub next_screening: Option<DateTime<Utc>>,
    pub watchlist_matches: Vec<WatchlistMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctionsScreening {
    pub status: ScreeningStatus,
    pub sanctioned: bool,
    pub last_screened: Option<DateTime<Utc>>,
    pub sanctions_lists: Vec<String>,
    pub matches: Vec<SanctionsMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepScreening {
    pub status: ScreeningStatus,
    pub is_pep: bool,
    pub last_screened: Option<DateTime<Utc>>,
    pub pep_matches: Vec<PepMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScreeningStatus {
    NotScreened,
    Pending,
    Clear,
    Match,
    FalsePositive,
    UnderReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistMatch {
    pub list_name: String,
    pub match_score: f64,
    pub match_reason: String,
    pub reviewed: bool,
    pub false_positive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctionsMatch {
    pub list_name: String,
    pub entity_name: String,
    pub match_score: f64,
    pub match_type: String,
    pub reviewed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepMatch {
    pub name: String,
    pub position: String,
    pub country: String,
    pub match_score: f64,
    pub reviewed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceDocument {
    pub document_type: ComplianceDocumentType,
    pub document_url: String,
    pub uploaded_at: DateTime<Utc>,
    pub verified: bool,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceDocumentType {
    GovernmentId,
    ProofOfAddress,
    BusinessRegistration,
    TaxCertificate,
    BankStatement,
    ComplianceCertificate,
    Other(String),
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[async_trait::async_trait]
pub trait LicensingManager {
    async fn create_license(&self, license: License) -> Result<Uuid>;
    async fn validate_license(&self, license_key: &str) -> Result<LicenseValidationResult>;
    async fn activate_license(&self, license_key: &str, activation_data: ActivationData) -> Result<()>;
    async fn deactivate_license(&self, license_key: &str) -> Result<()>;
    async fn renew_license(&self, license_key: &str, renewal_period: chrono::Duration) -> Result<()>;
    async fn revoke_license(&self, license_key: &str, reason: RevocationReason) -> Result<()>;
    async fn transfer_license(&self, license_key: &str, new_customer_id: Uuid) -> Result<()>;
    async fn get_license_usage(&self, license_key: &str) -> Result<UsageStatistics>;
    async fn enforce_license_limits(&self, license_key: &str, resource: &str, amount: u64) -> Result<bool>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseValidationResult {
    pub valid: bool,
    pub license: Option<License>,
    pub features: Vec<Feature>,
    pub limitations: LicenseLimitations,
    pub expires_at: Option<DateTime<Utc>>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationData {
    pub machine_fingerprint: String,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub activation_name: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RevocationReason {
    NonPayment,
    Violation,
    CustomerRequest,
    Security,
    Fraud,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStatistics {
    pub license_key: String,
    pub current_usage: HashMap<String, u64>,
    pub usage_history: Vec<UsageDataPoint>,
    pub peak_usage: HashMap<String, u64>,
    pub limits: HashMap<String, u64>,
    pub overage_charges: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageDataPoint {
    pub timestamp: DateTime<Utc>,
    pub metric: String,
    pub value: u64,
    pub metadata: HashMap<String, String>,
}

#[async_trait::async_trait]
pub trait BillingManager {
    async fn create_customer(&self, customer: Customer) -> Result<Uuid>;
    async fn update_customer(&self, id: Uuid, customer: Customer) -> Result<()>;
    async fn get_customer(&self, id: Uuid) -> Result<Customer>;
    async fn delete_customer(&self, id: Uuid) -> Result<()>;
    async fn create_subscription(&self, subscription: Subscription) -> Result<Uuid>;
    async fn update_subscription(&self, id: Uuid, changes: SubscriptionChanges) -> Result<()>;
    async fn cancel_subscription(&self, id: Uuid, cancellation: CancellationRequest) -> Result<()>;
    async fn process_payment(&self, payment_request: PaymentRequest) -> Result<PaymentResult>;
    async fn generate_invoice(&self, invoice_request: InvoiceRequest) -> Result<Invoice>;
    async fn calculate_usage_charges(&self, customer_id: Uuid, period: BillingPeriod) -> Result<Vec<UsageCharge>>;
    async fn apply_discounts(&self, customer_id: Uuid, charges: &mut Vec<LineItem>) -> Result<()>;
    async fn calculate_taxes(&self, customer_id: Uuid, charges: &[LineItem]) -> Result<Vec<TaxLineItem>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub plan_id: Uuid,
    pub status: SubscriptionStatus,
    pub billing_cycle: BillingCycle,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub trial_start: Option<DateTime<Utc>>,
    pub trial_end: Option<DateTime<Utc>>,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<DateTime<Utc>>,
    pub items: Vec<SubscriptionItem>,
    pub addons: Vec<Addon>,
    pub discounts: Vec<DiscountApplication>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    Trialing,
    Active,
    PastDue,
    Canceled,
    Unpaid,
    Incomplete,
    IncompleteExpired,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionItem {
    pub id: Uuid,
    pub price_id: Uuid,
    pub quantity: u32,
    pub unit_amount: Decimal,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Addon {
    pub id: Uuid,
    pub name: String,
    pub price: Decimal,
    pub billing_cycle: BillingCycle,
    pub quantity: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountApplication {
    pub discount_id: Uuid,
    pub coupon_code: Option<String>,
    pub amount_off: Option<Decimal>,
    pub percent_off: Option<Decimal>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

pub struct ComplianceSettings {
    pub gdpr_compliance: bool,
    pub ccpa_compliance: bool,
    pub sox_compliance: bool,
    pub pci_compliance: bool,
    pub data_residency_requirements: Vec<String>,
    pub audit_logging: bool,
    pub encryption_required: bool,
    pub retention_policies: HashMap<String, RetentionPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub retention_period_days: u32,
    pub auto_deletion: bool,
    pub archival_required: bool,
    pub legal_hold_support: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProvider {
    pub id: Uuid,
    pub provider_type: PaymentProviderType,
    pub configuration: ProviderConfiguration,
    pub supported_methods: Vec<PaymentMethodType>,
    pub supported_currencies: Vec<Currency>,
    pub enabled: bool,
    pub priority: u8,
    pub webhook_settings: WebhookSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentProviderType {
    Stripe,
    PayPal,
    Square,
    Adyen,
    Braintree,
    Authorize,
    Recurly,
    Paddle,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfiguration {
    pub api_keys: HashMap<String, String>,
    pub webhook_endpoints: Vec<String>,
    pub test_mode: bool,
    pub retry_configuration: RetryConfiguration,
    pub timeout_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfiguration {
    pub max_retries: u8,
    pub retry_delay_seconds: u32,
    pub exponential_backoff: bool,
    pub retry_conditions: Vec<RetryCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetryCondition {
    NetworkError,
    ServerError,
    RateLimit,
    Timeout,
    InsufficientFunds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSettings {
    pub enabled: bool,
    pub url: String,
    pub secret: String,
    pub events: Vec<WebhookEvent>,
    pub retry_policy: RetryConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookEvent {
    PaymentSucceeded,
    PaymentFailed,
    InvoiceCreated,
    InvoicePaid,
    InvoiceOverdue,
    SubscriptionCreated,
    SubscriptionUpdated,
    SubscriptionCanceled,
    CustomerCreated,
    CustomerUpdated,
    RefundCreated,
    ChargebackCreated,
    DisputeCreated,
}

// Additional structures for comprehensive billing system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingModel {
    pub id: Uuid,
    pub name: String,
    pub model_type: PricingModelType,
    pub currency: Currency,
    pub components: Vec<PricingComponent>,
    pub discounts: Vec<Discount>,
    pub taxes: Vec<TaxRule>,
    pub effective_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PricingModelType {
    Flat,
    Tiered,
    Volume,
    Package,
    UsageBased,
    Hybrid,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingComponent {
    pub id: Uuid,
    pub name: String,
    pub component_type: ComponentType,
    pub unit_price: Decimal,
    pub minimum_units: Option<u32>,
    pub maximum_units: Option<u32>,
    pub billing_frequency: BillingFrequency,
    pub proration_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentType {
    Fixed,
    PerUser,
    PerAPI,
    Storage,
    Bandwidth,
    Transaction,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingFrequency {
    OneTime,
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
    Usage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discount {
    pub id: Uuid,
    pub discount_type: DiscountType,
    pub value: Decimal,
    pub conditions: Vec<DiscountCondition>,
    pub validity: DiscountValidity,
    pub usage_limits: Option<UsageLimits>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscountType {
    Percentage,
    FixedAmount,
    FreeMonths,
    FreeUsage,
    BOGO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountCondition {
    pub condition_type: ConditionType,
    pub operator: ComparisonOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    MinimumAmount,
    CustomerSegment,
    ProductCategory,
    SubscriptionLength,
    NewCustomer,
    Renewal,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    In,
    NotIn,
    Contains,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountValidity {
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub time_restrictions: Option<TimeRestrictions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    pub days_of_week: Vec<u8>,
    pub hours_of_day: Vec<u8>,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageLimits {
    pub max_uses_total: Option<u32>,
    pub max_uses_per_customer: Option<u32>,
    pub max_uses_per_period: Option<u32>,
    pub period_type: Option<PeriodType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeriodType {
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRule {
    pub id: Uuid,
    pub name: String,
    pub tax_type: TaxType,
    pub rate: Decimal,
    pub jurisdiction: TaxJurisdiction,
    pub applicability: TaxApplicability,
    pub effective_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaxType {
    VAT,
    GST,
    SalesTax,
    ServiceTax,
    WithholdingTax,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxJurisdiction {
    pub country: String,
    pub state_province: Option<String>,
    pub city: Option<String>,
    pub postal_codes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxApplicability {
    pub product_categories: Vec<String>,
    pub customer_types: Vec<String>,
    pub transaction_types: Vec<String>,
    pub amount_thresholds: Option<AmountThreshold>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmountThreshold {
    pub minimum_amount: Option<Decimal>,
    pub maximum_amount: Option<Decimal>,
}

// Additional billing structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionChanges {
    pub plan_change: Option<PlanChange>,
    pub quantity_change: Option<QuantityChange>,
    pub addon_changes: Vec<AddonChange>,
    pub billing_cycle_change: Option<BillingCycle>,
    pub proration_behavior: ProrationBehavior,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanChange {
    pub new_plan_id: Uuid,
    pub effective_date: DateTime<Utc>,
    pub prorate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantityChange {
    pub item_id: Uuid,
    pub new_quantity: u32,
    pub effective_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddonChange {
    pub action: AddonAction,
    pub addon_id: Uuid,
    pub quantity: Option<u32>,
    pub effective_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddonAction {
    Add,
    Remove,
    Update,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProrationBehavior {
    CreateProrations,
    None,
    AlwaysInvoice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancellationRequest {
    pub reason: CancellationReason,
    pub cancel_at_period_end: bool,
    pub effective_date: Option<DateTime<Utc>>,
    pub refund_request: Option<RefundRequest>,
    pub feedback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CancellationReason {
    CustomerRequest,
    NonPayment,
    Downgrade,
    TooExpensive,
    MissingFeatures,
    TechnicalIssues,
    CompetitorSwitch,
    BusinessClosure,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    pub amount: Option<Decimal>,
    pub reason: RefundReason,
    pub refund_method: RefundMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefundReason {
    Cancellation,
    ServiceIssue,
    Billing,
    CustomerSatisfaction,
    Legal,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub customer_id: Uuid,
    pub amount: Decimal,
    pub currency: Currency,
    pub payment_method_id: Uuid,
    pub description: String,
    pub metadata: HashMap<String, String>,
    pub capture: bool,
    pub statement_descriptor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResult {
    pub payment_id: Uuid,
    pub status: PaymentStatus,
    pub amount_captured: Decimal,
    pub amount_refunded: Decimal,
    pub fees: Vec<Fee>,
    pub risk_assessment: PaymentRiskAssessment,
    pub provider_response: ProviderResponse,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Succeeded,
    Pending,
    Failed,
    Canceled,
    RequiresAction,
    RequiresCapture,
    RequiresConfirmation,
    RequiresPaymentMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fee {
    pub fee_type: FeeType,
    pub amount: Decimal,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeeType {
    Processing,
    Transaction,
    Currency,
    International,
    Risk,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRiskAssessment {
    pub risk_score: f64,
    pub risk_level: RiskLevel,
    pub factors: Vec<String>,
    pub recommended_action: RiskAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Elevated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskAction {
    Approve,
    Review,
    Decline,
    RequireAuthentication,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderResponse {
    pub provider: PaymentProviderType,
    pub transaction_id: String,
    pub response_code: String,
    pub response_message: String,
    pub raw_response: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceRequest {
    pub customer_id: Uuid,
    pub subscription_id: Option<Uuid>,
    pub line_items: Vec<LineItem>,
    pub due_date: DateTime<Utc>,
    pub auto_advance: bool,
    pub collection_method: CollectionMethod,
    pub currency: Currency,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub id: Uuid,
    pub description: String,
    pub quantity: Decimal,
    pub unit_amount: Decimal,
    pub total_amount: Decimal,
    pub tax_amount: Decimal,
    pub discount_amount: Decimal,
    pub item_type: LineItemType,
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineItemType {
    Subscription,
    Usage,
    OneTime,
    Setup,
    Discount,
    Tax,
    Fee,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Uuid,
    pub invoice_number: String,
    pub customer_id: Uuid,
    pub subscription_id: Option<Uuid>,
    pub status: InvoiceStatus,
    pub line_items: Vec<LineItem>,
    pub tax_lines: Vec<TaxLineItem>,
    pub discount_lines: Vec<DiscountLineItem>,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub discount_total: Decimal,
    pub total: Decimal,
    pub amount_paid: Decimal,
    pub amount_due: Decimal,
    pub currency: Currency,
    pub billing_reason: BillingReason,
    pub payment_settings: InvoicePaymentSettings,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub finalized_at: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
    pub voided_at: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Void,
    Uncollectible,
    PartiallyPaid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxLineItem {
    pub tax_rule_id: Uuid,
    pub tax_name: String,
    pub tax_rate: Decimal,
    pub taxable_amount: Decimal,
    pub tax_amount: Decimal,
    pub jurisdiction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountLineItem {
    pub discount_id: Uuid,
    pub discount_name: String,
    pub discount_type: DiscountType,
    pub discount_value: Decimal,
    pub discount_amount: Decimal,
    pub coupon_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingReason {
    SubscriptionCycle,
    SubscriptionCreate,
    SubscriptionUpdate,
    Usage,
    Manual,
    AutomaticPendingInvoiceItemInvoice,
    Subscription,
    SubscriptionThreshold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePaymentSettings {
    pub payment_method_types: Vec<String>,
    pub default_payment_method: Option<Uuid>,
    pub auto_advance: bool,
    pub collection_method: CollectionMethod,
    pub days_until_due: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPeriod {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub billing_cycle: BillingCycle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageCharge {
    pub metric: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub total_amount: Decimal,
    pub tier_breakdown: Vec<TierUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierUsage {
    pub tier_start: u64,
    pub tier_end: Option<u64>,
    pub unit_price: Decimal,
    pub quantity: Decimal,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub license_type: LicenseType,
    pub default_tier: LicenseTier,
    pub default_features: Vec<Feature>,
    pub default_limitations: LicenseLimitations,
    pub pricing_model_id: Option<Uuid>,
    pub contract_template: Option<String>,
    pub terms_and_conditions: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub active: bool,
}