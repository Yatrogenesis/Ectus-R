pub mod gdpr;
pub mod hipaa;
pub mod sox;
pub mod pci_dss;
pub mod iso27001;
pub mod nist;

pub use gdpr::*;
pub use hipaa::*;
pub use sox::*;
pub use pci_dss::*;
pub use iso27001::*;
pub use nist::*;

use crate::{ComplianceFramework, Control, ComplianceGap, Result};
use std::collections::HashMap;

pub trait FrameworkImplementation {
    fn get_framework_type(&self) -> ComplianceFramework;
    fn get_controls(&self) -> Vec<Control>;
    fn get_control_mappings(&self) -> HashMap<String, Vec<String>>;
    fn assess_compliance(&self, controls: &[Control]) -> Result<f64>;
    fn identify_gaps(&self, implemented_controls: &[String]) -> Vec<ComplianceGap>;
    fn get_implementation_guidance(&self, control_id: &str) -> Option<String>;
    fn get_testing_procedures(&self, control_id: &str) -> Vec<crate::TestingProcedure>;
}

pub struct FrameworkRegistry {
    frameworks: HashMap<ComplianceFramework, Box<dyn FrameworkImplementation + Send + Sync>>,
}

impl FrameworkRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            frameworks: HashMap::new(),
        };

        // Register all frameworks
        registry.register(Box::new(GDPRFramework::new()));
        registry.register(Box::new(HIPAAFramework::new()));
        registry.register(Box::new(SOXFramework::new()));
        registry.register(Box::new(PCIDSSFramework::new()));
        registry.register(Box::new(ISO27001Framework::new()));
        registry.register(Box::new(NISTFramework::new()));

        registry
    }

    pub fn register(&mut self, framework: Box<dyn FrameworkImplementation + Send + Sync>) {
        let framework_type = framework.get_framework_type();
        self.frameworks.insert(framework_type, framework);
    }

    pub fn get_framework(&self, framework_type: &ComplianceFramework) -> Option<&dyn FrameworkImplementation> {
        self.frameworks.get(framework_type).map(|f| f.as_ref())
    }

    pub fn get_all_frameworks(&self) -> Vec<ComplianceFramework> {
        self.frameworks.keys().cloned().collect()
    }

    pub fn get_controls_for_framework(&self, framework_type: &ComplianceFramework) -> Option<Vec<Control>> {
        self.get_framework(framework_type).map(|f| f.get_controls())
    }

    pub fn assess_framework_compliance(&self, framework_type: &ComplianceFramework, controls: &[Control]) -> Result<f64> {
        match self.get_framework(framework_type) {
            Some(framework) => framework.assess_compliance(controls),
            None => Err(format!("Framework not found: {:?}", framework_type).into()),
        }
    }

    pub fn cross_framework_mapping(&self) -> HashMap<String, Vec<(ComplianceFramework, String)>> {
        let mut mappings = HashMap::new();

        // Common security controls that map across frameworks
        let common_controls = vec![
            ("access_control", vec![
                (ComplianceFramework::GDPR, "Article 32".to_string()),
                (ComplianceFramework::HIPAA, "164.312(a)(1)".to_string()),
                (ComplianceFramework::SOX, "COSO-AC".to_string()),
                (ComplianceFramework::PCIDSS, "7.1".to_string()),
                (ComplianceFramework::ISO27001, "A.9".to_string()),
                (ComplianceFramework::NIST, "AC-1".to_string()),
            ]),
            ("encryption", vec![
                (ComplianceFramework::GDPR, "Article 32(1)(a)".to_string()),
                (ComplianceFramework::HIPAA, "164.312(a)(2)(iv)".to_string()),
                (ComplianceFramework::PCIDSS, "3.4".to_string()),
                (ComplianceFramework::ISO27001, "A.10.1".to_string()),
                (ComplianceFramework::NIST, "SC-13".to_string()),
            ]),
            ("audit_logging", vec![
                (ComplianceFramework::GDPR, "Article 30".to_string()),
                (ComplianceFramework::HIPAA, "164.312(b)".to_string()),
                (ComplianceFramework::SOX, "AU-3".to_string()),
                (ComplianceFramework::PCIDSS, "10.1".to_string()),
                (ComplianceFramework::ISO27001, "A.12.4".to_string()),
                (ComplianceFramework::NIST, "AU-2".to_string()),
            ]),
            ("incident_response", vec![
                (ComplianceFramework::GDPR, "Article 33".to_string()),
                (ComplianceFramework::HIPAA, "164.308(a)(6)".to_string()),
                (ComplianceFramework::PCIDSS, "12.10".to_string()),
                (ComplianceFramework::ISO27001, "A.16.1".to_string()),
                (ComplianceFramework::NIST, "IR-1".to_string()),
            ]),
            ("risk_assessment", vec![
                (ComplianceFramework::GDPR, "Article 35".to_string()),
                (ComplianceFramework::HIPAA, "164.308(a)(1)(ii)(A)".to_string()),
                (ComplianceFramework::SOX, "COSO-RA".to_string()),
                (ComplianceFramework::PCIDSS, "12.2".to_string()),
                (ComplianceFramework::ISO27001, "A.12.6".to_string()),
                (ComplianceFramework::NIST, "RA-1".to_string()),
            ]),
            ("data_protection", vec![
                (ComplianceFramework::GDPR, "Article 25".to_string()),
                (ComplianceFramework::HIPAA, "164.502".to_string()),
                (ComplianceFramework::ISO27001, "A.13.2".to_string()),
                (ComplianceFramework::NIST, "SI-12".to_string()),
            ]),
            ("vendor_management", vec![
                (ComplianceFramework::GDPR, "Article 28".to_string()),
                (ComplianceFramework::HIPAA, "164.502(e)".to_string()),
                (ComplianceFramework::SOX, "COSO-VM".to_string()),
                (ComplianceFramework::PCIDSS, "12.8".to_string()),
                (ComplianceFramework::ISO27001, "A.15.1".to_string()),
                (ComplianceFramework::NIST, "SA-9".to_string()),
            ]),
            ("training_awareness", vec![
                (ComplianceFramework::GDPR, "Article 39".to_string()),
                (ComplianceFramework::HIPAA, "164.308(a)(5)".to_string()),
                (ComplianceFramework::PCIDSS, "12.6".to_string()),
                (ComplianceFramework::ISO27001, "A.7.2".to_string()),
                (ComplianceFramework::NIST, "AT-2".to_string()),
            ]),
        ];

        for (control_name, framework_mappings) in common_controls {
            mappings.insert(control_name.to_string(), framework_mappings);
        }

        mappings
    }

    pub fn get_unified_control_set(&self) -> Vec<UnifiedControl> {
        let mut unified_controls = Vec::new();
        let mappings = self.cross_framework_mapping();

        for (control_name, framework_mappings) in mappings {
            unified_controls.push(UnifiedControl {
                id: control_name.clone(),
                name: self.humanize_control_name(&control_name),
                description: self.get_control_description(&control_name),
                framework_mappings: framework_mappings.into_iter().collect(),
                criticality: self.determine_criticality(&control_name),
                implementation_priority: self.determine_priority(&control_name),
            });
        }

        unified_controls
    }

    fn humanize_control_name(&self, control_name: &str) -> String {
        match control_name {
            "access_control" => "Access Control and Authentication".to_string(),
            "encryption" => "Data Encryption and Cryptography".to_string(),
            "audit_logging" => "Audit Logging and Monitoring".to_string(),
            "incident_response" => "Incident Response and Management".to_string(),
            "risk_assessment" => "Risk Assessment and Management".to_string(),
            "data_protection" => "Data Protection and Privacy".to_string(),
            "vendor_management" => "Third-Party and Vendor Management".to_string(),
            "training_awareness" => "Security Training and Awareness".to_string(),
            _ => control_name.replace("_", " ").to_string(),
        }
    }

    fn get_control_description(&self, control_name: &str) -> String {
        match control_name {
            "access_control" => "Implement proper access controls, authentication mechanisms, and authorization processes to ensure only authorized individuals can access sensitive data and systems.".to_string(),
            "encryption" => "Implement appropriate encryption measures to protect data in transit and at rest, using industry-standard cryptographic methods.".to_string(),
            "audit_logging" => "Maintain comprehensive audit logs and monitoring systems to track access, changes, and security events across all systems.".to_string(),
            "incident_response" => "Establish and maintain an incident response plan to detect, respond to, and recover from security incidents and data breaches.".to_string(),
            "risk_assessment" => "Conduct regular risk assessments to identify, analyze, and mitigate security and privacy risks to the organization.".to_string(),
            "data_protection" => "Implement data protection measures including data minimization, purpose limitation, and privacy by design principles.".to_string(),
            "vendor_management" => "Establish proper due diligence and ongoing monitoring of third-party vendors and service providers who handle sensitive data.".to_string(),
            "training_awareness" => "Provide regular security and privacy training to employees and maintain awareness programs to ensure compliance.".to_string(),
            _ => format!("Control for {}", control_name),
        }
    }

    fn determine_criticality(&self, control_name: &str) -> crate::Criticality {
        match control_name {
            "access_control" | "encryption" | "data_protection" => crate::Criticality::Critical,
            "audit_logging" | "incident_response" | "risk_assessment" => crate::Criticality::High,
            "vendor_management" | "training_awareness" => crate::Criticality::Medium,
            _ => crate::Criticality::Medium,
        }
    }

    fn determine_priority(&self, control_name: &str) -> ImplementationPriority {
        match control_name {
            "access_control" | "encryption" => ImplementationPriority::Critical,
            "data_protection" | "audit_logging" => ImplementationPriority::High,
            "incident_response" | "risk_assessment" => ImplementationPriority::Medium,
            "vendor_management" | "training_awareness" => ImplementationPriority::Low,
            _ => ImplementationPriority::Medium,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnifiedControl {
    pub id: String,
    pub name: String,
    pub description: String,
    pub framework_mappings: Vec<(ComplianceFramework, String)>,
    pub criticality: crate::Criticality,
    pub implementation_priority: ImplementationPriority,
}

#[derive(Debug, Clone)]
pub enum ImplementationPriority {
    Critical,
    High,
    Medium,
    Low,
}

pub struct ComplianceCalculator;

impl ComplianceCalculator {
    pub fn calculate_overall_score(framework_scores: &HashMap<ComplianceFramework, f64>) -> f64 {
        if framework_scores.is_empty() {
            return 0.0;
        }

        let total_score: f64 = framework_scores.values().sum();
        total_score / framework_scores.len() as f64
    }

    pub fn calculate_weighted_score(
        framework_scores: &HashMap<ComplianceFramework, f64>,
        weights: &HashMap<ComplianceFramework, f64>,
    ) -> f64 {
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for (framework, score) in framework_scores {
            if let Some(weight) = weights.get(framework) {
                weighted_sum += score * weight;
                total_weight += weight;
            }
        }

        if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            Self::calculate_overall_score(framework_scores)
        }
    }

    pub fn determine_compliance_level(score: f64) -> ComplianceLevel {
        match score {
            s if s >= 95.0 => ComplianceLevel::Excellent,
            s if s >= 85.0 => ComplianceLevel::Good,
            s if s >= 70.0 => ComplianceLevel::Satisfactory,
            s if s >= 50.0 => ComplianceLevel::NeedsImprovement,
            _ => ComplianceLevel::Poor,
        }
    }

    pub fn calculate_risk_score(gaps: &[ComplianceGap]) -> f64 {
        if gaps.is_empty() {
            return 0.0;
        }

        let total_risk: f64 = gaps.iter().map(|gap| {
            match gap.severity {
                crate::GapSeverity::Critical => 10.0,
                crate::GapSeverity::High => 7.0,
                crate::GapSeverity::Medium => 4.0,
                crate::GapSeverity::Low => 1.0,
            }
        }).sum();

        (total_risk / gaps.len() as f64).min(10.0)
    }
}

#[derive(Debug, Clone)]
pub enum ComplianceLevel {
    Excellent,
    Good,
    Satisfactory,
    NeedsImprovement,
    Poor,
}

pub struct FrameworkComparator;

impl FrameworkComparator {
    pub fn compare_frameworks(
        framework1: ComplianceFramework,
        framework2: ComplianceFramework,
        registry: &FrameworkRegistry,
    ) -> FrameworkComparison {
        let controls1 = registry.get_controls_for_framework(&framework1).unwrap_or_default();
        let controls2 = registry.get_controls_for_framework(&framework2).unwrap_or_default();

        let common_controls = Self::find_common_controls(&controls1, &controls2);
        let unique_to_first = Self::find_unique_controls(&controls1, &controls2);
        let unique_to_second = Self::find_unique_controls(&controls2, &controls1);

        FrameworkComparison {
            framework1,
            framework2,
            common_controls: common_controls.len(),
            unique_to_first: unique_to_first.len(),
            unique_to_second: unique_to_second.len(),
            similarity_score: Self::calculate_similarity(&controls1, &controls2),
            complexity_comparison: Self::compare_complexity(&controls1, &controls2),
        }
    }

    fn find_common_controls(controls1: &[Control], controls2: &[Control]) -> Vec<String> {
        let mut common = Vec::new();
        for control1 in controls1 {
            for control2 in controls2 {
                if Self::controls_similar(&control1, &control2) {
                    common.push(control1.control_id.clone());
                    break;
                }
            }
        }
        common
    }

    fn find_unique_controls(controls1: &[Control], controls2: &[Control]) -> Vec<String> {
        let mut unique = Vec::new();
        for control1 in controls1 {
            let mut found = false;
            for control2 in controls2 {
                if Self::controls_similar(&control1, &control2) {
                    found = true;
                    break;
                }
            }
            if !found {
                unique.push(control1.control_id.clone());
            }
        }
        unique
    }

    fn controls_similar(control1: &Control, control2: &Control) -> bool {
        // Simple similarity check based on control family and type
        control1.control_family == control2.control_family &&
        control1.control_type == control2.control_type
    }

    fn calculate_similarity(controls1: &[Control], controls2: &[Control]) -> f64 {
        if controls1.is_empty() || controls2.is_empty() {
            return 0.0;
        }

        let common = Self::find_common_controls(controls1, controls2);
        let total = controls1.len() + controls2.len() - common.len();

        common.len() as f64 / total as f64
    }

    fn compare_complexity(controls1: &[Control], controls2: &[Control]) -> ComplexityComparison {
        let avg_complexity1 = Self::average_complexity(controls1);
        let avg_complexity2 = Self::average_complexity(controls2);

        ComplexityComparison {
            framework1_complexity: avg_complexity1,
            framework2_complexity: avg_complexity2,
            difference: (avg_complexity1 - avg_complexity2).abs(),
        }
    }

    fn average_complexity(controls: &[Control]) -> f64 {
        if controls.is_empty() {
            return 0.0;
        }

        let total_complexity: f64 = controls.iter().map(|c| {
            match c.maturity_level {
                crate::MaturityLevel::Initial => 1.0,
                crate::MaturityLevel::Managed => 2.0,
                crate::MaturityLevel::Defined => 3.0,
                crate::MaturityLevel::QuantitativelyManaged => 4.0,
                crate::MaturityLevel::Optimizing => 5.0,
            }
        }).sum();

        total_complexity / controls.len() as f64
    }
}

#[derive(Debug, Clone)]
pub struct FrameworkComparison {
    pub framework1: ComplianceFramework,
    pub framework2: ComplianceFramework,
    pub common_controls: usize,
    pub unique_to_first: usize,
    pub unique_to_second: usize,
    pub similarity_score: f64,
    pub complexity_comparison: ComplexityComparison,
}

#[derive(Debug, Clone)]
pub struct ComplexityComparison {
    pub framework1_complexity: f64,
    pub framework2_complexity: f64,
    pub difference: f64,
}