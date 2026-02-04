# Security Configuration & SSL/TLS Setup

This guide covers security best practices for deploying R Commerce in production, including automatic SSL certificate provisioning with Let's Encrypt, TLS 1.3 configuration, and HSTS.

##  Overview

R Commerce implements comprehensive security features:

- **Automatic SSL Certificate Provisioning** via Let's Encrypt
- **TLS 1.3** minimum (TLS 1.2 disabled for security)
- **HSTS (HTTP Strict Transport Security)** with preloading option
- **Security Headers** (CSP, X-Frame-Options, etc.)
- **Rate Limiting** on authentication endpoints
- **Secure Cookie** configuration
- **CORS** with strict origin policies

##  Prerequisites

Before configuring SSL/TLS, ensure:

1. Domain name pointing to your server
2. Ports 80 and 443 open in firewall
3. Server publicly accessible on ports 80/443
4. Valid email address for Let's Encrypt

##  SSL/TLS Configuration

### Method 1: Automatic Let's Encrypt (Recommended)

Add to your `config.toml`:

```toml
[server]
host = "0.0.0.0"
port = 8080

[tls]
enabled = true
min_tls_version = "1.3"  # Force TLS 1.3, disable TLS 1.2
max_tls_version = "1.3"
ocsp_stapling = true

[tls.hsts]
enabled = true
max_age = 31536000  # 1 year in seconds
include_subdomains = true
preload = false     # Set to true only if you understand implications

[tls.lets_encrypt]
enabled = true
email = "admin@yourdomain.com"  # Required for Let's Encrypt
domains = ["api.yourstore.com"]  # Your domain(s)
acme_directory = "https://acme-v02.api.letsencrypt.org/directory"
use_staging = false               # Set to true for testing
renewal_threshold_days = 30
auto_renew = true
cache_dir = "/var/lib/rcommerce/certs"
```

**Security Notes:**
- `min_tls_version = "1.3"` disables TLS 1.1 and 1.2, improving security
- `preload = false` should only be set to `true` if you understand HSTS preloading
- Staging server (`use_staging = true`) for initial testing to avoid rate limits

### Method 2: Manual Certificates

If you have existing certificates:

```toml
[tls]
enabled = true
min_tls_version = "1.3"
max_tls_version = "1.3"

# Manual certificate paths
cert_file = "/path/to/cert.pem"
key_file = "/path/to/private.key"

[tls.hsts]
enabled = true
max_age = 31536000
include_subdomains = true
preload = false
```

**Security Notes:**
- Ensure private key has `600` permissions: `chmod 600 /path/to/private.key`
- Use strong 2048-bit or 4096-bit RSA keys or ECDSA keys

##  Deployment

### 1. Configure Let's Encrypt

```bash
# Create certificate cache directory
sudo mkdir -p /var/lib/rcommerce/certs
sudo chown rcommerce:rcommerce /var/lib/rcommerce/certs
sudo chmod 750 /var/lib/rcommerce/certs
```

### 2. Firewall Configuration

```bash
# Allow HTTP (80) and HTTPS (443)
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw --force enable
```

### 3. Start Server with TLS

```bash
# Production with Let's Encrypt
export RCOMMERCE_CONFIG=/etc/rcommerce/production.toml
./target/release/rcommerce server

# The server automatically enables TLS when [tls] enabled = true in config
```

##  HSTS (HTTP Strict Transport Security)

HSTS tells browsers to always use HTTPS for your domain.

### Configuration

```toml
[tls.hsts]
enabled = true
max_age = 31536000        # 1 year (minimum recommended)
include_subdomains = true # Apply to all subdomains
preload = false           # Set to true only for long-term HTTPS commitment
```

**⚠️ WARNING: Preload Considerations**

Setting `preload = true` submits your domain to browser preload lists:

- **Benefit:** Browsers will ALWAYS use HTTPS, even first visit
- **Risk:** Difficult to remove if you need to disable HTTPS
- **Recommendation:** Only enable after 3+ months of stable HTTPS
- **Removal:** Can take months to propagate to all browsers

**To enable preload:**

1. Ensure `max-age` is at least 31536000 (1 year)
2. Set `include_subdomains = true`
3. Set `preload = true`
4. Submit at: https://hstspreload.org/
5. Wait for approval (can take weeks)

##  TLS Cipher Suites

By default, R Commerce uses only TLS 1.3 cipher suites:

