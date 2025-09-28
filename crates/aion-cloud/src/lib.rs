pub mod providers;
pub mod terraform;
pub mod kubernetes;
pub mod monitoring;
pub mod deployment;
pub mod security;

pub use providers::*;
pub use terraform::*;
pub use kubernetes::*;
pub use monitoring::*;
pub use deployment::*;
pub use security::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloudProvider {
    AWS,
    GCP,
    Azure,
    DigitalOcean,
    Kubernetes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudCredentials {
    pub provider: CloudProvider,
    pub credentials: HashMap<String, String>,
    pub region: Option<String>,
    pub project_id: Option<String>,
    pub subscription_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudResource {
    pub id: Uuid,
    pub provider: CloudProvider,
    pub resource_type: String,
    pub name: String,
    pub region: String,
    pub status: ResourceStatus,
    pub tags: HashMap<String, String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceStatus {
    Creating,
    Running,
    Stopped,
    Deleting,
    Error,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub provider: CloudProvider,
    pub template_type: TemplateType,
    pub resources: Vec<ResourceDefinition>,
    pub variables: HashMap<String, VariableDefinition>,
    pub outputs: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateType {
    Terraform,
    CloudFormation,
    ARM,
    Kubernetes,
    Pulumi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDefinition {
    pub name: String,
    pub resource_type: String,
    pub properties: serde_json::Value,
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDefinition {
    pub description: String,
    pub variable_type: String,
    pub default: Option<serde_json::Value>,
    pub required: bool,
    pub sensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentPlan {
    pub id: Uuid,
    pub template_id: Uuid,
    pub variables: HashMap<String, serde_json::Value>,
    pub target_environment: String,
    pub estimated_cost: Option<f64>,
    pub resources_to_create: Vec<ResourceDefinition>,
    pub resources_to_update: Vec<ResourceDefinition>,
    pub resources_to_delete: Vec<String>,
    pub created_at: DateTime<Utc>,
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[async_trait::async_trait]
pub trait CloudProviderInterface {
    async fn authenticate(&self, credentials: &CloudCredentials) -> Result<()>;
    async fn list_resources(&self) -> Result<Vec<CloudResource>>;
    async fn create_resource(&self, definition: &ResourceDefinition) -> Result<CloudResource>;
    async fn update_resource(&self, id: &str, definition: &ResourceDefinition) -> Result<CloudResource>;
    async fn delete_resource(&self, id: &str) -> Result<()>;
    async fn get_resource_status(&self, id: &str) -> Result<ResourceStatus>;
    async fn estimate_cost(&self, plan: &DeploymentPlan) -> Result<f64>;
    async fn deploy_template(&self, template: &DeploymentTemplate, variables: HashMap<String, serde_json::Value>) -> Result<String>;
}