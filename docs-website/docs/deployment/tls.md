# TLS Configuration

Transport Layer Security (TLS) encrypts data in transit between clients and your R Commerce server. This guide covers TLS configuration options including manual certificates and automatic Let's Encrypt setup.

## Overview

R Commerce supports multiple TLS configuration methods:

- **Manual certificates** - Use existing certificates from your CA
- **Let's Encrypt** - Automatic certificate issuance and renewal
- **Reverse proxy TLS** - Terminate TLS at your load balancer

## Manual Certificate Configuration

### Using Your Own Certificates

If you have certificates from a commercial CA or internal PKI:

```toml
[server]
host = "0.0.0.0"
port = 443
tls_enabled = true
tls_cert_path = "/etc/rcommerce/certs/server.crt"
tls_key_path = "/etc/rcommerce/certs/server.key"
```

### Certificate Requirements

| Requirement | Specification |
|-------------|---------------|
| Format | PEM encoded |
| Key type | RSA 2048-bit or higher, or ECDSA P-256 |
| Certificate chain | Include intermediate certificates |
| Private key | Unencrypted or provide password |

### Certificate Chain File

Combine certificates in the correct order:

```bash
# Create full chain file
cat server.crt intermediate.crt root.crt > fullchain.crt

# Verify chain
openssl verify -CAfile root.crt -untrusted intermediate.crt server.crt
```

### Encrypted Private Keys

For encrypted private keys, provide the password:

```toml
[server.tls]
cert_path = "/etc/rcommerce/certs/server.crt"
key_path = "/etc/rcommerce/certs/server.key"
key_password = "${TLS_KEY_PASSWORD}"  # From environment variable
```

!!! warning "Security"
    Never commit passwords to version control. Use environment variables.

## Let's Encrypt Automatic Setup

### Overview

Let's Encrypt provides free, automatically renewed certificates. R Commerce supports ACME v2 protocol for certificate management.

### HTTP-01 Challenge

The HTTP challenge is the simplest method:

```toml
[server]
host = "0.0.0.0"
port = 443
tls_enabled = true

[server.tls.lets_encrypt]
enabled = true
email = "admin@yourdomain.com"
accept_tos = true  # Accept Let's Encrypt Terms of Service
challenge_type = "http-01"

# Optional: staging server for testing
# directory_url = "https://acme-staging-v02.api.letsencrypt.org/directory"
```

Requirements:

- Port 80 must be accessible for HTTP challenge
- Domain must resolve to your server
- Server must be reachable from the internet

### DNS-01 Challenge

Use DNS challenge for wildcard certificates or internal servers:

```toml
[server.tls.lets_encrypt]
enabled = true
email = "admin@yourdomain.com"
accept_tos = true
challenge_type = "dns-01"
dns_provider = "route53"  # or cloudflare, digitalocean, etc.

# Provider-specific configuration
[server.tls.lets_encrypt.dns_challenge.route53]
region = "us-east-1"
# Credentials from IAM role or environment variables
```

Supported DNS providers:

| Provider | Configuration |
|----------|---------------|
| Route53 | IAM role or AWS credentials |
| Cloudflare | API token |
| DigitalOcean | Personal access token |
| Google Cloud DNS | Service account JSON |

### Wildcard Certificates

DNS-01 challenge is required for wildcards:

```toml
[server.tls.lets_encrypt]
enabled = true
email = "admin@yourdomain.com"
accept_tos = true
challenge_type = "dns-01"
dns_provider = "cloudflare"
domains = ["*.yourdomain.com", "yourdomain.com"]
```

### Certificate Storage

Let's Encrypt certificates are stored in:

```
/var/lib/rcommerce/certificates/  # Linux
/opt/rcommerce/certificates/      # FreeBSD
~/Library/Application Support/rcommerce/certificates/  # macOS
```

Configure custom path:

```toml
[server.tls.lets_encrypt]
cert_dir = "/custom/path/to/certificates"
```

### Automatic Renewal

Certificates are automatically renewed:

- **Check interval**: Every 12 hours
- **Renewal threshold**: 30 days before expiry
- **Retry on failure**: Every hour

Monitor renewal status:

```bash
# Check certificate expiry
openssl x509 -in /var/lib/rcommerce/certificates/cert.pem -noout -dates

# View renewal logs
journalctl -u rcommerce -f | grep -i "certificate\|letsencrypt"
```

## HSTS Configuration

HTTP Strict Transport Security (HSTS) instructs browsers to always use HTTPS:

```toml
[server.tls.hsts]
enabled = true
max_age = 31536000  # 1 year in seconds
include_subdomains = true
preload = false  # Set to true only if submitting to preload list
```

### HSTS Headers

When enabled, responses include:

```http
Strict-Transport-Security: max-age=31536000; includeSubDomains
```

### Preloading

To submit to browser preload lists:

1. Ensure HSTS is enabled for at least 18 weeks
2. Set `preload = true`
3. Submit at [hstspreload.org](https://hstspreload.org/)

!!! warning "Irreversible"
    Preloading is difficult to undo. Ensure your entire domain supports HTTPS.

## TLS Configuration Options

### Protocol Versions

Configure minimum TLS version:

```toml
[server.tls]
min_version = "1.2"  # Options: "1.0", "1.1", "1.2", "1.3"
max_version = "1.3"
```

Recommended settings:

| Environment | Min Version | Max Version |
|-------------|-------------|-------------|
| Production | 1.2 | 1.3 |
| Legacy support | 1.0 | 1.3 |

### Cipher Suites

Control allowed cipher suites:

```toml
[server.tls]
cipher_suites = [
    "TLS_AES_256_GCM_SHA384",      # TLS 1.3
    "TLS_CHACHA20_POLY1305_SHA256", # TLS 1.3
    "TLS_AES_128_GCM_SHA256",      # TLS 1.3
    "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",  # TLS 1.2
    "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305",   # TLS 1.2
]
```

Modern recommended suites:

| Suite | TLS Version | Security |
|-------|-------------|----------|
| TLS_AES_256_GCM_SHA384 | 1.3 | Excellent |
| TLS_CHACHA20_POLY1305_SHA256 | 1.3 | Excellent |
| TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384 | 1.2 | Strong |
| TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305 | 1.2 | Strong |

### Certificate Verification

For mutual TLS (client certificates):

```toml
[server.tls]
client_auth = "optional"  # Options: "none", "optional", "require"
client_ca_path = "/etc/rcommerce/certs/ca.crt"
```

Modes:

| Mode | Description |
|------|-------------|
| `none` | No client certificate verification |
| `optional` | Verify if provided, allow without |
| `require` | Client certificate required |

## Reverse Proxy TLS

### Nginx

Terminate TLS at Nginx, use HTTP to backend:

```nginx
server {
    listen 443 ssl http2;
    server_name api.yourdomain.com;

    ssl_certificate /etc/nginx/certs/fullchain.crt;
    ssl_certificate_key /etc/nginx/certs/private.key;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers 'ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256';
    ssl_prefer_server_ciphers off;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

R Commerce configuration:

```toml
[server]
host = "127.0.0.1"  # Bind to localhost only
port = 8080
tls_enabled = false  # TLS handled by Nginx

[server.trust_proxy]
enabled = true
proxy_header = "X-Forwarded-Proto"
trusted_proxies = ["127.0.0.1", "10.0.0.0/8"]
```

### Traefik

```yaml
# docker-compose.yml
services:
  rcommerce:
    image: rcommerce/rcommerce:latest
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.rcommerce.rule=Host(`api.yourdomain.com`)"
      - "traefik.http.routers.rcommerce.tls=true"
      - "traefik.http.routers.rcommerce.tls.certresolver=letsencrypt"
```

### Caddy

```
api.yourdomain.com {
    reverse_proxy localhost:8080
    tls admin@yourdomain.com
}
```

## Certificate Renewal

### Automatic Renewal

Let's Encrypt certificates renew automatically. Monitor with:

```bash
# Check certificate status
rcommerce tls status

# Force renewal (for testing)
rcommerce tls renew --force

# View certificate details
rcommerce tls info
```

### Manual Renewal

For manual certificates, set up a renewal script:

```bash
#!/bin/bash
# /etc/cron.weekly/renew-certs

# Renew certificates (certbot example)
certbot renew --quiet

# Reload R Commerce to pick up new certificates
systemctl reload rcommerce

# Or send SIGHUP to process
kill -HUP $(cat /var/run/rcommerce.pid)
```

### Reload Without Downtime

R Commerce supports certificate hot-reloading:

```bash
# Send SIGHUP to reload certificates
kill -HUP $(cat /var/run/rcommerce.pid)

# Or use systemd
systemctl reload rcommerce
```

## Troubleshooting

### Certificate Not Found

```
ERROR: Certificate file not found: /etc/rcommerce/certs/server.crt
```

**Solutions:**

1. Verify file paths in configuration
2. Check file permissions (readable by rcommerce user)
3. Ensure certificates exist:
   ```bash
   ls -la /etc/rcommerce/certs/
   ```

### Let's Encrypt Failures

```
ERROR: ACME challenge failed: 403 Forbidden
```

**Common causes:**

1. **Port 80 blocked** - Ensure firewall allows HTTP
2. **DNS not resolving** - Verify domain points to server
3. **Rate limiting** - Let's Encrypt limits: 50 certificates/week per domain
4. **IPv6 issues** - Ensure AAAA records are correct

**Debug:**

```bash
# Test HTTP challenge endpoint
curl http://yourdomain.com/.well-known/acme-challenge/test

# Check DNS resolution
nslookup yourdomain.com

# View detailed logs
RUST_LOG=debug rcommerce server
```

### Certificate Expired

```
WARNING: Certificate expires in 5 days
```

**Solutions:**

1. Check auto-renewal is enabled
2. Verify renewal service is running:
   ```bash
   systemctl status rcommerce
   ```
3. Force renewal:
   ```bash
   rcommerce tls renew --force
   ```

### Weak Cipher Warnings

```
WARNING: Client connected using weak cipher: TLS_RSA_WITH_3DES_EDE_CBC_SHA
```

**Solutions:**

1. Update minimum TLS version to 1.2
2. Remove weak cipher suites from configuration
3. Test with SSL Labs:
   ```bash
   # After configuration changes
   curl -s https://www.ssllabs.com/ssltest/analyze.html?d=yourdomain.com
   ```

### Mixed Content Warnings

If using HTTPS but getting mixed content warnings:

1. Ensure all resources use HTTPS
2. Check `X-Forwarded-Proto` header is set by reverse proxy
3. Verify `trust_proxy` is configured correctly

## Security Best Practices

1. **Use TLS 1.2 minimum** - Disable older versions
2. **Enable HSTS** - Prevent downgrade attacks
3. **Use strong ciphers** - Follow Mozilla SSL Configuration Generator
4. **Monitor certificates** - Set up expiry alerts
5. **Automate renewal** - Don't rely on manual processes
6. **Test regularly** - Use SSL Labs or similar tools

## Related Documentation

- [Security Guide](./security/security.md)
- [Reverse Proxy Setup](./reverse-proxies/nginx.md)
- [Production Deployment](./binary.md)
