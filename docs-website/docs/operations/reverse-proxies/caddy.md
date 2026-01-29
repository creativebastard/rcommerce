# Caddy Reverse Proxy

Caddy is a modern, easy-to-use reverse proxy with automatic HTTPS.

## Why Caddy?

- **Automatic HTTPS**: Let's Encrypt integration out of the box
- **Simple Configuration**: Human-readable Caddyfile
- **HTTP/2 & HTTP/3**: Modern protocol support
- **Dynamic Reloads**: Config changes without restart

## Basic Configuration

Create `/etc/caddy/Caddyfile`:

```caddyfile
{
    auto_https off  # Disable if behind another proxy
    admin off       # Disable admin API (optional)
}

api.yourstore.com {
    # Reverse proxy to R Commerce
    reverse_proxy localhost:8080
    
    # File upload size
    request_body {
        max_size 50MB
    }
    
    # Security headers
    header {
        X-Frame-Options "SAMEORIGIN"
        X-Content-Type-Options "nosniff"
        X-XSS-Protection "1; mode=block"
        Referrer-Policy "strict-origin-when-cross-origin"
        -Server  # Remove server header
    }
    
    # Logging
    log {
        output file /var/log/caddy/access.log
        format json
    }
    
    # Compress responses
    encode gzip zstd
}
```

## Automatic HTTPS

Caddy automatically obtains and renews certificates:

```caddyfile
api.yourstore.com {
    reverse_proxy localhost:8080
    
    # TLS is automatic, but you can customize:
    tls {
        protocols tls1.2 tls1.3
        ciphers TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256 TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
    }
}
```

## Load Balancing

Multiple R Commerce instances:

```caddyfile
api.yourstore.com {
    reverse_proxy {
        to 10.0.1.10:8080 10.0.1.11:8080 10.0.1.12:8080
        
        # Load balancing policy
        lb_policy least_conn
        
        # Health checks
        health_uri /health
        health_interval 10s
        health_timeout 5s
    }
}
```

### Load Balancing Policies

| Policy | Description |
|--------|-------------|
| `random` | Random selection |
| `random_choose 2` | Random with 2 choices |
| `least_conn` | Fewest active connections |
| `round_robin` | Even distribution |
| `first` | First available |
| `ip_hash` | Based on client IP |
| `uri_hash` | Based on request URI |

## Rate Limiting

```caddyfile
{
    order rate_limit before basicauth
}

api.yourstore.com {
    # Rate limit: 10 requests per second per IP
    rate_limit {
        zone static_example {
            key static
            events 10
            window 1s
        }
    }
    
    reverse_proxy localhost:8080
}
```

Or use the `http.rate_limit` module:

```caddyfile
api.yourstore.com {
    rate_limit {
        zone ip_limit {
            key {remote_host}
            events 100
            window 1m
        }
    }
    
    reverse_proxy localhost:8080
}
```

## Caching

```caddyfile
{
    order cache before rewrite
}

api.yourstore.com {
    # Cache API responses
    cache {
        ttl 5m
        stale 1h
    }
    
    reverse_proxy localhost:8080
}
```

## WebSocket Support

Caddy handles WebSockets automatically:

```caddyfile
api.yourstore.com {
    # WebSocket connections are automatically upgraded
    reverse_proxy localhost:8080
    
    # Increase timeouts for long-lived connections
    timeouts {
        read_body 0
        read_header 30s
        write 0
        idle 5m
    }
}
```

## Request/Response Manipulation

```caddyfile
api.yourstore.com {
    # Add custom headers to upstream
    reverse_proxy localhost:8080 {
        header_up X-Real-IP {remote_host}
        header_up X-Forwarded-Proto {scheme}
    }
    
    # Modify response headers
    header_down Server "R Commerce"
    
    # Remove sensitive headers
    header_down -X-Powered-By
}
```

## Multiple Sites

```caddyfile
# API server
api.yourstore.com {
    reverse_proxy localhost:8080
}

# Admin panel
admin.yourstore.com {
    reverse_proxy localhost:8081
    
    # IP restriction
    @not_allowed {
        not remote_ip 10.0.0.0/8 172.16.0.0/12
    }
    respond @not_allowed "Forbidden" 403
}

# Static files
static.yourstore.com {
    root /var/www/static
    file_server
    encode gzip
}
```

## Logging

```caddyfile
api.yourstore.com {
    reverse_proxy localhost:8080
    
    log {
        output file /var/log/caddy/access.log {
            roll_size 100MB
            roll_keep 10
            roll_keep_days 30
        }
        format json {
            time_format iso8601
        }
    }
}
```

## Docker Compose

```yaml
version: '3.8'

services:
  caddy:
    image: caddy:2-alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./Caddyfile:/etc/caddy/Caddyfile
      - caddy_data:/data
      - caddy_config:/config
    depends_on:
      - rcommerce
      
  rcommerce:
    image: rcommerce:latest
    environment:
      - RCOMMERCE_CONFIG=/etc/rcommerce/config.toml
    volumes:
      - ./config.toml:/etc/rcommerce/config.toml
```

## Management Commands

```bash
# Validate configuration
caddy validate --config /etc/caddy/Caddyfile

# Reload configuration
caddy reload --config /etc/caddy/Caddyfile

# Start Caddy
caddy run --config /etc/caddy/Caddyfile

# Run as service
systemctl start caddy
systemctl enable caddy
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Certificate errors | Check DNS and firewall for ACME challenges |
| 502 errors | Verify R Commerce is running |
| Config won't load | Run `caddy validate` to check syntax |
| High memory | Adjust GOGC environment variable |

## Migration from Nginx

| Nginx | Caddy |
|-------|-------|
| `proxy_pass` | `reverse_proxy` |
| `ssl_certificate` | Automatic (or `tls`) |
| `gzip on` | `encode gzip` |
| `client_max_body_size` | `request_body max_size` |
| `add_header` | `header` |
| `access_log` | `log` |
