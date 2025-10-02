pub mod deployment;
pub mod service;
pub mod ingress;
pub mod configmap;
pub mod secret;
pub mod namespace;

pub use deployment::*;
pub use service::*;
pub use ingress::*;
pub use configmap::*;
pub use secret::*;
pub use namespace::*;

use k8s_openapi::api::apps::v1::{Deployment, DaemonSet, StatefulSet};
use k8s_openapi::api::core::v1::{Service, ConfigMap, Secret, Namespace, Pod, PersistentVolumeClaim};
use k8s_openapi::api::networking::v1::Ingress;
use k8s_openapi::api::batch::v1::{Job, CronJob};
use k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscaler;
use kube::{Client, Api, config::Config as KubeConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesCluster {
    pub id: Uuid,
    pub name: String,
    pub provider: crate::CloudProvider,
    pub region: String,
    pub kubernetes_version: String,
    pub node_groups: Vec<NodeGroup>,
    pub addons: Vec<ClusterAddon>,
    pub network_config: NetworkConfig,
    pub security_config: SecurityConfig,
    pub created_at: DateTime<Utc>,
    pub status: ClusterStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeGroup {
    pub name: String,
    pub instance_type: String,
    pub min_size: u32,
    pub max_size: u32,
    pub desired_size: u32,
    pub disk_size: u32,
    pub labels: HashMap<String, String>,
    pub taints: Vec<NodeTaint>,
    pub auto_scaling_enabled: bool,
    pub spot_instances_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeTaint {
    pub key: String,
    pub value: String,
    pub effect: TaintEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaintEffect {
    NoSchedule,
    PreferNoSchedule,
    NoExecute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterAddon {
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub configuration: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub vpc_id: Option<String>,
    pub subnet_ids: Vec<String>,
    pub service_cidr: String,
    pub pod_cidr: String,
    pub dns_cluster_ip: String,
    pub endpoint_private_access: bool,
    pub endpoint_public_access: bool,
    pub public_access_cidrs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub cluster_encryption_enabled: bool,
    pub encryption_kms_key_id: Option<String>,
    pub security_group_ids: Vec<String>,
    pub service_account_token_volume_projection: bool,
    pub audit_logging_enabled: bool,
    pub secrets_encryption_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterStatus {
    Creating,
    Active,
    Updating,
    Deleting,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesResource {
    pub id: Uuid,
    pub name: String,
    pub namespace: String,
    pub kind: String,
    pub api_version: String,
    pub labels: HashMap<String, String>,
    pub annotations: HashMap<String, String>,
    pub spec: serde_json::Value,
    pub status: ResourceStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesManifest {
    pub api_version: String,
    pub kind: String,
    pub metadata: ObjectMetadata,
    pub spec: serde_json::Value,
    pub status: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMetadata {
    pub name: String,
    pub namespace: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub annotations: Option<HashMap<String, String>>,
    pub owner_references: Option<Vec<OwnerReference>>,
    pub finalizers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerReference {
    pub api_version: String,
    pub kind: String,
    pub name: String,
    pub uid: String,
    pub controller: Option<bool>,
    pub block_owner_deletion: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSpec {
    pub replicas: Option<i32>,
    pub selector: LabelSelector,
    pub template: PodTemplateSpec,
    pub strategy: Option<DeploymentStrategy>,
    pub min_ready_seconds: Option<i32>,
    pub revision_history_limit: Option<i32>,
    pub paused: Option<bool>,
    pub progress_deadline_seconds: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelSelector {
    pub match_labels: Option<HashMap<String, String>>,
    pub match_expressions: Option<Vec<LabelSelectorRequirement>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelSelectorRequirement {
    pub key: String,
    pub operator: String,
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodTemplateSpec {
    pub metadata: Option<ObjectMetadata>,
    pub spec: Option<PodSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodSpec {
    pub containers: Vec<Container>,
    pub init_containers: Option<Vec<Container>>,
    pub restart_policy: Option<String>,
    pub termination_grace_period_seconds: Option<i64>,
    pub active_deadline_seconds: Option<i64>,
    pub dns_policy: Option<String>,
    pub node_selector: Option<HashMap<String, String>>,
    pub service_account_name: Option<String>,
    pub image_pull_secrets: Option<Vec<LocalObjectReference>>,
    pub affinity: Option<Affinity>,
    pub tolerations: Option<Vec<Toleration>>,
    pub volumes: Option<Vec<Volume>>,
    pub security_context: Option<PodSecurityContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    pub image: String,
    pub command: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
    pub working_dir: Option<String>,
    pub ports: Option<Vec<ContainerPort>>,
    pub env: Option<Vec<EnvVar>>,
    pub resources: Option<ResourceRequirements>,
    pub volume_mounts: Option<Vec<VolumeMount>>,
    pub liveness_probe: Option<Probe>,
    pub readiness_probe: Option<Probe>,
    pub startup_probe: Option<Probe>,
    pub security_context: Option<SecurityContext>,
    pub image_pull_policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerPort {
    pub name: Option<String>,
    pub host_port: Option<i32>,
    pub container_port: i32,
    pub protocol: Option<String>,
    pub host_ip: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub value: Option<String>,
    pub value_from: Option<EnvVarSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVarSource {
    pub field_ref: Option<ObjectFieldSelector>,
    pub resource_field_ref: Option<ResourceFieldSelector>,
    pub config_map_key_ref: Option<ConfigMapKeySelector>,
    pub secret_key_ref: Option<SecretKeySelector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectFieldSelector {
    pub api_version: Option<String>,
    pub field_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceFieldSelector {
    pub container_name: Option<String>,
    pub resource: String,
    pub divisor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMapKeySelector {
    pub name: Option<String>,
    pub key: String,
    pub optional: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretKeySelector {
    pub name: Option<String>,
    pub key: String,
    pub optional: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub limits: Option<HashMap<String, String>>,
    pub requests: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMount {
    pub name: String,
    pub read_only: Option<bool>,
    pub mount_path: String,
    pub sub_path: Option<String>,
    pub mount_propagation: Option<String>,
    pub sub_path_expr: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Probe {
    pub exec: Option<ExecAction>,
    pub http_get: Option<HTTPGetAction>,
    pub tcp_socket: Option<TCPSocketAction>,
    pub grpc: Option<GRPCAction>,
    pub initial_delay_seconds: Option<i32>,
    pub timeout_seconds: Option<i32>,
    pub period_seconds: Option<i32>,
    pub success_threshold: Option<i32>,
    pub failure_threshold: Option<i32>,
    pub termination_grace_period_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecAction {
    pub command: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPGetAction {
    pub path: Option<String>,
    pub port: serde_json::Value,
    pub host: Option<String>,
    pub scheme: Option<String>,
    pub http_headers: Option<Vec<HTTPHeader>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TCPSocketAction {
    pub port: serde_json::Value,
    pub host: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GRPCAction {
    pub port: i32,
    pub service: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    pub capabilities: Option<Capabilities>,
    pub privileged: Option<bool>,
    pub se_linux_options: Option<SELinuxOptions>,
    pub windows_options: Option<WindowsSecurityContextOptions>,
    pub run_as_user: Option<i64>,
    pub run_as_group: Option<i64>,
    pub run_as_non_root: Option<bool>,
    pub read_only_root_filesystem: Option<bool>,
    pub allow_privilege_escalation: Option<bool>,
    pub proc_mount: Option<String>,
    pub seccomp_profile: Option<SeccompProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    pub add: Option<Vec<String>>,
    pub drop: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SELinuxOptions {
    pub user: Option<String>,
    pub role: Option<String>,
    pub r#type: Option<String>,
    pub level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsSecurityContextOptions {
    pub gmsa_credential_spec_name: Option<String>,
    pub gmsa_credential_spec: Option<String>,
    pub run_as_user_name: Option<String>,
    pub host_process: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeccompProfile {
    pub r#type: String,
    pub localhost_profile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalObjectReference {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Affinity {
    pub node_affinity: Option<NodeAffinity>,
    pub pod_affinity: Option<PodAffinity>,
    pub pod_anti_affinity: Option<PodAntiAffinity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAffinity {
    pub required_during_scheduling_ignored_during_execution: Option<NodeSelector>,
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<PreferredSchedulingTerm>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSelector {
    pub node_selector_terms: Vec<NodeSelectorTerm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSelectorTerm {
    pub match_expressions: Option<Vec<NodeSelectorRequirement>>,
    pub match_fields: Option<Vec<NodeSelectorRequirement>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSelectorRequirement {
    pub key: String,
    pub operator: String,
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferredSchedulingTerm {
    pub weight: i32,
    pub preference: NodeSelectorTerm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodAffinity {
    pub required_during_scheduling_ignored_during_execution: Option<Vec<PodAffinityTerm>>,
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<WeightedPodAffinityTerm>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodAntiAffinity {
    pub required_during_scheduling_ignored_during_execution: Option<Vec<PodAffinityTerm>>,
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<WeightedPodAffinityTerm>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodAffinityTerm {
    pub label_selector: Option<LabelSelector>,
    pub namespaces: Option<Vec<String>>,
    pub topology_key: String,
    pub namespace_selector: Option<LabelSelector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedPodAffinityTerm {
    pub weight: i32,
    pub pod_affinity_term: PodAffinityTerm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Toleration {
    pub key: Option<String>,
    pub operator: Option<String>,
    pub value: Option<String>,
    pub effect: Option<String>,
    pub toleration_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub name: String,
    pub host_path: Option<HostPathVolumeSource>,
    pub empty_dir: Option<EmptyDirVolumeSource>,
    pub config_map: Option<ConfigMapVolumeSource>,
    pub secret: Option<SecretVolumeSource>,
    pub persistent_volume_claim: Option<PersistentVolumeClaimVolumeSource>,
    pub nfs: Option<NFSVolumeSource>,
    pub aws_elastic_block_store: Option<AWSElasticBlockStoreVolumeSource>,
    pub gce_persistent_disk: Option<GCEPersistentDiskVolumeSource>,
    pub azure_disk: Option<AzureDiskVolumeSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostPathVolumeSource {
    pub path: String,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyDirVolumeSource {
    pub medium: Option<String>,
    pub size_limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMapVolumeSource {
    pub name: Option<String>,
    pub items: Option<Vec<KeyToPath>>,
    pub default_mode: Option<i32>,
    pub optional: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretVolumeSource {
    pub secret_name: Option<String>,
    pub items: Option<Vec<KeyToPath>>,
    pub default_mode: Option<i32>,
    pub optional: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyToPath {
    pub key: String,
    pub path: String,
    pub mode: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentVolumeClaimVolumeSource {
    pub claim_name: String,
    pub read_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFSVolumeSource {
    pub server: String,
    pub path: String,
    pub read_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AWSElasticBlockStoreVolumeSource {
    pub volume_id: String,
    pub fs_type: Option<String>,
    pub partition: Option<i32>,
    pub read_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GCEPersistentDiskVolumeSource {
    pub pd_name: String,
    pub fs_type: Option<String>,
    pub partition: Option<i32>,
    pub read_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureDiskVolumeSource {
    pub disk_name: String,
    pub disk_uri: String,
    pub cache_mode: Option<String>,
    pub fs_type: Option<String>,
    pub read_only: Option<bool>,
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodSecurityContext {
    pub se_linux_options: Option<SELinuxOptions>,
    pub windows_options: Option<WindowsSecurityContextOptions>,
    pub run_as_user: Option<i64>,
    pub run_as_group: Option<i64>,
    pub run_as_non_root: Option<bool>,
    pub supplemental_groups: Option<Vec<i64>>,
    pub fs_group: Option<i64>,
    pub seccomp_profile: Option<SeccompProfile>,
    pub fs_group_change_policy: Option<String>,
    pub sysctls: Option<Vec<Sysctl>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sysctl {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentStrategy {
    pub r#type: Option<String>,
    pub rolling_update: Option<RollingUpdateDeployment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollingUpdateDeployment {
    pub max_unavailable: Option<serde_json::Value>,
    pub max_surge: Option<serde_json::Value>,
}

#[async_trait::async_trait]
pub trait KubernetesManager {
    async fn connect(&self, config: &KubeConfig) -> crate::Result<Client>;
    async fn create_namespace(&self, client: &Client, namespace: &str) -> crate::Result<()>;
    async fn deploy_manifest(&self, client: &Client, manifest: &KubernetesManifest) -> crate::Result<()>;
    async fn update_manifest(&self, client: &Client, manifest: &KubernetesManifest) -> crate::Result<()>;
    async fn delete_manifest(&self, client: &Client, manifest: &KubernetesManifest) -> crate::Result<()>;
    async fn get_resource_status(&self, client: &Client, namespace: &str, name: &str, kind: &str) -> crate::Result<ResourceStatus>;
    async fn list_resources(&self, client: &Client, namespace: Option<&str>) -> crate::Result<Vec<KubernetesResource>>;
    async fn scale_deployment(&self, client: &Client, namespace: &str, name: &str, replicas: i32) -> crate::Result<()>;
    async fn rollback_deployment(&self, client: &Client, namespace: &str, name: &str, revision: Option<i64>) -> crate::Result<()>;
    async fn get_logs(&self, client: &Client, namespace: &str, pod_name: &str, container: Option<&str>) -> crate::Result<String>;
}