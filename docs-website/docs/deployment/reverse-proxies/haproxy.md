# HAProxy Reverse Proxy

HAProxy is a high-performance TCP/HTTP load balancer and reverse proxy, ideal for high-traffic R Commerce deployments.

## Why HAProxy?

- **Extremely fast** - C-based, event-driven architecture
- **Advanced load balancing** - Multiple algorithms (round-robin, least-connections, etc.)
- **Health checks** - Sophisticated backend monitoring
- **SSL/TLS termination** - Efficient handling of HTTPS
- **Statistics** - Built-in monitoring dashboard

## Installation

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

## Basic Configuration

Create `/etc/haproxy/haproxy.cfg`:

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

    # SSL/TLS settings
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

# Stats page
frontend stats
    bind *:8404
    stats enable
    stats uri /stats
    stats refresh 10s

# HTTP to HTTPS redirect
frontend http_front
    bind *:80
    redirect scheme https code 301 if !{ ssl_fc }

# HTTPS frontend
frontend https_front
    bind *:443 ssl crt /etc/ssl/certs/rcommerce.pem
    
    # Security headers
    http-response set-header X-Frame-Options SAMEORIGIN
    http-response set-header X-Content-Type-Options nosniff
    
    default_backend rcommerce

# R Commerce backend
backend rcommerce
    balance roundrobin
    option httpchk GET /health
    http-check expect status 200
    
    server rcommerce1 127.0.0.1:8080 check inter 5s fall 3 rise 2
```

## SSL/TLS Configuration

### Using Let's Encrypt

```bash
# Obtain certificate
certbot certonly --standalone -d yourdomain.com

# Combine for HAProxy
cat /etc/letsencrypt/live/yourdomain.com/fullchain.pem \
    /etc/letsencrypt/live/yourdomain.com/privkey.pem \
    > /etc/ssl/certs/rcommerce.pem

chmod 600 /etc/ssl/certs/rcommerce.pem
```

## Load Balancing Multiple Instances

```haproxy
backend rcommerce
    balance roundrobin
    option httpchk GET /health
    
    server rcommerce1 192.168.1.10:8080 check inter 5s fall 3 rise 2
    server rcommerce2 192.168.1.11:8080 check inter 5s fall 3 rise 2
    server rcommerce3 192.168.1.12:8080 check inter 5s fall 3 rise 2 backup
```

## Management

```bash
# Test configuration
haproxy -c -f /etc/haproxy/haproxy.cfg

# Reload
systemctl reload haproxy

# View stats
curl http://localhost:8404/stats
```

## See Also

- [Caddy Reverse Proxy](caddy.md) - Modern, automatic HTTPS
- [Nginx Reverse Proxy](nginx.md) - Popular, feature-rich
- [Traefik Reverse Proxy](traefik.md) - Cloud-native, dynamic
