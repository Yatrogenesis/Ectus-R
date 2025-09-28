pub mod frameworks;
pub mod audit;
pub mod data_protection;
pub mod access_control;
pub mod monitoring;
pub mod reporting;
pub mod encryption;
pub mod retention;
pub mod consent;
pub mod risk_assessment;
pub mod incident_response;

pub use frameworks::*;
pub use audit::*;
pub use data_protection::*;
pub use access_control::*;
pub use monitoring::*;
pub use reporting::*;
pub use encryption::*;
pub use retention::*;
pub use consent::*;
pub use risk_assessment::*;
pub use incident_response::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceFramework {
    GDPR,
    HIPAA,
    SOX,
    PCIDSS,
    ISO27001,
    NIST,
    CCPA,
    PIPEDA,
    LGPD,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceProject {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub frameworks: Vec<ComplianceFramework>,
    pub organization: OrganizationInfo,
    pub data_categories: Vec<DataCategory>,
    pub systems: Vec<SystemInfo>,
    pub policies: Vec<Policy>,
    pub controls: Vec<Control>,
    pub assessments: Vec<RiskAssessment>,
    pub audits: Vec<Audit>,
    pub incidents: Vec<Incident>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub compliance_status: ComplianceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationInfo {
    pub name: String,
    pub industry: Industry,
    pub size: OrganizationSize,
    pub regions: Vec<Region>,
    pub contact_info: ContactInfo,
    pub regulatory_requirements: Vec<RegulatoryRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Industry {
    Healthcare,
    Financial,
    Technology,
    Retail,
    Manufacturing,
    Education,
    Government,
    NonProfit,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrganizationSize {
    Startup,         // < 50 employees
    Small,           // 50-250 employees
    Medium,          // 250-1000 employees
    Large,           // 1000-5000 employees
    Enterprise,      // > 5000 employees
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Region {
    EU,
    US,
    Canada,
    UK,
    APAC,
    LatinAmerica,
    MiddleEast,
    Africa,
    Global,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub dpo_email: Option<String>,          // Data Protection Officer
    pub privacy_officer_email: Option<String>,
    pub security_officer_email: Option<String>,
    pub compliance_officer_email: Option<String>,
    pub legal_contact_email: Option<String>,
    pub incident_response_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryRequirement {
    pub framework: ComplianceFramework,
    pub requirement_id: String,
    pub title: String,
    pub description: String,
    pub mandatory: bool,
    pub deadline: Option<DateTime<Utc>>,
    pub status: RequirementStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequirementStatus {
    NotStarted,
    InProgress,
    Implemented,
    Verified,
    NonCompliant,
    Exempt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCategory {
    pub id: Uuid,
    pub name: String,
    pub classification: DataClassification,
    pub sensitivity_level: SensitivityLevel,
    pub personal_data: bool,
    pub special_category: bool,
    pub retention_period: RetentionPeriod,
    pub processing_purposes: Vec<ProcessingPurpose>,
    pub legal_basis: Vec<LegalBasis>,
    pub data_subjects: Vec<DataSubjectCategory>,
    pub geographic_restrictions: Vec<Region>,
    pub encryption_required: bool,
    pub access_controls: Vec<AccessControlRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensitivityLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPeriod {
    pub duration_years: u32,
    pub trigger_event: Option<String>,
    pub legal_hold: bool,
    pub auto_deletion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingPurpose {
    ContractPerformance,
    LegalCompliance,
    LegitimateInterest,
    VitalInterests,
    PublicTask,
    Consent,
    Marketing,
    Analytics,
    Security,
    Research,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegalBasis {
    Consent,
    Contract,
    LegalObligation,
    VitalInterests,
    PublicTask,
    LegitimateInterests,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSubjectCategory {
    Customers,
    Employees,
    Prospects,
    Vendors,
    Minors,
    VulnerableGroups,
    PublicFigures,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlRequirement {
    pub role: String,
    pub permissions: Vec<Permission>,
    pub conditions: Vec<AccessCondition>,
    pub approval_required: bool,
    pub audit_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    Read,
    Write,
    Delete,
    Export,
    Share,
    Modify,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessCondition {
    TimeRestriction(String),
    LocationRestriction(String),
    NetworkRestriction(String),
    DeviceRestriction(String),
    MultiFactorAuth,
    Approval(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub id: Uuid,
    pub name: String,
    pub system_type: SystemType,
    pub criticality: Criticality,
    pub data_categories: Vec<Uuid>,
    pub hosting: HostingModel,
    pub vendors: Vec<VendorInfo>,
    pub security_controls: Vec<SecurityControl>,
    pub compliance_requirements: Vec<ComplianceFramework>,
    pub data_flows: Vec<DataFlow>,
    pub last_assessment: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemType {
    Database,
    WebApplication,
    MobileApp,
    API,
    DataWarehouse,
    Analytics,
    CRM,
    ERP,
    PaymentProcessor,
    IdentityProvider,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Criticality {
    Low,
    Medium,
    High,
    Critical,
    MissionCritical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HostingModel {
    OnPremise,
    Cloud(String),         // Cloud provider name
    Hybrid,
    SaaS(String),          // SaaS provider name
    Managed(String),       // Managed service provider
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorInfo {
    pub name: String,
    pub service_type: String,
    pub contract_date: DateTime<Utc>,
    pub contract_expiry: Option<DateTime<Utc>>,
    pub dpa_signed: bool,                    // Data Processing Agreement
    pub certifications: Vec<String>,
    pub security_assessment_date: Option<DateTime<Utc>>,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityControl {
    pub id: String,
    pub name: String,
    pub control_type: ControlType,
    pub framework_mapping: HashMap<ComplianceFramework, String>,
    pub implementation_status: ImplementationStatus,
    pub effectiveness: ControlEffectiveness,
    pub testing_frequency: TestingFrequency,
    pub last_tested: Option<DateTime<Utc>>,
    pub next_test_due: Option<DateTime<Utc>>,
    pub responsible_party: String,
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlType {
    Preventive,
    Detective,
    Corrective,
    Administrative,
    Technical,
    Physical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationStatus {
    NotImplemented,
    PartiallyImplemented,
    FullyImplemented,
    Disabled,
    NotApplicable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlEffectiveness {
    Ineffective,
    PartiallyEffective,
    Effective,
    NotTested,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestingFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
    AdHoc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,
    pub evidence_type: EvidenceType,
    pub description: String,
    pub file_path: Option<String>,
    pub collected_by: String,
    pub collected_at: DateTime<Utc>,
    pub hash: Option<String>,
    pub retention_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    Document,
    Screenshot,
    LogFile,
    Certificate,
    TestResult,
    Interview,
    Observation,
    Configuration,
    Code,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlow {
    pub id: Uuid,
    pub source_system: String,
    pub destination_system: String,
    pub data_categories: Vec<Uuid>,
    pub transfer_method: TransferMethod,
    pub encryption_in_transit: bool,
    pub encryption_at_rest: bool,
    pub cross_border: bool,
    pub legal_basis: Option<LegalBasis>,
    pub safeguards: Vec<String>,
    pub frequency: TransferFrequency,
    pub volume: DataVolume,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferMethod {
    API,
    FTP,
    SFTP,
    Database,
    FileShare,
    Email,
    PhysicalMedia,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferFrequency {
    RealTime,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    AdHoc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataVolume {
    Low,      // < 1GB
    Medium,   // 1GB - 100GB
    High,     // 100GB - 1TB
    VeryHigh, // > 1TB
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: Uuid,
    pub name: String,
    pub policy_type: PolicyType,
    pub frameworks: Vec<ComplianceFramework>,
    pub content: String,
    pub version: String,
    pub approved_by: String,
    pub approved_date: DateTime<Utc>,
    pub effective_date: DateTime<Utc>,
    pub review_date: DateTime<Utc>,
    pub status: PolicyStatus,
    pub attachments: Vec<String>,
    pub training_required: bool,
    pub acknowledgment_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyType {
    Privacy,
    Security,
    DataRetention,
    IncidentResponse,
    AccessControl,
    Vendor,
    Training,
    Acceptable,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyStatus {
    Draft,
    UnderReview,
    Approved,
    Active,
    Retired,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Control {
    pub id: Uuid,
    pub control_id: String,
    pub name: String,
    pub description: String,
    pub framework: ComplianceFramework,
    pub control_family: String,
    pub control_type: ControlType,
    pub implementation_guidance: String,
    pub testing_procedures: Vec<TestingProcedure>,
    pub maturity_level: MaturityLevel,
    pub automation_level: AutomationLevel,
    pub cost_impact: CostImpact,
    pub risk_reduction: f64,
    pub dependencies: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestingProcedure {
    pub procedure_id: String,
    pub description: String,
    pub frequency: TestingFrequency,
    pub method: TestingMethod,
    pub expected_result: String,
    pub responsible_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestingMethod {
    ManualReview,
    AutomatedScan,
    Interview,
    Observation,
    Documentation,
    SystemTest,
    PenetrationTest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaturityLevel {
    Initial,
    Managed,
    Defined,
    QuantitativelyManaged,
    Optimizing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationLevel {
    Manual,
    SemiAutomated,
    FullyAutomated,
    AIAssisted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CostImpact {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub overall_score: f64,
    pub framework_scores: HashMap<ComplianceFramework, f64>,
    pub critical_gaps: Vec<ComplianceGap>,
    pub improvement_recommendations: Vec<Recommendation>,
    pub next_assessment_due: DateTime<Utc>,
    pub certification_status: Vec<CertificationStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceGap {
    pub id: Uuid,
    pub severity: GapSeverity,
    pub framework: ComplianceFramework,
    pub control_id: String,
    pub description: String,
    pub risk_impact: RiskImpact,
    pub remediation_effort: RemediationEffort,
    pub due_date: Option<DateTime<Utc>>,
    pub responsible_party: String,
    pub status: GapStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GapSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskImpact {
    Negligible,
    Minor,
    Moderate,
    Major,
    Catastrophic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemediationEffort {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GapStatus {
    Identified,
    Assigned,
    InProgress,
    UnderReview,
    Resolved,
    Accepted,
    Deferred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: Uuid,
    pub priority: RecommendationPriority,
    pub category: RecommendationCategory,
    pub title: String,
    pub description: String,
    pub benefits: Vec<String>,
    pub estimated_effort: EstimatedEffort,
    pub estimated_cost: Option<f64>,
    pub timeline: String,
    pub resources_required: Vec<String>,
    pub dependencies: Vec<String>,
    pub metrics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationCategory {
    Process,
    Technology,
    Training,
    Policy,
    Governance,
    Risk,
    Audit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimatedEffort {
    pub hours: f64,
    pub complexity: EffortComplexity,
    pub skills_required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffortComplexity {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificationStatus {
    pub framework: ComplianceFramework,
    pub certification_body: String,
    pub certificate_number: Option<String>,
    pub issue_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub status: CertificationState,
    pub scope: String,
    pub next_audit_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertificationState {
    NotCertified,
    InProgress,
    Certified,
    Suspended,
    Revoked,
    Expired,
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[async_trait::async_trait]
pub trait ComplianceManager {
    async fn create_project(&self, project: ComplianceProject) -> Result<Uuid>;
    async fn update_project(&self, id: Uuid, project: ComplianceProject) -> Result<()>;
    async fn get_project(&self, id: Uuid) -> Result<ComplianceProject>;
    async fn assess_compliance(&self, project_id: Uuid) -> Result<ComplianceStatus>;
    async fn generate_gap_analysis(&self, project_id: Uuid, framework: ComplianceFramework) -> Result<Vec<ComplianceGap>>;
    async fn create_remediation_plan(&self, gaps: Vec<ComplianceGap>) -> Result<RemediationPlan>;
    async fn track_progress(&self, project_id: Uuid) -> Result<ProgressReport>;
    async fn generate_compliance_report(&self, project_id: Uuid, format: ReportFormat) -> Result<Vec<u8>>;
    async fn schedule_assessment(&self, project_id: Uuid, framework: ComplianceFramework, date: DateTime<Utc>) -> Result<Uuid>;
    async fn notify_stakeholders(&self, project_id: Uuid, notification_type: NotificationType) -> Result<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationPlan {
    pub id: Uuid,
    pub project_id: Uuid,
    pub gaps: Vec<ComplianceGap>,
    pub phases: Vec<RemediationPhase>,
    pub total_effort_hours: f64,
    pub estimated_cost: f64,
    pub timeline_months: u32,
    pub success_metrics: Vec<SuccessMetric>,
    pub risk_mitigation: Vec<RiskMitigation>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationPhase {
    pub phase_number: u32,
    pub name: String,
    pub description: String,
    pub duration_weeks: u32,
    pub gaps_addressed: Vec<Uuid>,
    pub deliverables: Vec<String>,
    pub dependencies: Vec<u32>,
    pub resources: Vec<String>,
    pub milestones: Vec<Milestone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub name: String,
    pub due_date: DateTime<Utc>,
    pub deliverable: String,
    pub success_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMetric {
    pub name: String,
    pub description: String,
    pub target_value: f64,
    pub measurement_method: String,
    pub frequency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMitigation {
    pub risk: String,
    pub probability: f64,
    pub impact: RiskImpact,
    pub mitigation_strategy: String,
    pub contingency_plan: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressReport {
    pub project_id: Uuid,
    pub overall_progress: f64,
    pub framework_progress: HashMap<ComplianceFramework, f64>,
    pub completed_gaps: u32,
    pub remaining_gaps: u32,
    pub overdue_items: Vec<OverdueItem>,
    pub upcoming_deadlines: Vec<UpcomingDeadline>,
    pub budget_status: BudgetStatus,
    pub timeline_status: TimelineStatus,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdueItem {
    pub item_type: String,
    pub description: String,
    pub due_date: DateTime<Utc>,
    pub days_overdue: u32,
    pub responsible_party: String,
    pub priority: RecommendationPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpcomingDeadline {
    pub item_type: String,
    pub description: String,
    pub due_date: DateTime<Utc>,
    pub days_remaining: u32,
    pub responsible_party: String,
    pub priority: RecommendationPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetStatus {
    pub total_budget: f64,
    pub spent_amount: f64,
    pub committed_amount: f64,
    pub remaining_amount: f64,
    pub projected_overage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimelineStatus {
    OnTrack,
    AtRisk,
    Delayed,
    Ahead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    PDF,
    Excel,
    Word,
    JSON,
    HTML,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    GapIdentified,
    DeadlineApproaching,
    AssessmentDue,
    CertificationExpiring,
    ComplianceAchieved,
    ViolationDetected,
    AuditScheduled,
}