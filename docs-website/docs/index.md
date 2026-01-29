---
title: Home
hide:
  - navigation
  - toc
---

# R Commerce

<h2 style="font-weight: 300; margin-top: -0.5em;">High-Performance Headless E-Commerce Platform</h2>

<div style="text-align: center; padding: 2em 0;">
  <img src="assets/hero-diagram.svg" alt="R Commerce Architecture" style="max-width: 800px; width: 100%;">
</div>

## Why R Commerce?

<div class="grid cards" markdown>

- **Blazing Fast**

  ---

  Sub-10ms API responses with Rust's zero-cost abstractions. Handle 10,000+ concurrent users per instance.

- **Memory Safe**

  ---

  Rust's ownership model eliminates entire classes of bugs. No garbage collection pauses, no memory leaks.

- **Headless Architecture**

  ---

  API-first design powers any frontend. React, Vue, mobile apps, IoT devices - use what you love.

- **Multi-Database**

  ---

  PostgreSQL, MySQL, or SQLite. Choose the right database for each deployment scenario.

- **6 Payment Gateways**

  ---

  Stripe, PayPal, WeChat Pay, AliPay, Airwallex, Braintree included. Easy to add more.

- **Global Shipping**

  ---

  Multi-carrier support with real-time rates, label generation, and tracking integration.

</div>

## Architecture Overview

```mermaid
graph TB
    subgraph "Frontend Layer"
        WEB[Web App]
        MOBILE[Mobile App]
        POS[POS System]
    end
    
    subgraph "API Layer"
        API[REST API<br/>Axum Framework]
        WS[WebSocket<br/>Real-time]
        GQL[GraphQL<br/>Optional]
    end
    
    subgraph "Service Layer"
        OS[Order Service]
        PS[Product Service]
        PAYS[Payment Service]
        SS[Shipping Service]
    end
    
    subgraph "Data Layer"
        DB[(PostgreSQL/MySQL)]
        REDIS[(Redis Cache)]
    end
    
    WEB --> API
    MOBILE --> API
    POS --> API
    
    API --> OS
    API --> PS
    API --> PAYS
    API --> SS
    
    OS --> DB
    PS --> DB
    PAYS --> DB
    SS --> DB
    
    API --> REDIS
    WS --> REDIS
```

## Core Features

| Feature | Description | Status |
|---------|-------------|--------|
| **Product Management** | Simple, Variable, Subscription, Digital, Bundle products | Complete |
| **Order Management** | Full lifecycle with editing capabilities | Complete |
| **Payment Processing** | 6 gateways with fraud detection | Complete |
| **Shipping** | Multi-carrier with real-time rates | Complete |
| **Subscriptions** | Recurring billing with dunning management | Complete |
| **Notifications** | Email, SMS, Push, Webhooks | Complete |
| **Redis Caching** | Session, Rate Limit, Job Queue | Complete |
| **WebSocket** | Real-time updates and pub/sub | Complete |

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/creativebastard/rcommerce.git
cd rcommerce

# Build the project
cargo build --release

# Run with default configuration
./target/release/rcommerce-server
```

### Docker Deployment

```bash
# Start with Docker Compose
docker-compose up -d

# Services available:
# - API: http://localhost:8080
# - Database: PostgreSQL on port 5432
# - Redis: port 6379
```

## Documentation

<div class="grid cards" markdown>

- **Getting Started**

  ---

  Quick start guide, installation instructions, and initial configuration.

  [Get Started](getting-started/quickstart.md)

- **API Reference**

  ---

  Complete REST API documentation with examples and error codes.

  [View API](api-reference/index.md)

- **Payment Gateways**

  ---

  Configure Stripe, Airwallex, WeChat Pay, AliPay, and more.

  [Configure Payments](payment-gateways/index.md)

- **Deployment**

  ---

  Production deployment guides for Docker, Kubernetes, and bare metal.

  [Deploy](deployment/index.md)

- **Operations**

  ---

  Scaling, monitoring, backups, reverse proxies, and security.

  [Operations](operations/index.md)

- **Development**

  ---

  Developer guide, CLI reference, and configuration options.

  [Develop](development/index.md)

- **Migration**

  ---

  Migrate from Shopify, WooCommerce, Magento, or Medusa.

  [Migrate](migration/index.md)

- **Architecture**

  ---

  Deep dive into system design, data models, and integration patterns.

  [Architecture](architecture/overview.md)

</div>

## Example API Usage

### Create a Product

```bash
curl -X POST http://localhost:8080/api/v1/products \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "title": "Premium Widget",
    "product_type": "simple",
    "price": 29.99,
    "inventory_quantity": 100
  }'
```

### Create an Order

```bash
curl -X POST http://localhost:8080/api/v1/orders \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "customer_id": "uuid-here",
    "items": [{
      "product_id": "product-uuid",
      "quantity": 2
    }],
    "shipping_address": { ... }
  }'
```

## Supported Platforms

| Platform | Deployment Method |
|----------|-------------------|
| Linux | systemd, Docker, Kubernetes |
| macOS | launchd, Docker |
| FreeBSD | rc.d, iocage jails |
| Docker | Docker Compose, Swarm |
| Kubernetes | Helm charts, Operators |

## Performance Benchmarks

| Metric | Value |
|--------|-------|
| Binary Size | ~20 MB |
| Memory Usage | 10-50 MB |
| API Response Time | < 10ms avg |
| Concurrent Users | 10,000+ per instance |
| Cold Start | < 1 second |

## Roadmap

- [x] Phase 0: Project Foundation
- [x] Phase 1: Core Infrastructure
- [x] Phase 2: Order & Product System
- [x] Phase 3: Payment & Shipping
- [x] Phase 4: Real-time Features
- [ ] Phase 5: Advanced Features (In Progress)

---

<div style="text-align: center; padding: 2em 0;">

**Ready to build?** [Get Started](getting-started/quickstart.md)

</div>

---

<p style="text-align: center;">
Built with care in Rust • Dual Licensed (AGPL-3.0 / Commercial) • 
<a href="https://github.com/creativebastard/rcommerce">GitHub</a>
</p>
