# TLS 配置

传输层安全（TLS）加密客户端与您的 R Commerce 服务器之间传输的数据。本指南涵盖 TLS 配置选项，包括手动证书和自动 Let's Encrypt 设置。

## 概述

R Commerce 支持多种 TLS 配置方法：

- **手动证书** - 使用来自您的 CA 的现有证书
- **Let's Encrypt** - 自动证书颁发和续期
- **反向代理 TLS** - 在负载均衡器处终止 TLS

## 手动证书配置

### 使用您自己的证书

如果您有来自商业 CA 或内部 PKI 的证书：

```toml
[server]
host = "0.0.0.0"
port = 443
tls_enabled = true
tls_cert_path = "/etc/rcommerce/certs/server.crt"
tls_key_path = "/etc/rcommerce/certs/server.key"
```

### 证书要求

| 要求 | 规范 |
|-------------|---------------|
| 格式 | PEM 编码 |
| 密钥类型 | RSA 2048 位或更高，或 ECDSA P-256 |
| 证书链 | 包含中间证书 |
| 私钥 | 未加密或提供密码 |

### 证书链文件

按正确顺序组合证书：

```bash
# 创建完整链文件
cat server.crt intermediate.crt root.crt > fullchain.crt

# 验证链
openssl verify -CAfile root.crt -untrusted intermediate.crt server.crt
```

### 加密的私钥

对于加密的私钥，提供密码：

```toml
[server.tls]
cert_path = "/etc/rcommerce/certs/server.crt"
key_path = "/etc/rcommerce/certs/server.key"
key_password = "${TLS_KEY_PASSWORD}"  # 来自环境变量
```

!!! warning "安全"
    切勿将密码提交到版本控制。使用环境变量。

## Let's Encrypt 自动设置

### 概述

Let's Encrypt 提供免费、自动续期的证书。R Commerce 支持 ACME v2 协议进行证书管理。

### HTTP-01 挑战

HTTP 挑战是最简单的方法：

```toml
[server]
host = "0.0.0.0"
port = 443
tls_enabled = true

[server.tls.lets_encrypt]
enabled = true
email = "admin@yourdomain.com"
accept_tos = true  # 接受 Let's Encrypt 服务条款
challenge_type = "http-01"

# 可选：用于测试的暂存服务器
# directory_url = "https://acme-staging-v02.api.letsencrypt.org/directory"
```

要求：

- 端口 80 必须可访问以进行 HTTP 挑战
- 域名必须解析到您的服务器
- 服务器必须可从互联网访问

### DNS-01 挑战

对通配符证书或内部服务器使用 DNS 挑战：

```toml
[server.tls.lets_encrypt]
enabled = true
email = "admin@yourdomain.com"
accept_tos = true
challenge_type = "dns-01"
dns_provider = "route53"  # 或 cloudflare、digitalocean 等

# 提供商特定的配置
[server.tls.lets_encrypt.dns_challenge.route53]
region = "us-east-1"
# 来自 IAM 角色或环境变量的凭证
```

支持的 DNS 提供商：

| 提供商 | 配置 |
|----------|---------------|
| Route53 | IAM 角色或 AWS 凭证 |
| Cloudflare | API 令牌 |
| DigitalOcean | 个人访问令牌 |
| Google Cloud DNS | 服务账户 JSON |

### 通配符证书

DNS-01 挑战需要通配符：

```toml
[server.tls.lets_encrypt]
enabled = true
email = "admin@yourdomain.com"
accept_tos = true
challenge_type = "dns-01"
dns_provider = "cloudflare"
domains = ["*.yourdomain.com", "yourdomain.com"]
```

### 证书存储

Let's Encrypt 证书存储在：

```
/var/lib/rcommerce/certificates/  # Linux
/opt/rcommerce/certificates/      # FreeBSD
~/Library/Application Support/rcommerce/certificates/  # macOS
```

配置自定义路径：

```toml
[server.tls.lets_encrypt]
cert_dir = "/custom/path/to/certificates"
```

### 自动续期

证书自动续期：

- **检查间隔**：每 12 小时
- **续期阈值**：到期前 30 天
- **失败重试**：每小时

监控续期状态：

```bash
# 检查证书到期时间
openssl x509 -in /var/lib/rcommerce/certificates/cert.pem -noout -dates

# 查看续期日志
journalctl -u rcommerce -f | grep -i "certificate\|letsencrypt"
```

## HSTS 配置

HTTP 严格传输安全（HSTS）指示浏览器始终使用 HTTPS：

```toml
[server.tls.hsts]
enabled = true
max_age = 31536000  # 1 年的秒数
include_subdomains = true
preload = false  # 仅当提交到预加载列表时设置为 true
```

### HSTS 头

启用时，响应包含：

```http
Strict-Transport-Security: max-age=31536000; includeSubDomains
```

### 预加载

要提交到浏览器预加载列表：

