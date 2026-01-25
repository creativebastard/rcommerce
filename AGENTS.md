# AGENTS.md - R Commerce Headless E-Commerce Platform

This file provides essential information for AI coding agents working on the R Commerce project.

---

## Project Overview

**R Commerce** is a high-performance, Rust-based headless e-commerce platform designed for multi-platform deployment and enterprise-scale operations. It follows an API-first architecture, providing maximum flexibility for frontend implementations.

### Key Characteristics

- **Language**: Rust (edition 2021, minimum version 1.70.0)
- **Architecture**: Headless/API-first (REST with WebSocket support)
- **Database**: Multi-database support (PostgreSQL, MySQL, SQLite)
- **Cache**: In-memory or Redis
- **License**: MIT

### Performance Targets

| Metric | Target |
|--------|--------|
| Binary Size | ~20MB (release build) |
| Memory Usage | 10-50MB runtime |
| API Response | Sub-10ms average |
| Concurrent Users | 10,000+ per instance |
| Startup Time | < 1 second |

---

## Project Structure

```
gokart/
├── Cargo.toml              # Workspace manifest
├── Cargo.lock              # Dependency lock file
├── crates/
│   ├── rcommerce-core/     # Core library - models, traits, repositories
│   │   └── src/
│   │       ├── lib.rs          # Public exports
│   │       ├── config.rs       # Configuration structure
│   │       ├── error.rs        # Error types and handling
│   │       ├── common.rs       # Common utilities
│   │       ├── db.rs           # Database connection pooling
│   │       ├── traits.rs       # Core traits (Entity, Repository, etc.)
│   │       ├── models/         # Data models
│   │       │   ├── mod.rs
│   │       │   ├── product.rs
│   │       │   ├── customer.rs
│   │       │   ├── order.rs
│   │       │   └── address.rs
│   │       ├── repository/     # Database repositories
│   │       │   ├── mod.rs
│   │       │   ├── product_repository.rs
│   │       │   ├── customer_repository.rs
│   │       │   └── order_repository.rs
│   │       ├── services/       # Business logic services
│   │       │   ├── mod.rs
│   │       │   ├── product_service.rs
│   │       │   ├── customer_service.rs
│   │       │   ├── order_service.rs
│   │       │   └── auth_service.rs
│   │       ├── payment/        # Payment gateway integrations
│   │       │   ├── mod.rs
│   │       │   ├── gateways/
│   │       │   └── tests.rs
│   │       ├── order/          # Order lifecycle management
│   │       ├── inventory/      # Inventory tracking & reservations
│   │       ├── notification/   # Email/SMS/webhook system
│   │       ├── websocket/      # Real-time WebSocket connections
│   │       ├── cache/          # Caching (Redis, in-memory)
│   │       ├── jobs/           # Background job processing
│   │       ├── middleware/     # Rate limiting middleware
│   │       └── performance/    # Performance monitoring & optimization
│   ├── rcommerce-api/      # HTTP API server (Axum)
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── server.rs       # HTTP server setup
│   │       ├── state.rs        # Application state
│   │       ├── routes/         # API route handlers
│   │       │   ├── mod.rs
│   │       │   ├── product.rs
│   │       │   ├── customer.rs
│   │       │   ├── order.rs
│   │       │   └── auth.rs
│   │       ├── middleware/     # API middleware
│   │       └── tls/            # TLS certificate management
│   └── rcommerce-cli/      # Command-line management tool
│       └── src/
│           └── main.rs         # CLI entry point
├── crates/rcommerce-core/migrations/
│   └── 001_initial_schema.sql  # Database schema
├── docs/                   # Documentation
│   ├── architecture/       # Architecture documentation
│   ├── api/                # API design docs
│   ├── deployment/         # Deployment guides
│   ├── development/        # Developer guides
│   └── project/            # Project status docs
└── test_api.sh             # API testing script
```

---

## Technology Stack

### Core Framework

| Component | Crate | Purpose |
|-----------|-------|---------|
| Async Runtime | `tokio` | Async runtime with full features |
| HTTP Server | `axum` | Web framework with macros |
| Database | `sqlx` | Async SQL with compile-time checking |
| Serialization | `serde` | JSON serialization |
| Validation | `validator` | Input validation |

