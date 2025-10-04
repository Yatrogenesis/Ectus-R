# On-Call Setup and Procedures
## AION-R Platform

**Version:** 1.0
**Last Updated:** 2025-10-04
**Owner:** Platform Engineering Team

---

## Table of Contents

1. [Overview](#overview)
2. [On-Call Rotation Schedule](#on-call-rotation-schedule)
3. [On-Call Responsibilities](#on-call-responsibilities)
4. [Response Time SLAs](#response-time-slas)
5. [Escalation Tiers](#escalation-tiers)
6. [On-Call Handoff Procedures](#on-call-handoff-procedures)
7. [Tools and Access](#tools-and-access)
8. [Compensation and Time Off](#compensation-and-time-off)
9. [Best Practices](#best-practices)

---

## Overview

### Purpose

The on-call rotation ensures 24/7 coverage for the AION-R platform, providing rapid response to incidents and maintaining service reliability.

### Scope

- **Primary on-call:** First responder for all alerts
- **Secondary on-call:** Backup and escalation point
- **Coverage:** 24/7/365
- **Duration:** 1 week per rotation (Monday 00:00 to Monday 00:00 UTC)

### Eligibility

Engineers eligible for on-call rotation must:
- Complete on-call training program
- Have production access credentials
- Have completed at least 2 shadow rotations
- Be familiar with incident response playbook
- Have PagerDuty mobile app installed

---

## On-Call Rotation Schedule

### Rotation Structure

**Team:** Platform Engineering (8 engineers)

**Rotation Schedule:**
- Rotation length: 1 week
- Handoff time: Monday 00:00 UTC
- Notice period: Schedule published 4 weeks in advance

**Tiers:**
1. **Primary:** First responder
2. **Secondary:** Backup (escalation after 15 minutes)
3. **Tertiary:** Team lead (escalation for SEV-1 or >1 hour incidents)

### Current Schedule

See PagerDuty for live schedule: https://aion.pagerduty.com/schedules

**Example Rotation (Q4 2025):**

| Week | Primary | Secondary | Tertiary |
|------|---------|-----------|----------|
| Oct 7-13 | Engineer A | Engineer B | Team Lead X |
| Oct 14-20 | Engineer B | Engineer C | Team Lead X |
| Oct 21-27 | Engineer C | Engineer D | Team Lead Y |
| Oct 28-Nov 3 | Engineer D | Engineer E | Team Lead Y |

### Schedule Modifications

**Swap Process:**
1. Find someone to swap with (check #on-call-swaps Slack channel)
2. Both parties confirm in writing
3. Update PagerDuty schedule
4. Notify team in #platform-alerts

**Emergency Coverage:**
If unable to cover shift:
1. Immediately notify team lead
2. Post in #platform-alerts
3. Team lead arranges emergency coverage
4. Document reason for future planning

---

## On-Call Responsibilities

### Primary Duties

#### 1. Alert Response

**Requirements:**
- Acknowledge alerts within 5 minutes
- Begin investigation within 10 minutes
- Provide initial status update within 15 minutes

**Response channels:**
- PagerDuty mobile app (critical)
- Slack (#platform-alerts)
- Email (non-critical)

#### 2. Incident Management

**For SEV-1/2:**
- Act as Incident Commander (or delegate)
- Coordinate response team
- Provide regular status updates
- Document incident timeline
- Ensure proper escalation

**For SEV-3/4:**
- Investigate and resolve independently
- Document in ticket system
- Escalate if stuck >1 hour

#### 3. Monitoring and Proactive Actions

**Daily tasks:**
- Review dashboard health metrics
- Check for patterns in alerts
- Review overnight logs for warnings
- Verify backup completion
- Test critical user flows

**Monitoring checklist:**
```bash
# Check service health
kubectl get pods -n aion-production --watch

# Review metrics
# Open Grafana: http://grafana.aion.internal:3000
# Dashboard: AION Overview

# Check for anomalies
- Error rate: <0.1% expected
- Latency p95: <500ms expected
- Active sessions: Normal range 100-1000

# Verify backups
aws s3 ls s3://aion-backups/postgres/$(date +%Y-%m-%d)/
```

#### 4. Documentation

**Required documentation:**
- Incident timeline for SEV-1/2
- Runbook updates based on learnings
- Known issues log
- Handoff notes for next on-call

---

## Response Time SLAs

### Alert Acknowledgement

| Severity | Target | Maximum |
|----------|--------|---------|
| SEV-1 | 5 minutes | 10 minutes |
| SEV-2 | 10 minutes | 20 minutes |
| SEV-3 | 30 minutes | 1 hour |
| SEV-4 | 2 hours | Next business day |

### Initial Status Update

| Severity | Target | Channel |
|----------|--------|---------|
| SEV-1 | 15 minutes | #incidents + status page |
| SEV-2 | 30 minutes | #incidents |
| SEV-3 | 1 hour | Ticket update |
| SEV-4 | 4 hours | Ticket update |

### Escalation Timeouts

**Automatic escalation if:**
- Alert not acknowledged within maximum time
- No status update provided in 30 minutes (SEV-1/2)
- Incident unresolved after:
  - SEV-1: 1 hour → Escalate to secondary + management
  - SEV-2: 2 hours → Escalate to secondary
  - SEV-3: 4 hours → Escalate to team lead

---

## Escalation Tiers

### Tier 1: Primary On-Call

**Escalate to Tier 2 when:**
- Unable to acknowledge within 5 minutes
- Need domain expertise (AI, database, etc.)
- Incident complexity exceeds comfort level
- SEV-1 requiring multiple responders

**Escalation procedure:**
```bash
# 1. Page secondary on-call via PagerDuty
# 2. Post in #incidents:
"ESCALATION TO SECONDARY
Incident: [Link]
Reason: [Why escalating]
Current status: [Brief summary]"

# 3. Brief secondary on current state
# 4. Transfer IC role or work together
```

### Tier 2: Secondary On-Call

**Escalate to Tier 3 when:**
- SEV-1 incident
- Incident duration >2 hours
- Cross-team coordination needed
- Decision requires management approval
- Customer communication needed

### Tier 3: Team Lead / Engineering Manager

**Escalate to Tier 4 when:**
- Incident duration >4 hours
- Major customer impact
- Media/PR attention
- Legal/compliance concerns
- C-level stakeholder inquiry

### Tier 4: VP Engineering / CTO

**Reserved for:**
- Major outages (>4 hours)
- Company-wide impact
- Security breaches
- Regulatory reporting required

---

## On-Call Handoff Procedures

### Handoff Timing

- **Day:** Monday morning, 00:00 UTC
- **Duration:** 30-minute call
- **Required attendees:** Outgoing primary + Incoming primary
- **Optional:** Secondary, team lead

### Handoff Checklist

#### Outgoing On-Call

**Before handoff call:**
- [ ] Review all incidents from past week
- [ ] Document any recurring issues
- [ ] Update known issues log
- [ ] Prepare handoff notes
- [ ] Update runbooks if needed

**Handoff document template:**

```markdown
# On-Call Handoff - [Date]

## Summary
- Incidents this week: [Count by severity]
- Major incidents: [Brief list]
- System health: [Overall assessment]

## Active Issues
### Issue 1: [Title]
- Status: [Current state]
- Impact: [User impact]
- Next steps: [What needs doing]
- Owner: [If not resolved]

## Recurring Problems
- [Problem description]
- Frequency: [How often]
- Mitigation: [Current workaround]
- Action item: [Long-term fix needed]

## Known Issues
- [Issue 1]: [Description and impact]
- [Issue 2]: [Description and impact]

## Recent Changes
- [Date]: [Deployment or change]
- Impact: [Any issues observed]

## Monitoring Notes
- [Any monitoring anomalies]
- [Dashboard issues]
- [Alert tuning needed]

## Recommendations
- [Suggested improvements]
- [Documentation updates needed]
- [Tools or access issues]
```

#### Incoming On-Call

**During handoff call:**
- [ ] Ask clarifying questions
- [ ] Verify understanding of active issues
- [ ] Confirm escalation contacts
- [ ] Test alert delivery
- [ ] Verify access to all tools

**After handoff:**
- [ ] Read through handoff notes
- [ ] Review incident response playbook
- [ ] Check PagerDuty schedule
- [ ] Verify phone notifications working
- [ ] Confirm laptop/tools ready
- [ ] Review recent deployments

**Access verification checklist:**
```bash
# 1. Verify Kubernetes access
kubectl get pods -n aion-production

# 2. Test monitoring access
# Grafana: http://grafana.aion.internal:3000
# Prometheus: http://prometheus.aion.internal:9090

# 3. Verify PagerDuty
# Mobile app: Test notification delivery

# 4. Check Slack
# Channels: #incidents, #platform-alerts

# 5. Test deployment access
# GitHub Actions: Verify permissions
# Kubernetes: Verify deploy permissions
```

---

## Tools and Access

### Required Tools

#### 1. PagerDuty

**Purpose:** Alert notification and incident management

**Setup:**
1. Install mobile app: iOS/Android
2. Configure notification sounds (loud, unique)
3. Enable push notifications
4. Add phone number for SMS backup
5. Test notification delivery

**Configuration:**
- Profile: https://aion.pagerduty.com/users/me
- Notification rules:
  - Push notification: Immediately
  - SMS: After 2 minutes
  - Phone call: After 5 minutes

#### 2. Slack

**Required channels:**
- #incidents - Active incident coordination
- #platform-alerts - Automated alerts
- #on-call-swaps - Schedule changes
- #platform - Team communication

**Configuration:**
- Enable mobile notifications for #incidents
- Set up DND schedule (except #incidents)

#### 3. Kubernetes Access

**Setup:**
```bash
# 1. Install kubectl
brew install kubectl  # macOS
# or download from kubernetes.io

# 2. Configure contexts
kubectl config use-context production

# 3. Verify access
kubectl get pods -n aion-production

# 4. Set up aliases
echo "alias k='kubectl'" >> ~/.bashrc
echo "alias kp='kubectl get pods -n aion-production'" >> ~/.bashrc
```

#### 4. Monitoring Tools

**Grafana:**
- URL: http://grafana.aion.internal:3000
- Login: SSO or admin credentials
- Bookmark: AION Overview dashboard

**Prometheus:**
- URL: http://prometheus.aion.internal:9090
- Direct query access

**Jaeger:**
- URL: http://jaeger.aion.internal:16686
- Trace analysis

**Kibana (Logs):**
- URL: http://kibana.aion.internal:5601
- Index: `aion-*`

#### 5. VPN Access

**Required for:**
- Production access
- Monitoring tools
- Database access

**Setup:**
1. Install VPN client
2. Import configuration
3. Test connection
4. Set up auto-connect

### Access Checklist

Before going on-call, verify:
- [ ] PagerDuty mobile app installed and tested
- [ ] Kubernetes access working
- [ ] VPN configured and tested
- [ ] Grafana access verified
- [ ] GitHub access confirmed
- [ ] Slack notifications enabled
- [ ] Phone charged and service working
- [ ] Laptop charged and ready
- [ ] Internet connection stable

---

## Compensation and Time Off

### On-Call Pay

**Structure:**
- Base on-call stipend: $XXX per week
- Additional incident pay: $YY per hour (after hours)
- Weekend incident pay: $ZZ per hour

**Payment schedule:**
- Processed with regular payroll
- Documented in HRIS system

### Incident Hours

**Billable time:**
- After-hours incidents (outside 9am-5pm local time)
- Weekend incidents (Saturday/Sunday)
- Minimum billing: 30 minutes per incident
- Rounded to nearest 15 minutes

**Not billable:**
- Business hours incidents (part of normal duties)
- Proactive monitoring (included in base stipend)
- Handoff calls (part of rotation)

### Time Off During On-Call

**Planned Time Off:**
- Request coverage minimum 2 weeks advance
- Find your own swap (or team lead assists)
- Not on-call during vacation/PTO

**Sick Days:**
- Notify team lead immediately
- Team lead arranges emergency coverage
- No penalty for legitimate illness

### Comp Time

**After major incidents:**
- SEV-1 incidents >4 hours: 1 comp day
- SEV-1 incidents >8 hours: 2 comp days
- Weekend SEV-1: Automatic Monday comp day

**Usage:**
- Use within 90 days
- Manager approval required
- Coordinate with team

---

## Best Practices

### Preparation

**Before your rotation:**
1. Review recent incidents
2. Update runbooks if needed
3. Test all access
4. Read handoff notes
5. Review monitoring dashboards
6. Check battery: laptop, phone, backup phone

**During your rotation:**
1. Keep phone charged and nearby
2. Test internet connectivity
3. Stay within 15 minutes of laptop
4. Avoid alcohol if you might be impaired
5. Have backup communication method

### Communication

**Best practices:**
1. Acknowledge alerts immediately (even if still investigating)
2. Provide frequent updates (better to over-communicate)
3. Document everything in incident timeline
4. Ask for help early (don't be a hero)
5. Thank team members who assist

**Communication template:**
```
INCIDENT UPDATE - [HH:MM UTC]
Status: [Investigating/Identified/Monitoring/Resolved]
Impact: [Current user impact]
Actions: [What's been done]
Next: [What's next]
ETA: [When expect update/resolution]
```

### Incident Response

**Key principles:**
1. Safety first (don't make it worse)
2. Stop the bleeding (mitigate before fix)
3. Preserve evidence (for post-incident review)
4. Document actions (write as you go)
5. Learn and improve (update runbooks)

**When in doubt:**
- Escalate early
- Ask questions
- Rollback over rolling forward
- Safety over perfection

### Self-Care

**During rotation:**
- Get adequate sleep
- Take breaks
- Step away from computer between incidents
- Don't check alerts constantly (trust PagerDuty)

**After rotation:**
- Decompress after handoff
- Take comp time if earned
- Provide feedback on rotation
- Update documentation

**Burnout prevention:**
- Rotate fairly (no back-to-back weeks)
- Respect time off
- Recognize contributions
- Improve automation to reduce alerts

---

## Appendix

### A. Emergency Contact Information

**On-Call Tiers:**
```
Primary On-Call
├─ See PagerDuty schedule

Secondary On-Call
├─ See PagerDuty schedule

Team Lead
├─ Name: [Team Lead Name]
├─ Slack: @team-lead
├─ Phone: +1-XXX-XXX-XXXX
└─ Email: team.lead@aion.com

Engineering Manager
├─ Name: [Manager Name]
├─ Slack: @eng-manager
├─ Phone: +1-XXX-XXX-XXXX (emergencies only)
└─ Email: manager@aion.com

VP Engineering
├─ Name: [VP Name]
├─ Slack: @vp-eng
├─ Phone: +1-XXX-XXX-XXXX (SEV-1 only)
└─ Email: vp.eng@aion.com
```

### B. Common Alert Responses

**High Error Rate:**
1. Check recent deployments
2. Review logs for error patterns
3. Check database health
4. Consider rollback
5. See: `/docs/runbooks/high_error_rate.md`

**Service Down:**
1. Check pod status
2. Review recent changes
3. Check resource availability
4. Restart pods if needed
5. See: `/docs/runbooks/service_down.md`

**High Latency:**
1. Check database connection pool
2. Review slow query log
3. Check AI inference duration
4. Check for resource constraints
5. See: `/docs/runbooks/high_latency.md`

### C. Quick Reference Commands

```bash
# Service status
kubectl get pods -n aion-production
kubectl get svc -n aion-production
kubectl top pods -n aion-production

# Recent deployments
kubectl rollout history deployment/aion-web-api -n aion-production

# Logs
kubectl logs -f deployment/aion-web-api -n aion-production --tail=100

# Rollback
kubectl rollout undo deployment/aion-web-api -n aion-production

# Scale
kubectl scale deployment/aion-web-api --replicas=6 -n aion-production

# Metrics
curl http://prometheus:9090/api/v1/query?query=up
```

### D. On-Call Training Program

**Required training (before first rotation):**
1. Incident response playbook review (1 hour)
2. Runbook walkthroughs (2 hours)
3. Tool access setup (1 hour)
4. Shadow rotation #1 (1 week)
5. Shadow rotation #2 (1 week)
6. Solo rotation with team lead backup (1 week)

**Continuing education:**
- Post-incident review attendance
- Quarterly runbook reviews
- Tool updates training
- Process improvement feedback

---

**Document Version:** 1.0
**Last Reviewed:** 2025-10-04
**Next Review:** 2025-11-04
**Owner:** Platform Engineering Team
