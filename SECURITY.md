# Security Policy

## Supported Versions

| Version | Supported          | Security Updates |
| ------- | ------------------ | ---------------- |
| 1.x.x   | :white_check_mark: | Until 2026-12-31 |
| 0.x.x   | :x:                | No longer supported |

---

## Reporting a Vulnerability

**DO NOT** open public GitHub issues for security vulnerabilities.

### How to Report

**Email:** security@ectus-r.com (or pako.molina@gmail.com with [SECURITY] prefix)

**Include:**
- Description of the vulnerability
- Steps to reproduce
- Affected versions
- Potential impact
- Suggested fix (if any)

**Response Timeline:**
- Acknowledgment: Within 48 hours
- Initial assessment: Within 5 business days
- Fix timeline: Depends on severity (see below)

### Severity Levels

| Severity | Response Time | Example |
|----------|--------------|---------|
| **Critical** | 24-48 hours | RCE, SQL injection, auth bypass |
| **High** | 5-7 days | XSS, CSRF, sensitive data exposure |
| **Medium** | 14-21 days | DoS, information disclosure |
| **Low** | 30-60 days | Minor info leaks, edge cases |

---

## Security Features

### Authentication & Authorization

✅ **Implemented:**
- Argon2 password hashing (OWASP recommended)
- JWT tokens with expiration
- Session management via Redis
- Rate limiting (60 req/min per IP)
- API key authentication

```rust
// secrets_manager.rs:58-120
impl SecretsManager {
    pub async fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        // ChaCha20-Poly1305 encryption
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        self.cipher.encrypt(&nonce, plaintext)?
    }
}
```

### Input Validation

✅ **Implemented:**
- SQL parameterization (no raw queries)
- XSS prevention (sanitized outputs)
- CSRF tokens on state-changing operations
- Content-Type validation
- File upload restrictions (type, size)

```rust
// All DB queries use SQLx parameterized queries
sqlx::query!(
    "SELECT * FROM users WHERE email = $1",
    email  // Parameterized - prevents SQL injection
)
```

### Network Security

✅ **Implemented:**
- HTTPS/TLS enforced in production
- CORS configuration (restrict origins in prod)
- HTTP security headers:
  - `X-Frame-Options: DENY`
  - `X-Content-Type-Options: nosniff`
  - `Strict-Transport-Security: max-age=31536000`
  - `Content-Security-Policy: default-src 'self'`

### Secret Management

✅ **Implemented:**
- Environment variables (never hardcoded)
- Encrypted storage (ChaCha20-Poly1305)
- Cloudflare Workers secrets (encrypted at rest)
- Automatic secret rotation (planned for v1.1)

⚠️ **Your Responsibility:**
- Rotate API keys every 90 days
- Use strong JWT secrets (32+ bytes)
- Never commit `.env` to git
- Use separate secrets per environment

---

## Known Security Considerations

### 1. LLM-Generated Code

**Risk:** AI may generate code with vulnerabilities.

**Mitigation:**
- Automated security scanning in QA cycle
- All generated code reviewed by autocorrection
- Integration with `cargo audit`, `npm audit`
- Human code review recommended for production

**Status:** Partial mitigation. Users must validate.

### 2. API Keys in Environment

**Risk:** `.env` file contains sensitive keys.

**Mitigation:**
- `.env` excluded in `.gitignore`
- Secrets encrypted in Cloudflare Workers
- Docker secrets for container deployments
- File permissions: `chmod 600 .env`

**Best Practice:**
```bash
# Set restrictive permissions
chmod 600 .env
chmod 600 LICENSE-COMMERCIAL.md  # Contains contact info only
```

### 3. Docker Development Ports

**Risk:** Development `docker-compose.yml` exposes ports to localhost.

**Mitigation:**
- Only binds to `127.0.0.1` (not `0.0.0.0`)
- Production uses internal networks only
- No ports exposed in `docker-compose.production.yml`

**Warning in README:**
```markdown
⚠️ Development docker-compose.yml is for local dev only.
   DO NOT use in production. Use docker-compose.production.yml.
```

### 4. Dependency Vulnerabilities

**Risk:** Third-party crates/packages may have CVEs.

**Mitigation:**
- Automated scanning in CI/CD (see below)
- Dependabot enabled on GitHub
- Monthly security audits
- Pin versions in production

**Commands:**
```bash
# Rust dependencies
cargo audit

# Node.js dependencies
cd web-dashboard && npm audit

# Fix non-breaking issues
npm audit fix
```

---

## Security Scanning (Automated)

### CI/CD Integration

Added to `.github/workflows/ci-cd.yml`:

```yaml
security-audit:
  name: Security Audit
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4

    - name: Rust Security Audit
      uses: actions-rs/audit-check@v1
      with:
        token: ${{ secrets.GITHUB_TOKEN }}

    - name: Node.js Security Audit
      working-directory: web-dashboard
      run: |
        npm audit --audit-level=high
        npm audit --json > audit-report.json

    - name: Upload Security Report
      uses: actions/upload-artifact@v3
      with:
        name: security-audit
        path: web-dashboard/audit-report.json
```

### Manual Scanning

```bash
# Full security scan
./scripts/security-scan.sh

# Rust only
cargo audit

# Frontend only
cd web-dashboard && npm audit

# Check for secrets in git history
git secrets --scan-history

# SAST (Static Application Security Testing)
cargo clippy -- -D warnings
```

---

## Secure Development Practices

### Code Review Checklist

Before merging PRs, verify:

