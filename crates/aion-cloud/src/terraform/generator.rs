use crate::{CloudProvider, DeploymentTemplate, ResourceDefinition, VariableDefinition};
use crate::terraform::{
    TerraformConfig, TerraformBlock, RequiredProvider, ProviderConfig, TerraformVariable,
    TerraformResource, TerraformOutput, TerraformGenerator, LifecycleBlock
};
use serde_json::{json, Value};
use std::collections::HashMap;

pub struct DefaultTerraformGenerator;

impl DefaultTerraformGenerator {
    pub fn new() -> Self {
        Self
    }

    fn get_provider_source(&self, provider: CloudProvider) -> &'static str {
        match provider {
            CloudProvider::AWS => "hashicorp/aws",
            CloudProvider::GCP => "hashicorp/google",
            CloudProvider::Azure => "hashicorp/azurerm",
            CloudProvider::DigitalOcean => "digitalocean/digitalocean",
            CloudProvider::Kubernetes => "hashicorp/kubernetes",
        }
    }

    fn get_provider_version(&self, provider: CloudProvider) -> &'static str {
        match provider {
            CloudProvider::AWS => "~> 5.0",
            CloudProvider::GCP => "~> 5.0",
            CloudProvider::Azure => "~> 3.0",
            CloudProvider::DigitalOcean => "~> 2.0",
            CloudProvider::Kubernetes => "~> 2.0",
        }
    }

    fn convert_aws_resource(&self, resource: &ResourceDefinition) -> TerraformResource {
        let mut config = HashMap::new();

        match resource.resource_type.as_str() {
            "ec2_instance" => {
                if let Some(ami) = resource.properties.get("ami_id") {
                    config.insert("ami".to_string(), ami.clone());
                }
                if let Some(instance_type) = resource.properties.get("instance_type") {
                    config.insert("instance_type".to_string(), instance_type.clone());
                }
                if let Some(key_name) = resource.properties.get("key_name") {
                    config.insert("key_name".to_string(), key_name.clone());
                }
                if let Some(subnet_id) = resource.properties.get("subnet_id") {
                    config.insert("subnet_id".to_string(), subnet_id.clone());
                }
                if let Some(security_groups) = resource.properties.get("security_group_ids") {
                    config.insert("security_groups".to_string(), security_groups.clone());
                }

                // Add default tags
                config.insert("tags".to_string(), json!({
                    "Name": resource.name,
                    "ManagedBy": "AION",
                    "Environment": "${var.environment}"
                }));
            },
            "s3_bucket" => {
                config.insert("bucket".to_string(), Value::String(resource.name.clone()));

                if let Some(versioning) = resource.properties.get("versioning_enabled") {
                    config.insert("versioning".to_string(), json!({
                        "enabled": versioning
                    }));
                }

                if let Some(encryption) = resource.properties.get("server_side_encryption") {
                    config.insert("server_side_encryption_configuration".to_string(), json!({
                        "rule": {
                            "apply_server_side_encryption_by_default": {
                                "sse_algorithm": encryption
                            }
                        }
                    }));
                }
            },
            "lambda_function" => {
                config.insert("function_name".to_string(), Value::String(resource.name.clone()));

                if let Some(runtime) = resource.properties.get("runtime") {
                    config.insert("runtime".to_string(), runtime.clone());
                }
                if let Some(handler) = resource.properties.get("handler") {
                    config.insert("handler".to_string(), handler.clone());
                }
                if let Some(filename) = resource.properties.get("filename") {
                    config.insert("filename".to_string(), filename.clone());
                }
                if let Some(memory_size) = resource.properties.get("memory_size") {
                    config.insert("memory_size".to_string(), memory_size.clone());
                }
                if let Some(timeout) = resource.properties.get("timeout") {
                    config.insert("timeout".to_string(), timeout.clone());
                }

                // Add IAM role reference
                config.insert("role".to_string(), Value::String("${aws_iam_role.lambda_role.arn}".to_string()));
            },
            "rds_instance" => {
                config.insert("identifier".to_string(), Value::String(resource.name.clone()));

                if let Some(engine) = resource.properties.get("engine") {
                    config.insert("engine".to_string(), engine.clone());
                }
                if let Some(engine_version) = resource.properties.get("engine_version") {
                    config.insert("engine_version".to_string(), engine_version.clone());
                }
                if let Some(instance_class) = resource.properties.get("instance_class") {
                    config.insert("instance_class".to_string(), instance_class.clone());
                }
                if let Some(allocated_storage) = resource.properties.get("allocated_storage") {
                    config.insert("allocated_storage".to_string(), allocated_storage.clone());
                }
                if let Some(db_name) = resource.properties.get("db_name") {
                    config.insert("db_name".to_string(), db_name.clone());
                }

                // Security settings
                config.insert("skip_final_snapshot".to_string(), Value::Bool(true));
                config.insert("deletion_protection".to_string(), Value::Bool(false));
            },
            _ => {
                // Generic resource handling
                config = resource.properties.clone();
            }
        }

        let mut depends_on = Vec::new();
        for dep in &resource.depends_on {
            depends_on.push(format!("aws_{}.{}", dep, dep));
        }

        TerraformResource {
            config,
            lifecycle: Some(LifecycleBlock {
                create_before_destroy: Some(true),
                prevent_destroy: None,
                ignore_changes: None,
                replace_triggered_by: None,
            }),
            depends_on: if depends_on.is_empty() { None } else { Some(depends_on) },
            count: None,
            for_each: None,
            provider: None,
        }
    }

    fn convert_gcp_resource(&self, resource: &ResourceDefinition) -> TerraformResource {
        let mut config = HashMap::new();

        match resource.resource_type.as_str() {
            "compute_instance" => {
                config.insert("name".to_string(), Value::String(resource.name.clone()));
                config.insert("zone".to_string(), Value::String("${var.zone}".to_string()));

                if let Some(machine_type) = resource.properties.get("machine_type") {
                    config.insert("machine_type".to_string(), machine_type.clone());
                }

                // Boot disk configuration
                if let Some(image) = resource.properties.get("image") {
                    config.insert("boot_disk".to_string(), json!({
                        "initialize_params": {
                            "image": image
                        }
                    }));
                }

                // Network interface
                config.insert("network_interface".to_string(), json!({
                    "network": "default",
                    "access_config": {}
                }));

                // Labels (GCP equivalent of tags)
                config.insert("labels".to_string(), json!({
                    "managed_by": "aion",
                    "environment": "${var.environment}"
                }));
            },
            "storage_bucket" => {
                config.insert("name".to_string(), Value::String(resource.name.clone()));
                config.insert("location".to_string(), Value::String("${var.region}".to_string()));

                if let Some(storage_class) = resource.properties.get("storage_class") {
                    config.insert("storage_class".to_string(), storage_class.clone());
                }

                // Versioning
                if let Some(versioning) = resource.properties.get("versioning_enabled") {
                    config.insert("versioning".to_string(), json!({
                        "enabled": versioning
                    }));
                }
            },
            "cloud_function" => {
                config.insert("name".to_string(), Value::String(resource.name.clone()));
                config.insert("region".to_string(), Value::String("${var.region}".to_string()));

                if let Some(runtime) = resource.properties.get("runtime") {
                    config.insert("runtime".to_string(), runtime.clone());
                }
                if let Some(entry_point) = resource.properties.get("entry_point") {
                    config.insert("entry_point".to_string(), entry_point.clone());
                }
                if let Some(source_archive_bucket) = resource.properties.get("source_archive_bucket") {
                    config.insert("source_archive_bucket".to_string(), source_archive_bucket.clone());
                }
                if let Some(source_archive_object) = resource.properties.get("source_archive_object") {
                    config.insert("source_archive_object".to_string(), source_archive_object.clone());
                }

                // Trigger
                config.insert("trigger".to_string(), json!({
                    "https_trigger": {}
                }));
            },
            _ => {
                config = resource.properties.clone();
            }
        }

        TerraformResource {
            config,
            lifecycle: None,
            depends_on: None,
            count: None,
            for_each: None,
            provider: None,
        }
    }

    fn convert_azure_resource(&self, resource: &ResourceDefinition) -> TerraformResource {
        let mut config = HashMap::new();

        match resource.resource_type.as_str() {
            "virtual_machine" => {
                config.insert("name".to_string(), Value::String(resource.name.clone()));
                config.insert("location".to_string(), Value::String("${var.location}".to_string()));
                config.insert("resource_group_name".to_string(), Value::String("${var.resource_group_name}".to_string()));

                if let Some(vm_size) = resource.properties.get("vm_size") {
                    config.insert("vm_size".to_string(), vm_size.clone());
                }

                // OS disk
                config.insert("storage_os_disk".to_string(), json!({
                    "name": format!("{}-osdisk", resource.name),
                    "caching": "ReadWrite",
                    "create_option": "FromImage",
                    "managed_disk_type": "Standard_LRS"
                }));

                // Image reference
                if let Some(image_reference) = resource.properties.get("image_reference") {
                    config.insert("storage_image_reference".to_string(), image_reference.clone());
                }

                // OS profile
                config.insert("os_profile".to_string(), json!({
                    "computer_name": resource.name,
                    "admin_username": "${var.admin_username}",
                    "admin_password": "${var.admin_password}"
                }));

                // Network interface
                config.insert("network_interface_ids".to_string(),
                    Value::Array(vec![Value::String("${azurerm_network_interface.main.id}".to_string())]));

                // Tags
                config.insert("tags".to_string(), json!({
                    "ManagedBy": "AION",
                    "Environment": "${var.environment}"
                }));
            },
            "storage_account" => {
                config.insert("name".to_string(), Value::String(resource.name.clone()));
                config.insert("location".to_string(), Value::String("${var.location}".to_string()));
                config.insert("resource_group_name".to_string(), Value::String("${var.resource_group_name}".to_string()));

                if let Some(account_tier) = resource.properties.get("account_tier") {
                    config.insert("account_tier".to_string(), account_tier.clone());
                } else {
                    config.insert("account_tier".to_string(), Value::String("Standard".to_string()));
                }

                if let Some(replication_type) = resource.properties.get("account_replication_type") {
                    config.insert("account_replication_type".to_string(), replication_type.clone());
                } else {
                    config.insert("account_replication_type".to_string(), Value::String("LRS".to_string()));
                }
            },
            "function_app" => {
                config.insert("name".to_string(), Value::String(resource.name.clone()));
                config.insert("location".to_string(), Value::String("${var.location}".to_string()));
                config.insert("resource_group_name".to_string(), Value::String("${var.resource_group_name}".to_string()));
                config.insert("app_service_plan_id".to_string(), Value::String("${azurerm_app_service_plan.main.id}".to_string()));
                config.insert("storage_account_name".to_string(), Value::String("${azurerm_storage_account.main.name}".to_string()));
                config.insert("storage_account_access_key".to_string(), Value::String("${azurerm_storage_account.main.primary_access_key}".to_string()));

                if let Some(runtime) = resource.properties.get("runtime") {
                    config.insert("app_settings".to_string(), json!({
                        "FUNCTIONS_WORKER_RUNTIME": runtime
                    }));
                }
            },
            _ => {
                config = resource.properties.clone();
            }
        }

        TerraformResource {
            config,
            lifecycle: None,
            depends_on: None,
            count: None,
            for_each: None,
            provider: None,
        }
    }
}

