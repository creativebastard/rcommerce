# Traefik 反向代理

Traefik 是一个现代的、云原生的反向代理和负载均衡器，可自动发现服务并处理动态配置。

## 为什么选择 Traefik？

- **自动服务发现** - 与 Docker、Kubernetes、Consul 集成
- **动态配置** - 配置更改无需重启
- **自动 HTTPS** - 开箱即用的 Let's Encrypt 集成
- **现代仪表板** - 实时监控 Web UI
- **云原生** - 为容器和微服务构建

## 安装

### Docker（推荐）

```yaml
version: '3'

services:
  traefik:
    image: traefik:v3.0
    command:
      - "--api.insecure=true"
      - "--providers.docker=true"
      - "--providers.docker.exposedbydefault=false"
      - "--entrypoints.web.address=:80"
      - "--entrypoints.websecure.address=:443"
      - "--certificatesresolvers.letsencrypt.acme.tlschallenge=true"
      - "--certificatesresolvers.letsencrypt.acme.email=admin@example.com"
      - "--certificatesresolvers.letsencrypt.acme.storage=/letsencrypt/acme.json"
    ports:
      - "80:80"
      - "443:443"
      - "8080:8080"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - ./letsencrypt:/letsencrypt

  rcommerce:
    image: rcommerce:latest
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.rcommerce.rule=Host(`yourdomain.com`)"
      - "traefik.http.routers.rcommerce.entrypoints=websecure"
      - "traefik.http.routers.rcommerce.tls.certresolver=letsencrypt"
      - "traefik.http.services.rcommerce.loadbalancer.server.port=8080"
```

## 静态配置

创建 `/etc/traefik/traefik.yaml`：

```yaml
global:
  checkNewVersion: false
  sendAnonymousUsage: false

api:
  dashboard: true

entryPoints:
  web:
    address: ":80"
    http:
      redirections:
        entryPoint:
          to: websecure
          scheme: https
  websecure:
    address: ":443"

providers:
  docker:
    exposedByDefault: false
  file:
    directory: /etc/traefik/dynamic

certificatesResolvers:
  letsencrypt:
    acme:
      email: admin@yourdomain.com
      storage: /etc/traefik/acme.json
      tlsChallenge: {}
```

## 动态配置

创建 `/etc/traefik/dynamic/rcommerce.yaml`：

```yaml
http:
  routers:
    rcommerce:
      rule: "Host(`yourdomain.com`)"
      service: rcommerce
      entryPoints:
        - websecure
      tls:
        certResolver: letsencrypt

  services:
    rcommerce:
      loadBalancer:
        servers:
          - url: "http://127.0.0.1:8080"
        healthCheck:
          path: /health
          interval: 10s
```

## Docker 标签

```yaml
services:
  rcommerce:
    image: rcommerce:latest
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.rcommerce.rule=Host(`yourdomain.com`)"
      - "traefik.http.routers.rcommerce.entrypoints=websecure"
      - "traefik.http.routers.rcommerce.tls.certresolver=letsencrypt"
      - "traefik.http.services.rcommerce.loadbalancer.server.port=8080"
```

## 管理

```bash
# 测试配置
traefik --configfile=/etc/traefik/traefik.yaml

# 访问仪表板
curl http://localhost:8080/dashboard/
```

## 另请参阅

- [Caddy 反向代理](caddy.md)
- [Nginx 反向代理](nginx.md)
- [HAProxy 反向代理](haproxy.md)
