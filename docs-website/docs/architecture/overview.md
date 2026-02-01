# Architectural Overview

## Project Vision

**R Commerce** is a lightweight, high-performance headless ecommerce platform built entirely in Rust. It provides a complete API-first solution for businesses that need a fast, reliable, and self-contained ecommerce backend without the bloat of traditional monolithic platforms.

## Why Rust?

### Performance & Resource Efficiency
- **Memory Safety Without Garbage Collection**: Rust's ownership model eliminates runtime garbage collection pauses, providing consistent low-latency performance critical for ecommerce transactions
- **Zero-Cost Abstractions**: High-level features compile down to efficient machine code, ensuring excellent performance without sacrificing developer experience
- **Predictable Resource Usage**: Single-digit millisecond response times with minimal memory footprint (typically 10-50MB per instance vs 500MB-2GB for JVM/Node alternatives)

### Reliability & Safety
- **Compile-Time Guarantees**: Prevent entire classes of bugs (null pointer dereferences, buffer overflows, data races) before code reaches production
- **Type System**: Strong static typing catches errors early, reducing runtime failures during critical payment operations
- **Fearless Concurrency**: Leverage multi-core processors safely for high-throughput order processing and API handling

### Operational Benefits
- **Single Binary Deployment**: Compile to a single executable (~20MB) - no runtime dependencies, container-friendly
- **Predictable Performance**: Consistent CPU/memory usage patterns simplify capacity planning and autoscaling
- **Long-Term Maintenance**: Rust's stability guarantees and backward compatibility reduce technical debt accumulation

## Why Headless?

### Architectural Advantages

1. **Frontend Agnostic** - Use any frontend technology (React, Vue, mobile apps, IoT devices) without backend constraints
2. **Microservices Ready** - Integrate with existing systems or scale components independently
3. **API-First Design** - Every feature is accessible programmatically, enabling automation and integration
4. **Reduced Attack Surface** - No admin UI means fewer attack vectors; security through simplicity
5. **Multi-Channel Commerce** - Single backend powers web, mobile, POS, marketplaces simultaneously

### Total Cost of Ownership

While headless requires frontend development, it reduces:
- Hosting costs (lighter backend)
- Maintenance overhead (less code to maintain)
- Security incidents (smaller attack surface)
- Upgrade complexity (no UI migrations)

## Core Design Principles

### 1. **Modularity Over Monolith**
- Plugin-based architecture for payments, shipping, and notifications
- Feature flags to enable/disable functionality
- Stratified design: Core → Services → API → Plugins

### 2. **Configuration as Code**
- All configuration via TOML/JSON files or environment variables
- No database configuration tables (except dynamic business data)
- Version-controlled configuration enables GitOps workflows

### 3. **API Completeness**
- 100% of functionality exposed via REST APIs
- No hidden admin-only features
- Comprehensive webhooks for external integration

### 4. **PostgreSQL Optimized**
- Designed for PostgreSQL 14+ with advanced features
- Migration system for schema management
- Performance optimized queries

### 5. **Observability Built-In**
- Structured logging (JSON format)
- Prometheus metrics endpoints
- Distributed tracing support
- Health check endpoints

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        API Layer                             │
│  (REST - Axum/Tokio)                                        │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                    Service Layer                             │
│  OrderService, ProductService, PaymentService, etc         │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                   Core Domain Layer                          │
│  Entities, Value Objects, Business Logic                   │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                Data Access Layer                             │
│  Repository Pattern, PostgreSQL                            │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│              Database/External Services                      │
│  PostgreSQL, Redis, Payment Gateways, Shipping APIs        │
└─────────────────────────────────────────────────────────────┘
```

## Key Components

### API Gateway (Axum)
- HTTP request routing and validation
- Authentication/authorization middleware
- Rate limiting and request logging
- CORS and security headers
- WebSocket support for real-time updates

### Core Services
- **Order Management**: Lifecycle, status transitions, fraud detection
- **Product Catalog**: Categories, variants, inventory management
- **Customer Management**: Profiles, addresses, order history
- **Payment Processing**: Multi-gateway orchestration
- **Shipping Integration**: Label generation, tracking
- **Webhook Dispatcher**: Event-driven notifications

### Plugin System
- Dynamic loading of payment/shipping providers
- Trait-based interface for extensibility
- Configuration-driven provider selection
- Sandboxed execution environment

### Background Workers
- Async task processing using Tokio
- Order fulfillment automation
- Payment status synchronization
- Email notification sending
- Inventory synchronization

## Performance Targets

| Metric | Target |
|--------|--------|
| API Response Time | P50 < 10ms, P99 < 50ms |
| Throughput | > 10,000 requests/second on 4-core VM |
| Memory Usage | < 50MB per instance at rest |
| Startup Time | < 1 second |
| Order Processing | > 1,000 orders/second sustained |

## Deployment Characteristics

- **Single Binary**: ~20MB executable
- **Container Image**: ~50MB (alpine-based)
- **Configuration**: Single TOML file + environment variables
- **Horizontal Scaling**: Stateless design, scale via load balancers
- **High Availability**: Multi-instance deployment with shared database

## Comparison with Alternatives

| Feature | R Commerce | WooCommerce | Shopify (Headless) | Medusa |
|---------|------------|-------------|-------------------|--------|
| Language | Rust | PHP | Ruby/Node | Node.js |
| Performance | Excellent | Moderate | Good | Good |
| Self-Hosted | Yes | Yes | No | Yes |
| Resource Usage | Very Low | High | N/A | Moderate |
| Complexity | Low | High (with plugins) | Medium | Medium |
| API Completeness | 100% | Partial | 100% | 100% |

## Project Structure

```
crates/
├── rcommerce-core/     # Core library - models, traits, config, repositories
├── rcommerce-api/      # HTTP API server (Axum)
└── rcommerce-cli/      # Command-line management tool
```

### Core Modules

- **Models**: Product, Customer, Order, Address, Payment, API Keys, etc.
- **Config**: Multi-format configuration (TOML, env vars)
- **Traits**: Repository pattern, plugin architecture
- **Repositories**: PostgreSQL repository implementations
- **Error Handling**: Comprehensive error types with HTTP mapping
- **WebSocket**: Real-time updates and notifications
- **Cache**: Redis and in-memory caching support

## Next Steps

Continue reading:
- [Data Model](./data-model.md) - Core entities and relationships
- [Database Abstraction](./database-abstraction.md) - Repository pattern and data access
- [Order Management](./order-management.md) - Order lifecycle and fulfillment
- [Notifications](./notifications.md) - Email and webhook system
- [Media Storage](./media-storage.md) - File upload and storage backends
