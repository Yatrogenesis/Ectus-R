use crate::{
    ComplianceFramework, Control, ComplianceGap, ControlType, MaturityLevel, AutomationLevel,
    CostImpact, TestingFrequency, TestingMethod, TestingProcedure, GapSeverity, RiskImpact,
    RemediationEffort, GapStatus, Result
};
use crate::frameworks::FrameworkImplementation;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

pub struct GDPRFramework {
    controls: Vec<Control>,
}

impl GDPRFramework {
    pub fn new() -> Self {
        Self {
            controls: Self::define_gdpr_controls(),
        }
    }

    fn define_gdpr_controls() -> Vec<Control> {
        vec![
            // Article 5 - Principles of processing personal data
            Control {
                id: Uuid::new_v4(),
                control_id: "GDPR-5.1".to_string(),
                name: "Lawfulness, fairness and transparency".to_string(),
                description: "Personal data shall be processed lawfully, fairly and in a transparent manner".to_string(),
                framework: ComplianceFramework::GDPR,
                control_family: "Data Processing Principles".to_string(),
                control_type: ControlType::Administrative,
                implementation_guidance: "Implement clear data processing policies, obtain proper legal basis, and provide transparent privacy notices to data subjects.".to_string(),
                testing_procedures: vec![
                    TestingProcedure {
                        procedure_id: "GDPR-5.1-T1".to_string(),
                        description: "Review privacy notices for transparency and completeness".to_string(),
                        frequency: TestingFrequency::Quarterly,
                        method: TestingMethod::ManualReview,
                        expected_result: "Privacy notices clearly explain data processing purposes and legal basis".to_string(),
                        responsible_role: "Data Protection Officer".to_string(),
                    },
                    TestingProcedure {
                        procedure_id: "GDPR-5.1-T2".to_string(),
                        description: "Audit data processing activities for legal basis".to_string(),
                        frequency: TestingFrequency::SemiAnnually,
                        method: TestingMethod::Documentation,
                        expected_result: "All processing activities have documented valid legal basis".to_string(),
                        responsible_role: "Legal Team".to_string(),
                    },
                ],
                maturity_level: MaturityLevel::Defined,
                automation_level: AutomationLevel::SemiAutomated,
                cost_impact: CostImpact::Medium,
                risk_reduction: 8.5,
                dependencies: vec![],
            },

            // Article 6 - Lawfulness of processing
            Control {
                id: Uuid::new_v4(),
                control_id: "GDPR-6.1".to_string(),
                name: "Legal basis for processing".to_string(),
                description: "Processing is lawful only if at least one legal basis applies".to_string(),
                framework: ComplianceFramework::GDPR,
                control_family: "Legal Basis".to_string(),
                control_type: ControlType::Administrative,
                implementation_guidance: "Document legal basis for each processing activity and ensure it remains valid throughout processing.".to_string(),
                testing_procedures: vec![
                    TestingProcedure {
                        procedure_id: "GDPR-6.1-T1".to_string(),
                        description: "Verify legal basis documentation for all processing activities".to_string(),
                        frequency: TestingFrequency::Quarterly,
                        method: TestingMethod::Documentation,
                        expected_result: "Each processing activity has documented valid legal basis".to_string(),
                        responsible_role: "Privacy Team".to_string(),
                    },
                ],
                maturity_level: MaturityLevel::Defined,
                automation_level: AutomationLevel::Manual,
                cost_impact: CostImpact::Low,
                risk_reduction: 9.0,
                dependencies: vec![],
            },

            // Article 7 - Conditions for consent
            Control {
                id: Uuid::new_v4(),
                control_id: "GDPR-7.1".to_string(),
                name: "Consent management".to_string(),
                description: "When processing is based on consent, demonstrate that consent was given".to_string(),
                framework: ComplianceFramework::GDPR,
                control_family: "Consent Management".to_string(),
                control_type: ControlType::Technical,
                implementation_guidance: "Implement consent management system with clear opt-in mechanisms and withdrawal capabilities.".to_string(),
                testing_procedures: vec![
                    TestingProcedure {
                        procedure_id: "GDPR-7.1-T1".to_string(),
                        description: "Test consent capture and withdrawal mechanisms".to_string(),
                        frequency: TestingFrequency::Monthly,
                        method: TestingMethod::SystemTest,
                        expected_result: "Consent can be easily given and withdrawn through system interfaces".to_string(),
                        responsible_role: "Technical Team".to_string(),
                    },
                ],
                maturity_level: MaturityLevel::QuantitativelyManaged,
                automation_level: AutomationLevel::FullyAutomated,
                cost_impact: CostImpact::High,
                risk_reduction: 8.0,
                dependencies: vec![],
            },

            // Article 25 - Data protection by design and by default
            Control {
                id: Uuid::new_v4(),
                control_id: "GDPR-25.1".to_string(),
                name: "Privacy by design".to_string(),
                description: "Implement data protection by design and by default".to_string(),
                framework: ComplianceFramework::GDPR,
                control_family: "Privacy Engineering".to_string(),
                control_type: ControlType::Technical,
                implementation_guidance: "Integrate privacy considerations into system design and implement privacy-preserving defaults.".to_string(),
                testing_procedures: vec![
                    TestingProcedure {
                        procedure_id: "GDPR-25.1-T1".to_string(),
                        description: "Review system designs for privacy by design implementation".to_string(),
                        frequency: TestingFrequency::Quarterly,
                        method: TestingMethod::ManualReview,
                        expected_result: "System designs incorporate privacy by design principles".to_string(),
                        responsible_role: "Privacy Architect".to_string(),
                    },
                ],
                maturity_level: MaturityLevel::Defined,
                automation_level: AutomationLevel::SemiAutomated,
                cost_impact: CostImpact::High,
                risk_reduction: 9.5,
                dependencies: vec![],
            },

            // Article 30 - Records of processing activities
            Control {
                id: Uuid::new_v4(),
                control_id: "GDPR-30.1".to_string(),
                name: "Records of processing".to_string(),
                description: "Maintain records of processing activities".to_string(),
                framework: ComplianceFramework::GDPR,
                control_family: "Documentation".to_string(),
                control_type: ControlType::Administrative,
                implementation_guidance: "Maintain comprehensive records of all data processing activities including purposes, categories of data, and retention periods.".to_string(),
                testing_procedures: vec![
                    TestingProcedure {
                        procedure_id: "GDPR-30.1-T1".to_string(),
                        description: "Review completeness and accuracy of processing records".to_string(),
                        frequency: TestingFrequency::Quarterly,
                        method: TestingMethod::Documentation,
                        expected_result: "Processing records are complete and up-to-date".to_string(),
                        responsible_role: "Data Protection Officer".to_string(),
                    },
                ],
                maturity_level: MaturityLevel::Managed,
                automation_level: AutomationLevel::SemiAutomated,
                cost_impact: CostImpact::Medium,
                risk_reduction: 7.0,
                dependencies: vec![],
            },

            // Article 32 - Security of processing
            Control {
                id: Uuid::new_v4(),
                control_id: "GDPR-32.1".to_string(),
                name: "Technical and organizational measures".to_string(),
                description: "Implement appropriate technical and organizational measures".to_string(),
                framework: ComplianceFramework::GDPR,
                control_family: "Data Security".to_string(),
                control_type: ControlType::Technical,
                implementation_guidance: "Implement encryption, access controls, backup procedures, and other security measures appropriate to the risk.".to_string(),
                testing_procedures: vec![
                    TestingProcedure {
                        procedure_id: "GDPR-32.1-T1".to_string(),
                        description: "Test encryption implementation for data at rest and in transit".to_string(),
                        frequency: TestingFrequency::Monthly,
                        method: TestingMethod::SystemTest,
                        expected_result: "Data is properly encrypted using approved algorithms".to_string(),
                        responsible_role: "Security Team".to_string(),
                    },
                    TestingProcedure {
                        procedure_id: "GDPR-32.1-T2".to_string(),
                        description: "Review and test access control mechanisms".to_string(),
                        frequency: TestingFrequency::Monthly,
                        method: TestingMethod::SystemTest,
                        expected_result: "Access controls properly restrict data access to authorized personnel".to_string(),
                        responsible_role: "Security Team".to_string(),
                    },
                ],
                maturity_level: MaturityLevel::QuantitativelyManaged,
                automation_level: AutomationLevel::FullyAutomated,
                cost_impact: CostImpact::High,
                risk_reduction: 9.0,
                dependencies: vec![],
            },

            // Article 33 - Notification of breach to supervisory authority
            Control {
                id: Uuid::new_v4(),
                control_id: "GDPR-33.1".to_string(),
                name: "Breach notification to authority".to_string(),
                description: "Notify supervisory authority of data breaches within 72 hours".to_string(),
                framework: ComplianceFramework::GDPR,
                control_family: "Incident Response".to_string(),
                control_type: ControlType::Administrative,
                implementation_guidance: "Implement breach detection and notification procedures to ensure timely reporting to authorities.".to_string(),
                testing_procedures: vec![
                    TestingProcedure {
                        procedure_id: "GDPR-33.1-T1".to_string(),
                        description: "Test breach notification procedures through simulation".to_string(),
                        frequency: TestingFrequency::SemiAnnually,
                        method: TestingMethod::SystemTest,
                        expected_result: "Breach notification can be completed within required timeframes".to_string(),
                        responsible_role: "Incident Response Team".to_string(),
                    },
                ],
                maturity_level: MaturityLevel::Defined,
                automation_level: AutomationLevel::SemiAutomated,
                cost_impact: CostImpact::Medium,
                risk_reduction: 8.0,
                dependencies: vec![],
            },

            // Article 34 - Communication of breach to data subject
            Control {
                id: Uuid::new_v4(),
                control_id: "GDPR-34.1".to_string(),
                name: "Breach notification to data subjects".to_string(),
                description: "Communicate data breaches to affected data subjects when high risk".to_string(),
                framework: ComplianceFramework::GDPR,
                control_family: "Incident Response".to_string(),
                control_type: ControlType::Administrative,
                implementation_guidance: "Implement procedures to assess breach risk and notify affected individuals when required.".to_string(),
                testing_procedures: vec![
                    TestingProcedure {
                        procedure_id: "GDPR-34.1-T1".to_string(),
                        description: "Test data subject notification procedures".to_string(),
                        frequency: TestingFrequency::SemiAnnually,
                        method: TestingMethod::SystemTest,
                        expected_result: "Data subject notifications can be sent efficiently to affected individuals".to_string(),
                        responsible_role: "Communications Team".to_string(),
                    },
                ],
                maturity_level: MaturityLevel::Defined,
                automation_level: AutomationLevel::SemiAutomated,
                cost_impact: CostImpact::Medium,
                risk_reduction: 7.5,
                dependencies: vec![],
            },

            // Article 35 - Data protection impact assessment
            Control {
                id: Uuid::new_v4(),
                control_id: "GDPR-35.1".to_string(),
                name: "Data protection impact assessment".to_string(),
                description: "Conduct DPIA for high-risk processing activities".to_string(),
                framework: ComplianceFramework::GDPR,
                control_family: "Privacy Assessment".to_string(),
                control_type: ControlType::Administrative,
                implementation_guidance: "Implement DPIA procedures for identifying and assessing privacy risks in high-risk processing activities.".to_string(),
                testing_procedures: vec![
                    TestingProcedure {
                        procedure_id: "GDPR-35.1-T1".to_string(),
                        description: "Review DPIA procedures and documentation".to_string(),
                        frequency: TestingFrequency::Quarterly,
                        method: TestingMethod::Documentation,
                        expected_result: "DPIAs are conducted for all high-risk processing activities".to_string(),
                        responsible_role: "Privacy Team".to_string(),
                    },
                ],
                maturity_level: MaturityLevel::Defined,
                automation_level: AutomationLevel::Manual,
                cost_impact: CostImpact::Medium,
                risk_reduction: 8.5,
                dependencies: vec![],
            },

            // Data Subject Rights
            Control {
                id: Uuid::new_v4(),
                control_id: "GDPR-DSR.1".to_string(),
                name: "Data subject rights management".to_string(),
                description: "Implement procedures for handling data subject rights requests".to_string(),
                framework: ComplianceFramework::GDPR,
                control_family: "Data Subject Rights".to_string(),
                control_type: ControlType::Administrative,
                implementation_guidance: "Implement systems and procedures to handle access, rectification, erasure, portability, and objection requests.".to_string(),
                testing_procedures: vec![
                    TestingProcedure {
                        procedure_id: "GDPR-DSR.1-T1".to_string(),
                        description: "Test data subject request fulfillment processes".to_string(),
                        frequency: TestingFrequency::Monthly,
                        method: TestingMethod::SystemTest,
                        expected_result: "Data subject requests are processed within required timeframes".to_string(),
                        responsible_role: "Privacy Team".to_string(),
                    },
                ],
                maturity_level: MaturityLevel::QuantitativelyManaged,
                automation_level: AutomationLevel::FullyAutomated,
                cost_impact: CostImpact::High,
                risk_reduction: 8.0,
                dependencies: vec![],
            },
        ]
    }
}

