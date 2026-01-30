# Redis 缓存

R Commerce 使用 Redis 进行高性能缓存和会话存储。本指南涵盖配置、监控和最佳实践。

## 概述

Redis 用于：

- **产品目录缓存** - 快速产品查询
- **会话存储** - 购物车和用户会话
- **速率限制** - API 节流
- **令牌黑名单** - JWT 撤销
- **WebSocket 会话** - 实时连接状态

## 配置

### 基本配置

```toml
[cache]
cache_type = "Redis"
redis_url = "redis://localhost:6379/0"
redis_pool_size = 20
```

### 生产环境配置

```toml
[cache]
cache_type = "Redis"
redis_url = "redis://redis-cluster:6379/0"
redis_pool_size = 50
```

### 环境变量

您也可以通过环境变量设置 Redis URL：

```bash
export REDIS_URL=redis://localhost:6379/0
```

## 运行 Redis

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

## 缓存命名空间

R Commerce 使用键前缀来避免冲突：

| 命名空间 | 用途 | 示例键 |
|-----------|---------|-------------|
| `product` | 产品目录缓存 | `product:123e4567` |
| `session` | 用户会话 | `session:abc123` |
| `cart` | 购物车 | `cart:user:123` |
| `rate:limit` | 速率限制 | `rate:limit:ip:192.168.1.1` |
| `token:blacklist` | 撤销的令牌 | `token:blacklist:xyz789` |

## 监控

### Redis CLI

```bash
# 连接 Redis
redis-cli

# 检查连接客户端
INFO clients

# 内存使用
INFO memory

# 键统计
DBSIZE

# 慢查询
SLOWLOG GET 10
```

### 健康检查

服务器在启动时记录 Redis 连接状态：

```
INFO 正在连接到 Redis redis://localhost:6379/0...
INFO Redis 连接成功
```

如果 Redis 不可用，服务器将继续运行而不使用缓存：

```
WARN 无法连接到 Redis：连接被拒绝。继续运行而不使用缓存。
```

## 性能调优

### 连接池大小

推荐的池大小：

| 环境 | 池大小 | 说明 |
|-------------|-----------|-------|
| 开发环境 | 5 | 最小连接数 |
| 生产环境 | 20-50 | 基于负载 |
| 高流量 | 100+ | 使用 Redis 集群 |

### TTL 配置

默认 TTL 值：

- **产品**：1 小时
- **会话**：2 小时
- **速率限制**：1 小时
- **API 缓存**：5 分钟

## 高可用性

### Redis Sentinel

用于自动故障转移：

```toml
[cache]
redis_url = "redis://sentinel:26379"
sentinel_enabled = true
sentinel_nodes = ["sentinel1:26379", "sentinel2:26379"]
sentinel_service = "mymaster"
```

### Redis 集群

用于水平扩展：

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

## 故障排除

### 连接错误

```
错误：Redis 连接错误：连接被拒绝
```

**解决方案**：检查 Redis 是否正在运行且 URL 是否正确。

### 内存问题

```
警告：Redis 内存使用率过高
```

**解决方案**：
- 启用键过期（TTL）
- 设置 `maxmemory-policy allkeys-lru`
- 增加 Redis 服务器内存

### 慢查询

```bash
# 查找慢命令
redis-cli SLOWLOG GET 10
```

## 安全

### 启用认证

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

### 网络安全

- 使用防火墙规则限制 Redis 访问
- 将 Redis 绑定到本地主机或私有网络
- 永远不要将 Redis 暴露到公共互联网
