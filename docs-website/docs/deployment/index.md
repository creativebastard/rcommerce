# Deployment Guide

Deploy R Commerce to production with confidence using these comprehensive deployment guides.

## Deployment Options

| Method | Best For | Complexity |
|--------|----------|------------|
| [Docker](./docker.md) | Most deployments | Low |
| [Kubernetes](./kubernetes.md) | Large scale, multi-region | High |
| [Linux (systemd)](./linux/systemd.md) | Traditional Linux servers | Medium |
| [Linux (Manual)](./linux/manual.md) | Custom setups | Medium |
| [FreeBSD Jails](./freebsd/jails.md) | FreeBSD environments | Medium |
| [FreeBSD (rc.d)](./freebsd/rc.d.md) | Traditional FreeBSD | Medium |
| [macOS (launchd)](./macos/launchd.md) | Development/Mac servers | Low |
| [Binary](./binary.md) | Minimal dependencies | Low |

## Quick Start

For most production deployments, we recommend Docker:

```bash
# Clone the repository
git clone https://gitee.com/captainjez/gocart.git
cd gocart

# Copy and edit configuration
cp config/production.toml config/myconfig.toml
# Edit database, security settings

# Start with Docker Compose
docker-compose -f docker-compose.prod.yml up -d
```

## Pre-Deployment Checklist

- [ ] Database provisioned and accessible
- [ ] Redis configured (for caching/sessions)
- [ ] SSL/TLS certificates ready
- [ ] Domain DNS configured
- [ ] Environment variables set
- [ ] Security settings reviewed
- [ ] Backup strategy in place
- [ ] Monitoring configured

## System Requirements

### Minimum

- **CPU**: 1 core
- **RAM**: 512MB
- **Disk**: 1GB
- **Network**: 100Mbps

### Recommended

- **CPU**: 2+ cores
- **RAM**: 2GB+
- **Disk**: 10GB SSD
- **Network**: 1Gbps

### Database

- PostgreSQL 14+ (recommended)
- MySQL 8.0+
- SQLite 3.35+ (development only)

## Configuration

All deployment methods use the same configuration file format:

```toml
[server]
host = "0.0.0.0"
port = 8080

[database]
db_type = "Postgres"
host = "localhost"
port = 5432
database = "rcommerce"
username = "rcommerce"
password = "${DB_PASSWORD}"

[cache]
cache_type = "Redis"
redis_url = "redis://localhost:6379"

[security.jwt]
secret = "${JWT_SECRET}"
```

## Next Steps

Choose your deployment method:

1. **[Docker](./docker.md)** - Containerized deployment
2. **[Kubernetes](./kubernetes.md)** - Orchestrated deployment
3. **[Linux](./linux/systemd.md)** - Traditional Linux server
4. **[FreeBSD](./freebsd/jails.md)** - FreeBSD deployment
5. **[macOS](./macos/launchd.md)** - macOS server
