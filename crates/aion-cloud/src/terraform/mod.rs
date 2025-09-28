pub mod generator;
pub mod parser;
pub mod validator;

pub use generator::*;
pub use parser::*;
pub use validator::*;

use crate::{DeploymentTemplate, ResourceDefinition, CloudProvider, VariableDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformConfig {
    pub terraform: TerraformBlock,
    pub provider: HashMap<String, ProviderConfig>,
    pub variable: HashMap<String, TerraformVariable>,
    pub resource: HashMap<String, HashMap<String, TerraformResource>>,
    pub output: HashMap<String, TerraformOutput>,
    pub data: Option<HashMap<String, HashMap<String, TerraformData>>>,
    pub locals: Option<HashMap<String, serde_json::Value>>,
    pub module: Option<HashMap<String, TerraformModule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformBlock {
    pub required_version: Option<String>,
    pub required_providers: Option<HashMap<String, RequiredProvider>>,
    pub backend: Option<HashMap<String, serde_json::Value>>,
    pub experiments: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredProvider {
    pub source: String,
    pub version: Option<String>,
    pub configuration_aliases: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub region: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub token: Option<String>,
    pub project: Option<String>,
    pub credentials: Option<String>,
    pub subscription_id: Option<String>,
    pub tenant_id: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub alias: Option<String>,
    #[serde(flatten)]
    pub additional_config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformVariable {
    pub description: Option<String>,
    pub r#type: Option<String>,
    pub default: Option<serde_json::Value>,
    pub validation: Option<Vec<VariableValidation>>,
    pub sensitive: Option<bool>,
    pub nullable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableValidation {
    pub condition: String,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformResource {
    #[serde(flatten)]
    pub config: HashMap<String, serde_json::Value>,
    pub lifecycle: Option<LifecycleBlock>,
    pub depends_on: Option<Vec<String>>,
    pub count: Option<serde_json::Value>,
    pub for_each: Option<serde_json::Value>,
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleBlock {
    pub create_before_destroy: Option<bool>,
    pub prevent_destroy: Option<bool>,
    pub ignore_changes: Option<Vec<String>>,
    pub replace_triggered_by: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformOutput {
    pub value: serde_json::Value,
    pub description: Option<String>,
    pub sensitive: Option<bool>,
    pub depends_on: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformData {
    #[serde(flatten)]
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformModule {
    pub source: String,
    pub version: Option<String>,
    #[serde(flatten)]
    pub variables: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformPlan {
    pub format_version: String,
    pub terraform_version: String,
    pub variables: HashMap<String, serde_json::Value>,
    pub planned_values: PlannedValues,
    pub resource_changes: Vec<ResourceChange>,
    pub configuration: TerraformConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedValues {
    pub root_module: RootModule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootModule {
    pub resources: Vec<PlannedResource>,
    pub child_modules: Option<Vec<ChildModule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedResource {
    pub address: String,
    pub mode: String,
    pub r#type: String,
    pub name: String,
    pub provider_name: String,
    pub schema_version: u32,
    pub values: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildModule {
    pub address: String,
    pub resources: Vec<PlannedResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceChange {
    pub address: String,
    pub module_address: Option<String>,
    pub mode: String,
    pub r#type: String,
    pub name: String,
    pub provider_name: String,
    pub change: Change,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub actions: Vec<String>,
    pub before: Option<serde_json::Value>,
    pub after: Option<serde_json::Value>,
    pub after_unknown: Option<serde_json::Value>,
    pub before_sensitive: Option<serde_json::Value>,
    pub after_sensitive: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformConfiguration {
    pub provider_config: HashMap<String, ProviderConfig>,
    pub root_module: ConfigurationModule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationModule {
    pub resources: Vec<ConfigurationResource>,
    pub variables: HashMap<String, TerraformVariable>,
    pub outputs: HashMap<String, TerraformOutput>,
    pub module_calls: Option<HashMap<String, ModuleCall>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationResource {
    pub address: String,
    pub mode: String,
    pub r#type: String,
    pub name: String,
    pub provider_config_key: String,
    pub expressions: serde_json::Value,
    pub schema_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleCall {
    pub source: String,
    pub expressions: serde_json::Value,
}

pub trait TerraformGenerator {
    fn generate_terraform(&self, template: &DeploymentTemplate) -> crate::Result<TerraformConfig>;
    fn generate_provider_config(&self, provider: CloudProvider) -> ProviderConfig;
    fn generate_variables(&self, variables: &HashMap<String, VariableDefinition>) -> HashMap<String, TerraformVariable>;
    fn generate_resources(&self, resources: &[ResourceDefinition]) -> HashMap<String, HashMap<String, TerraformResource>>;
    fn generate_outputs(&self, outputs: &HashMap<String, String>) -> HashMap<String, TerraformOutput>;
}

pub trait TerraformParser {
    fn parse_terraform_file(&self, content: &str) -> crate::Result<TerraformConfig>;
    fn parse_terraform_plan(&self, content: &str) -> crate::Result<TerraformPlan>;
    fn extract_resources(&self, config: &TerraformConfig) -> Vec<ResourceDefinition>;
    fn extract_variables(&self, config: &TerraformConfig) -> HashMap<String, VariableDefinition>;
}

pub trait TerraformValidator {
    fn validate_syntax(&self, content: &str) -> crate::Result<Vec<ValidationError>>;
    fn validate_configuration(&self, config: &TerraformConfig) -> crate::Result<Vec<ValidationError>>;
    fn validate_plan(&self, plan: &TerraformPlan) -> crate::Result<Vec<ValidationError>>;
    fn check_security_issues(&self, config: &TerraformConfig) -> crate::Result<Vec<SecurityIssue>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub severity: Severity,
    pub message: String,
    pub location: Option<Location>,
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub severity: Severity,
    pub message: String,
    pub location: Option<Location>,
    pub recommendation: String,
    pub cwe_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub filename: String,
    pub line: u32,
    pub column: u32,
}

pub struct TerraformFormatter;

impl TerraformFormatter {
    pub fn format_hcl(&self, config: &TerraformConfig) -> crate::Result<String> {
        // Convert TerraformConfig to HCL format
        let mut hcl_content = String::new();

        // Add terraform block
        if let Some(terraform_block) = self.format_terraform_block(&config.terraform) {
            hcl_content.push_str(&terraform_block);
            hcl_content.push_str("\n\n");
        }

        // Add provider blocks
        for (provider_name, provider_config) in &config.provider {
            let provider_block = self.format_provider_block(provider_name, provider_config);
            hcl_content.push_str(&provider_block);
            hcl_content.push_str("\n\n");
        }

        // Add variable blocks
        for (var_name, var_config) in &config.variable {
            let var_block = self.format_variable_block(var_name, var_config);
            hcl_content.push_str(&var_block);
            hcl_content.push_str("\n\n");
        }

        // Add resource blocks
        for (resource_type, resources) in &config.resource {
            for (resource_name, resource_config) in resources {
                let resource_block = self.format_resource_block(resource_type, resource_name, resource_config);
                hcl_content.push_str(&resource_block);
                hcl_content.push_str("\n\n");
            }
        }

        // Add output blocks
        for (output_name, output_config) in &config.output {
            let output_block = self.format_output_block(output_name, output_config);
            hcl_content.push_str(&output_block);
            hcl_content.push_str("\n\n");
        }

        Ok(hcl_content)
    }

    fn format_terraform_block(&self, terraform: &TerraformBlock) -> Option<String> {
        let mut content = String::from("terraform {\n");

        if let Some(version) = &terraform.required_version {
            content.push_str(&format!("  required_version = \"{}\"\n", version));
        }

        if let Some(providers) = &terraform.required_providers {
            content.push_str("  required_providers {\n");
            for (name, provider) in providers {
                content.push_str(&format!("    {} = {{\n", name));
                content.push_str(&format!("      source = \"{}\"\n", provider.source));
                if let Some(version) = &provider.version {
                    content.push_str(&format!("      version = \"{}\"\n", version));
                }
                content.push_str("    }\n");
            }
            content.push_str("  }\n");
        }

        content.push_str("}");
        Some(content)
    }

    fn format_provider_block(&self, name: &str, config: &ProviderConfig) -> String {
        let mut content = format!("provider \"{}\" {{\n", name);

        if let Some(region) = &config.region {
            content.push_str(&format!("  region = \"{}\"\n", region));
        }

        if let Some(project) = &config.project {
            content.push_str(&format!("  project = \"{}\"\n", project));
        }

        if let Some(subscription_id) = &config.subscription_id {
            content.push_str(&format!("  subscription_id = \"{}\"\n", subscription_id));
        }

        content.push_str("}");
        content
    }

    fn format_variable_block(&self, name: &str, config: &TerraformVariable) -> String {
        let mut content = format!("variable \"{}\" {{\n", name);

        if let Some(description) = &config.description {
            content.push_str(&format!("  description = \"{}\"\n", description));
        }

        if let Some(var_type) = &config.r#type {
            content.push_str(&format!("  type = {}\n", var_type));
        }

        if let Some(default) = &config.default {
            content.push_str(&format!("  default = {}\n", serde_json::to_string(default).unwrap_or_default()));
        }

        if let Some(sensitive) = config.sensitive {
            content.push_str(&format!("  sensitive = {}\n", sensitive));
        }

        content.push_str("}");
        content
    }

    fn format_resource_block(&self, resource_type: &str, name: &str, config: &TerraformResource) -> String {
        let mut content = format!("resource \"{}\" \"{}\" {{\n", resource_type, name);

        for (key, value) in &config.config {
            content.push_str(&format!("  {} = {}\n", key, self.format_value(value)));
        }

        content.push_str("}");
        content
    }

    fn format_output_block(&self, name: &str, config: &TerraformOutput) -> String {
        let mut content = format!("output \"{}\" {{\n", name);

        content.push_str(&format!("  value = {}\n", self.format_value(&config.value)));

        if let Some(description) = &config.description {
            content.push_str(&format!("  description = \"{}\"\n", description));
        }

        if let Some(sensitive) = config.sensitive {
            content.push_str(&format!("  sensitive = {}\n", sensitive));
        }

        content.push_str("}");
        content
    }

    fn format_value(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => format!("\"{}\"", s),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.format_value(v)).collect();
                format!("[{}]", items.join(", "))
            },
            serde_json::Value::Object(obj) => {
                let items: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("{} = {}", k, self.format_value(v)))
                    .collect();
                format!("{{\n    {}\n  }}", items.join("\n    "))
            },
            serde_json::Value::Null => "null".to_string(),
        }
    }
}