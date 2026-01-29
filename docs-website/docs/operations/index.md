# Operations Guide

Run R Commerce in production with confidence. This section covers day-to-day operations, monitoring, scaling, and maintenance.

## Topics

### Infrastructure

- [Reverse Proxies](./reverse-proxies/caddy.md) - Caddy, Nginx, HAProxy, Traefik
- [Scaling](./scaling.md) - Horizontal and vertical scaling
- [Monitoring](./monitoring.md) - Metrics, logging, alerting
- [Redis](./redis.md) - Cache and session management

### Maintenance

- [Backups](./backups.md) - Database and file backups
- [Security](./security.md) - Hardening and best practices

## Quick Reference

### Health Checks

```bash
# API health
curl http://localhost:8080/health

# Database health
curl http://localhost:8080/health/db

# Full status
curl http://localhost:8080/health/detailed
```

### Common Commands

```bash
# Check service status
systemctl status rcommerce  # Linux
launchctl list | grep rcommerce  # macOS
service rcommerce status  # FreeBSD

# View logs
journalctl -u rcommerce -f  # Linux
tail -f /var/log/rcommerce.log  # FreeBSD

# Database operations
rcommerce db status
rcommerce db migrate
rcommerce db reset
```

### Performance Metrics

Monitor these key metrics:

| Metric | Target | Alert Threshold |
|--------|--------|-----------------|
| Response Time | < 10ms p50 | > 50ms |
| Error Rate | < 0.1% | > 1% |
| CPU Usage | < 70% | > 90% |
| Memory Usage | < 80% | > 95% |
| DB Connections | < 80% pool | > 95% pool |

## Support

For operational issues:

1. Check logs first
2. Review monitoring dashboards
3. Consult specific guide sections
4. Open issue on GitHub
