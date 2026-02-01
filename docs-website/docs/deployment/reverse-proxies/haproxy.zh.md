# HAProxy 反向代理

HAProxy 是一个高性能的 TCP/HTTP 负载均衡器和反向代理，非常适合高流量的 R Commerce 部署。

## 为什么选择 HAProxy？

- **极快** - 基于 C，事件驱动架构
- **高级负载均衡** - 多种算法（轮询、最少连接等）
- **健康检查** - 复杂的后端监控
- **SSL/TLS 终止** - 高效的 HTTPS 处理
- **统计信息** - 内置监控仪表板

## 安装

### Linux (Debian/Ubuntu)

```bash
apt-get update
apt-get install haproxy
```

### Linux (CentOS/RHEL)

```bash
yum install haproxy
```

### FreeBSD

```bash
pkg install haproxy
sysrc haproxy_enable=YES
```

## 基本配置

创建 `/etc/haproxy/haproxy.cfg`：

```haproxy
global
    log /dev/log local0
    log /dev/log local1 notice
    chroot /var/lib/haproxy
    stats socket /run/haproxy/admin.sock mode 660 level admin
    stats timeout 30s
    user haproxy
    group haproxy
    daemon

    # SSL/TLS 设置
    ssl-default-bind-ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256
    ssl-default-bind-options ssl-min-ver TLSv1.2 no-tls-tickets

defaults
    log global
    mode http
    option httplog
    option dontlognull
    timeout connect 5000
    timeout client 50000
    timeout server 50000

# 统计页面
frontend stats
    bind *:8404
    stats enable
    stats uri /stats
    stats refresh 10s

# HTTP 到 HTTPS 重定向
frontend http_front
    bind *:80
    redirect scheme https code 301 if !{ ssl_fc }

# HTTPS 前端
frontend https_front
    bind *:443 ssl crt /etc/ssl/certs/rcommerce.pem
    
    # 安全头
    http-response set-header X-Frame-Options SAMEORIGIN
    http-response set-header X-Content-Type-Options nosniff
    
    default_backend rcommerce

# R Commerce 后端
backend rcommerce
    balance roundrobin
    option httpchk GET /health
    http-check expect status 200
    
    server rcommerce1 127.0.0.1:8080 check inter 5s fall 3 rise 2
```

## SSL/TLS 配置

### 使用 Let's Encrypt

```bash
# 获取证书
certbot certonly --standalone -d yourdomain.com

# 为 HAProxy 合并
cat /etc/letsencrypt/live/yourdomain.com/fullchain.pem \
    /etc/letsencrypt/live/yourdomain.com/privkey.pem \
    > /etc/ssl/certs/rcommerce.pem

chmod 600 /etc/ssl/certs/rcommerce.pem
```

## 负载均衡多个实例

```haproxy
backend rcommerce
    balance roundrobin
    option httpchk GET /health
    
    server rcommerce1 192.168.1.10:8080 check inter 5s fall 3 rise 2
    server rcommerce2 192.168.1.11:8080 check inter 5s fall 3 rise 2
    server rcommerce3 192.168.1.12:8080 check inter 5s fall 3 rise 2 backup
```

## 管理

```bash
# 测试配置
haproxy -c -f /etc/haproxy/haproxy.cfg

# 重载
systemctl reload haproxy

# 查看统计
curl http://localhost:8404/stats
```

## 另请参阅

- [Caddy 反向代理](caddy.md) - 现代，自动 HTTPS
- [Nginx 反向代理](nginx.md) - 流行，功能丰富
- [Traefik 反向代理](traefik.md) - 云原生，动态