impl TerraformGenerator for DefaultTerraformGenerator {
    fn generate_terraform(&self, template: &DeploymentTemplate) -> crate::Result<TerraformConfig> {
        let mut required_providers = HashMap::new();

        // Add required provider
        required_providers.insert(
            template.provider.to_string().to_lowercase(),
            RequiredProvider {
                source: self.get_provider_source(template.provider.clone()).to_string(),
                version: Some(self.get_provider_version(template.provider.clone()).to_string()),
                configuration_aliases: None,
            }
        );

        let terraform_block = TerraformBlock {
            required_version: Some(">= 1.0".to_string()),
            required_providers: Some(required_providers),
            backend: Some(HashMap::from([
                ("s3".to_string(), json!({
                    "bucket": "${var.terraform_state_bucket}",
                    "key": format!("aion/{}/terraform.tfstate", template.name),
                    "region": "${var.terraform_state_region}",
                    "encrypt": true
                }))
            ])),
            experiments: None,
        };

        let mut providers = HashMap::new();
        providers.insert(
            template.provider.to_string().to_lowercase(),
            self.generate_provider_config(template.provider.clone())
        );

        let variables = self.generate_variables(&template.variables);
        let resources = self.generate_resources(&template.resources);
        let outputs = self.generate_outputs(&template.outputs);

        Ok(TerraformConfig {
            terraform: terraform_block,
            provider: providers,
            variable: variables,
            resource: resources,
            output: outputs,
            data: None,
            locals: None,
            module: None,
        })
    }