### Additional Dependencies

| Category | Crates |
|----------|--------|
| Cache | `redis` (1.0), `dashmap`, `lru` |
| WebSocket | `tokio-tungstenite`, `futures` |
| Authentication | `jsonwebtoken`, `jwt-simple` |
| Email | `lettre`, `handlebars` (templating) |
| HTTP Client | `reqwest` |
| Decimal | `rust_decimal` (financial precision) |
| Testing | `tokio-test`, `mockall`, `wiremock` |
| CLI | `clap` (derive features) |

---

## Build and Test Commands

### Building

```bash
# Build all crates in debug mode
cargo build --workspace

# Build release optimized binary (~20MB)
cargo build --release

# Check without building (fast)
cargo check --workspace

# Build specific crate
cargo build -p rcommerce-core
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p rcommerce-core

# Run with output visible
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests (requires test database)
cargo test --test integration
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Fix auto-fixable issues
cargo clippy --fix

# Check dependencies for vulnerabilities
cargo audit
```

---

## Configuration System

### Configuration File (TOML)

Configuration is loaded from TOML files. Default search paths:
1. Path specified in `RCOMMERCE_CONFIG` environment variable
2. `./config/default.toml`
3. `./config/production.toml`
4. `/etc/rcommerce/config.toml`

### Key Configuration Sections

```toml
[server]
host = "0.0.0.0"
port = 8080
worker_threads = 0  # 0 = use CPU core count

[database]
db_type = "Postgres"  # Or Mysql, Sqlite
host = "localhost"
port = 5432
database = "rcommerce"
username = "rcommerce"
password = "password"
pool_size = 20

[cache]
cache_type = "Memory"  # Or Redis
max_size_mb = 100
redis_url = "redis://localhost:6379"

[security]
api_key_prefix_length = 8
api_key_secret_length = 32

[security.jwt]
secret = "change_this_in_production"
expiry_hours = 24

[media]
storage_type = "Local"
local_path = "./uploads"

[notifications]
enabled = true

[rate_limiting]
enabled = true
requests_per_minute = 60

[features]
debug_api = true
metrics = true
health_check = true
```

---

## Code Organization Conventions

### Module Pattern

Each module follows this structure:

```rust
// lib.rs or mod.rs
pub mod sub_module;
pub use sub_module::*;

// module implementation
pub struct ModuleType { ... }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_something() { ... }
}
```

### Error Handling

- Use the custom `Error` enum from `rcommerce_core::error`
- All errors implement `std::error::Error`
- Error type includes HTTP status code mapping
- Helper constructors: `Error::validation()`, `Error::not_found()`, etc.

```rust
use rcommerce_core::{Error, Result};

fn do_something() -> Result<Thing> {
    if invalid {
        return Err(Error::validation("Field is required"));
    }
    Ok(thing)
}
```

### Repository Pattern

```rust
#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>>;
    async fn create(&self, product: &Product) -> Result<Product>;
    async fn update(&self, product: &Product) -> Result<Product>;
    async fn delete(&self, id: Uuid) -> Result<bool>;
    async fn list(&self, pagination: Pagination) -> Result<Vec<Product>>;
}
```

### Service Layer Pattern

```rust
pub struct ProductService<R: ProductRepository> {
    repository: R,
}

impl<R: ProductRepository> ProductService<R> {
    pub async fn create_product(&self, input: CreateProductInput) -> Result<Product> {
        // Validation
        // Business logic
        // Repository call
    }
}
```

---

## Testing Strategy

### Unit Tests

- Located inline in source files under `#[cfg(test)]` modules
- Use `mockall` for mocking dependencies
- Test individual functions in isolation

### Integration Tests

- Located in `src/*/tests.rs` or `tests/` directories
- Test complete workflows
- Require test database

### API Testing

- Use the `test_api.sh` script for endpoint testing
- Tests against a running server instance
- Uses SQLite for test database

```bash
# Run full API test suite
./test_api.sh
```

### Test Database Setup

```bash
# Create test database (PostgreSQL)
psql -U postgres -c "CREATE DATABASE rcommerce_test;"
psql -U postgres -c "CREATE USER rcommerce_test WITH PASSWORD 'testpass';"
psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE rcommerce_test TO rcommerce_test;"
```

