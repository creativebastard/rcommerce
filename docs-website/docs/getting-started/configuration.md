# Configuration Guide

This guide covers the essential configuration options for R Commerce.

## Configuration File Format

R Commerce uses TOML format for configuration files, with support for environment variable overrides.

## Configuration File Locations

Configuration is loaded from TOML files in the following order:

1. Path specified in `RCOMMERCE_CONFIG` environment variable
2. `./config/default.toml`
3. `./config/production.toml`
4. `/etc/rcommerce/config.toml`

## Minimal Configuration

A minimal configuration file for development:

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

## Server Configuration

```toml
[server]
# HTTP server settings
host = "0.0.0.0"           # Bind address (0.0.0.0 for all interfaces)
port = 8080                # HTTP port
graceful_shutdown_timeout = "30s"  # How long to wait for connections to close

# Request settings
max_request_size = "10MB"  # Maximum request body size
keep_alive = true          # Enable HTTP keep-alive
keep_alive_timeout = "75s" # Keep-alive timeout

# Worker threads (0 = automatic based on CPU count)
worker_threads = 0

# Rate limiting (per IP)
rate_limit_per_minute = 1000
rate_limit_burst = 200

# CORS settings
[cors]
enabled = true
allowed_origins = ["*"]    # or specific origins: ["https://store.com"]
allowed_methods = ["GET", "POST", "PUT", "PATCH", "DELETE"]
allowed_headers = ["Content-Type", "Authorization"]
expose_headers = ["X-Request-ID", "X-Rate-Limit-Remaining"]
allow_credentials = true
max_age = "1h"
```

## Database Configuration

### PostgreSQL

```toml
[database]
type = "postgres"
host = "localhost"
port = 5432
username = "rcommerce"
password = "secure_password"
database = "rcommerce_prod"

# Connection pooling
pool_size = 20
max_lifetime = "30min"
idle_timeout = "10min"
connection_timeout = "30s"

# SSL/TLS settings
ssl_mode = "prefer"  # Options: "disable", "prefer", "require"
```

## Cache Configuration

### Redis Cache

```toml
[cache]
provider = "redis"

[cache.redis]
host = "127.0.0.1"
port = 6379
database = 0
password = "your_redis_password"

# Connection pooling
pool_size = 10
max_lifetime = "30min"
idle_timeout = "10min"
connection_timeout = "5s"

# Key prefixes
key_prefix = "rcommerce:"

# Cache namespaces
[cache.namespaces]
products = { ttl_seconds = 3600 }
orders = { ttl_seconds = 1800 }
customers = { ttl_seconds = 3600 }
sessions = { ttl_seconds = 7200 }
rate_limits = { ttl_seconds = 60 }
```

### In-Memory Cache

```toml
[cache]
provider = "memory"

[cache.memory]
max_size_mb = 100
ttl_seconds = 300
```

## Payment Configuration

```toml
[payments]
default_gateway = "stripe"
auto_capture = true
capture_delay_seconds = 0
supported_currencies = ["USD", "EUR", "GBP", "JPY", "CAD", "AUD"]

# Fraud detection
enable_fraud_check = true
risk_threshold = 75

# Stripe Configuration
[payments.stripe]
enabled = true
secret_key = "sk_live_your_secret_key"
publishable_key = "pk_live_your_publishable_key"
webhook_secret = "whsec_your_webhook_secret"

# Airwallex Configuration
[payments.airwallex]
enabled = false
client_id = "your_client_id"
api_key = "your_api_key"
webhook_secret = "your_webhook_secret"

# PayPal Configuration
[payments.paypal]
enabled = false
client_id = "your_client_id"
client_secret = "your_client_secret"

# Manual/Bank Transfer
[payments.manual]
enabled = true
instructions = "Please transfer to: Bank: ... Account: ..."
```

## Shipping Configuration

```toml
[shipping]
default_provider = "shipstation"
default_weight_unit = "lb"
default_dimension_unit = "in"

# ShipStation Configuration
[shipping.shipstation]
enabled = true
api_key = "your_api_key"
api_secret = "your_api_secret"

# EasyPost Configuration
[shipping.easypost]
enabled = false
api_key = "your_api_key"
```

## Notification Configuration

```toml
[notifications]
from_name = "Your Store"
from_email = "orders@yourstore.com"
queue_size = 1000
worker_count = 2
retry_attempts = 3

# Email Configuration
[notifications.email]
provider = "smtp"

[notifications.email.smtp]
host = "smtp.sendgrid.net"
port = 587
username = "apikey"
password = "SG.your_api_key"
use_tls = true

# SMS Configuration
[notifications.sms]
enabled = false
provider = "twilio"

[notifications.sms.twilio]
account_sid = "your_account_sid"
auth_token = "your_auth_token"
from_number = "+1234567890"
```

## Logging Configuration

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

## Security Configuration

```toml
[security]
api_key_prefix_length = 8
api_key_secret_length = 32

# JWT settings
[security.jwt]
secret = "your_jwt_secret_key_here_32_chars_min"
expiry_hours = 24

# Session settings
[security.sessions]
enabled = true
ttl_hours = 24

# Webhook security
[security.webhooks]
require_https = true
verify_signatures = true
```

## Feature Flags

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

## Environment Variable Overrides

All configuration options can be overridden via environment variables using the pattern:

```bash
RCOMMERCE_<SECTION>_<SUBSECTION>_<KEY>=value
```

Examples:

```bash
# Server
export RCOMMERCE_SERVER_HOST=0.0.0.0
export RCOMMERCE_SERVER_PORT=3000

# Database
export RCOMMERCE_DATABASE_TYPE=postgres
export RCOMMERCE_DATABASE_HOST=db.example.com
export RCOMMERCE_DATABASE_PASSWORD=secure_password

# Payments
export RCOMMERCE_PAYMENTS_STRIPE_SECRET_KEY=sk_live_xxx

# Security
export RCOMMERCE_SECURITY_JWT_SECRET=your_secret_here
```

## Configuration Validation

Validate your configuration:

```bash
# Validate configuration file
rcommerce config validate

# Test database connection
rcommerce db check

# Test payment gateway configuration
rcommerce gateway test stripe
```

## Production Configuration Example

```toml
# Production configuration
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

## Next Steps

- [Development Guide](../development/index.md) - Set up your development environment
- [Deployment Guide](../deployment/index.md) - Deploy to production
- [API Reference](../api-reference/index.md) - Start building with the API
