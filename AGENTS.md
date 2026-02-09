# AGENTS.md - R Commerce Headless E-Commerce Platform

This file provides essential information for AI coding agents working on the R Commerce project.

---

## Project Overview

**R Commerce** is a high-performance, Rust-based headless e-commerce platform designed for multi-platform deployment and enterprise-scale operations. It follows an API-first architecture, providing maximum flexibility for frontend implementations.

### Key Characteristics

- **Language**: Rust (edition 2021, minimum version 1.70.0)
- **Architecture**: Headless/API-first (REST with WebSocket support)
- **Database**: PostgreSQL
- **Cache**: In-memory (DashMap/LRU) or Redis
- **License**: Dual-licensed under AGPL-3.0 and Commercial License
- **Repository**: https://gitee.com/captainjez/gocart

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
│   ├── rcommerce-core/     # Core library - models, traits, repositories, services
│   │   └── src/
│   │       ├── lib.rs          # Public exports
│   │       ├── config.rs       # Configuration structure (TOML-based)
│   │       ├── error.rs        # Error types and handling
│   │       ├── common.rs       # Common utilities
│   │       ├── db.rs           # Database connection pooling
│   │       ├── traits.rs       # Core traits (Repository, Service)
│   │       ├── models/         # Data models
│   │       │   ├── mod.rs
│   │       │   ├── product.rs
│   │       │   ├── customer.rs
│   │       │   ├── order.rs
│   │       │   ├── address.rs
│   │       │   ├── cart.rs
│   │       │   ├── coupon.rs
│   │       │   └── subscription.rs
│   │       ├── repository/     # Database repositories
│   │       │   ├── mod.rs
│   │       │   ├── product_repository.rs
│   │       │   ├── customer_repository.rs
│   │       │   ├── order_repository.rs
│   │       │   ├── cart_repository.rs
│   │       │   └── coupon_repository.rs
│   │       ├── services/       # Business logic services
│   │       │   ├── mod.rs
│   │       │   ├── product_service.rs
│   │       │   ├── customer_service.rs
│   │       │   ├── order_service.rs
│   │       │   ├── auth_service.rs
│   │       │   ├── cart_service.rs
│   │       │   └── coupon_service.rs
│   │       ├── payment/        # Payment gateway integrations
│   │       │   ├── mod.rs
│   │       │   ├── gateways/
│   │       │   │   ├── stripe.rs
│   │       │   │   ├── airwallex.rs
│   │       │   │   ├── alipay.rs
│   │       │   │   └── wechatpay.rs
│   │       │   └── dunning.rs
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
│   │       │   ├── auth.rs
│   │       │   ├── cart.rs
│   │       │   └── coupon.rs
│   │       ├── middleware/     # API middleware
│   │       └── tls/            # TLS certificate management
│   └── rcommerce-cli/      # Command-line management tool
│       └── src/
│           └── main.rs         # CLI entry point
├── crates/rcommerce-core/migrations/
│   ├── 001_initial_schema.sql  # Database schema
│   └── 002_carts_and_coupons.sql
├── scripts/                # Utility scripts
│   ├── test_api.sh
│   ├── run_e2e_tests.sh
│   └── test_complete_system.sh
└── test_config.toml        # Test configuration
```

---

## Technology Stack

### Core Framework

| Component | Crate | Purpose |
|-----------|-------|---------|
| Async Runtime | `tokio` (1.35) | Async runtime with full features |
| HTTP Server | `axum` (0.7) | Web framework with macros |
| Database | `sqlx` (0.8) | Async SQL with compile-time checking |
| Serialization | `serde` (1.0) | JSON serialization |
| Validation | `validator` (0.16) | Input validation |

### Additional Dependencies

| Category | Crates |
|----------|--------|
| Cache | `redis` (1.0), `dashmap` (5.5), `lru` (0.12) |
| WebSocket | `tokio-tungstenite` (0.21), `futures` (0.3) |
| Authentication | `jsonwebtoken` (9.2), `jwt-simple` (0.12) |
| Email | `lettre` (0.11), `handlebars` (5.1) |
| HTTP Client | `reqwest` (0.11) |
| Decimal | `rust_decimal` (1.33) - financial precision |
| Testing | `tokio-test` (0.4), `mockall` (0.12), `wiremock` (0.6) |
| CLI | `clap` (4.4) with derive features |
| Crypto | `sha2`, `hmac`, `rsa`, `bcrypt` |

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

### Running the Application

```bash
# Run with default config
cargo run --bin rcommerce -- server

