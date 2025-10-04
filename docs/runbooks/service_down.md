# Runbook: Service Down

**Alert Name:** `ServiceDown`
**Severity:** Critical
**Component:** API
**Threshold:** Service unreachable for 2 minutes

---

## Description

The AION Web API service is not responding to health checks and is considered down. This is a complete service outage affecting all users.

---

## Impact

- **User Impact:** CRITICAL - Complete service outage
- **Business Impact:** CRITICAL - All functionality unavailable
- **SLA Impact:** Direct SLA violation (99.9% uptime target)

---

## Diagnosis

### Step 1: Verify Service Status

```bash
# Check systemd status
systemctl status aion-web-api

# Check process
ps aux | grep aion-web-api

# Check listening ports
netstat -tulpn | grep 8080

# Check recent crashes
journalctl -u aion-web-api --since "10 minutes ago" | grep -i "error\|crash\|fatal"
```

### Step 2: Check System Resources

```bash
# Check disk space
df -h

# Check memory
free -h

# Check CPU load
top -bn1 | head -20

# Check OOM kills
dmesg | grep -i "out of memory"
journalctl -k | grep -i "killed process"
```

### Step 3: Check Dependencies

```bash
# PostgreSQL
systemctl status postgresql
psql -h localhost -U postgres -c "SELECT 1"

# Redis
systemctl status redis
redis-cli ping

# Network connectivity
ping -c 3 8.8.8.8
```

---

## Mitigation

### Immediate Actions (0-2 minutes)

1. **Restart the service**
   ```bash
   systemctl restart aion-web-api

   # Verify startup
   journalctl -u aion-web-api -f

   # Check health
   curl http://localhost:8080/health
   ```

2. **If restart fails, check logs**
   ```bash
   journalctl -u aion-web-api -n 200 --no-pager
   ```

3. **If database is the issue**
   ```bash
   # Restart PostgreSQL
   systemctl restart postgresql

   # Or failover to replica
   pg_ctl promote -D /var/lib/postgresql/data
   ```

### Kubernetes Environment

```bash
# Check pod status
kubectl get pods -l app=aion-web-api

# Check pod logs
kubectl logs -l app=aion-web-api --tail=100

# Restart pods
kubectl rollout restart deployment/aion-web-api

# If persistent failure, rollback
kubectl rollout undo deployment/aion-web-api
```

### Emergency Fallback (2-5 minutes)

If primary region is down:

1. **Failover to backup region**
   ```bash
   # Update DNS to point to backup region
   aws route53 change-resource-record-sets \
     --hosted-zone-id Z1234567890ABC \
     --change-batch file://failover-dns.json
   ```

2. **Activate standby instances**
   ```bash
   # Scale up standby deployment
   kubectl scale deployment aion-web-api-standby --replicas=5
   ```

---

## Resolution Checklist

- [ ] Service restarted successfully
- [ ] Health check passes
- [ ] All pods running (if Kubernetes)
- [ ] Database connectivity confirmed
- [ ] External API connectivity confirmed
- [ ] Error rate < 1%
- [ ] Latency < 500ms p95
- [ ] Incident documented

---

## Post-Incident

### Immediate (within 1 hour)

1. Verify all metrics are normal
2. Run smoke tests
3. Monitor for 30 minutes
4. Update status page
5. Notify stakeholders of resolution

### Follow-up (within 24 hours)

1. Complete incident report
2. Root cause analysis
3. Post-mortem meeting
4. Action items assignment

---

## Prevention

1. **Implement health checks**
   - Liveness probe: `/health`
   - Readiness probe: `/ready`
   - Startup probe: `/startup`

2. **Resource limits**
   ```yaml
   resources:
     requests:
       memory: "512Mi"
       cpu: "500m"
     limits:
       memory: "2Gi"
       cpu: "2000m"
   ```

3. **Auto-scaling**
   ```yaml
   autoscaling:
     enabled: true
     minReplicas: 3
     maxReplicas: 10
     targetCPUUtilizationPercentage: 70
   ```

4. **Circuit breakers** for external dependencies

5. **Chaos engineering** to test failure scenarios

---

## Communication Template

### Initial Alert (within 2 minutes)

```
INCIDENT: Service Down
Status: Investigating
Impact: Complete service outage
Started: YYYY-MM-DD HH:MM UTC
ETA: Under investigation
Updates: Every 5 minutes in #incident-channel
```

### Resolution (when resolved)

```
RESOLVED: Service Down
Status: Resolved
Duration: XX minutes
Root Cause: [Brief description]
Next Steps: Post-mortem scheduled for [date/time]
Incident Report: [Link]
```

---

## Escalation

- **Immediate:** Page on-call engineer (PagerDuty)
- **5 minutes:** Escalate to engineering manager
- **10 minutes:** Escalate to VP Engineering
- **15 minutes:** Notify CEO/CTO
- **30 minutes:** External communication (status page, Twitter)

**On-call Contacts:**
- Primary: +1-XXX-XXX-XXXX
- Secondary: +1-XXX-XXX-XXXX
- Manager: +1-XXX-XXX-XXXX

---

## Related

- [High Error Rate Runbook](./high_error_rate.md)
- [Database Connection Issues](./db_connections.md)
- [Disaster Recovery Plan](../DR_PLAN.md)
- [Incident Response Process](../INCIDENT_RESPONSE.md)

---

**Last Updated:** 2025-10-04
**Version:** 1.0
**Owner:** DevOps Team