    fn generate_provider_config(&self, provider: CloudProvider) -> ProviderConfig {
        match provider {
            CloudProvider::AWS => ProviderConfig {
                region: Some("${var.aws_region}".to_string()),
                access_key: None, // Use IAM roles instead
                secret_key: None,
                token: None,
                project: None,
                credentials: None,
                subscription_id: None,
                tenant_id: None,
                client_id: None,
                client_secret: None,
                alias: None,
                additional_config: HashMap::from([
                    ("default_tags".to_string(), json!({
                        "tags": {
                            "ManagedBy": "AION",
                            "Environment": "${var.environment}"
                        }
                    }))
                ]),
            },
            CloudProvider::GCP => ProviderConfig {
                region: Some("${var.gcp_region}".to_string()),
                project: Some("${var.gcp_project}".to_string()),
                credentials: Some("${var.gcp_credentials_file}".to_string()),
                access_key: None,
                secret_key: None,
                token: None,
                subscription_id: None,
                tenant_id: None,
                client_id: None,
                client_secret: None,
                alias: None,
                additional_config: HashMap::new(),
            },
            CloudProvider::Azure => ProviderConfig {
                subscription_id: Some("${var.azure_subscription_id}".to_string()),
                tenant_id: Some("${var.azure_tenant_id}".to_string()),
                client_id: Some("${var.azure_client_id}".to_string()),
                client_secret: Some("${var.azure_client_secret}".to_string()),
                region: None,
                access_key: None,
                secret_key: None,
                token: None,
                project: None,
                credentials: None,
                alias: None,
                additional_config: HashMap::from([
                    ("features".to_string(), json!({}))
                ]),
            },
            _ => ProviderConfig {
                region: None,
                access_key: None,
                secret_key: None,
                token: None,
                project: None,
                credentials: None,
                subscription_id: None,
                tenant_id: None,
                client_id: None,
                client_secret: None,
                alias: None,
                additional_config: HashMap::new(),
            },
        }
    }