---

## Database Schema

### Core Tables

- `products` - Product catalog
- `product_variants` - Product variations (size, color, etc.)
- `product_images` - Product media
- `customers` - Customer accounts
- `addresses` - Customer addresses
- `orders` - Order headers
- `order_items` - Order line items
- `fulfillments` - Shipping fulfillments
- `payments` - Payment records
- `audit_logs` - Audit trail
- `api_keys` - API authentication

### Database Types (PostgreSQL)

```sql
currency - ENUM ('USD', 'EUR', 'GBP', 'JPY', 'AUD', 'CAD', 'CNY', 'HKD', 'SGD')
order_status - ENUM ('pending', 'confirmed', 'processing', 'on_hold', 'completed', 'cancelled', 'refunded')
fulfillment_status - ENUM ('pending', 'processing', 'partial', 'shipped', 'delivered', 'cancelled', 'returned')
payment_status - ENUM ('pending', 'authorized', 'paid', 'failed', 'cancelled', 'refunded')
```

---

## Security Considerations

### Authentication

- API key authentication for service-to-service
- JWT tokens for user sessions
- Configurable token expiry

### Rate Limiting

- Per-IP rate limiting (configurable thresholds)
- Higher limits for authenticated API keys
- DDoS protection mode

### Best Practices (Required)

1. **Never commit secrets** - Use environment variables
2. **Validate all inputs** - Use `validator` crate
3. **Parameterized queries** - SQLx prevents SQL injection
4. **Rate limiting** - Built into middleware
5. **CORS configuration** - Restrict origins in production

### Security Checklist Before Deployment

- [ ] Change default JWT secret
- [ ] Enable rate limiting
- [ ] Configure CORS properly (not `*`)
- [ ] Use TLS/SSL certificates
- [ ] Set secure database passwords
- [ ] Disable debug API in production

---

## Deployment

### Docker Deployment

```bash
# Build Docker image
docker build -t rcommerce:latest .

# Run with docker-compose
docker-compose up -d
```

Supported orchestrators:
- Docker Compose (single node)
- Kubernetes (multi-node)
- FreeBSD Jails
- Linux systemd

### Production Build

```bash
# Optimize for production
cargo build --release

# Binary location
target/release/rcommerce
```

---

## CLI Usage

```bash
# Start server
rcommerce server -H 0.0.0.0 -P 8080

# Database operations
rcommerce db migrate
rcommerce db reset
rcommerce db seed
rcommerce db status

# Product management
rcommerce product list
rcommerce product create
rcommerce product get <id>

# Order management
rcommerce order list
rcommerce order get <id>

# Show configuration
rcommerce config
```

---

## Common Development Tasks

### Adding a New Model

1. Add struct to `crates/rcommerce-core/src/models/`
2. Add `sqlx::FromRow` derive for database mapping
3. Add `Serialize`, `Deserialize` for API
4. Create corresponding table in migrations

### Adding a New API Endpoint

1. Add route handler in `crates/rcommerce-api/src/routes/`
2. Register in `routes/mod.rs`
3. Add to server router in `server.rs`
4. Add integration test

### Adding a Payment Gateway

1. Implement `PaymentGateway` trait in `crates/rcommerce-core/src/payment/gateways/`
2. Add configuration to `Config`
3. Add provider factory method

---

## Debugging Tips

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run
RUST_LOG=rcommerce_api=debug,sqlx=warn cargo run
```

### Full Backtrace on Panic

```bash
RUST_BACKTRACE=1 cargo run
RUST_BACKTRACE=full cargo run
```

### Database Query Logging

Enable in configuration:
```toml
[database]
log_queries = true
log_slow_queries = 1000  # ms
```

---

## Documentation

- `docs/architecture/` - Design patterns and decisions
- `docs/api/` - API specifications
- `docs/deployment/` - Deployment guides
- `docs/development/` - Development guides
- `README.md` - Quick start guide

---

## Repository

- **URL**: https://gitee.com/captainjez/gocart
- **Primary Branch**: `master`
- **License**: MIT

---

*This file should be updated when significant architectural changes occur.*
