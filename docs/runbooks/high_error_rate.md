# Runbook: High HTTP Error Rate

**Alert Name:** `HighHTTPErrorRate`
**Severity:** Critical
**Component:** API
**Threshold:** 5% error rate over 5 minutes

---

## Description

The HTTP 5xx error rate has exceeded 5% over the last 5 minutes, indicating a significant problem with the service that is affecting users.

---

## Impact

- **User Impact:** HIGH - Users are experiencing service errors
- **Business Impact:** HIGH - Loss of service availability affects all users
- **SLA Impact:** May violate 99.9% availability SLA if sustained

---

## Diagnosis

### Step 1: Check Service Health

```bash
# Check if service is running
curl http://localhost:8080/health

# Check recent logs
journalctl -u aion-web-api -n 100 --no-pager

# Check Prometheus metrics
curl http://localhost:9090/metrics | grep http_requests_total
```

### Step 2: Identify Error Sources

```bash
# Check error breakdown by endpoint
# Query Prometheus:
sum by (path, status) (rate(http_requests_total{status=~"5.."}[5m]))

# Check application logs for stack traces
tail -f /var/log/aion/web-api.log | grep ERROR
```

### Step 3: Check Dependencies

```bash
# Check database connectivity
psql -h localhost -U ectus_r -c "SELECT 1;"

# Check Redis
redis-cli ping

# Check external API health
curl https://api.openai.com/v1/health
```

---

## Mitigation

### Immediate Actions (0-5 minutes)

1. **If service is down**: Restart the service
   ```bash
   systemctl restart aion-web-api
   ```

2. **If database is down**: Restart database or failover
   ```bash
   systemctl restart postgresql
   # Or failover to replica
   ```

3. **If external API is failing**: Enable circuit breaker
   ```bash
   # Update configuration to enable circuit breaker
   curl -X POST http://localhost:8080/admin/circuit-breaker/enable
   ```

### Short-term Fix (5-30 minutes)

1. **Scale horizontally** if traffic spike
   ```bash
   kubectl scale deployment aion-web-api --replicas=10
   ```

2. **Rate limit** if under attack
   ```bash
   # Update rate limits in configuration
   kubectl apply -f k8s/rate-limits.yaml
   ```

3. **Rollback** if recent deployment caused issue
   ```bash
   kubectl rollout undo deployment/aion-web-api
   ```

---

## Resolution

### Root Cause Analysis

After mitigation, investigate:

1. Check git commits in last 24 hours
2. Review deployment history
3. Analyze error patterns
4. Review infrastructure changes

### Prevention

1. Add regression tests for identified issue
2. Improve monitoring coverage
3. Update deployment procedures
4. Document lessons learned

---

## Communication

### During Incident

1. Create incident channel: `#incident-YYYYMMDD-NNN`
2. Post initial status update
3. Update stakeholders every 15 minutes
4. Escalate to management if >30 minutes

### After Resolution

1. Post-mortem meeting within 48 hours
2. Document root cause
3. Assign action items
4. Update runbook with learnings

---

## Escalation

- **Level 1** (0-15 min): On-call engineer
- **Level 2** (15-30 min): Engineering manager
- **Level 3** (30+ min): VP Engineering + CTO

**On-call Phone:** +1-XXX-XXX-XXXX (PagerDuty)

---

## Related

- [Service Architecture](../ARCHITECTURE.md)
- [Deployment Guide](../DEPLOYMENT.md)
- [API Documentation](../API.md)
- [High Latency Runbook](./high_latency.md)
- [Service Down Runbook](./service_down.md)

---

**Last Updated:** 2025-10-04
**Version:** 1.0
**Owner:** DevOps Team