impl FrameworkImplementation for GDPRFramework {
    fn get_framework_type(&self) -> ComplianceFramework {
        ComplianceFramework::GDPR
    }

    fn get_controls(&self) -> Vec<Control> {
        self.controls.clone()
    }

    fn get_control_mappings(&self) -> HashMap<String, Vec<String>> {
        let mut mappings = HashMap::new();

        mappings.insert("data_minimization".to_string(), vec![
            "GDPR-5.1".to_string(),
        ]);

        mappings.insert("purpose_limitation".to_string(), vec![
            "GDPR-5.1".to_string(),
            "GDPR-6.1".to_string(),
        ]);

        mappings.insert("consent_management".to_string(), vec![
            "GDPR-7.1".to_string(),
        ]);

        mappings.insert("privacy_by_design".to_string(), vec![
            "GDPR-25.1".to_string(),
        ]);

        mappings.insert("data_security".to_string(), vec![
            "GDPR-32.1".to_string(),
        ]);

        mappings.insert("breach_notification".to_string(), vec![
            "GDPR-33.1".to_string(),
            "GDPR-34.1".to_string(),
        ]);

        mappings.insert("privacy_impact_assessment".to_string(), vec![
            "GDPR-35.1".to_string(),
        ]);

        mappings.insert("data_subject_rights".to_string(), vec![
            "GDPR-DSR.1".to_string(),
        ]);

        mappings.insert("records_of_processing".to_string(), vec![
            "GDPR-30.1".to_string(),
        ]);

        mappings
    }

