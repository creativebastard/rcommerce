# Nginx Reverse Proxy

Nginx is a high-performance reverse proxy and load balancer for R Commerce.

## Basic Configuration

```nginx
upstream rcommerce {
    server 127.0.0.1:8080;
    keepalive 32;
}

server {
    listen 80;
    server_name api.yourstore.com;
    
    # Redirect to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name api.yourstore.com;
    
    # SSL certificates
    ssl_certificate /etc/letsencrypt/live/api.yourstore.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.yourstore.com/privkey.pem;
    
    # SSL configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256;
    ssl_prefer_server_ciphers off;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 1d;
    
    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    
    # File upload size
    client_max_body_size 50M;
    
    # Proxy to R Commerce
    location / {
        proxy_pass http://rcommerce;
        proxy_http_version 1.1;
        
        # Connection handling
        proxy_set_header Connection "";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header X-Forwarded-Port $server_port;
        
        # Timeouts
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 300s;
        
        # Buffering
        proxy_buffering off;
        proxy_request_buffering off;
        
        # WebSocket support
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
    
    # Health check endpoint (bypass some headers)
    location /health {
        proxy_pass http://rcommerce;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        access_log off;
    }
}
```

## Load Balancing

Multiple R Commerce instances:

```nginx
upstream rcommerce {
    least_conn;
    
    server 10.0.1.10:8080 weight=5;
    server 10.0.1.11:8080 weight=5;
    server 10.0.1.12:8080 backup;
    
    keepalive 32;
}
```

### Load Balancing Methods

| Method | Use Case |
|--------|----------|
| `round_robin` | Even distribution (default) |
| `least_conn` | Long-running connections |
| `ip_hash` | Session persistence |
| `hash $request_id` | Request-based routing |

## Rate Limiting

```nginx
# Define rate limit zones
limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
limit_req_zone $binary_remote_addr zone=auth:10m rate=1r/s;
limit_conn_zone $binary_remote_addr zone=addr:10m;

server {
    # General API rate limiting
    location /api/ {
        limit_req zone=api burst=20 nodelay;
        limit_conn addr 10;
        
        proxy_pass http://rcommerce;
        # ...
    }
    
    # Stricter auth rate limiting
    location /api/v1/auth/ {
        limit_req zone=auth burst=5 nodelay;
        
        proxy_pass http://rcommerce;
        # ...
    }
}
```

## Caching

```nginx
# Cache configuration
proxy_cache_path /var/cache/nginx levels=1:2 keys_zone=rcommerce:10m 
                 max_size=1g inactive=60m use_temp_path=off;

server {
    location /api/v1/products {
        proxy_cache rcommerce;
        proxy_cache_valid 200 5m;
        proxy_cache_valid 404 1m;
        proxy_cache_use_stale error timeout updating;
        proxy_cache_background_update on;
        proxy_cache_lock on;
        
        proxy_pass http://rcommerce;
        # ...
    }
}
```

## SSL/TLS Configuration

### Let's Encrypt

```bash
# Install certbot
sudo apt install certbot python3-certbot-nginx

# Obtain certificate
sudo certbot --nginx -d api.yourstore.com

# Auto-renewal
sudo certbot renew --dry-run
```

### Strong SSL Config

```nginx
ssl_protocols TLSv1.2 TLSv1.3;
ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384;
ssl_prefer_server_ciphers off;
ssl_session_timeout 1d;
ssl_session_cache shared:SSL:50m;
ssl_session_tickets off;

# OCSP Stapling
ssl_stapling on;
ssl_stapling_verify on;
ssl_trusted_certificate /etc/letsencrypt/live/api.yourstore.com/chain.pem;
resolver 8.8.8.8 8.8.4.4 valid=300s;
resolver_timeout 5s;
```

## WebSocket Support

```nginx
location /ws {
    proxy_pass http://rcommerce;
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_read_timeout 86400s;
    proxy_send_timeout 86400s;
}
```

## Logging

```nginx
# Custom log format
log_format rcommerce '$remote_addr - $remote_user [$time_local] '
                     '"$request" $status $body_bytes_sent '
                     '"$http_referer" "$http_user_agent" '
                     '$request_time $upstream_response_time';

server {
    access_log /var/log/nginx/rcommerce-access.log rcommerce;
    error_log /var/log/nginx/rcommerce-error.log warn;
}
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| 502 Bad Gateway | Check R Commerce is running on correct port |
| 504 Gateway Timeout | Increase proxy_read_timeout |
| WebSocket fails | Ensure Upgrade headers are set |
| SSL errors | Check certificate paths and permissions |
| High memory | Adjust worker_processes and worker_connections |

## Performance Tuning

```nginx
# /etc/nginx/nginx.conf
worker_processes auto;
worker_rlimit_nofile 65535;

events {
    worker_connections 4096;
    use epoll;
    multi_accept on;
}

http {
    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;
    types_hash_max_size 2048;
    
    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_proxied any;
    gzip_comp_level 6;
    gzip_types application/json;
}
```