```
TLS_AES_128_GCM_SHA256
TLS_AES_256_GCM_SHA384
TLS_CHACHA20_POLY1305_SHA256
```

**Why TLS 1.3 only?**
- Perfect forward secrecy by default
- Faster handshake (1-RTT)
- Removes obsolete cipher suites
- Removes vulnerable features (renegotiation, compression)

If you absolutely must support TLS 1.2 (not recommended), modify cipher suites in configuration.

## ️ Security Headers

R Commerce automatically adds these security headers:

| Header | Value | Purpose |
|--------|-------|---------|
| `Strict-Transport-Security` | `max-age=31536000; includeSubDomains` | Force HTTPS |
| `X-Frame-Options` | `DENY` | Prevent clickjacking |
| `X-Content-Type-Options` | `nosniff` | Prevent MIME sniffing |
| `X-XSS-Protection` | `1; mode=block` | XSS protection |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Privacy |
| `Permissions-Policy` | `geolocation=(), microphone=(), camera=()` | Restrict features |
| `Content-Security-Policy` | `default-src 'self'` | XSS/data injection |

##  Certificate Monitoring

Monitor your certificates using standard tools:

```bash
# Check certificate expiry using OpenSSL
echo | openssl s_client -servername api.yourstore.com -connect api.yourstore.com:443 2>/dev/null | openssl x509 -noout -dates

# Check certificate details
echo | openssl s_client -servername api.yourstore.com -connect api.yourstore.com:443 2>/dev/null | openssl x509 -noout -text

# Check certificate expiry from file
openssl x509 -in /var/lib/rcommerce/certs/cert.pem -noout -enddate
```

Sample output:
```
notBefore=Jan 15 10:30:00 2024 GMT
notAfter=Apr 14 10:30:00 2024 GMT
```

##  SSL Labs Testing

Test your SSL configuration:

1. Visit: https://www.ssllabs.com/ssltest/
2. Enter: `api.yourstore.com`
3. Wait for test to complete
4. Verify grade: **A+** expected

**Expected Results:**
- Certificate: Valid, trusted
- Protocols: TLS 1.3 only
- Cipher Suites: Only TLS 1.3 suites
- Grade: A+

##  Troubleshooting

### Certificate Issues

**Problem:** Certificate not being obtained

```bash
# Check logs
journalctl -u rcommerce -f

# Common issues:
# 1. Domain not pointing to server
# 2. Firewall blocking ports 80/443
# 3. Domain validation failed
```

**Solution:**
```bash
# Test HTTP challenge manually
curl http://api.yourstore.com/.well-known/acme-challenge/test

# Should return challenge token
# If not, check DNS and firewall
```

**Problem:** Let's Encrypt rate limit exceeded

```bash
# Error: "too many registrations for this IP"
# or "too many failed authorizations recently"
```

**Solution:**
1. Switch to staging server in config:
```toml
[tls.lets_encrypt]
use_staging = true
```
2. Wait 1 hour for rate limit to reset
3. Fix the underlying issue (DNS, firewall)
4. Test with staging first
5. Switch back to production

**Problem:** Certificate file permissions denied

```bash
# Check permissions
ls -la /var/lib/rcommerce/certs/

# Fix permissions
sudo chown -R rcommerce:rcommerce /var/lib/rcommerce/certs
sudo chmod 750 /var/lib/rcommerce/certs
sudo chmod 600 /var/lib/rcommerce/certs/*-key.pem
sudo chmod 644 /var/lib/rcommerce/certs/*-cert.pem
```

### HSTS Issues

**Problem:** Site inaccessible after enabling HSTS

```bash
# Clear HSTS cache in browser
# Chrome: chrome://net-internals/#hsts
# Delete domain security policy
```

**Solution:** Use staging environment first

```toml
[tls.lets_encrypt]
use_staging = true  # Switch to production only when ready
```

### TLS Connection Issues

**Problem:** Clients cannot connect with "protocol version" errors

**Solution:** Check TLS version configuration

```toml
[tls]
# Ensure both min and max are set to 1.3
min_tls_version = "1.3"
max_tls_version = "1.3"
```

**Problem:** "certificate signed by unknown authority"

**Solution:** 
1. For staging certificates: Install the staging CA in your test client
2. For production: Ensure system CA certificates are up to date:
```bash
# Ubuntu/Debian
sudo apt-get update && sudo apt-get install ca-certificates

# Test certificate chain
openssl s_client -connect api.yourstore.com:443 -servername api.yourstore.com
```