    fn assess_compliance(&self, implemented_controls: &[Control]) -> Result<f64> {
        let total_controls = self.controls.len() as f64;
        let implemented_count = implemented_controls.len() as f64;

        // Calculate weighted score based on control criticality
        let mut weighted_score = 0.0;
        let mut total_weight = 0.0;

        for control in &self.controls {
            let weight = match control.control_id.as_str() {
                "GDPR-32.1" | "GDPR-25.1" => 3.0,  // Critical controls
                "GDPR-33.1" | "GDPR-34.1" | "GDPR-35.1" => 2.5,  // High importance
                "GDPR-5.1" | "GDPR-6.1" | "GDPR-DSR.1" => 2.0,   // Important
                _ => 1.0,  // Standard
            };

            total_weight += weight;

            if implemented_controls.iter().any(|ic| ic.control_id == control.control_id) {
                weighted_score += weight;
            }
        }

        let compliance_percentage = if total_weight > 0.0 {
            (weighted_score / total_weight) * 100.0
        } else {
            0.0
        };

        Ok(compliance_percentage)
    }

    fn identify_gaps(&self, implemented_control_ids: &[String]) -> Vec<ComplianceGap> {
        let mut gaps = Vec::new();

        for control in &self.controls {
            if !implemented_control_ids.contains(&control.control_id) {
                let severity = match control.control_id.as_str() {
                    "GDPR-32.1" | "GDPR-25.1" => GapSeverity::Critical,
                    "GDPR-33.1" | "GDPR-34.1" | "GDPR-35.1" => GapSeverity::High,
                    "GDPR-5.1" | "GDPR-6.1" | "GDPR-DSR.1" => GapSeverity::Medium,
                    _ => GapSeverity::Low,
                };

                let risk_impact = match severity {
                    GapSeverity::Critical => RiskImpact::Catastrophic,
                    GapSeverity::High => RiskImpact::Major,
                    GapSeverity::Medium => RiskImpact::Moderate,
                    GapSeverity::Low => RiskImpact::Minor,
                };

                let effort = match control.cost_impact {
                    CostImpact::Low => RemediationEffort::Low,
                    CostImpact::Medium => RemediationEffort::Medium,
                    CostImpact::High => RemediationEffort::High,
                    CostImpact::VeryHigh => RemediationEffort::VeryHigh,
                };

                gaps.push(ComplianceGap {
                    id: Uuid::new_v4(),
                    severity,
                    framework: ComplianceFramework::GDPR,
                    control_id: control.control_id.clone(),
                    description: format!("Missing implementation: {}", control.name),
                    risk_impact,
                    remediation_effort: effort,
                    due_date: Some(Utc::now() + chrono::Duration::days(match severity {
                        GapSeverity::Critical => 30,
                        GapSeverity::High => 60,
                        GapSeverity::Medium => 90,
                        GapSeverity::Low => 180,
                    })),
                    responsible_party: "Privacy Team".to_string(),
                    status: GapStatus::Identified,
                });
            }
        }

        gaps
    }

