# Caddy 反向代理

Caddy 是一个现代的、易于使用的反向代理，具有自动 HTTPS 功能。

## 为什么选择 Caddy？

- **自动 HTTPS**: 开箱即用的 Let's Encrypt 集成
- **简单配置**: 人类可读的 Caddyfile
- **HTTP/2 & HTTP/3**: 现代协议支持
- **动态重载**: 无需重启即可更改配置

## 基本配置

创建 `/etc/caddy/Caddyfile`：

```caddyfile
{
    auto_https off  # 如果在另一个代理后面则禁用
    admin off       # 禁用管理 API（可选）
}

api.yourstore.com {
    # 反向代理到 R Commerce
    reverse_proxy localhost:8080
    
    # 文件上传大小
    request_body {
        max_size 50MB
    }
    
    # 安全头
    header {
        X-Frame-Options "SAMEORIGIN"
        X-Content-Type-Options "nosniff"
        X-XSS-Protection "1; mode=block"
        Referrer-Policy "strict-origin-when-cross-origin"
        -Server  # 移除服务器头
    }
    
    # 日志
    log {
        output file /var/log/caddy/access.log
        format json
    }
    
    # 压缩响应
    encode gzip zstd
}
```

## 自动 HTTPS

Caddy 自动获取和续期证书：

```caddyfile
api.yourstore.com {
    reverse_proxy localhost:8080
    
    # TLS 是自动的，但您可以自定义：
    tls {
        protocols tls1.2 tls1.3
        ciphers TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256 TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
    }
}
```

## 负载均衡

多个 R Commerce 实例：

```caddyfile
api.yourstore.com {
    reverse_proxy {
        to 10.0.1.10:8080 10.0.1.11:8080 10.0.1.12:8080
        
        # 负载均衡策略
        lb_policy least_conn
        
        # 健康检查
        health_uri /health
        health_interval 10s
        health_timeout 5s
    }
}
```

### 负载均衡策略

| 策略 | 说明 |
|--------|-------------|
| `random` | 随机选择 |
| `random_choose 2` | 随机选择 2 个 |
| `least_conn` | 最少活跃连接 |
| `round_robin` | 均匀分布 |
| `first` | 第一个可用 |
| `ip_hash` | 基于客户端 IP |
| `uri_hash` | 基于请求 URI |

## 速率限制

```caddyfile
{
    order rate_limit before basicauth
}

api.yourstore.com {
    # 速率限制：每个 IP 每秒 10 个请求
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

或使用 `http.rate_limit` 模块：

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

## 缓存

```caddyfile
{
    order cache before rewrite
}

api.yourstore.com {
    # 缓存 API 响应
    cache {
        ttl 5m
        stale 1h
    }
    
    reverse_proxy localhost:8080
}
```

## WebSocket 支持

Caddy 自动处理 WebSocket：

```caddyfile
api.yourstore.com {
    # WebSocket 连接自动升级
    reverse_proxy localhost:8080
    
    # 增加长连接的超时
    timeouts {
        read_body 0
        read_header 30s
        write 0
        idle 5m
    }
}
```

## 请求/响应操作

```caddyfile
api.yourstore.com {
    # 添加自定义头到上游
    reverse_proxy localhost:8080 {
        header_up X-Real-IP {remote_host}
        header_up X-Forwarded-Proto {scheme}
    }
    
    # 修改响应头
    header_down Server "R Commerce"
    
    # 移除敏感头
    header_down -X-Powered-By
}
```

## 多站点

```caddyfile
# API 服务器
api.yourstore.com {
    reverse_proxy localhost:8080
}

# 管理面板
admin.yourstore.com {
    reverse_proxy localhost:8081
    
    # IP 限制
    @not_allowed {
        not remote_ip 10.0.0.0/8 172.16.0.0/12
    }
    respond @not_allowed "Forbidden" 403
}

# 静态文件
static.yourstore.com {
    root /var/www/static
    file_server
    encode gzip
}
```

## 日志

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

## 管理命令

```bash
# 验证配置
caddy validate --config /etc/caddy/Caddyfile

# 重载配置
caddy reload --config /etc/caddy/Caddyfile

# 启动 Caddy
caddy run --config /etc/caddy/Caddyfile

# 作为服务运行
systemctl start caddy
systemctl enable caddy
```

## 故障排除

| 问题 | 解决方案 |
|-------|----------|
| 证书错误 | 检查 DNS 和防火墙以进行 ACME 挑战 |
| 502 错误 | 验证 R Commerce 是否正在运行 |
| 配置无法加载 | 运行 `caddy validate` 检查语法 |
| 高内存 | 调整 GOGC 环境变量 |

## 从 Nginx 迁移

| Nginx | Caddy |
|-------|-------|
| `proxy_pass` | `reverse_proxy` |
| `ssl_certificate` | 自动（或 `tls`） |
| `gzip on` | `encode gzip` |
| `client_max_body_size` | `request_body max_size` |
| `add_header` | `header` |
| `access_log` | `log` |