- [ ] No hardcoded secrets or API keys
- [ ] SQL queries use parameterization
- [ ] User inputs are validated/sanitized
- [ ] Error messages don't leak sensitive info
- [ ] Authentication required for protected endpoints
- [ ] Rate limiting on expensive operations
- [ ] CORS configuration appropriate for environment
- [ ] Dependencies updated and scanned
- [ ] Security headers present in responses
- [ ] TLS/HTTPS enforced (production)

### Testing Security

```rust
#[tokio::test]
async fn test_sql_injection_prevention() {
    let malicious_input = "'; DROP TABLE users; --";
    let result = get_user_by_email(malicious_input).await;

    // Should return error, not execute SQL
    assert!(result.is_err());
}

#[tokio::test]
async fn test_unauthorized_access() {
    let response = client.get("/api/admin").send().await?;
    assert_eq!(response.status(), 401);  // Unauthorized
}
```

### Secrets in Git History

**Prevention:**
```bash
# Install git-secrets
brew install git-secrets  # macOS
# or
apt-get install git-secrets  # Linux

# Setup
cd /path/to/Ectus-R
git secrets --install
git secrets --register-aws  # AWS keys
git secrets --add 'sk-[0-9a-zA-Z]{32,}'  # OpenAI keys
git secrets --add 'gsk_[0-9a-zA-Z]{40,}'  # Groq keys
```

**Remediation (if committed):**
```bash
# Remove from history
git filter-branch --force --index-filter \
  "git rm --cached --ignore-unmatch .env" \
  --prune-empty --tag-name-filter cat -- --all

# Force push (DANGEROUS)
git push --force --all
git push --force --tags

# Rotate compromised secrets immediately
```

---

## Compliance

### OWASP Top 10 (2021)

| Risk | Status | Notes |
|------|--------|-------|
| A01:2021 – Broken Access Control | ✅ Mitigated | JWT + Role-based auth |
| A02:2021 – Cryptographic Failures | ✅ Mitigated | ChaCha20-Poly1305, TLS |
| A03:2021 – Injection | ✅ Mitigated | Parameterized queries |
| A04:2021 – Insecure Design | ✅ Addressed | Threat modeling done |
| A05:2021 – Security Misconfiguration | ⚠️ Partial | User must configure |
| A06:2021 – Vulnerable Components | ✅ Automated | Dependabot + audits |
| A07:2021 – Auth & Session Failures | ✅ Mitigated | Argon2 + secure sessions |
| A08:2021 – Software & Data Integrity | ✅ Mitigated | Signed releases |
| A09:2021 – Logging Failures | ✅ Implemented | Structured logging |
| A10:2021 – SSRF | ✅ Mitigated | URL validation |

**Overall:** 100% addressed (8/10 fully mitigated, 2/10 require user config)

### GDPR Compliance

✅ **Features:**
- User data deletion API
- Data export functionality
- Consent management
- Data encryption at rest
- Right to be forgotten

⚠️ **User Responsibility:**
- Implement privacy policy
- Handle consent in your app
- Configure data retention
- Deploy in EU if serving EU users

---

## Incident Response Plan

### In Case of Breach

1. **Immediate (0-1 hour):**
   - Isolate affected systems
   - Disable compromised accounts/keys
   - Preserve logs and evidence
   - Notify security team

2. **Assessment (1-4 hours):**
   - Determine scope and impact
   - Identify root cause
   - Document timeline
   - Estimate affected users

3. **Containment (4-24 hours):**
   - Deploy patches/fixes
   - Rotate all secrets
   - Update access controls
   - Monitor for further attacks

4. **Communication (24-72 hours):**
   - Notify affected users (if applicable)
   - Public disclosure (if appropriate)
   - Report to authorities (if required by law)
   - Update security advisories

5. **Recovery (1-2 weeks):**
   - Restore services
   - Conduct post-mortem
   - Implement preventive measures
   - Update security documentation

### Contact

**Security Team:** security@ectus-r.com
**PGP Key:** [Available on request]
**Response Time:** 24-48 hours

---

## Bug Bounty Program

**Status:** Coming in Q2 2025

**Planned Rewards:**
- Critical: $500 - $5,000
- High: $250 - $1,000
- Medium: $100 - $500
- Low: $50 - $200

**In Scope:**
- Authentication bypass
- SQL injection
- XSS/CSRF
- RCE
- Sensitive data exposure

**Out of Scope:**
- Social engineering
- Physical attacks
- DoS (except application-level)
- Issues in dependencies (report to upstream)

---

## Security Roadmap

### Q1 2025 (Completed)
- ✅ OWASP Top 10 compliance
- ✅ Automated security scanning
- ✅ Encrypted secret storage
- ✅ Rate limiting

### Q2 2025 (Planned)
- ⏳ 2FA/MFA support
- ⏳ Security audit by third party
- ⏳ Bug bounty program launch
- ⏳ SOC 2 Type II certification prep

### Q3 2025 (Planned)
- ⏳ Automated secret rotation
- ⏳ Advanced threat detection
- ⏳ Pen-testing engagement
- ⏳ Security training for contributors

---

## Additional Resources

- [OWASP Cheat Sheets](https://cheatsheetseries.owasp.org/)
- [Rust Security](https://rustsec.org/)
- [Node.js Security Best Practices](https://nodejs.org/en/docs/guides/security/)
- [Cloudflare Security](https://developers.cloudflare.com/fundamentals/security/)

---

**Last Updated:** 2025-10-01
**Version:** 1.0
**Next Review:** 2025-04-01

For security questions: security@ectus-r.com