    fn get_implementation_guidance(&self, control_id: &str) -> Option<String> {
        self.controls
            .iter()
            .find(|c| c.control_id == control_id)
            .map(|c| c.implementation_guidance.clone())
    }

    fn get_testing_procedures(&self, control_id: &str) -> Vec<TestingProcedure> {
        self.controls
            .iter()
            .find(|c| c.control_id == control_id)
            .map(|c| c.testing_procedures.clone())
            .unwrap_or_default()
    }
}

pub struct GDPRAssessment;

impl GDPRAssessment {
    pub fn conduct_dpia(
        processing_activity: &str,
        data_categories: &[String],
        processing_purposes: &[String],
        legal_basis: &str,
    ) -> DPIAResult {
        let mut risk_score = 0.0;
        let mut high_risk_factors = Vec::new();

        // Assess risk based on data categories
        for category in data_categories {
            match category.to_lowercase().as_str() {
                "health" | "biometric" | "genetic" => {
                    risk_score += 3.0;
                    high_risk_factors.push("Special category data (health/biometric/genetic)".to_string());
                },
                "financial" | "criminal" => {
                    risk_score += 2.5;
                    high_risk_factors.push("Sensitive financial or criminal data".to_string());
                },
                "children" | "minors" => {
                    risk_score += 2.0;
                    high_risk_factors.push("Data concerning children".to_string());
                },
                "location" | "tracking" => {
                    risk_score += 1.5;
                    high_risk_factors.push("Location or tracking data".to_string());
                },
                _ => risk_score += 0.5,
            }
        }

        // Assess risk based on processing purposes
        for purpose in processing_purposes {
            match purpose.to_lowercase().as_str() {
                "profiling" | "automated_decision" => {
                    risk_score += 2.5;
                    high_risk_factors.push("Automated decision making or profiling".to_string());
                },
                "surveillance" | "monitoring" => {
                    risk_score += 2.0;
                    high_risk_factors.push("Systematic monitoring or surveillance".to_string());
                },
                "large_scale" => {
                    risk_score += 1.5;
                    high_risk_factors.push("Large scale processing".to_string());
                },
                _ => risk_score += 0.5,
            }
        }

        let requires_dpia = risk_score >= 3.0 || high_risk_factors.len() >= 2;

        DPIAResult {
            processing_activity: processing_activity.to_string(),
            risk_score,
            requires_dpia,
            high_risk_factors,
            recommendations: Self::generate_dpia_recommendations(risk_score, &high_risk_factors),
        }
    }

