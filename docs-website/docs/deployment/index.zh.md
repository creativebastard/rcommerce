# 部署指南

使用这些全面的部署指南，自信地将 R Commerce 部署到生产环境。

## 部署选项

| 方法 | 最适合 | 复杂度 |
|--------|----------|------------|
| [Docker](./docker.md) | 大多数部署 | 低 |
| [Kubernetes](./kubernetes.md) | 大规模、多区域 | 高 |
| [Linux (systemd)](./linux/systemd.md) | 传统 Linux 服务器 | 中 |
| [Linux (手动)](./linux/manual.md) | 自定义设置 | 中 |
| [FreeBSD Jails](./freebsd/jails.md) | FreeBSD 环境 | 中 |
| [FreeBSD (rc.d)](./freebsd/rc.d.md) | 传统 FreeBSD | 中 |
| [macOS (launchd)](./macos/launchd.md) | 开发/Mac 服务器 | 低 |
| [二进制](./binary.md) | 最小依赖 | 低 |

## 快速开始

对于大多数生产部署，我们推荐 Docker：

```bash
# 克隆仓库
git clone https://gitee.com/captainjez/gocart.git
cd gocart

# 复制并编辑配置
cp config/production.toml config/myconfig.toml
# 编辑数据库、安全设置

# 使用 Docker Compose 启动
docker-compose -f docker-compose.prod.yml up -d
```

## 部署前检查清单

- [ ] 数据库已配置并可访问
- [ ] Redis 已配置（用于缓存/会话）
- [ ] SSL/TLS 证书已准备就绪
- [ ] 域名 DNS 已配置
- [ ] 环境变量已设置
- [ ] 安全设置已审核
- [ ] 备份策略已到位
- [ ] 监控已配置

## 系统要求

### 最低配置

- **CPU**: 1 核
- **内存**: 512MB
- **磁盘**: 1GB
- **网络**: 100Mbps

### 推荐配置

- **CPU**: 2+ 核
- **内存**: 2GB+
- **磁盘**: 10GB SSD
- **网络**: 1Gbps

### 数据库

- PostgreSQL 14+

## 配置

所有部署方法使用相同的配置文件格式：

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

## 下一步

选择您的部署方法：

1. **[Docker](./docker.md)** - 容器化部署
2. **[Kubernetes](./kubernetes.md)** - 编排部署
3. **[Linux](./linux/systemd.md)** - 传统 Linux 服务器
4. **[FreeBSD](./freebsd/jails.md)** - FreeBSD 部署
5. **[macOS](./macos/launchd.md)** - macOS 服务器