    fn generate_variables(&self, variables: &HashMap<String, VariableDefinition>) -> HashMap<String, TerraformVariable> {
        let mut tf_variables = HashMap::new();

        // Add standard variables
        tf_variables.insert("environment".to_string(), TerraformVariable {
            description: Some("Environment name (dev, staging, prod)".to_string()),
            r#type: Some("string".to_string()),
            default: Some(Value::String("dev".to_string())),
            validation: Some(vec![]),
            sensitive: Some(false),
            nullable: Some(false),
        });

        tf_variables.insert("region".to_string(), TerraformVariable {
            description: Some("Primary region for resources".to_string()),
            r#type: Some("string".to_string()),
            default: Some(Value::String("us-east-1".to_string())),
            validation: None,
            sensitive: Some(false),
            nullable: Some(false),
        });

        // Convert template variables
        for (name, var_def) in variables {
            tf_variables.insert(name.clone(), TerraformVariable {
                description: Some(var_def.description.clone()),
                r#type: Some(var_def.variable_type.clone()),
                default: var_def.default.clone(),
                validation: None,
                sensitive: Some(var_def.sensitive),
                nullable: Some(!var_def.required),
            });
        }

        tf_variables
    }

    fn generate_resources(&self, resources: &[ResourceDefinition]) -> HashMap<String, HashMap<String, TerraformResource>> {
        let mut tf_resources = HashMap::new();

        for resource in resources {
            let provider_prefix = match resource.resource_type.as_str() {
                s if s.starts_with("ec2_") || s.starts_with("s3_") || s.starts_with("lambda_") || s.starts_with("rds_") => "aws",
                s if s.starts_with("compute_") || s.starts_with("storage_") || s.starts_with("cloud_") => "google",
                s if s.starts_with("virtual_") || s.starts_with("storage_account") || s.starts_with("function_app") => "azurerm",
                _ => "aws", // Default
            };

            let tf_resource_type = format!("{}_{}", provider_prefix, resource.resource_type);

            let tf_resource = match provider_prefix {
                "aws" => self.convert_aws_resource(resource),
                "google" => self.convert_gcp_resource(resource),
                "azurerm" => self.convert_azure_resource(resource),
                _ => self.convert_aws_resource(resource),
            };

            tf_resources
                .entry(tf_resource_type)
                .or_insert_with(HashMap::new)
                .insert(resource.name.clone(), tf_resource);
        }

        tf_resources
    }

    fn generate_outputs(&self, outputs: &HashMap<String, String>) -> HashMap<String, TerraformOutput> {
        let mut tf_outputs = HashMap::new();

        for (name, value_expr) in outputs {
            tf_outputs.insert(name.clone(), TerraformOutput {
                value: Value::String(value_expr.clone()),
                description: Some(format!("Output value for {}", name)),
                sensitive: Some(false),
                depends_on: None,
            });
        }

        // Add common outputs
        tf_outputs.insert("resource_ids".to_string(), TerraformOutput {
            value: Value::String("${local.resource_ids}".to_string()),
            description: Some("Map of all created resource IDs".to_string()),
            sensitive: Some(false),
            depends_on: None,
        });

        tf_outputs.insert("deployment_time".to_string(), TerraformOutput {
            value: Value::String("${timestamp()}".to_string()),
            description: Some("Deployment timestamp".to_string()),
            sensitive: Some(false),
            depends_on: None,
        });

        tf_outputs
    }
}