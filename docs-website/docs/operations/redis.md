# Redis Caching

R Commerce uses Redis for high-performance caching and session storage. This guide covers configuration, monitoring, and best practices.

## Overview

Redis is used for:

- **Product Catalog Caching** - Fast product lookups
- **Session Storage** - Shopping cart and user sessions
- **Rate Limiting** - API throttling
- **Token Blacklist** - JWT revocation
- **WebSocket Sessions** - Real-time connection state

## Configuration

### Basic Configuration

```toml
[cache]
cache_type = "Redis"
redis_url = "redis://localhost:6379/0"
redis_pool_size = 20
```

### Production Configuration

```toml
[cache]
cache_type = "Redis"
redis_url = "redis://redis-cluster:6379/0"
redis_pool_size = 50
```

### Environment Variable

You can also set Redis URL via environment:

```bash
export REDIS_URL=redis://localhost:6379/0
```

## Running Redis

### Docker

```bash
docker run -d \
  --name rcommerce-redis \
  -p 6379:6379 \
  redis:7-alpine
```

### Docker Compose

```yaml
version: '3.8'
services:
  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    command: redis-server --appendonly yes

volumes:
  redis_data:
```

### Homebrew (macOS)

```bash
brew install redis
brew services start redis
```

## Cache Namespaces

R Commerce uses key prefixes to avoid collisions:

| Namespace | Purpose | Example Key |
|-----------|---------|-------------|
| `product` | Product catalog cache | `product:123e4567` |
| `session` | User sessions | `session:abc123` |
| `cart` | Shopping carts | `cart:user:123` |
| `rate:limit` | Rate limiting | `rate:limit:ip:192.168.1.1` |
| `token:blacklist` | Revoked tokens | `token:blacklist:xyz789` |

## Monitoring

### Redis CLI

```bash
# Connect to Redis
redis-cli

# Check connected clients
INFO clients

# Memory usage
INFO memory

# Key statistics
DBSIZE

# Slow queries
SLOWLOG GET 10
```

### Health Check

The server logs Redis connection status on startup:

```
INFO Connecting to Redis at redis://localhost:6379/0...
INFO Redis connected successfully
```

If Redis is unavailable, the server continues without caching:

```
WARN Failed to connect to Redis: Connection refused. Continuing without cache.
```

## Performance Tuning

### Connection Pool Size

Recommended pool sizes:

| Environment | Pool Size | Notes |
|-------------|-----------|-------|
| Development | 5 | Minimal connections |
| Production | 20-50 | Based on load |
| High Traffic | 100+ | With Redis Cluster |

### TTL Configuration

Default TTL values:

- **Products**: 1 hour
- **Sessions**: 2 hours
- **Rate Limits**: 1 hour
- **API Cache**: 5 minutes

## High Availability

### Redis Sentinel

For automatic failover:

```toml
[cache]
redis_url = "redis://sentinel:26379"
sentinel_enabled = true
sentinel_nodes = ["sentinel1:26379", "sentinel2:26379"]
sentinel_service = "mymaster"
```

### Redis Cluster

For horizontal scaling:

```toml
[cache]
redis_url = "redis://node1:6379"
cluster_enabled = true
cluster_nodes = [
    "node1:6379",
    "node2:6379",
    "node3:6379"
]
```

## Troubleshooting

### Connection Errors

```
ERROR: Redis connection error: Connection refused
```

**Solution**: Check Redis is running and URL is correct.

### Memory Issues

```
WARNING: Redis memory usage high
```

**Solution**: 
- Enable key expiration (TTL)
- Set `maxmemory-policy allkeys-lru`
- Increase Redis server memory

### Slow Queries

```bash
# Find slow commands
redis-cli SLOWLOG GET 10
```

## Security

### Enable Authentication

```toml
[cache]
redis_url = "redis://:password@localhost:6379/0"
```

### TLS/SSL

```toml
[cache]
redis_url = "rediss://localhost:6380/0"
use_tls = true
verify_certificate = true
```

### Network Security

- Use firewall rules to restrict Redis access
- Bind Redis to localhost or private network
- Never expose Redis to the public internet
