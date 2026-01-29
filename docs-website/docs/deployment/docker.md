# Docker Deployment

Deploy R Commerce using Docker and Docker Compose.

## Quick Start

```bash
docker-compose up -d
```

## Production Docker Compose

```yaml
version: '3.8'

services:
  rcommerce:
    image: rcommerce:latest
    ports:
      - "8080:8080"
    environment:
      - RCOMMERCE_CONFIG=/etc/rcommerce/config.toml
    volumes:
      - ./config.toml:/etc/rcommerce/config.toml
      - uploads:/app/uploads
    depends_on:
      - postgres
      - redis
    restart: unless-stopped
    
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: rcommerce
      POSTGRES_USER: rcommerce
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: unless-stopped
    
  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data
    restart: unless-stopped

volumes:
  postgres_data:
  redis_data:
  uploads:
```

See [Linux systemd](./linux/systemd.md) for non-Docker deployment.
