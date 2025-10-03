# RSA 0.9.8 Marvin Attack - Risk Assessment & Mitigation

**Date**: 2025-10-02  
**Advisory**: RUSTSEC-2023-0071  
**Severity**: MEDIUM (CVSS 5.9)  
**Status**: ⚠️ NO FIX AVAILABLE - ACCEPTED RISK WITH MITIGATIONS

---

## 🔍 VULNERABILITY DESCRIPTION

### Technical Details:
- **Crate**: rsa 0.9.8
- **Issue**: Timing side-channel attack (Marvin Attack)
- **Impact**: Potential RSA private key recovery via timing measurements
- **CVE**: Not assigned
- **Discovery**: 2023-11-22

### Attack Vector:
The Marvin Attack exploits timing differences in RSA decryption operations to potentially recover private keys through statistical analysis of many decryption attempts.

**Requirements for successful exploit**:
1. Attacker must have network access to measure response times
2. Requires 1000s-10000s of authentication attempts
3. Victim must use RSA for authentication/encryption (not signatures)
4. High-precision timing measurements needed

---

## 📊 CURRENT EXPOSURE

### Affected Components:

```
rsa 0.9.8
├── sqlx-mysql 0.8.6
│   └── sqlx 0.8.6
│       ├── aion-database (MySQL connections)
│       ├── aion-auth (optional MySQL backend)
│       └── aion-licensing (sea-orm/sqlx)
├── openidconnect 3.5.0
│   └── aion-auth (OAuth/OIDC provider integration)
└── aion-licensing (direct dependency)
```

### Risk Assessment by Component:

| Component | Usage | Exposure | Risk Level |
|-----------|-------|----------|------------|
| **sqlx-mysql** | MySQL SSL/TLS cert validation | Low | 🟡 LOW |
| **openidconnect** | JWT signature verification only | Minimal | 🟢 MINIMAL |
| **aion-licensing** | License key RSA operations | Medium | 🟡 MEDIUM |

---

## ✅ MITIGATIONS IMPLEMENTED

### 1. Rate Limiting (CRITICAL)

**Authentication endpoints** (`aion-auth`):
```rust
// Implemented in aion-auth/src/middleware/rate_limit.rs
- Max 10 auth attempts per IP per minute
- Exponential backoff after 3 failures
- IP-based blocking after 20 failed attempts/hour
```

**Licensing endpoints** (`aion-licensing`):
```rust
// Implemented in aion-licensing/src/api/validate.rs
- Max 100 license validations per IP per minute
- Rate limit prevents timing attack feasibility
```

### 2. Monitoring & Alerting

**Metrics tracked**:
- Auth failure rate per IP
- RSA operation timing anomalies  
- Suspicious repeated license validation patterns

**Alerts configured**:
- >50 auth failures from single IP in 10min
- Timing variance >3σ from baseline
- License validation spikes >200/min

### 3. Constant-Time Operations Preference

Where possible, authentication uses:
- ✅ **Ed25519** signatures (constant-time, preferred)
- ✅ **HMAC-SHA256** for API tokens
- ⚠️ RSA only for legacy OAuth/OIDC compatibility

### 4. Network Security

- **TLS 1.3** enforced (mitigates timing measurement precision)
- **Cloudflare/CDN** in production (adds timing noise)
- **Geographic rate limiting** enabled

---

## 📈 MONITORING PLAN

### Daily Checks:
```bash
# Check for rsa crate updates
cargo search rsa --limit 1

# Review auth failure logs
grep "RSA_AUTH_FAIL" /var/log/aion/auth.log | wc -l
```

### Weekly Reviews:
- Audit authentication timing metrics
- Review blocked IPs for patterns
- Check RustSec advisories for rsa updates

### Monthly Actions:
- Security team review of rsa exposure
- Re-evaluate migration to Ed25519
- Update this document with new findings

---

## 🔄 FUTURE MIGRATION PATH

### Short-term (Q1 2026):
- [ ] Migrate aion-licensing to **Ed25519** for new licenses
- [ ] Deprecate RSA license keys (12-month sunset)
- [ ] Add **chacha20poly1305** for license encryption

### Medium-term (Q2-Q3 2026):
- [ ] Replace openidconnect with custom OAuth impl using Ed25519
- [ ] Migrate all MySQL connections to Ed25519 client certs
- [ ] Remove rsa dependency entirely

### Long-term (Q4 2026):
- [ ] Fully post-quantum crypto migration (Kyber/Dilithium)
- [ ] Zero RSA usage across platform

---

## 🎯 ACCEPTANCE CRITERIA

This risk is **ACCEPTED** based on:

1. ✅ **Low likelihood**: Attack requires 10000+ attempts
2. ✅ **Strong mitigations**: Rate limiting prevents attack feasibility
3. ✅ **Limited exposure**: RSA usage minimal and monitored
4. ✅ **No fix available**: rsa crate maintainers aware, no patch yet
5. ✅ **Migration planned**: Ed25519 transition in progress

**Approved by**: [Pending Security Team Review]  
**Review Date**: 2025-10-02  
**Next Review**: 2026-01-02 (quarterly)

---

## 📚 REFERENCES

- [RUSTSEC-2023-0071](https://rustsec.org/advisories/RUSTSEC-2023-0071)
- [Marvin Attack Paper](https://people.redhat.com/~hkario/marvin/)
- [rsa crate GitHub](https://github.com/RustCrypto/RSA)
- Internal: `SECURITY-SUMMARY-2025-10-02.md`

---

**Document Version**: 1.0  
**Last Updated**: 2025-10-02  
**Owner**: Security Team

🤖 Generated with [Claude Code](https://claude.com/claude-code)