    fn generate_dpia_recommendations(risk_score: f64, risk_factors: &[String]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if risk_score >= 4.0 {
            recommendations.push("Conduct full DPIA with external privacy expert consultation".to_string());
            recommendations.push("Consider alternative processing methods to reduce risk".to_string());
            recommendations.push("Implement additional safeguards and monitoring".to_string());
        } else if risk_score >= 3.0 {
            recommendations.push("Conduct internal DPIA assessment".to_string());
            recommendations.push("Document risk mitigation measures".to_string());
        }

        if risk_factors.iter().any(|f| f.contains("automated decision")) {
            recommendations.push("Implement meaningful human review for automated decisions".to_string());
            recommendations.push("Provide clear information about decision logic to data subjects".to_string());
        }

        if risk_factors.iter().any(|f| f.contains("children")) {
            recommendations.push("Implement age verification mechanisms".to_string());
            recommendations.push("Obtain parental consent where required".to_string());
        }

        if risk_factors.iter().any(|f| f.contains("Special category")) {
            recommendations.push("Ensure explicit consent or other valid legal basis for special category data".to_string());
            recommendations.push("Implement enhanced security measures for special category data".to_string());
        }

        recommendations
    }
}

#[derive(Debug, Clone)]
pub struct DPIAResult {
    pub processing_activity: String,
    pub risk_score: f64,
    pub requires_dpia: bool,
    pub high_risk_factors: Vec<String>,
    pub recommendations: Vec<String>,
}