1. 确保 HSTS 已启用至少 18 周
2. 设置 `preload = true`
3. 在 [hstspreload.org](https://hstspreload.org/) 提交

!!! warning "不可逆"
    预加载很难撤销。确保您的整个域名支持 HTTPS。

## TLS 配置选项

### 协议版本

配置最低 TLS 版本：

```toml
[server.tls]
min_version = "1.2"  # 选项："1.0"、"1.1"、"1.2"、"1.3"
max_version = "1.3"
```

推荐设置：

| 环境 | 最低版本 | 最高版本 |
|-------------|-------------|-------------|
| 生产环境 | 1.2 | 1.3 |
| 遗留支持 | 1.0 | 1.3 |

### 密码套件

控制允许的密码套件：

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

现代推荐套件：

| 套件 | TLS 版本 | 安全性 |
|-------|-------------|----------|
| TLS_AES_256_GCM_SHA384 | 1.3 | 优秀 |
| TLS_CHACHA20_POLY1305_SHA256 | 1.3 | 优秀 |
| TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384 | 1.2 | 强 |
| TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305 | 1.2 | 强 |

### 证书验证

对于双向 TLS（客户端证书）：

```toml
[server.tls]
client_auth = "optional"  # 选项："none"、"optional"、"require"
client_ca_path = "/etc/rcommerce/certs/ca.crt"
```

模式：

| 模式 | 说明 |
|------|-------------|
| `none` | 无客户端证书验证 |
| `optional` | 如果提供则验证，允许无证书 |
| `require` | 需要客户端证书 |

## 反向代理 TLS

### Nginx

在 Nginx 终止 TLS，使用 HTTP 到后端：

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

R Commerce 配置：

```toml
[server]
host = "127.0.0.1"  # 仅绑定到 localhost
port = 8080
tls_enabled = false  # TLS 由 Nginx 处理

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

## 证书续期

### 自动续期

Let's Encrypt 证书自动续期。使用以下方式监控：

```bash
# 检查证书状态
rcommerce tls status

# 强制续期（用于测试）
rcommerce tls renew --force

# 查看证书详情
rcommerce tls info
```

### 手动续期

对于手动证书，设置续期脚本：

```bash
#!/bin/bash
# /etc/cron.weekly/renew-certs

# 续期证书（certbot 示例）
certbot renew --quiet

# 重新加载 R Commerce 以获取新证书
systemctl reload rcommerce

# 或发送 SIGHUP 到进程
kill -HUP $(cat /var/run/rcommerce.pid)
```

### 无停机重新加载

R Commerce 支持证书热重载：

```bash
# 发送 SIGHUP 重新加载证书
kill -HUP $(cat /var/run/rcommerce.pid)

# 或使用 systemd
systemctl reload rcommerce
```

## 故障排除

### 证书未找到

```
ERROR: Certificate file not found: /etc/rcommerce/certs/server.crt
```

**解决方案：**

1. 验证配置中的文件路径
2. 检查文件权限（rcommerce 用户可读）
3. 确保证书存在：
   ```bash
   ls -la /etc/rcommerce/certs/
   ```

### Let's Encrypt 失败

```
ERROR: ACME challenge failed: 403 Forbidden
```

**常见原因：**

1. **端口 80 被阻止** - 确保防火墙允许 HTTP
2. **DNS 未解析** - 验证域名指向服务器
3. **速率限制** - Let's Encrypt 限制：每个域名每周 50 个证书
4. **IPv6 问题** - 确保 AAAA 记录正确

**调试：**

```bash
# 测试 HTTP 挑战端点
curl http://yourdomain.com/.well-known/acme-challenge/test

# 检查 DNS 解析
nslookup yourdomain.com

# 查看详细日志
RUST_LOG=debug rcommerce server
```

### 证书过期

```
WARNING: Certificate expires in 5 days
```

**解决方案：**

1. 检查自动续期是否启用
2. 验证续期服务是否正在运行：
   ```bash
   systemctl status rcommerce
   ```
3. 强制续期：
   ```bash
   rcommerce tls renew --force
   ```

### 弱密码警告

```
WARNING: Client connected using weak cipher: TLS_RSA_WITH_3DES_EDE_CBC_SHA
```

**解决方案：**

1. 将最低 TLS 版本更新到 1.2
2. 从配置中删除弱密码套件
3. 使用 SSL Labs 测试：
   ```bash
   # 配置更改后
   curl -s https://www.ssllabs.com/ssltest/analyze.html?d=yourdomain.com
   ```

### 混合内容警告

如果使用 HTTPS 但收到混合内容警告：

1. 确保所有资源使用 HTTPS
2. 检查 `X-Forwarded-Proto` 头是否由反向代理设置
3. 验证 `trust_proxy` 配置正确

## 安全最佳实践

1. **使用最低 TLS 1.2** - 禁用旧版本
2. **启用 HSTS** - 防止降级攻击
3. **使用强密码** - 遵循 Mozilla SSL 配置生成器
4. **监控证书** - 设置到期警报
5. **自动化续期** - 不要依赖手动流程
6. **定期测试** - 使用 SSL Labs 或类似工具

## 相关文档

- [安全指南](./security/security.md)
- [反向代理设置](./reverse-proxies/nginx.md)
- [生产部署](./binary.md)
