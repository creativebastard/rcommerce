# R Commerce - Headless E-Commerce Platform

A high-performance, Rust-based headless e-commerce platform designed for multi-platform deployment and enterprise-scale operations.

## 🚀 Features

- **Headless Architecture**: API-first design for maximum flexibility
- **Multi-Platform Support**: FreeBSD (Jails), Linux (Systemd/Docker), macOS (LaunchDaemon)
- **Performance**: Sub-10ms API response, 10-50MB memory footprint, ~20MB binary
- **Database Flexibility**: PostgreSQL, MySQL, SQLite support via SQLx
- **Extensible Plugin System**: Payment gateways, shipping providers, storage backends
- **Enterprise Ready**: Audit logs, API key auth, rate limiting, comprehensive logging

## 📦 Project Structure

```
crates/
├── rcommerce-core/     # Core library - models, traits, config, repositories
├── rcommerce-api/      # HTTP API server (Axum)
└── rcommerce-cli/      # Command-line management tool
```

### Core Modules

- **Models**: Product, Customer, Order, Address, Payment, etc.
- **Config**: Multi-format configuration (TOML, env vars)
- **Traits**: Repository pattern, plugin architecture
- **Repositories**: PostgreSQL repository implementations
- **Error Handling**: Comprehensive error types with HTTP mapping

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   API Layer     │───▶│  Service Layer   │───▶│ Repository Layer│
│  (Axum/Tokio)   │    │  (Business Logic)│    │  (PostgreSQL)   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                             │
                             ▼
                    ┌──────────────────┐
                    │   Plugin System  │
                    │ (Payments, Ship, │
                    │  Storage, etc.)  │
                    └──────────────────┘
```

## 🛠️ Development

### Prerequisites

- Rust 1.75+
- PostgreSQL 14+ (or MySQL/SQLite)
- Node.js (for client testing)

### Building

```bash
# Build all crates
cargo build --workspace

# Build release optimized binary
cargo build --release

# Run tests
cargo test --workspace

# Check without building
cargo check --workspace
```

### Configuration

Create a `config.toml` file:

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
password = "yourpassword"
pool_size = 20

[logging]
level = "info"
format = "Json"

[media]
storage_type = "Local"
local_path = "./uploads"
```

Run with:

```bash
export RCOMMERCE_CONFIG=./config.toml
cargo run --bin rcommerce
```

## 🗄️ Database Setup

### PostgreSQL

```bash
# Create database
createdb rcommerce

# Run migrations (manual for now)
psql rcommerce < crates/rcommerce-core/migrations/001_initial_schema.sql
```

### Schema

Core tables:
- `products`, `product_variants`, `product_images`
- `customers`, `addresses`
- `orders`, `order_items`, `fulfillments`
- `payments`
- `audit_logs`
- `api_keys`

## 🔌 Plugin System

### Payment Gateways
- Stripe
- Airwallex
- PayPal

### Shipping Providers
- ShipStation
- Dianxiaomi ERP

### Storage Backends
- Local filesystem
- S3-compatible (AWS, MinIO)
- Google Cloud Storage
- Azure Blob Storage

## 📊 Performance Targets

- **Binary Size**: ~20MB (release build)
- **Memory Usage**: 10-50MB runtime
- **API Response**: Sub-10ms average
- **Concurrent Users**: 10,000+ per instance

## 🧪 Testing

```bash
# Unit tests
cargo test --lib

# Integration tests (require test DB)
cargo test --test 'integration'

# Run with test coverage
cargo tarpaulin --workspace
```

## 🚦 Roadmap

### Phase 0: Foundation ✅
- ✅ Core data models
- ✅ Repository pattern
- ✅ Configuration system
- ✅ Error handling
- ✅ Database migrations
- ✅ Basic repositories

### Phase 1: MVP (Current)
- API server setup (Axum)
- Product CRUD endpoints
- Customer management
- Order lifecycle
- Basic authentication

### Phase 2: Core E-Commerce
- Payment integration
- Shipping calculation
- Tax calculation
- Email notifications
- Inventory management

### Phase 3: Advanced Features
- Multi-tenancy
- Plugin system
- Cache layer
- Search (Meilisearch/Typesense)
- Analytics

## 📚 Documentation

Full documentation is available at **[docs.rcommerce.app](https://docs.rcommerce.app)**

- [API Reference](https://docs.rcommerce.app/api/)
- [Architecture](https://docs.rcommerce.app/architecture/)
- [Deployment](https://docs.rcommerce.app/deployment/)
- [Plugins](https://docs.rcommerce.app/plugins/)

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

Quick start:
1. Read the [CLA](CLA.md) - all contributors must sign
2. Fork the repository
3. Create feature branch (`git checkout -b feature/amazing-feature`)
4. Commit changes (`git commit -m 'Add amazing feature'`)
5. Push to branch (`git push origin feature/amazing-feature`)
6. Open Pull Request

## 📄 License

R Commerce is dual-licensed:

- **[AGPL-3.0](LICENSE-AGPL)** - Free for open source and non-commercial use
- **[Commercial License](LICENSE-COMMERCIAL)** - For proprietary use, premium support, and white-label rights

See [LICENSE.md](LICENSE.md) for details.

### Quick License Guide

| Use Case | License Required |
|----------|-----------------|
| Personal projects | AGPL-3.0 (free) |
| Open source SaaS | AGPL-3.0 (free) |
| Commercial SaaS with modifications kept private | Commercial License |
| Enterprise with premium support | Commercial License |
| White-label/OEM | Commercial License |

Contact [sales@rcommerce.app](mailto:sales@rcommerce.app) for commercial licensing.

## 🆘 Support

- GitHub Issues: [Issue Tracker](https://gitee.com/captainjez/gocart/issues)
- Documentation: [docs.rcommerce.app](https://docs.rcommerce.app)
- Email: support@rcommerce.app

---

**R Commerce** - Headless e-commerce redefined with Rust performance and reliability.