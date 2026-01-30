# Nginx 反向代理

Nginx 是 R Commerce 的高性能反向代理和负载均衡器。

## 基本配置

```nginx
upstream rcommerce {
    server 127.0.0.1:8080;
    keepalive 32;
}

server {
    listen 80;
    server_name api.yourstore.com;
    
    # 重定向到 HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name api.yourstore.com;
    
    # SSL 证书
    ssl_certificate /etc/letsencrypt/live/api.yourstore.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.yourstore.com/privkey.pem;
    
    # SSL 配置
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256;
    ssl_prefer_server_ciphers off;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 1d;
    
    # 安全头
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    
    # 文件上传大小
    client_max_body_size 50M;
    
    # 代理到 R Commerce
    location / {
        proxy_pass http://rcommerce;
        proxy_http_version 1.1;
        
        # 连接处理
        proxy_set_header Connection "";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header X-Forwarded-Port $server_port;
        
        # 超时
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 300s;
        
        # 缓冲
        proxy_buffering off;
        proxy_request_buffering off;
        
        # WebSocket 支持
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
    
    # 健康检查端点（绕过一些头）
    location /health {
        proxy_pass http://rcommerce;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        access_log off;
    }
}
```

## 负载均衡

多个 R Commerce 实例：

```nginx
upstream rcommerce {
    least_conn;
    
    server 10.0.1.10:8080 weight=5;
    server 10.0.1.11:8080 weight=5;
    server 10.0.1.12:8080 backup;
    
    keepalive 32;
}
```

### 负载均衡方法

| 方法 | 使用场景 |
|--------|----------|
| `round_robin` | 均匀分布（默认） |
| `least_conn` | 长连接 |
| `ip_hash` | 会话持久性 |
| `hash $request_id` | 基于请求的路由 |

## 速率限制

```nginx
# 定义速率限制区域
limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
limit_req_zone $binary_remote_addr zone=auth:10m rate=1r/s;
limit_conn_zone $binary_remote_addr zone=addr:10m;

server {
    # 通用 API 速率限制
    location /api/ {
        limit_req zone=api burst=20 nodelay;
        limit_conn addr 10;
        
        proxy_pass http://rcommerce;
        # ...
    }
    
    # 更严格的认证速率限制
    location /api/v1/auth/ {
        limit_req zone=auth burst=5 nodelay;
        
        proxy_pass http://rcommerce;
        # ...
    }
}
```

## 缓存

```nginx
# 缓存配置
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

## SSL/TLS 配置

### Let's Encrypt

```bash
# 安装 certbot
sudo apt install certbot python3-certbot-nginx

# 获取证书
sudo certbot --nginx -d api.yourstore.com

# 自动续期
sudo certbot renew --dry-run
```

### 强 SSL 配置

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

## WebSocket 支持

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

## 日志

```nginx
# 自定义日志格式
log_format rcommerce '$remote_addr - $remote_user [$time_local] '
                     '"$request" $status $body_bytes_sent '
                     '"$http_referer" "$http_user_agent" '
                     '$request_time $upstream_response_time';

server {
    access_log /var/log/nginx/rcommerce-access.log rcommerce;
    error_log /var/log/nginx/rcommerce-error.log warn;
}
```

## 故障排除

| 问题 | 解决方案 |
|-------|----------|
| 502 Bad Gateway | 检查 R Commerce 是否在正确的端口上运行 |
| 504 Gateway Timeout | 增加 proxy_read_timeout |
| WebSocket 失败 | 确保设置了 Upgrade 头 |
| SSL 错误 | 检查证书路径和权限 |
| 高内存 | 调整 worker_processes 和 worker_connections |

## 性能调优

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
    
    # Gzip 压缩
    gzip on;
    gzip_vary on;
    gzip_proxied any;
    gzip_comp_level 6;
    gzip_types application/json;
}
```
