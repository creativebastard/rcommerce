# 配置指南

本指南涵盖 R Commerce 的基本配置选项。

## 配置文件格式

R Commerce 使用 TOML 格式的配置文件，支持环境变量覆盖。

## 配置文件位置

配置按以下顺序从 TOML 文件加载：

1. `RCOMMERCE_CONFIG` 环境变量中指定的路径
2. `./config/default.toml`
3. `./config/production.toml`
4. `/etc/rcommerce/config.toml`

## 最小配置

开发环境的最小配置文件：

```toml
[server]
host = "127.0.0.1"
port = 8080

[database]
db_type = "Postgres"
host = "localhost"
port = 5432
database = "rcommerce"
username = "rcommerce"
password = "your_password"
pool_size = 20

[cache]
provider = "memory"
```

## 服务器配置

```toml
[server]
# HTTP 服务器设置
host = "0.0.0.0"           # 绑定地址（0.0.0.0 表示所有接口）
port = 8080                # HTTP 端口
graceful_shutdown_timeout = "30s"  # 等待连接关闭的时间

# 请求设置
max_request_size = "10MB"  # 最大请求体大小
keep_alive = true          # 启用 HTTP keep-alive
keep_alive_timeout = "75s" # Keep-alive 超时时间

# 工作线程（0 = 根据 CPU 核心数自动）
worker_threads = 0

# 速率限制（每个 IP）
rate_limit_per_minute = 1000
rate_limit_burst = 200

# CORS 设置
[cors]
enabled = true
allowed_origins = ["*"]    # 或特定来源：["https://store.com"]
allowed_methods = ["GET", "POST", "PUT", "PATCH", "DELETE"]
allowed_headers = ["Content-Type", "Authorization"]
expose_headers = ["X-Request-ID", "X-Rate-Limit-Remaining"]
allow_credentials = true
max_age = "1h"
```

## 数据库配置

### PostgreSQL

```toml
[database]
db_type = "Postgres"
host = "localhost"
port = 5432
username = "rcommerce"
password = "secure_password"
database = "rcommerce_prod"

# 连接池
pool_size = 20
max_lifetime = "30min"
idle_timeout = "10min"
connection_timeout = "30s"

# SSL/TLS 设置
ssl_mode = "prefer"  # 选项："disable", "prefer", "require"
```

## 缓存配置

### 内存缓存

```toml
[cache]
cache_type = "Memory"
max_size_mb = 100
```

### Redis 缓存

```toml
[cache]
cache_type = "Redis"
redis_url = "redis://localhost:6379/0"
redis_pool_size = 20
```

## 安全配置

```toml
[security]
# JWT 设置
jwt_secret = "your-secret-key-min-32-chars-long"
jwt_expiry_hours = 24

# API 密钥设置
api_key_prefix_length = 8
api_key_secret_length = 32

# 密码策略
min_password_length = 8
require_uppercase = true
require_lowercase = true
require_numbers = true
require_special_chars = true

# 会话设置
session_timeout = "24h"
max_sessions_per_user = 5
```

## 日志配置

```toml
[logging]
level = "info"  # 选项：trace, debug, info, warn, error
format = "json" # 选项：json, pretty

# 日志输出
output = "stdout" # 选项：stdout, file, both
log_file = "/var/log/rcommerce/app.log"

# 日志轮转
max_log_size_mb = 100
max_log_files = 10
```

## 环境变量覆盖

任何配置值都可以通过环境变量覆盖：

```bash
# 格式：RCOMMERCE_<SECTION>_<KEY>
export RCOMMERCE_SERVER_PORT=9000
export RCOMMERCE_DATABASE_PASSWORD=secret
export RCOMMERCE_SECURITY_JWT_SECRET=mysecret
```

## 完整示例

```toml
[server]
host = "0.0.0.0"
port = 8080
worker_threads = 4

[database]
db_type = "Postgres"
host = "localhost"
port = 5432
database = "rcommerce"
username = "rcommerce"
password = "password"
pool_size = 20

[cache]
cache_type = "Redis"
redis_url = "redis://localhost:6379/0"
redis_pool_size = 20

[security]
jwt_secret = "your-secret-key-change-in-production"
jwt_expiry_hours = 24

[logging]
level = "info"
format = "json"
```

## 配置验证

```bash
# 验证配置
rcommerce config --check

# 测试配置并显示有效配置
rcommerce config --test

# 显示当前配置
rcommerce config show
```

## 生产环境建议

1. **使用强密码** - 数据库和 JWT 密钥
2. **启用 SSL/TLS** - 用于数据库和 API
3. **配置防火墙** - 限制对服务的访问
4. **使用 Redis** - 用于缓存和会话
5. **启用日志轮转** - 防止磁盘空间不足
6. **监控和告警** - 设置健康检查
