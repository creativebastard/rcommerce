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
type = "postgres"
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
ssl_mode = "prefer"  # 选项："disable"、"prefer"、"require"
```

## 缓存配置

### Redis 缓存

```toml
[cache]
provider = "redis"

[cache.redis]
host = "127.0.0.1"
port = 6379
database = 0
password = "your_redis_password"

# 连接池
pool_size = 10
max_lifetime = "30min"
idle_timeout = "10min"
connection_timeout = "5s"

# 键前缀
key_prefix = "rcommerce:"

# 缓存命名空间
[cache.namespaces]
products = { ttl_seconds = 3600 }
orders = { ttl_seconds = 1800 }
customers = { ttl_seconds = 3600 }
sessions = { ttl_seconds = 7200 }
rate_limits = { ttl_seconds = 60 }
```

### 内存缓存

```toml
[cache]
provider = "memory"

[cache.memory]
max_size_mb = 100
ttl_seconds = 300
```

## 支付配置

```toml
[payments]
default_gateway = "stripe"
auto_capture = true
capture_delay_seconds = 0
supported_currencies = ["USD", "EUR", "GBP", "JPY", "CAD", "AUD"]

# 欺诈检测
enable_fraud_check = true
risk_threshold = 75

# Stripe 配置
[payments.stripe]
enabled = true
secret_key = "sk_live_your_secret_key"
publishable_key = "pk_live_your_publishable_key"
webhook_secret = "whsec_your_webhook_secret"

# Airwallex 配置
[payments.airwallex]
enabled = false
client_id = "your_client_id"
api_key = "your_api_key"
webhook_secret = "your_webhook_secret"

# PayPal 配置
[payments.paypal]
enabled = false
client_id = "your_client_id"
client_secret = "your_client_secret"

# 手动/银行转账
[payments.manual]
enabled = true
instructions = "Please transfer to: Bank: ... Account: ..."
```

## 配送配置

```toml
[shipping]
default_provider = "shipstation"
default_weight_unit = "lb"
default_dimension_unit = "in"

# ShipStation 配置
[shipping.shipstation]
enabled = true
api_key = "your_api_key"
api_secret = "your_api_secret"

# EasyPost 配置
[shipping.easypost]
enabled = false
api_key = "your_api_key"
```

## 通知配置

```toml
[notifications]
from_name = "Your Store"
from_email = "orders@yourstore.com"
queue_size = 1000
worker_count = 2
retry_attempts = 3

# 邮件配置
[notifications.email]
provider = "smtp"

[notifications.email.smtp]
host = "smtp.sendgrid.net"
port = 587
username = "apikey"
password = "SG.your_api_key"
use_tls = true

# 短信配置
[notifications.sms]
enabled = false
provider = "twilio"

[notifications.sms.twilio]
account_sid = "your_account_sid"
auth_token = "your_auth_token"
from_number = "+1234567890"
```

## 日志配置

```toml
[logging]
level = "info"
format = "json"

[logging.outputs]
console = { enabled = true, format = "text" }

[logging.file]
enabled = true
path = "/var/log/rcommerce/rcommerce.log"
rotation = "daily"
max_size = "100MB"
max_files = 10
```

## 安全配置

```toml
[security]
api_key_prefix_length = 8
api_key_secret_length = 32

# JWT 设置
[security.jwt]
secret = "your_jwt_secret_key_here_32_chars_min"
expiry_hours = 24

# 会话设置
[security.sessions]
enabled = true
ttl_hours = 24

# Webhook 安全
[security.webhooks]
require_https = true
verify_signatures = true
```

## 功能开关

```toml
[features]
products = true
orders = true
customers = true
payments = true
shipping = true
discounts = true
taxes = true
inventory = true
returns = true
gift_cards = true
graphql_api = true
webhooks = true
realtime_updates = true
caching = true
background_jobs = true
```

## 环境变量覆盖

所有配置选项都可以使用以下模式通过环境变量覆盖：

```bash
RCOMMERCE_<SECTION>_<SUBSECTION>_<KEY>=value
```

示例：

```bash
# 服务器
export RCOMMERCE_SERVER_HOST=0.0.0.0
export RCOMMERCE_SERVER_PORT=3000

# 数据库
export RCOMMERCE_DATABASE_TYPE=postgres
export RCOMMERCE_DATABASE_HOST=db.example.com
export RCOMMERCE_DATABASE_PASSWORD=secure_password

# 支付
export RCOMMERCE_PAYMENTS_STRIPE_SECRET_KEY=sk_live_xxx

# 安全
export RCOMMERCE_SECURITY_JWT_SECRET=your_secret_here
```

## 配置验证

验证您的配置：

```bash
# 验证配置文件
rcommerce config validate

# 测试数据库连接
rcommerce db check

# 测试支付网关配置
rcommerce gateway test stripe
```

## 生产配置示例

```toml
# 生产配置
[server]
host = "0.0.0.0"
port = 8080
worker_threads = 0
rate_limit_per_minute = 1000

[database]
type = "postgres"
host = "prod-db.internal.example.com"
port = 5432
username = "rcommerce_prod"
password = "${DB_PASSWORD}"
ssl_mode = "require"
pool_size = 50

[payments]
default_gateway = "stripe"
auto_capture = true

[payments.stripe]
secret_key = "${STRIPE_SECRET_KEY}"
webhook_secret = "${STRIPE_WEBHOOK_SECRET}"

[cache]
provider = "redis"

[cache.redis]
host = "cache.internal.example.com"
port = 6379
password = "${REDIS_PASSWORD}"
pool_size = 20

[notifications.email]
provider = "sendgrid"

[notifications.email.sendgrid]
api_key = "${SENDGRID_API_KEY}"

[logging]
level = "info"
format = "json"

[security.jwt]
secret = "${JWT_SECRET}"

[cors]
enabled = true
allowed_origins = ["https://store.example.com"]
allow_credentials = true

[features]
products = true
orders = true
customers = true
payments = true
shipping = true
```

## CORS 配置

CORS（跨域资源共享）可以在您的 `config.toml` 中配置：

```toml
[cors]
allowed_origins = ["https://yourdomain.com", "https://app.yourdomain.com"]
allowed_methods = ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
allowed_headers = ["authorization", "content-type", "x-requested-with"]
allow_credentials = true
max_age = 3600
```

### 安全警告

切勿在生产环境中使用 `allowed_origins = ["*"]`。这会允许任何网站向您的 API 发出请求。

## 下一步

- [开发指南](../development/index.md) - 设置开发环境
- [部署指南](../deployment/index.md) - 部署到生产环境
- [API 参考](../api-reference/index.md) - 开始使用 API 构建
