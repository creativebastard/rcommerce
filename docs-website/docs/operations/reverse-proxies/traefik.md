# Traefik Reverse Proxy

Traefik is a modern, cloud-native reverse proxy and load balancer that automatically discovers services and handles dynamic configuration.

## Why Traefik?

- **Automatic service discovery** - Integrates with Docker, Kubernetes, Consul
- **Dynamic configuration** - No restarts needed for config changes
- **Automatic HTTPS** - Let's Encrypt integration out of the box
- **Modern dashboard** - Real-time monitoring web UI
- **Cloud-native** - Built for containers and microservices

## Installation

### Docker (Recommended)

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

## Static Configuration

Create `/etc/traefik/traefik.yaml`:

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

## Dynamic Configuration

Create `/etc/traefik/dynamic/rcommerce.yaml`:

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

## Docker Labels

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

## Management

```bash
# Test configuration
traefik --configfile=/etc/traefik/traefik.yaml

# Access dashboard
curl http://localhost:8080/dashboard/
```

## See Also

- [Caddy Reverse Proxy](caddy.md)
- [Nginx Reverse Proxy](nginx.md)
- [HAProxy Reverse Proxy](haproxy.md)
