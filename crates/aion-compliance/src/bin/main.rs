use aion_compliance::{
    ComplianceFramework, ComplianceProject, OrganizationInfo, Industry, OrganizationSize,
    Region, ContactInfo, frameworks::FrameworkRegistry, ComplianceManager, ReportFormat,
    NotificationType, Result
};
use clap::{App, Arg, SubCommand};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::init();

    let matches = App::new("AION Compliance Manager")
        .version("1.0.0")
        .author("AION Team <team@aion.dev>")
        .about("Comprehensive compliance management for GDPR, HIPAA, SOX, PCI-DSS and more")
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize a new compliance project")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .value_name("NAME")
                        .help("Project name")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("frameworks")
                        .short("f")
                        .long("frameworks")
                        .value_name("FRAMEWORKS")
                        .help("Compliance frameworks (gdpr,hipaa,sox,pci-dss)")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("industry")
                        .short("i")
                        .long("industry")
                        .value_name("INDUSTRY")
                        .help("Industry type")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("size")
                        .short("s")
                        .long("size")
                        .value_name("SIZE")
                        .help("Organization size (startup,small,medium,large,enterprise)")
                        .takes_value(true)
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("assess")
                .about("Assess compliance for a project")
                .arg(
                    Arg::with_name("project")
                        .short("p")
                        .long("project")
                        .value_name("PROJECT_ID")
                        .help("Project ID")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("framework")
                        .short("f")
                        .long("framework")
                        .value_name("FRAMEWORK")
                        .help("Specific framework to assess")
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("gaps")
                .about("Identify compliance gaps")
                .arg(
                    Arg::with_name("project")
                        .short("p")
                        .long("project")
                        .value_name("PROJECT_ID")
                        .help("Project ID")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("framework")
                        .short("f")
                        .long("framework")
                        .value_name("FRAMEWORK")
                        .help("Framework to analyze")
                        .takes_value(true)
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("report")
                .about("Generate compliance report")
                .arg(
                    Arg::with_name("project")
                        .short("p")
                        .long("project")
                        .value_name("PROJECT_ID")
                        .help("Project ID")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("format")
                        .short("f")
                        .long("format")
                        .value_name("FORMAT")
                        .help("Report format (pdf,excel,json)")
                        .takes_value(true)
                        .default_value("pdf")
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("FILE")
                        .help("Output file path")
                        .takes_value(true)
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("frameworks")
                .about("List available compliance frameworks")
        )
        .subcommand(
            SubCommand::with_name("controls")
                .about("List controls for a framework")
                .arg(
                    Arg::with_name("framework")
                        .short("f")
                        .long("framework")
                        .value_name("FRAMEWORK")
                        .help("Framework name")
                        .takes_value(true)
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("compare")
                .about("Compare two compliance frameworks")
                .arg(
                    Arg::with_name("framework1")
                        .long("first")
                        .value_name("FRAMEWORK1")
                        .help("First framework")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("framework2")
                        .long("second")
                        .value_name("FRAMEWORK2")
                        .help("Second framework")
                        .takes_value(true)
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("dpia")
                .about("Conduct Data Protection Impact Assessment (GDPR)")
                .arg(
                    Arg::with_name("activity")
                        .short("a")
                        .long("activity")
                        .value_name("ACTIVITY")
                        .help("Processing activity description")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("data-types")
                        .short("d")
                        .long("data-types")
                        .value_name("TYPES")
                        .help("Comma-separated data types")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("purposes")
                        .short("p")
                        .long("purposes")
                        .value_name("PURPOSES")
                        .help("Comma-separated processing purposes")
                        .takes_value(true)
                        .required(true)
                )
        )
        .get_matches();

    let registry = FrameworkRegistry::new();

    match matches.subcommand() {
        ("init", Some(init_matches)) => {
            let name = init_matches.value_of("name").unwrap();
            let frameworks_str = init_matches.value_of("frameworks").unwrap();
            let industry_str = init_matches.value_of("industry").unwrap();
            let size_str = init_matches.value_of("size").unwrap();

            create_project(name, frameworks_str, industry_str, size_str).await?;
        },
        ("assess", Some(assess_matches)) => {
            let project_id = assess_matches.value_of("project").unwrap();
            let framework = assess_matches.value_of("framework");

            assess_compliance(project_id, framework, &registry).await?;
        },
        ("gaps", Some(gaps_matches)) => {
            let project_id = gaps_matches.value_of("project").unwrap();
            let framework_str = gaps_matches.value_of("framework").unwrap();

            analyze_gaps(project_id, framework_str, &registry).await?;
        },
        ("report", Some(report_matches)) => {
            let project_id = report_matches.value_of("project").unwrap();
            let format_str = report_matches.value_of("format").unwrap();
            let output_path = report_matches.value_of("output").unwrap();

            generate_report(project_id, format_str, output_path).await?;
        },
        ("frameworks", Some(_)) => {
            list_frameworks(&registry);
        },
        ("controls", Some(controls_matches)) => {
            let framework_str = controls_matches.value_of("framework").unwrap();
            list_controls(framework_str, &registry)?;
        },
        ("compare", Some(compare_matches)) => {
            let framework1_str = compare_matches.value_of("framework1").unwrap();
            let framework2_str = compare_matches.value_of("framework2").unwrap();

            compare_frameworks(framework1_str, framework2_str, &registry)?;
        },
        ("dpia", Some(dpia_matches)) => {
            let activity = dpia_matches.value_of("activity").unwrap();
            let data_types_str = dpia_matches.value_of("data-types").unwrap();
            let purposes_str = dpia_matches.value_of("purposes").unwrap();

            conduct_dpia(activity, data_types_str, purposes_str)?;
        },
        _ => {
            eprintln!("Invalid command. Use --help for usage information.");
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn create_project(
    name: &str,
    frameworks_str: &str,
    industry_str: &str,
    size_str: &str,
) -> Result<()> {
    println!("Creating compliance project: {}", name);

    let frameworks = parse_frameworks(frameworks_str)?;
    let industry = parse_industry(industry_str)?;
    let size = parse_organization_size(size_str)?;

    let project = ComplianceProject {
        id: Uuid::new_v4(),
        name: name.to_string(),
        description: format!("Compliance project for {}", name),
        frameworks,
        organization: OrganizationInfo {
            name: name.to_string(),
            industry,
            size,
            regions: vec![Region::US], // Default
            contact_info: ContactInfo {
                dpo_email: None,
                privacy_officer_email: None,
                security_officer_email: None,
                compliance_officer_email: None,
                legal_contact_email: None,
                incident_response_email: "incident@company.com".to_string(),
            },
            regulatory_requirements: Vec::new(),
        },
        data_categories: Vec::new(),
        systems: Vec::new(),
        policies: Vec::new(),
        controls: Vec::new(),
        assessments: Vec::new(),
        audits: Vec::new(),
        incidents: Vec::new(),
        created_at: Utc::now(),
        last_updated: Utc::now(),
        compliance_status: aion_compliance::ComplianceStatus {
            overall_score: 0.0,
            framework_scores: HashMap::new(),
            critical_gaps: Vec::new(),
            improvement_recommendations: Vec::new(),
            next_assessment_due: Utc::now() + chrono::Duration::days(30),
            certification_status: Vec::new(),
        },
    };

    // Save project to file
    let project_json = serde_json::to_string_pretty(&project)?;
    let filename = format!("{}-compliance-project.json", name.replace(" ", "_").to_lowercase());
    fs::write(&filename, project_json)?;

    println!("Project created successfully!");
    println!("Project ID: {}", project.id);
    println!("Saved to: {}", filename);
    println!("Frameworks: {:?}", project.frameworks);

    Ok(())
}

async fn assess_compliance(
    project_id: &str,
    framework: Option<&str>,
    registry: &FrameworkRegistry,
) -> Result<()> {
    println!("Assessing compliance for project: {}", project_id);

    // In a real implementation, this would load the project from a database
    // For now, we'll demonstrate with the available frameworks

    if let Some(framework_str) = framework {
        let framework_type = parse_framework(framework_str)?;

        if let Some(framework_impl) = registry.get_framework(&framework_type) {
            let controls = framework_impl.get_controls();
            let score = framework_impl.assess_compliance(&controls)?;

            println!("\nFramework: {:?}", framework_type);
            println!("Total Controls: {}", controls.len());
            println!("Compliance Score: {:.1}%", score);

            if score >= 95.0 {
                println!("Status: Excellent ✅");
            } else if score >= 85.0 {
                println!("Status: Good ✅");
            } else if score >= 70.0 {
                println!("Status: Satisfactory ⚠️");
            } else if score >= 50.0 {
                println!("Status: Needs Improvement ⚠️");
            } else {
                println!("Status: Poor ❌");
            }
        }
    } else {
        // Assess all frameworks
        println!("\nAssessing all available frameworks:");

        for framework_type in registry.get_all_frameworks() {
            if let Some(framework_impl) = registry.get_framework(&framework_type) {
                let controls = framework_impl.get_controls();
                let score = framework_impl.assess_compliance(&controls)?;

                println!("\n{:?}:", framework_type);
                println!("  Controls: {}", controls.len());
                println!("  Score: {:.1}%", score);
            }
        }
    }

    Ok(())
}

async fn analyze_gaps(
    project_id: &str,
    framework_str: &str,
    registry: &FrameworkRegistry,
) -> Result<()> {
    println!("Analyzing compliance gaps for project: {}", project_id);

    let framework_type = parse_framework(framework_str)?;

    if let Some(framework_impl) = registry.get_framework(&framework_type) {
        // For demonstration, assume no controls are implemented
        let implemented_controls = vec![];
        let gaps = framework_impl.identify_gaps(&implemented_controls);

        println!("\nGap Analysis for {:?}:", framework_type);
        println!("Total Gaps Found: {}", gaps.len());

        let critical_gaps = gaps.iter().filter(|g| matches!(g.severity, aion_compliance::GapSeverity::Critical)).count();
        let high_gaps = gaps.iter().filter(|g| matches!(g.severity, aion_compliance::GapSeverity::High)).count();
        let medium_gaps = gaps.iter().filter(|g| matches!(g.severity, aion_compliance::GapSeverity::Medium)).count();
        let low_gaps = gaps.iter().filter(|g| matches!(g.severity, aion_compliance::GapSeverity::Low)).count();

        println!("  Critical: {} ❌", critical_gaps);
        println!("  High: {} ⚠️", high_gaps);
        println!("  Medium: {} ⚠️", medium_gaps);
        println!("  Low: {} ℹ️", low_gaps);

        println!("\nTop Priority Gaps:");
        for gap in gaps.iter().take(5) {
            println!("  • {} ({}): {}", gap.control_id, format!("{:?}", gap.severity), gap.description);
        }

        if let Some(due_date) = gaps.iter().filter_map(|g| g.due_date).min() {
            println!("\nNext Deadline: {}", due_date.format("%Y-%m-%d"));
        }
    }

    Ok(())
}

async fn generate_report(
    project_id: &str,
    format_str: &str,
    output_path: &str,
) -> Result<()> {
    println!("Generating compliance report for project: {}", project_id);

    let format = match format_str.to_lowercase().as_str() {
        "pdf" => ReportFormat::PDF,
        "excel" => ReportFormat::Excel,
        "json" => ReportFormat::JSON,
        "html" => ReportFormat::HTML,
        _ => return Err("Invalid report format".into()),
    };

    // For demonstration, create a simple JSON report
    let report_data = serde_json::json!({
        "project_id": project_id,
        "generated_at": Utc::now(),
        "format": format_str,
        "summary": {
            "overall_score": 75.5,
            "total_controls": 45,
            "implemented_controls": 34,
            "critical_gaps": 2,
            "status": "Needs Improvement"
        },
        "frameworks": {
            "GDPR": {
                "score": 80.0,
                "status": "Good",
                "gaps": 3
            },
            "HIPAA": {
                "score": 70.0,
                "status": "Satisfactory",
                "gaps": 5
            }
        }
    });

    match format {
        ReportFormat::JSON => {
            fs::write(output_path, serde_json::to_string_pretty(&report_data)?)?;
        },
        _ => {
            // For other formats, create a simple text version
            let content = format!(
                "AION Compliance Report\n===================\n\nProject: {}\nGenerated: {}\n\n{}",
                project_id,
                Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
                serde_json::to_string_pretty(&report_data)?
            );
            fs::write(output_path, content)?;
        }
    }

    println!("Report generated: {}", output_path);
    Ok(())
}

fn list_frameworks(registry: &FrameworkRegistry) {
    println!("Available Compliance Frameworks:");
    println!("================================");

    for framework in registry.get_all_frameworks() {
        if let Some(framework_impl) = registry.get_framework(&framework) {
            let controls = framework_impl.get_controls();
            println!("• {:?}", framework);
            println!("  Controls: {}", controls.len());
            println!("  Description: {}", get_framework_description(&framework));
            println!();
        }
    }
}

fn list_controls(framework_str: &str, registry: &FrameworkRegistry) -> Result<()> {
    let framework_type = parse_framework(framework_str)?;

    if let Some(framework_impl) = registry.get_framework(&framework_type) {
        let controls = framework_impl.get_controls();

        println!("Controls for {:?}:", framework_type);
        println!("===================");

        for control in controls {
            println!("• {} - {}", control.control_id, control.name);
            println!("  Family: {}", control.control_family);
            println!("  Type: {:?}", control.control_type);
            println!("  Maturity: {:?}", control.maturity_level);
            println!("  Cost Impact: {:?}", control.cost_impact);
            println!();
        }
    }

    Ok(())
}

fn compare_frameworks(
    framework1_str: &str,
    framework2_str: &str,
    registry: &FrameworkRegistry,
) -> Result<()> {
    let framework1 = parse_framework(framework1_str)?;
    let framework2 = parse_framework(framework2_str)?;

    let comparison = aion_compliance::frameworks::FrameworkComparator::compare_frameworks(
        framework1.clone(),
        framework2.clone(),
        registry,
    );

    println!("Framework Comparison: {:?} vs {:?}", framework1, framework2);
    println!("=============================================");
    println!("Common Controls: {}", comparison.common_controls);
    println!("Unique to {:?}: {}", framework1, comparison.unique_to_first);
    println!("Unique to {:?}: {}", framework2, comparison.unique_to_second);
    println!("Similarity Score: {:.1}%", comparison.similarity_score * 100.0);
    println!("Complexity Difference: {:.1}", comparison.complexity_comparison.difference);

    Ok(())
}

fn conduct_dpia(activity: &str, data_types_str: &str, purposes_str: &str) -> Result<()> {
    let data_types: Vec<String> = data_types_str.split(',').map(|s| s.trim().to_string()).collect();
    let purposes: Vec<String> = purposes_str.split(',').map(|s| s.trim().to_string()).collect();

    let dpia_result = aion_compliance::frameworks::gdpr::GDPRAssessment::conduct_dpia(
        activity,
        &data_types,
        &purposes,
        "consent",
    );

    println!("Data Protection Impact Assessment");
    println!("===============================");
    println!("Activity: {}", dpia_result.processing_activity);
    println!("Risk Score: {:.1}/10", dpia_result.risk_score);
    println!("DPIA Required: {}", if dpia_result.requires_dpia { "Yes ⚠️" } else { "No ✅" });

    if !dpia_result.high_risk_factors.is_empty() {
        println!("\nHigh Risk Factors:");
        for factor in &dpia_result.high_risk_factors {
            println!("  • {}", factor);
        }
    }

    if !dpia_result.recommendations.is_empty() {
        println!("\nRecommendations:");
        for recommendation in &dpia_result.recommendations {
            println!("  • {}", recommendation);
        }
    }

    Ok(())
}

fn parse_frameworks(frameworks_str: &str) -> Result<Vec<ComplianceFramework>> {
    let mut frameworks = Vec::new();

    for framework_str in frameworks_str.split(',') {
        frameworks.push(parse_framework(framework_str.trim())?);
    }

    Ok(frameworks)
}

fn parse_framework(framework_str: &str) -> Result<ComplianceFramework> {
    match framework_str.to_lowercase().as_str() {
        "gdpr" => Ok(ComplianceFramework::GDPR),
        "hipaa" => Ok(ComplianceFramework::HIPAA),
        "sox" => Ok(ComplianceFramework::SOX),
        "pci-dss" | "pcidss" => Ok(ComplianceFramework::PCIDSS),
        "iso27001" | "iso-27001" => Ok(ComplianceFramework::ISO27001),
        "nist" => Ok(ComplianceFramework::NIST),
        "ccpa" => Ok(ComplianceFramework::CCPA),
        _ => Err(format!("Unknown framework: {}", framework_str).into()),
    }
}

fn parse_industry(industry_str: &str) -> Result<Industry> {
    match industry_str.to_lowercase().as_str() {
        "healthcare" => Ok(Industry::Healthcare),
        "financial" => Ok(Industry::Financial),
        "technology" | "tech" => Ok(Industry::Technology),
        "retail" => Ok(Industry::Retail),
        "manufacturing" => Ok(Industry::Manufacturing),
        "education" => Ok(Industry::Education),
        "government" => Ok(Industry::Government),
        "nonprofit" => Ok(Industry::NonProfit),
        _ => Ok(Industry::Other(industry_str.to_string())),
    }
}

fn parse_organization_size(size_str: &str) -> Result<OrganizationSize> {
    match size_str.to_lowercase().as_str() {
        "startup" => Ok(OrganizationSize::Startup),
        "small" => Ok(OrganizationSize::Small),
        "medium" => Ok(OrganizationSize::Medium),
        "large" => Ok(OrganizationSize::Large),
        "enterprise" => Ok(OrganizationSize::Enterprise),
        _ => Err(format!("Unknown organization size: {}", size_str).into()),
    }
}

fn get_framework_description(framework: &ComplianceFramework) -> &'static str {
    match framework {
        ComplianceFramework::GDPR => "EU General Data Protection Regulation - Privacy and data protection",
        ComplianceFramework::HIPAA => "Health Insurance Portability and Accountability Act - Healthcare data protection",
        ComplianceFramework::SOX => "Sarbanes-Oxley Act - Financial reporting and corporate governance",
        ComplianceFramework::PCIDSS => "Payment Card Industry Data Security Standard - Payment card data protection",
        ComplianceFramework::ISO27001 => "ISO/IEC 27001 - Information security management systems",
        ComplianceFramework::NIST => "NIST Cybersecurity Framework - Cybersecurity risk management",
        ComplianceFramework::CCPA => "California Consumer Privacy Act - Consumer privacy rights",
        ComplianceFramework::PIPEDA => "Personal Information Protection and Electronic Documents Act - Canadian privacy law",
        ComplianceFramework::LGPD => "Lei Geral de Proteção de Dados - Brazilian data protection law",
        ComplianceFramework::Custom(_) => "Custom compliance framework",
    }
}