# Run with specific config file
cargo run --bin rcommerce -- -c ./config.toml server

# Run API tests
./scripts/test_api.sh
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
db_type = "Postgres"
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
pub trait Repository<T, ID>: Send + Sync {
    async fn find_by_id(&self, id: ID) -> Result<Option<T>>;
    async fn create(&self, entity: T) -> Result<T>;
    async fn update(&self, entity: T) -> Result<T>;
    async fn delete(&self, id: ID) -> Result<bool>;
    async fn list(&self) -> Result<Vec<T>>;
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

- Located in `crates/*/tests/` directories
- Test complete workflows
- Require test database

### API Testing

- Use the `scripts/test_api.sh` script for endpoint testing
- Tests against a running server instance
- Uses PostgreSQL for test database

```bash
# Run full API test suite
./scripts/test_api.sh
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
- `carts` - Shopping carts
- `cart_items` - Cart line items
- `coupons` - Discount coupons
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

## Authentication & Authorization

### JWT Authentication

The API uses JWT (JSON Web Tokens) for user authentication.

#### Auth Endpoints

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/api/v1/auth/register` | POST | No | Create new customer account |
| `/api/v1/auth/login` | POST | No | Authenticate and get tokens |
| `/api/v1/auth/refresh` | POST | No | Refresh access token |

#### Token Types

- **Access Token**: Short-lived (24 hours), used for API requests
- **Refresh Token**: Long-lived (7 days), used to get new access tokens

#### Using Authentication

```bash
# Register
POST /api/v1/auth/register
{
  "email": "user@example.com",
  "password": "securepassword123",
  "first_name": "John",
  "last_name": "Doe"
}

# Login
POST /api/v1/auth/login
{
  "email": "user@example.com",
  "password": "securepassword123"
}
# Response: { "access_token": "...", "refresh_token": "...", "expires_in": 86400 }

# Access protected endpoint
GET /api/v1/customers
Authorization: Bearer <access_token>
```

#### Protected Routes

These routes require JWT authentication:
- `GET/POST /api/v1/customers`
- `GET/POST /api/v1/orders`
- `GET/POST /api/v1/carts/*`
- `GET/POST /api/v1/payments/*`
- `GET/POST /api/v1/coupons`

Public routes (no auth required):
- `GET /api/v1/products`
- `POST /api/v1/auth/*`

### API Key Authentication

API keys provide service-to-service authentication with fine-grained scope-based permissions. Unlike JWT tokens which are user-centric and session-based, API keys are designed for programmatic access and long-term integrations.

#### API Keys vs JWT Tokens

| Feature | API Keys | JWT Tokens |
|---------|----------|------------|
| Purpose | Service-to-service auth | User session auth |
| Lifetime | Long-term (until revoked) | Short-term (hours/days) |
| Scopes | Fine-grained permissions | User role-based |
| Storage | Database (hash only) | Stateless (client-side) |
| Use Case | Integrations, automation | Web/mobile apps |

#### API Key Format

API keys use a two-part format: `prefix.secret`

- **Prefix** (8 characters): Identifies the key in logs and CLI operations. Stored in plaintext for lookup.
- **Secret** (32 characters): The actual secret used for authentication. Hashed before storage.

Example: `ak_1a2b3c4d.x9y8z7w6v5u4t3s2r1q0p9o8n7m6l5k4`

```bash
# Create API key (secret shown only once!)
rcommerce api-key create --name "My Integration" --scopes "products:read,orders:write"
# Output: Key: ak_1a2b3c4d.x9y8z7w6v5u4t3s2r1q0p9o8n7m6l5k4
```

#### Storage Security

API keys are stored securely using cryptographic hashing:

- **Secret**: Hashed with SHA-256 (never stored in plaintext)
- **Prefix**: Stored in plaintext for lookup purposes
- **Scopes**: Stored as comma-separated list
- **Metadata**: Name, created_at, expires_at, last_used_at, revoked status

Database schema for `api_keys` table:

```sql
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    prefix VARCHAR(16) UNIQUE NOT NULL,
    secret_hash VARCHAR(64) NOT NULL,  -- SHA-256 hash
    name VARCHAR(255) NOT NULL,
    scopes TEXT NOT NULL,              -- Comma-separated scopes
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    revoked_reason TEXT,
    created_by UUID REFERENCES customers(id)
);

CREATE INDEX idx_api_keys_prefix ON api_keys(prefix);
CREATE INDEX idx_api_keys_active ON api_keys(prefix) 
    WHERE revoked_at IS NULL AND (expires_at IS NULL OR expires_at > NOW());
```

### Scope-Based Permissions

The API key system uses a hierarchical scope-based permission model.

#### Scope Format

Scopes follow the pattern: `resource:action`

Examples:
- `products:read` - Read access to products
- `orders:write` - Create and update orders
- `customers:admin` - Full admin access to customers

#### Available Resources

| Resource | Description |
|----------|-------------|
| `products` | Product catalog and variants |
| `orders` | Order management |
| `customers` | Customer accounts |
| `carts` | Shopping carts |
| `coupons` | Discount coupons |
| `payments` | Payment processing |
| `inventory` | Inventory tracking |
| `webhooks` | Webhook configuration |
| `users` | User management |
| `settings` | System settings |
| `reports` | Analytics and reports |
| `imports` | Data imports |
| `exports` | Data exports |

#### Available Actions

| Action | Permissions |
|--------|-------------|
| `read` | GET operations, list, view details |
| `write` | POST/PUT/PATCH operations, create and update |
| `admin` | DELETE operations, full control including configuration |

#### Permission Hierarchy

Actions follow a hierarchy where higher permissions include lower ones:

```
admin > write > read
```

- `resource:admin` grants read, write, and admin access
- `resource:write` grants read and write access
- `resource:read` grants only read access

#### Wildcard Scopes

For convenience, you can use action-only wildcards:

- `read` - Read access to **all** resources (equivalent to `*:read`)
- `write` - Write access to **all** resources
- `admin` - Admin access to **all** resources

Examples:
```bash
# Read-only access to everything
--scopes "read"

# Read and write access to everything
--scopes "write"

# Full admin access
--scopes "admin"

# Mixed: read all, write only products and orders
--scopes "read,products:write,orders:write"
```

### Predefined Scope Presets

The system provides convenient preset scopes for common use cases:

```rust
use rcommerce_core::auth::scope_presets;

// Read-only access to all resources
scope_presets::read_only()
// Returns: ["read"]

// Read and write access to all resources
scope_presets::read_write()
// Returns: ["write"]

// Full admin access to all resources
scope_presets::admin()
// Returns: ["admin"]

// Customer-facing access (products, carts, orders, customers)
scope_presets::customer()
// Returns: ["products:read", "carts:read", "carts:write", 
//           "orders:read", "orders:write", "customers:read", "customers:write"]

// Product management (read-only)
scope_presets::products_read_only()
// Returns: ["products:read"]

// Inventory management
scope_presets::inventory_manager()
// Returns: ["products:read", "products:write", 
//           "inventory:read", "inventory:write", "inventory:admin"]

// Webhook processing
scope_presets::webhook_handler()
// Returns: ["webhooks:read", "webhooks:write", 
//           "orders:read", "payments:read"]
```

#### Using Presets in CLI

```bash
# Create read-only API key
rcommerce api-key create --name "Analytics Reader" --scopes "read"

# Create admin API key
rcommerce api-key create --name "System Admin" --scopes "admin"

# Create customer-facing API key
rcommerce api-key create --name "Storefront" --scopes "products:read,carts:write,orders:write"
```

### Using API Key Middleware

The API provides middleware for validating API keys and extracting authentication context.

#### Combined Authentication Middleware

For endpoints that accept both API keys and JWT tokens:

```rust
use axum::{
    routing::get,
    Router,
    middleware,
    extract::Extension,
};
use rcommerce_api::middleware::auth::combined_auth_middleware;
use rcommerce_core::repositories::ApiKeyRepository;
use rcommerce_core::services::AuthService;

let app = Router::new()
    .route("/api/v1/products", get(list_products))
    .layer(middleware::from_fn(|req, next| {
        combined_auth_middleware(
            Extension(state.api_key_repository.clone()),
            Extension(state.auth_service.clone()),
            req,
            next,
        )
    }));
```

#### API-Key-Only Middleware

For service-to-service endpoints that only accept API keys:

```rust
use rcommerce_api::middleware::auth::api_key_middleware;

let app = Router::new()
    .route("/api/v1/webhooks", post(handle_webhook))
    .layer(middleware::from_fn(|req, next| {
        api_key_middleware(
            Extension(state.api_key_repository.clone()),
            req,
            next,
        )
    }));
```

### Checking Permissions in Handlers

Once authenticated via API key, handlers can check permissions using the `ApiKeyAuth` extension:

```rust
use axum::{
    extract::Extension,
    response::IntoResponse,
    Json,
};
use rcommerce_core::auth::{ApiKeyAuth, Resource, Action};

async fn create_product(
    Extension(auth): Extension<ApiKeyAuth>,
    Json(input): Json<CreateProductInput>,
) -> impl IntoResponse {
    // Check if the API key has write permission for products
    if !auth.can(Resource::Products, Action::Write) {
        return StatusCode::FORBIDDEN.into_response();
    }
    
    // Proceed with product creation
    let product = service.create_product(input).await;
    Json(product).into_response()
}

async fn delete_product(
    Extension(auth): Extension<ApiKeyAuth>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    // Admin permission required for deletion
    if !auth.can(Resource::Products, Action::Admin) {
        return StatusCode::FORBIDDEN.into_response();
    }
    
    service.delete_product(id).await;
    StatusCode::NO_CONTENT.into_response()
}
```

#### Permission Checking Methods

```rust
// Check specific resource and action
auth.can(Resource::Orders, Action::Write)

// Check read access (convenience method)
auth.can_read(Resource::Products)

// Check write access (convenience method)
auth.can_write(Resource::Customers)

// Check admin access (convenience method)
auth.can_admin(Resource::Settings)

// Get all scopes
let scopes = auth.scopes();

// Get API key metadata
let key_id = auth.key_id();
let key_name = auth.key_name();
```

### CLI Commands for API Keys

```bash
# Create API key with specific scopes
rcommerce api-key create --name "Product Manager" --scopes "products:read,products:write"

# Create read-only API key
rcommerce api-key create --name "Reader" --scopes "read"

# Create admin API key
rcommerce api-key create --name "Admin" --scopes "admin"

# Create API key with expiration
rcommerce api-key create --name "Temporary" --scopes "read" --expires "2025-12-31"

# List all API keys (shows metadata, not secrets)
rcommerce api-key list

# Get details for a specific API key
rcommerce api-key get <prefix>

# Revoke API key (soft delete, keeps record)
rcommerce api-key revoke <prefix> --reason "Compromised"

# Delete API key permanently (hard delete)
rcommerce api-key delete <prefix>

# Update API key scopes
rcommerce api-key update <prefix> --scopes "products:read,orders:read,orders:write"
```

#### Using API Keys in Requests

```bash
# Include API key in Authorization header
Authorization: Bearer <prefix>.<secret>

# Example
Authorization: Bearer ak_1a2b3c4d.x9y8z7w6v5u4t3s2r1q0p9o8n7m6l5k4
```

### Configuration

```toml
[security.jwt]
secret = "your-secure-secret-key"  # Change in production!
expiry_hours = 24
refresh_expiry_hours = 168  # 7 days

[security]
api_key_prefix_length = 8
api_key_secret_length = 32
```

---

## Security Considerations

### Authentication

- API key authentication for service-to-service
- JWT tokens for user sessions
- Configurable token expiry
- bcrypt password hashing (cost 12)

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

# API Key management
rcommerce api-key list
rcommerce api-key create --name "My App" --scopes "read,write"
rcommerce api-key get <prefix>
rcommerce api-key revoke <prefix> --reason "Compromised"
rcommerce api-key delete <prefix>

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

## Documentation Guidelines

### Documentation Structure

We maintain documentation in **two locations** that must be kept in sync:

1. **`docs/`** - General project documentation (architecture, design specs, guides)
2. **`docs-website/`** - User-facing documentation website (MkDocs format)

### When Updating Documentation

**ALWAYS update both locations when making documentation changes:**

```bash
# Example: Updating authentication documentation
# 1. Update technical spec in docs/
vim docs/api/01-api-design.md

# 2. Update user-facing docs in docs-website/
vim docs-website/docs/api-reference/authentication.md

# 3. Update this file if adding new patterns
vim AGENTS.md
```

### Documentation Types

| Location | Purpose | Audience |
|----------|---------|----------|
| `docs/architecture/` | Design decisions, technical specs | Developers |
| `docs/api/` | API design specifications | Developers |
| `docs/deployment/` | Deployment guides | DevOps |
| `docs-website/docs/api-reference/` | API usage docs | API users |
| `docs-website/docs/getting-started/` | Quick start guides | New users |
| `docs-website/docs/deployment/` | Deployment tutorials | Users |
| `README.md` | Project overview | Everyone |
| `AGENTS.md` | Agent guidelines | AI agents |

### Adding New Documentation

1. **Technical docs** (for developers): Add to `docs/`
2. **User docs** (for API users): Add to `docs-website/docs/`
3. **Update both** when changing:
   - API endpoints or authentication
   - CLI commands
   - Configuration options
   - Deployment procedures

### Documentation Format

- Use Markdown for all documentation
- Include code examples
- Keep language consistent between docs
- Add table of contents for long docs

---

## Documentation

- `docs/` - General documentation (technical specs, architecture)
- `docs-website/` - User-facing documentation website
- `README.md` - Quick start guide
- `CONTRIBUTING.md` - Contribution guidelines
- `SECURITY.md` - Security policy

---

## Repository

- **URL**: https://gitee.com/captainjez/gocart
- **GitHub Mirror**: https://github.com/creativebastard/rcommerce
- **Primary Branch**: `master`
- **License**: Dual-licensed (AGPL-3.0 / Commercial)

---

## Agent Guidelines

### Git Workflow

**When committing changes:**
1. Always commit with a descriptive message
2. **Push to both remotes:**
   ```bash
   git push origin master    # Gitee
   git push github master:main   # GitHub
   ```

### Documentation Updates

**CRITICAL: When making ANY documentation changes, you MUST rebuild the documentation site.**

1. Update the relevant markdown files in `docs/` and/or `docs-website/docs/`
2. **ALWAYS rebuild the documentation site:**
   ```bash
   cd docs-website
   ./build.sh              # Or: mkdocs build --clean
   ```
3. Create the deployment archive:
   ```bash
   tar -czf site.tar.gz site/
   ```
4. Commit BOTH the source changes AND the rebuilt `site.tar.gz`
5. Push to both Gitee and GitHub

**Note:** The documentation site uses MkDocs with the Material theme and i18n plugin for Chinese/English support. The build script handles dependency installation automatically.

---

*This file should be updated when significant architectural changes occur.*
