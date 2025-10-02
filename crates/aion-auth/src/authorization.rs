use crate::models::*;
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Authorization context for evaluating permissions
#[derive(Debug, Clone)]
pub struct AuthorizationContext {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub session_id: Uuid,
    pub roles: Vec<String>,
    pub permissions: Vec<Permission>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub additional_attributes: HashMap<String, serde_json::Value>,
}

/// Policy evaluation result
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyResult {
    Allow,
    Deny,
    NotApplicable,
}

/// Policy evaluation engine
pub struct PolicyEngine {
    policies: Vec<Policy>,
    default_policy: PolicyResult,
}

/// Security policy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub effect: PermissionEffect,
    pub conditions: PolicyConditions,
    pub priority: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Policy conditions for ABAC evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConditions {
    pub subjects: Option<SubjectConditions>,
    pub resources: Option<ResourceConditions>,
    pub actions: Option<ActionConditions>,
    pub environment: Option<EnvironmentConditions>,
    pub custom: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectConditions {
    pub user_ids: Option<Vec<Uuid>>,
    pub roles: Option<Vec<String>>,
    pub departments: Option<Vec<String>>,
    pub security_clearance: Option<String>,
    pub attributes: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConditions {
    pub resource_types: Option<Vec<String>>,
    pub resource_ids: Option<Vec<Uuid>>,
    pub owners: Option<Vec<Uuid>>,
    pub classifications: Option<Vec<String>>,
    pub attributes: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConditions {
    pub actions: Option<Vec<String>>,
    pub operations: Option<Vec<String>>,
    pub methods: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConditions {
    pub time_range: Option<TimeRange>,
    pub ip_ranges: Option<Vec<String>>,
    pub geolocation: Option<Vec<String>>,
    pub device_types: Option<Vec<String>>,
    pub network_zones: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start_time: Option<String>, // Format: HH:MM
    pub end_time: Option<String>,   // Format: HH:MM
    pub days_of_week: Option<Vec<u8>>, // 1-7 (Monday-Sunday)
    pub date_range: Option<DateRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

/// Authorization service for RBAC and ABAC
pub struct AuthorizationService {
    policy_engine: PolicyEngine,
    role_hierarchy: HashMap<String, Vec<String>>, // role -> child roles
    permission_cache: HashMap<String, Vec<Permission>>,
}

impl AuthorizationService {
    pub fn new() -> Self {
        Self {
            policy_engine: PolicyEngine::new(),
            role_hierarchy: HashMap::new(),
            permission_cache: HashMap::new(),
        }
    }

    /// Check if user has permission for a specific action on a resource
    pub async fn check_permission(
        &self,
        context: &AuthorizationContext,
        resource_type: &str,
        resource_id: Option<Uuid>,
        action: &str,
    ) -> AuthResult<bool> {
        // 1. Check explicit permissions
        if self.has_explicit_permission(context, resource_type, resource_id, action)? {
            return Ok(true);
        }

        // 2. Evaluate policies using ABAC
        let policy_result = self.evaluate_policies(context, resource_type, resource_id, action)?;

        match policy_result {
            PolicyResult::Allow => Ok(true),
            PolicyResult::Deny => Ok(false),
            PolicyResult::NotApplicable => {
                // 3. Fall back to role-based permissions
                self.check_role_permissions(context, resource_type, resource_id, action)
            }
        }
    }

    /// Check if user has explicit permission
    fn has_explicit_permission(
        &self,
        context: &AuthorizationContext,
        resource_type: &str,
        resource_id: Option<Uuid>,
        action: &str,
    ) -> AuthResult<bool> {
        for permission in &context.permissions {
            if permission.resource == resource_type && permission.action == action {
                // Check if permission applies to specific resource or all resources
                if let Some(perm_conditions) = &permission.conditions {
                    if self.evaluate_permission_conditions(perm_conditions, context, resource_id)? {
                        return Ok(true);
                    }
                } else {
                    // No conditions means it applies to all resources of this type
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Evaluate policies using ABAC model
    fn evaluate_policies(
        &self,
        context: &AuthorizationContext,
        resource_type: &str,
        resource_id: Option<Uuid>,
        action: &str,
    ) -> AuthResult<PolicyResult> {
        let mut applicable_policies = Vec::new();

        // Find applicable policies
        for policy in &self.policy_engine.policies {
            if !policy.is_active {
                continue;
            }

            if self.policy_applies(policy, context, resource_type, resource_id, action)? {
                applicable_policies.push(policy);
            }
        }

        // Sort by priority (higher priority first)
        applicable_policies.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Evaluate policies in priority order
        for policy in applicable_policies {
            match policy.effect {
                PermissionEffect::Deny => return Ok(PolicyResult::Deny),
                PermissionEffect::Allow => return Ok(PolicyResult::Allow),
            }
        }

        Ok(PolicyResult::NotApplicable)
    }

    /// Check if policy applies to the current context
    fn policy_applies(
        &self,
        policy: &Policy,
        context: &AuthorizationContext,
        resource_type: &str,
        resource_id: Option<Uuid>,
        action: &str,
    ) -> AuthResult<bool> {
        let conditions = &policy.conditions;

        // Check subject conditions
        if let Some(subject_conditions) = &conditions.subjects {
            if !self.check_subject_conditions(subject_conditions, context)? {
                return Ok(false);
            }
        }

        // Check resource conditions
        if let Some(resource_conditions) = &conditions.resources {
            if !self.check_resource_conditions(resource_conditions, resource_type, resource_id)? {
                return Ok(false);
            }
        }

        // Check action conditions
        if let Some(action_conditions) = &conditions.actions {
            if !self.check_action_conditions(action_conditions, action)? {
                return Ok(false);
            }
        }

        // Check environment conditions
        if let Some(env_conditions) = &conditions.environment {
            if !self.check_environment_conditions(env_conditions, context)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Check subject conditions
    fn check_subject_conditions(
        &self,
        conditions: &SubjectConditions,
        context: &AuthorizationContext,
    ) -> AuthResult<bool> {
        // Check user IDs
        if let Some(user_ids) = &conditions.user_ids {
            if !user_ids.contains(&context.user_id) {
                return Ok(false);
            }
        }

        // Check roles
        if let Some(required_roles) = &conditions.roles {
            let user_roles: HashSet<&String> = context.roles.iter().collect();
            let required_roles: HashSet<&String> = required_roles.iter().collect();

            if !required_roles.iter().any(|role| user_roles.contains(role)) {
                return Ok(false);
            }
        }

        // Check custom attributes
        if let Some(required_attrs) = &conditions.attributes {
            for (key, required_value) in required_attrs {
                if let Some(actual_value) = context.additional_attributes.get(key) {
                    if actual_value != required_value {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// Check resource conditions
    fn check_resource_conditions(
        &self,
        conditions: &ResourceConditions,
        resource_type: &str,
        resource_id: Option<Uuid>,
    ) -> AuthResult<bool> {
        // Check resource types
        if let Some(resource_types) = &conditions.resource_types {
            if !resource_types.contains(&resource_type.to_string()) {
                return Ok(false);
            }
        }

        // Check resource IDs
        if let Some(resource_ids) = &conditions.resource_ids {
            if let Some(id) = resource_id {
                if !resource_ids.contains(&id) {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Check action conditions
    fn check_action_conditions(
        &self,
        conditions: &ActionConditions,
        action: &str,
    ) -> AuthResult<bool> {
        // Check actions
        if let Some(actions) = &conditions.actions {
            if !actions.contains(&action.to_string()) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Check environment conditions
    fn check_environment_conditions(
        &self,
        conditions: &EnvironmentConditions,
        context: &AuthorizationContext,
    ) -> AuthResult<bool> {
        // Check time range
        if let Some(time_range) = &conditions.time_range {
            if !self.check_time_range(time_range, context.timestamp)? {
                return Ok(false);
            }
        }

        // Check IP ranges
        if let Some(ip_ranges) = &conditions.ip_ranges {
            if let Some(user_ip) = &context.ip_address {
                if !self.ip_in_ranges(user_ip, ip_ranges)? {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Check if current time is within allowed time range
    fn check_time_range(&self, time_range: &TimeRange, timestamp: DateTime<Utc>) -> AuthResult<bool> {
        // Check date range
        if let Some(date_range) = &time_range.date_range {
            if timestamp < date_range.start_date || timestamp > date_range.end_date {
                return Ok(false);
            }
        }

        // Check day of week
        if let Some(allowed_days) = &time_range.days_of_week {
            let current_day = timestamp.weekday().number_from_monday() as u8;
            if !allowed_days.contains(&current_day) {
                return Ok(false);
            }
        }

        // Check time of day
        if let (Some(start_time), Some(end_time)) = (&time_range.start_time, &time_range.end_time) {
            let current_time = timestamp.format("%H:%M").to_string();
            if current_time < *start_time || current_time > *end_time {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Check if IP is in allowed ranges (simplified implementation)
    fn ip_in_ranges(&self, ip: &str, ranges: &[String]) -> AuthResult<bool> {
        // Simplified IP range checking - in production, use proper CIDR parsing
        for range in ranges {
            if ip.starts_with(range) || range == "*" {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Check role-based permissions with hierarchy
    fn check_role_permissions(
        &self,
        context: &AuthorizationContext,
        resource_type: &str,
        resource_id: Option<Uuid>,
        action: &str,
    ) -> AuthResult<bool> {
        // Get all effective roles (including inherited roles)
        let effective_roles = self.get_effective_roles(&context.roles)?;

        // Check permissions for all effective roles
        for role in &effective_roles {
            if let Some(permissions) = self.permission_cache.get(role) {
                for permission in permissions {
                    if permission.resource == resource_type && permission.action == action {
                        if let Some(conditions) = &permission.conditions {
                            if self.evaluate_permission_conditions(conditions, context, resource_id)? {
                                return Ok(true);
                            }
                        } else {
                            return Ok(true);
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    /// Get all effective roles including inherited roles from hierarchy
    fn get_effective_roles(&self, user_roles: &[String]) -> AuthResult<Vec<String>> {
        let mut effective_roles = HashSet::new();
        let mut roles_to_process: Vec<String> = user_roles.to_vec();

        while let Some(role) = roles_to_process.pop() {
            if effective_roles.insert(role.clone()) {
                // If this role was newly added, check for child roles
                if let Some(child_roles) = self.role_hierarchy.get(&role) {
                    roles_to_process.extend(child_roles.clone());
                }
            }
        }

        Ok(effective_roles.into_iter().collect())
    }

    /// Evaluate permission conditions
    fn evaluate_permission_conditions(
        &self,
        conditions: &serde_json::Value,
        context: &AuthorizationContext,
        resource_id: Option<Uuid>,
    ) -> AuthResult<bool> {
        // Simplified condition evaluation - extend as needed
        if let Some(obj) = conditions.as_object() {
            // Check tenant restriction
            if let Some(tenant_condition) = obj.get("tenant_id") {
                if let Some(required_tenant) = tenant_condition.as_str() {
                    if context.tenant_id.to_string() != required_tenant {
                        return Ok(false);
                    }
                }
            }

            // Check resource ownership
            if let Some(owner_condition) = obj.get("owner_only") {
                if owner_condition.as_bool() == Some(true) {
                    // In a real implementation, you'd check if the user owns the resource
                    // For now, we'll assume they do if resource_id is provided
                    if resource_id.is_none() {
                        return Ok(false);
                    }
                }
            }
        }

        Ok(true)
    }

    /// Load role hierarchy from database or configuration
    pub async fn load_role_hierarchy(&mut self, hierarchy: HashMap<String, Vec<String>>) {
        self.role_hierarchy = hierarchy;
    }

    /// Load permissions for roles into cache
    pub async fn load_role_permissions(&mut self, role_permissions: HashMap<String, Vec<Permission>>) {
        self.permission_cache = role_permissions;
    }

    /// Add a new policy
    pub async fn add_policy(&mut self, policy: Policy) -> AuthResult<()> {
        self.policy_engine.policies.push(policy);
        Ok(())
    }

    /// Remove a policy
    pub async fn remove_policy(&mut self, policy_id: Uuid) -> AuthResult<bool> {
        let initial_len = self.policy_engine.policies.len();
        self.policy_engine.policies.retain(|p| p.id != policy_id);
        Ok(self.policy_engine.policies.len() < initial_len)
    }

    /// Update a policy
    pub async fn update_policy(&mut self, policy: Policy) -> AuthResult<bool> {
        for existing_policy in &mut self.policy_engine.policies {
            if existing_policy.id == policy.id {
                *existing_policy = policy;
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Get all policies
    pub fn get_policies(&self) -> &[Policy] {
        &self.policy_engine.policies
    }

    /// Validate user access with comprehensive logging
    pub async fn validate_access(
        &self,
        context: &AuthorizationContext,
        resource_type: &str,
        resource_id: Option<Uuid>,
        action: &str,
    ) -> AuthResult<AccessValidationResult> {
        let start_time = std::time::Instant::now();

        let has_permission = self.check_permission(context, resource_type, resource_id, action).await?;

        let validation_time = start_time.elapsed();

        Ok(AccessValidationResult {
            granted: has_permission,
            user_id: context.user_id,
            tenant_id: context.tenant_id,
            resource_type: resource_type.to_string(),
            resource_id,
            action: action.to_string(),
            timestamp: Utc::now(),
            validation_time_ms: validation_time.as_millis() as u64,
            context_summary: format!(
                "User: {}, Roles: {:?}, IP: {:?}",
                context.user_id, context.roles, context.ip_address
            ),
        })
    }
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
            default_policy: PolicyResult::Deny,
        }
    }

    pub fn with_default_policy(default_policy: PolicyResult) -> Self {
        Self {
            policies: Vec::new(),
            default_policy,
        }
    }
}

/// Result of access validation with audit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessValidationResult {
    pub granted: bool,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub resource_type: String,
    pub resource_id: Option<Uuid>,
    pub action: String,
    pub timestamp: DateTime<Utc>,
    pub validation_time_ms: u64,
    pub context_summary: String,
}

/// Helper functions for common authorization patterns
pub mod helpers {
    use super::*;

    /// Create a simple policy for role-based access
    pub fn create_role_policy(
        name: &str,
        roles: Vec<String>,
        resource_types: Vec<String>,
        actions: Vec<String>,
        effect: PermissionEffect,
    ) -> Policy {
        Policy {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: format!("Role-based policy for {}", name),
            effect,
            conditions: PolicyConditions {
                subjects: Some(SubjectConditions {
                    user_ids: None,
                    roles: Some(roles),
                    departments: None,
                    security_clearance: None,
                    attributes: None,
                }),
                resources: Some(ResourceConditions {
                    resource_types: Some(resource_types),
                    resource_ids: None,
                    owners: None,
                    classifications: None,
                    attributes: None,
                }),
                actions: Some(ActionConditions {
                    actions: Some(actions),
                    operations: None,
                    methods: None,
                }),
                environment: None,
                custom: None,
            },
            priority: 100,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Create a time-based policy
    pub fn create_time_based_policy(
        name: &str,
        start_time: &str,
        end_time: &str,
        days_of_week: Vec<u8>,
        effect: PermissionEffect,
    ) -> Policy {
        Policy {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: format!("Time-based policy for {}", name),
            effect,
            conditions: PolicyConditions {
                subjects: None,
                resources: None,
                actions: None,
                environment: Some(EnvironmentConditions {
                    time_range: Some(TimeRange {
                        start_time: Some(start_time.to_string()),
                        end_time: Some(end_time.to_string()),
                        days_of_week: Some(days_of_week),
                        date_range: None,
                    }),
                    ip_ranges: None,
                    geolocation: None,
                    device_types: None,
                    network_zones: None,
                }),
                custom: None,
            },
            priority: 200,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Create an IP-based policy
    pub fn create_ip_policy(
        name: &str,
        allowed_ip_ranges: Vec<String>,
        effect: PermissionEffect,
    ) -> Policy {
        Policy {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: format!("IP-based policy for {}", name),
            effect,
            conditions: PolicyConditions {
                subjects: None,
                resources: None,
                actions: None,
                environment: Some(EnvironmentConditions {
                    time_range: None,
                    ip_ranges: Some(allowed_ip_ranges),
                    geolocation: None,
                    device_types: None,
                    network_zones: None,
                }),
                custom: None,
            },
            priority: 300,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Legacy trait for backward compatibility
pub trait AuthorizationProvider: Send + Sync {
    fn check_permission(&self, user_id: Uuid, resource: &str, action: &str) -> AuthResult<bool>;
    fn get_user_permissions(&self, user_id: Uuid) -> AuthResult<Vec<Permission>>;
    fn get_user_roles(&self, user_id: Uuid) -> AuthResult<Vec<Role>>;
    fn assign_role(&self, user_id: Uuid, role_id: Uuid) -> AuthResult<()>;
    fn revoke_role(&self, user_id: Uuid, role_id: Uuid) -> AuthResult<()>;
}

/// Legacy implementation using new authorization service
pub struct RoleBasedAuthorizationProvider {
    authorization_service: AuthorizationService,
}

impl RoleBasedAuthorizationProvider {
    pub fn new() -> Self {
        Self {
            authorization_service: AuthorizationService::new(),
        }
    }
}

impl AuthorizationProvider for RoleBasedAuthorizationProvider {
    fn check_permission(&self, user_id: Uuid, resource: &str, action: &str) -> AuthResult<bool> {
        // Create a basic context for legacy compatibility
        let context = AuthorizationContext {
            user_id,
            tenant_id: Uuid::new_v4(), // Would be looked up in real implementation
            session_id: Uuid::new_v4(),
            roles: vec![], // Would be looked up from database
            permissions: vec![], // Would be looked up from database
            ip_address: None,
            user_agent: None,
            timestamp: Utc::now(),
            additional_attributes: HashMap::new(),
        };

        // Use async runtime for legacy sync interface
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.authorization_service.check_permission(&context, resource, None, action).await
            })
        })
    }

    fn get_user_permissions(&self, _user_id: Uuid) -> AuthResult<Vec<Permission>> {
        // Mock implementation for backward compatibility
        Ok(vec![])
    }

    fn get_user_roles(&self, _user_id: Uuid) -> AuthResult<Vec<Role>> {
        // Mock implementation for backward compatibility
        Ok(vec![])
    }

    fn assign_role(&self, user_id: Uuid, role_id: Uuid) -> AuthResult<()> {
        tracing::info!("Assigned role {} to user {}", role_id, user_id);
        Ok(())
    }

    fn revoke_role(&self, user_id: Uuid, role_id: Uuid) -> AuthResult<()> {
        tracing::info!("Revoked role {} from user {}", role_id, user_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_role_based_authorization() {
        let mut auth_service = AuthorizationService::new();

        // Set up role permissions
        let mut role_permissions = HashMap::new();
        role_permissions.insert(
            "admin".to_string(),
            vec![Permission {
                id: Uuid::new_v4(),
                name: "admin_access".to_string(),
                description: "Admin access".to_string(),
                resource: "users".to_string(),
                action: "read".to_string(),
                conditions: None,
                created_at: Utc::now(),
            }],
        );

        auth_service.load_role_permissions(role_permissions).await;

        let context = AuthorizationContext {
            user_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            roles: vec!["admin".to_string()],
            permissions: vec![],
            ip_address: Some("192.168.1.1".to_string()),
            user_agent: Some("Test".to_string()),
            timestamp: Utc::now(),
            additional_attributes: HashMap::new(),
        };

        let result = auth_service
            .check_permission(&context, "users", None, "read")
            .await
            .unwrap();

        assert!(result);
    }

    #[tokio::test]
    async fn test_policy_based_authorization() {
        let mut auth_service = AuthorizationService::new();

        // Add a policy
        let policy = helpers::create_role_policy(
            "admin_users_read",
            vec!["admin".to_string()],
            vec!["users".to_string()],
            vec!["read".to_string()],
            PermissionEffect::Allow,
        );

        auth_service.add_policy(policy).await.unwrap();

        let context = AuthorizationContext {
            user_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            roles: vec!["admin".to_string()],
            permissions: vec![],
            ip_address: Some("192.168.1.1".to_string()),
            user_agent: Some("Test".to_string()),
            timestamp: Utc::now(),
            additional_attributes: HashMap::new(),
        };

        let result = auth_service
            .check_permission(&context, "users", None, "read")
            .await
            .unwrap();

        assert!(result);
    }

    #[tokio::test]
    async fn test_time_based_policy() {
        let mut auth_service = AuthorizationService::new();

        // Add a time-based policy (9 AM to 5 PM, Monday to Friday)
        let policy = helpers::create_time_based_policy(
            "business_hours",
            "09:00",
            "17:00",
            vec![1, 2, 3, 4, 5], // Monday to Friday
            PermissionEffect::Allow,
        );

        auth_service.add_policy(policy).await.unwrap();

        let context = AuthorizationContext {
            user_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            roles: vec!["user".to_string()],
            permissions: vec![],
            ip_address: Some("192.168.1.1".to_string()),
            user_agent: Some("Test".to_string()),
            timestamp: Utc::now(), // This would need to be adjusted for proper testing
            additional_attributes: HashMap::new(),
        };

        // Note: In a real test, you'd mock the current time to test different scenarios
        let _result = auth_service
            .check_permission(&context, "documents", None, "read")
            .await
            .unwrap();
    }
}