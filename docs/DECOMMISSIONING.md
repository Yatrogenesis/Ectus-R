# Ectus-R Decommissioning Procedures

**Document Version:** 1.0
**Last Updated:** 2025-10-04
**Owner:** Platform Engineering Team
**Classification:** Internal

---

## Table of Contents

1. [Overview](#overview)
2. [Decommissioning Types](#decommissioning-types)
3. [Pre-Decommissioning](#pre-decommissioning)
4. [Data Export and Backup](#data-export-and-backup)
5. [Service Shutdown](#service-shutdown)
6. [Data Deletion](#data-deletion)
7. [Infrastructure Cleanup](#infrastructure-cleanup)
8. [Compliance and Audit](#compliance-and-audit)
9. [Communication](#communication)
10. [Checklists](#checklists)

---

## Overview

This document outlines procedures for decommissioning Ectus-R services, features, or the entire platform. These procedures ensure:

- Customer data is safely migrated or deleted
- Compliance with GDPR, CCPA, and other regulations
- Proper resource cleanup and cost optimization
- Audit trail for all decommissioning activities

---

## Decommissioning Types

### 1. Feature Decommissioning
Removing a specific feature while keeping the service running.

**Timeline:** 3-6 months

**Examples:**
- Deprecating an API endpoint
- Removing an experimental feature
- Sunsetting a legacy module

### 2. Service Decommissioning
Shutting down a microservice or component.

**Timeline:** 6-12 months

**Examples:**
- Retiring a microservice
- Migrating to a new architecture
- Consolidating services

### 3. Complete Platform Decommissioning
Full shutdown of the Ectus-R platform.

**Timeline:** 12-18 months

**Required Approvals:**
- CEO
- CTO
- Legal counsel
- Board of directors

---

## Pre-Decommissioning

### Phase 1: Planning (3-6 months before)

#### 1.1 Impact Assessment

```bash
# Identify affected users
psql -h $DB_HOST -U postgres ectus_r << EOF
SELECT
    COUNT(DISTINCT user_id) as active_users,
    COUNT(*) as total_sessions,
    feature_name
FROM feature_usage
WHERE feature_name = 'feature_to_decommission'
    AND last_used > NOW() - INTERVAL '90 days'
GROUP BY feature_name;
EOF
```

#### 1.2 Stakeholder Analysis

- [ ] Identify all stakeholders
- [ ] Document customer impact
- [ ] Assess business impact
- [ ] Legal review (contracts, SLAs)
- [ ] Security review (data handling)
- [ ] Compliance review (GDPR, CCPA)

#### 1.3 Migration Plan

Create migration guide for users:

```markdown
# Migration Guide: [Feature Name]

## Overview
[Brief description of change]

## Timeline
- **Announcement:** YYYY-MM-DD
- **Deprecation:** YYYY-MM-DD (read-only)
- **Sunset:** YYYY-MM-DD (fully removed)

## Alternative Solutions
1. [Recommended alternative]
2. [Manual process]
3. [Third-party tool]

## Migration Steps
[Detailed migration instructions]

## Support
- Documentation: [URL]
- Support email: migrations@ectus-r.com
- Office hours: Every Tuesday 2-4pm UTC
```

### Phase 2: Announcement (6-12 months before)

#### 2.1 Public Communication

- [ ] Blog post announcement
- [ ] Email to all affected users
- [ ] In-app notifications
- [ ] API deprecation headers
- [ ] Documentation updates
- [ ] Status page notice

**Deprecation Header Example:**

```rust
// Add to API responses
response.headers_mut().insert(
    "Sunset",
    "Sat, 31 Dec 2025 23:59:59 GMT".parse().unwrap()
);
response.headers_mut().insert(
    "Deprecation",
    "true".parse().unwrap()
);
response.headers_mut().insert(
    "Link",
    "<https://docs.ectus-r.com/migrations/feature-x>; rel=\"deprecation\"".parse().unwrap()
);
```

#### 2.2 Monitoring

Set up metrics to track:
- Usage decline over time
- Migration progress
- Support requests related to decommissioning

```promql
# Track feature usage over time
sum(rate(feature_requests_total{feature="deprecated_feature"}[1d]))
```

---

## Data Export and Backup

### User Data Export

#### 1. Provide Self-Service Export

```rust
// Endpoint for users to export their data
#[axum::debug_handler]
async fn export_user_data(
    State(state): State<AppState>,
    user_id: Uuid,
) -> Result<Json<UserDataExport>, StatusCode> {
    let user_data = UserDataExport {
        profile: fetch_user_profile(user_id).await?,
        projects: fetch_user_projects(user_id).await?,
        files: fetch_user_files(user_id).await?,
        analytics: fetch_user_analytics(user_id).await?,
        export_date: Utc::now(),
    };

    // Log export for compliance
    audit_log.record(AuditEvent::DataExport {
        user_id,
        timestamp: Utc::now(),
        data_types: vec!["profile", "projects", "files", "analytics"],
    }).await?;

    Ok(Json(user_data))
}
```

#### 2. Automated Backup Procedure

```bash
#!/bin/bash
# backup-before-decommission.sh

set -e

BACKUP_DATE=$(date +%Y%m%d)
BACKUP_DIR="/backups/decommission-${BACKUP_DATE}"
SERVICE_NAME="aion-web-api"

# Create backup directory
mkdir -p "${BACKUP_DIR}"

# 1. Database backup
echo "Backing up database..."
pg_dump \
    -h $DB_HOST \
    -U $DB_USER \
    -Fc \
    ectus_r \
    > "${BACKUP_DIR}/database-${BACKUP_DATE}.dump"

# 2. Object storage backup
echo "Backing up object storage..."
aws s3 sync \
    s3://ectus-r-production/ \
    "${BACKUP_DIR}/s3/" \
    --storage-class GLACIER

# 3. Configuration backup
echo "Backing up configurations..."
kubectl get configmap,secret -n production -o yaml \
    > "${BACKUP_DIR}/k8s-configs.yaml"

# 4. Monitoring data export
echo "Exporting monitoring data..."
curl -G http://prometheus:9090/api/v1/query_range \
    --data-urlencode 'query=up{job="'"${SERVICE_NAME}"'"}' \
    --data-urlencode 'start='$(date -d '90 days ago' +%s) \
    --data-urlencode 'end='$(date +%s) \
    > "${BACKUP_DIR}/metrics-${BACKUP_DATE}.json"

# 5. Logs export
echo "Exporting logs..."
kubectl logs -l app=${SERVICE_NAME} --since=90d \
    > "${BACKUP_DIR}/logs-${BACKUP_DATE}.txt"

# 6. Create checksum manifest
echo "Creating checksum manifest..."
find "${BACKUP_DIR}" -type f -exec sha256sum {} \; \
    > "${BACKUP_DIR}/checksums.txt"

# 7. Compress and encrypt
echo "Compressing and encrypting backup..."
tar -czf - "${BACKUP_DIR}" | \
    gpg --encrypt --recipient backup@ectus-r.com \
    > "${BACKUP_DIR}.tar.gz.gpg"

# 8. Upload to long-term storage
echo "Uploading to long-term storage..."
aws s3 cp \
    "${BACKUP_DIR}.tar.gz.gpg" \
    s3://ectus-r-decommission-backups/ \
    --storage-class DEEP_ARCHIVE

echo "Backup completed: ${BACKUP_DIR}.tar.gz.gpg"
```

---

## Service Shutdown

### Gradual Shutdown Procedure

#### Phase 1: Read-Only Mode (30-60 days)

```rust
// Enable read-only mode
async fn read_only_middleware(
    request: Request,
    next: Next,
) -> Response {
    match request.method() {
        &Method::GET | &Method::HEAD | &Method::OPTIONS => {
            next.run(request).await
        }
        _ => {
            Response::builder()
                .status(StatusCode::SERVICE_UNAVAILABLE)
                .header("Retry-After", "never")
                .header("Sunset", "Sat, 31 Dec 2025 23:59:59 GMT")
                .body(Body::from(json!({
                    "error": "Service is in read-only mode before shutdown",
                    "message": "This service will be decommissioned on 2025-12-31",
                    "migration_guide": "https://docs.ectus-r.com/migrations/service-x"
                }).to_string()))
                .unwrap()
                .into_response()
        }
    }
}
```

#### Phase 2: Service Shutdown

```bash
#!/bin/bash
# shutdown-service.sh

set -e

SERVICE="aion-web-api"
NAMESPACE="production"

echo "Starting graceful shutdown of ${SERVICE}..."

# 1. Remove from load balancer
echo "Removing from load balancer..."
kubectl patch service ${SERVICE} \
    -n ${NAMESPACE} \
    -p '{"spec":{"selector":{"decommissioned":"true"}}}'

# 2. Wait for connections to drain
echo "Waiting for connections to drain (5 minutes)..."
sleep 300

# 3. Scale down to 1 replica
echo "Scaling down to 1 replica..."
kubectl scale deployment ${SERVICE} --replicas=1 -n ${NAMESPACE}

# 4. Wait and monitor
echo "Monitoring for active connections..."
while [[ $(kubectl exec -it deployment/${SERVICE} -n ${NAMESPACE} -- \
    netstat -an | grep ESTABLISHED | wc -l) -gt 0 ]]; do
    echo "Waiting for connections to close..."
    sleep 60
done

# 5. Final shutdown
echo "Shutting down service..."
kubectl delete deployment ${SERVICE} -n ${NAMESPACE}

echo "Service shutdown complete"
```

---

## Data Deletion

### GDPR-Compliant Data Deletion

```rust
// Data deletion service
pub struct DataDeletionService {
    db: PgPool,
    s3_client: S3Client,
    audit_log: AuditLogger,
}

impl DataDeletionService {
    /// Delete all user data in compliance with GDPR
    pub async fn delete_user_data(
        &self,
        user_id: Uuid,
        deletion_reason: DeletionReason,
    ) -> Result<DeletionReport> {
        let mut report = DeletionReport::new(user_id);

        // 1. Anonymize personal data
        self.anonymize_user_profile(user_id).await?;
        report.anonymized.push("user_profile");

        // 2. Delete files from S3
        let file_keys = self.list_user_files(user_id).await?;
        for key in file_keys {
            self.s3_client.delete_object()
                .bucket("ectus-r-production")
                .key(&key)
                .send()
                .await?;
            report.deleted_files.push(key);
        }

        // 3. Delete database records
        sqlx::query!(
            "DELETE FROM user_sessions WHERE user_id = $1",
            user_id
        ).execute(&self.db).await?;

        sqlx::query!(
            "DELETE FROM user_preferences WHERE user_id = $1",
            user_id
        ).execute(&self.db).await?;

        // 4. Mark user as deleted (for compliance audit trail)
        sqlx::query!(
            "UPDATE users SET
                deleted_at = NOW(),
                deletion_reason = $2,
                email = 'deleted_' || id || '@deleted.local',
                name = 'Deleted User'
            WHERE id = $1",
            user_id,
            deletion_reason.to_string()
        ).execute(&self.db).await?;

        // 5. Create audit log
        self.audit_log.record(AuditEvent::UserDataDeleted {
            user_id,
            timestamp: Utc::now(),
            deletion_reason,
            items_deleted: report.total_items(),
        }).await?;

        report.completed_at = Some(Utc::now());
        Ok(report)
    }
}
```

---

## Infrastructure Cleanup

### Kubernetes Resource Cleanup

```bash
#!/bin/bash
# cleanup-kubernetes.sh

NAMESPACE="production"
SERVICE="aion-web-api"

# Delete all resources
kubectl delete deployment ${SERVICE} -n ${NAMESPACE}
kubectl delete service ${SERVICE} -n ${NAMESPACE}
kubectl delete ingress ${SERVICE} -n ${NAMESPACE}
kubectl delete hpa ${SERVICE} -n ${NAMESPACE}
kubectl delete pvc -l app=${SERVICE} -n ${NAMESPACE}
kubectl delete configmap -l app=${SERVICE} -n ${NAMESPACE}
kubectl delete secret -l app=${SERVICE} -n ${NAMESPACE}

# Clean up volumes
kubectl delete pv $(kubectl get pv -o json | jq -r \
    '.items[] | select(.spec.claimRef.name | contains("'"${SERVICE}"'")) | .metadata.name')
```

### Cloud Resources Cleanup

```bash
#!/bin/bash
# cleanup-aws.sh

# Delete Load Balancers
aws elbv2 delete-load-balancer \
    --load-balancer-arn arn:aws:elasticloadbalancing:...

# Delete Target Groups
aws elbv2 delete-target-group \
    --target-group-arn arn:aws:elasticloadbalancing:...

# Delete Auto Scaling Groups
aws autoscaling delete-auto-scaling-group \
    --auto-scaling-group-name ectus-r-production \
    --force-delete

# Delete RDS instances (after final backup)
aws rds delete-db-instance \
    --db-instance-identifier ectus-r-production \
    --final-db-snapshot-identifier ectus-r-final-snapshot-$(date +%Y%m%d)

# Delete S3 buckets (after archival)
aws s3 rb s3://ectus-r-production --force

# Delete CloudWatch log groups
aws logs delete-log-group \
    --log-group-name /aws/ecs/ectus-r-production
```

---

## Compliance and Audit

### Decommissioning Audit Trail

Required documentation:

1. **Decision Record**
   - Who approved decommissioning
   - Business justification
   - Risk assessment

2. **Communication Log**
   - All customer notifications
   - Internal communications
   - Support tickets related to decommissioning

3. **Data Handling Certificate**
   - Data export completion
   - Data deletion verification
   - Backup verification

4. **Infrastructure Teardown Report**
   - Resources deleted
   - Cost savings
   - Security clearance

### Compliance Checklist

- [ ] GDPR: Right to data portability provided
- [ ] GDPR: Right to erasure fulfilled
- [ ] CCPA: Data deletion requests honored
- [ ] SOC 2: Audit trail maintained
- [ ] ISO 27001: Secure disposal procedures followed
- [ ] HIPAA: PHI properly disposed (if applicable)

---

## Communication

### Timeline

| Milestone | Communication |
|-----------|---------------|
| T-12 months | Initial announcement |
| T-6 months | Reminder + migration guide |
| T-3 months | Deprecation warning in API |
| T-1 month | Final warning, read-only mode |
| T-1 week | Daily reminders |
| T-0 | Shutdown |
| T+1 week | Post-shutdown summary |

### Templates

**Initial Announcement Email:**

```
Subject: Important: [Feature/Service] Decommissioning Notice

Dear Ectus-R User,

We are writing to inform you that [feature/service] will be decommissioned on [date].

TIMELINE:
- Today: Announcement
- [Date]: Deprecation (read-only)
- [Date]: Complete shutdown

WHAT THIS MEANS:
- [Specific impact to users]

MIGRATION OPTIONS:
1. [Recommended alternative]
2. [Self-hosted option]
3. [Data export only]

SUPPORT:
- Migration guide: [URL]
- Support email: migrations@ectus-r.com
- Q&A session: [Date/Time]

We appreciate your understanding.

Best regards,
Ectus-R Team
```

---

## Checklists

### Feature Decommissioning Checklist

- [ ] Impact assessment completed
- [ ] Legal approval obtained
- [ ] Migration guide published
- [ ] Users notified (6 months before)
- [ ] API deprecation headers added
- [ ] Analytics tracking enabled
- [ ] Read-only mode enabled (30 days before)
- [ ] Final backup created
- [ ] Feature disabled
- [ ] Code removed from codebase
- [ ] Documentation archived
- [ ] Post-mortem completed

### Service Decommissioning Checklist

- [ ] Stakeholder approval
- [ ] Customer impact assessment
- [ ] Migration plan created
- [ ] 12-month notification sent
- [ ] 6-month reminder sent
- [ ] 3-month deprecation notice
- [ ] 1-month final warning
- [ ] Data export tools provided
- [ ] Final backup completed
- [ ] Service stopped
- [ ] Data deleted
- [ ] Infrastructure cleaned up
- [ ] DNS records removed
- [ ] SSL certificates revoked
- [ ] Cost savings verified
- [ ] Audit report completed

### Complete Platform Decommissioning Checklist

- [ ] Board approval
- [ ] Legal review
- [ ] Customer refund plan
- [ ] Employee transition plan
- [ ] 18-month notice period
- [ ] Asset liquidation plan
- [ ] IP/patent transfer
- [ ] Data destruction certificate
- [ ] Final audit
- [ ] Company dissolution (if applicable)

---

## Related Documents

- [Data Retention Policy](./DATA_RETENTION.md)
- [GDPR Compliance Guide](./GDPR.md)
- [Backup and Recovery](./BACKUP_RECOVERY.md)
- [Incident Response](./INCIDENT_RESPONSE.md)

---

**Approval:**
- Platform Engineering: [Signature]
- Legal: [Signature]
- Security: [Signature]
- Compliance: [Signature]

**Version History:**
| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-10-04 | DevOps Team | Initial version |