### Port Binding Issues

**Problem:** "Permission denied" when binding to port 80 or 443

**Solution:** 
1. Use capabilities (recommended):
```bash
sudo setcap cap_net_bind_service=+ep ./target/release/rcommerce
```
2. Or use a reverse proxy (nginx/traefik) on ports 80/443
3. Or run with sudo (not recommended for production)

### OCSP Stapling Issues

**Problem:** OCSP errors in logs

**Solution:**
```toml
[tls]
# Temporarily disable OCSP stapling for debugging
ocsp_stapling = false
```

Note: OCSP stapling requires the certificate to have a valid OCSP responder URL.

##  Security Best Practices

### 1. Minimum TLS Version

**Always use TLS 1.3 minimum:**

```toml
[tls]
min_tls_version = "1.3"  # Never use 1.2
max_tls_version = "1.3"
```

**Why?**
- TLS 1.2 has known vulnerabilities (POODLE, BEAST, etc.)
- TLS 1.3 has perfect forward secrecy by default
- Faster handshake (1-RTT vs 2-RTT)
- Removes insecure cipher suites

### 2. Certificate Permissions

**Set proper file permissions:**

```bash
# Private key should be 600 (owner read/write only)
chmod 600 /var/lib/rcommerce/certs/*-key.pem

# Certificates should be 644 (world readable)
chmod 644 /var/lib/rcommerce/certs/*-cert.pem

# Cache directory should be 750
chmod 750 /var/lib/rcommerce/certs
chown rcommerce:rcommerce /var/lib/rcommerce/certs
```

### 3. TLS Configuration Validation

**Always validate your TLS configuration:**

```bash
# Test SSL configuration
nmap --script ssl-enum-ciphers -p 443 api.yourstore.com

# Or use SSL Labs scan (external)
# https://www.ssllabs.com/ssltest/
```

### 3. Domain Validation

**Use separate domains:**
- `api.yourstore.com` for API
- `admin.yourstore.com` for admin (internal only)
- `cdn.yourstore.com` for assets

### 4. Monitoring

**Monitor certificate expiry:**

```bash
# Add to crontab (daily check with email alert)
0 0 * * * /usr/local/bin/check_cert_expiry.sh api.yourstore.com 30

# Example check_cert_expiry.sh script:
#!/bin/bash
DOMAIN=$1
DAYS=$2
EXPIRY=$(echo | openssl s_client -servername $DOMAIN -connect $DOMAIN:443 2>/dev/null | openssl x509 -noout -enddate | cut -d= -f2)
EXPIRY_EPOCH=$(date -d "$EXPIRY" +%s)
NOW_EPOCH=$(date +%s)
DAYS_UNTIL=$(( (EXPIRY_EPOCH - NOW_EPOCH) / 86400 ))

if [ $DAYS_UNTIL -lt $DAYS ]; then
    echo "Certificate for $DOMAIN expires in $DAYS_UNTIL days" | mail -s "SSL Certificate Expiry Warning" admin@yourstore.com
fi
```

**Set up alerts for:**
- Certificate expiry < 30 days
- Failed renewal attempts
- SSL handshake failures

### 5. Backup Certificates

**Keep offline backups:**

```bash
# Monthly backup
0 0 1 * * tar -czf /backup/rcommerce-certs-$(date +%Y%m%d).tar.gz /var/lib/rcommerce/certs
```

##  Production Checklist

Before going live:

- [x] Domain points to server
- [x] Ports 80/443 open
- [x] Let's Encrypt email configured
- [x] TLS 1.3 enforced
- [x] HSTS enabled
- [x] Certificate cache directory created
- [x] Proper file permissions set
- [x] Firewall configured
- [x] Monitoring alerts set up
- [x] Staging tested first
- [x] Backup strategy in place

##  Additional Resources

- [Let's Encrypt Documentation](https://letsencrypt.org/docs/)
- [Mozilla SSL Configuration Generator](https://ssl-config.mozilla.org/)
- [SSL Labs Best Practices](https://github.com/ssllabs/research/wiki/SSL-and-TLS-Deployment-Best-Practices)
- [HSTS Preload List](https://hstspreload.org/)

---

*Monitoring and observability documentation coming soon.*