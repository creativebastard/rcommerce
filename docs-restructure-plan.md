# R Commerce Documentation Restructuring Plan

## Executive Summary

This document outlines a comprehensive restructuring plan for the R Commerce documentation website (`docs-website/`). The current structure has several organizational issues that need to be addressed to improve discoverability, maintainability, and user experience.

---

## 1. Current State Analysis

### 1.1 Existing Documentation Structure

```
docs-website/docs/
├── index.md (+ .zh.md)                    # Home page
├── getting-started/
│   ├── quickstart.md (+ .zh.md)
│   ├── installation.md (+ .zh.md)
│   └── configuration.md (+ .zh.md)
├── api-reference/
│   ├── index.md (+ .zh.md)                # API Overview
│   ├── authentication.md (+ .zh.md)
│   ├── scopes.md (+ .zh.md)
│   ├── errors.md (+ .zh.md)
│   ├── products.md (+ .zh.md)
│   ├── orders.md (+ .zh.md)
│   ├── customers.md (+ .zh.md)
│   ├── cart.md (+ .zh.md)
│   ├── coupons.md (+ .zh.md)
│   ├── payments.md (+ .zh.md)
│   ├── statistics.md (+ .zh.md)
│   ├── subscriptions.md (+ .zh.md)
│   ├── webhooks.md (+ .zh.md)
│   └── graphql.md (+ .zh.md)
├── payment-gateways/
│   ├── index.md (+ .zh.md)
│   ├── stripe.md (+ .zh.md)
│   ├── airwallex.md (+ .zh.md)
│   ├── alipay.md (+ .zh.md)
│   ├── wechatpay.md (+ .zh.md)
│   └── webhooks.md (+ .zh.md)
├── deployment/
│   ├── index.md (+ .zh.md)
│   ├── docker.md (+ .zh.md)
│   ├── binary.md (+ .zh.md)
│   ├── kubernetes.md (+ .zh.md)
│   ├── redis.md (+ .zh.md)
│   ├── scaling.md (+ .zh.md)
│   ├── linux/
│   │   ├── systemd.md (+ .zh.md)
│   │   └── manual.md (+ .zh.md)
│   ├── freebsd/
│   │   ├── standalone.md (+ .zh.md)
│   │   ├── jails.md (+ .zh.md)
│   │   └── rc.d.md (+ .zh.md)
│   ├── macos/
│   │   └── launchd.md (+ .zh.md)
│   ├── reverse-proxies/
│   │   ├── caddy.md (+ .zh.md)
│   │   ├── nginx.md (+ .zh.md)
│   │   ├── haproxy.md (+ .zh.md)
│   │   └── traefik.md (+ .zh.md)
│   ├── monitoring/
│   │   └── monitoring.md (+ .zh.md)
│   ├── backups/
│   │   └── backups.md (+ .zh.md)
│   └── security/
│       └── security.md (+ .zh.md)
├── architecture/
│   ├── overview.md (+ .zh.md)
│   ├── data-model.md (+ .zh.md)
│   ├── database-abstraction.md (+ .zh.md)
│   ├── order-management.md (+ .zh.md)
│   ├── notifications.md (+ .zh.md)
│   └── media-storage.md (+ .zh.md)
├── guides/                                  # PROBLEMATIC - Mixed content
│   ├── api-keys.md (+ .zh.md)             # Should be in api-reference/
│   ├── dunning.md (+ .zh.md)              # Should be in api-reference/
│   └── shipping.md (+ .zh.md)             # Should be in api-reference/
├── development/
│   ├── index.md (+ .zh.md)
│   ├── local-setup.md (+ .zh.md)
│   ├── contributing.md (+ .zh.md)
│   ├── testing.md (+ .zh.md)
│   └── cli-reference.md (+ .zh.md)
├── migration/
│   ├── index.md (+ .zh.md)
│   ├── shopify.md (+ .zh.md)
│   ├── woocommerce.md (+ .zh.md)
│   ├── magento.md (+ .zh.md)
│   └── medusa.md (+ .zh.md)
└── includes/
    └── abbreviations.md
```

### 1.2 Translation Status Summary

| Section | English Files | Chinese Files | Coverage |
|---------|--------------|---------------|----------|
| Getting Started | 3 | 3 | 100% |
| API Reference | 13 | 13 | 100% |
| Payment Gateways | 6 | 6 | 100% |
| Deployment | 15 | 15 | 100% |
| Architecture | 6 | 6 | 100% |
| Guides | 3 | 3 | 100% |
| Development | 5 | 5 | 100% |
| Migration | 5 | 5 | 100% |
| **Total** | **56** | **56** | **100%** |

### 1.3 Identified Issues

#### Issue 1: API Content in Guides Section
The `guides/` directory contains API endpoint documentation that should be in `api-reference/`:
- `guides/shipping.md` - Contains extensive API endpoint documentation (lines 325-385)
- `guides/dunning.md` - Contains API endpoint documentation (lines 316-461)
- `guides/api-keys.md` - Contains API usage examples but is more of a guide

#### Issue 2: Missing API Reference Documentation
Based on analysis of the actual API routes in `crates/rcommerce-api/src/routes/`, the following API modules are missing dedicated documentation:

| Module | Status | File Location |
|--------|--------|---------------|
| Auth | ❌ Missing | `api-reference/auth.md` |
| Admin | ❌ Missing | `api-reference/admin.md` |
| Dunning | ❌ Missing | `api-reference/dunning.md` |
| Shipping | ❌ Missing | `api-reference/shipping.md` |
| Addresses | ❌ Missing | `api-reference/addresses.md` |
| Inventory | ❌ Missing | `api-reference/inventory.md` |
| Fulfillment | ❌ Missing | `api-reference/fulfillment.md` |

#### Issue 3: Inconsistent Organization
- No clear separation between API reference (endpoint docs) and guides (how-to content)
- No dedicated SDK/Integration section for language-specific examples
- Architecture docs are mixed with technical implementation details

#### Issue 4: Missing Content
- No comprehensive getting started tutorial
- No SDK documentation for JavaScript/TypeScript, Python, Rust, PHP
- No webhook event reference
- No error handling guide

---

## 2. Proposed New Structure

### 2.1 Top-Level Navigation

```
nav:
  - Home: index.md
  
  - Getting Started:
    - Overview: getting-started/index.md
    - Quick Start: getting-started/quickstart.md
    - Installation: getting-started/installation.md
    - Configuration: getting-started/configuration.md
    - First Steps Tutorial: getting-started/first-steps.md  # NEW
  
  - API Reference:
    - Overview: api-reference/index.md
    - Authentication: api-reference/authentication.md
    - Scopes & Permissions: api-reference/scopes.md
    - Error Codes: api-reference/errors.md
    - Pagination & Filtering: api-reference/pagination.md  # NEW
    - Rate Limiting: api-reference/rate-limiting.md        # NEW
    - Products: api-reference/products.md
    - Orders: api-reference/orders.md
    - Customers: api-reference/customers.md
    - Addresses: api-reference/addresses.md                 # NEW
    - Cart: api-reference/cart.md
    - Coupons: api-reference/coupons.md
    - Payments: api-reference/payments.md
    - Subscriptions: api-reference/subscriptions.md
    - Dunning: api-reference/dunning.md                     # NEW
    - Shipping: api-reference/shipping.md                   # NEW
    - Inventory: api-reference/inventory.md                 # NEW
    - Fulfillment: api-reference/fulfillment.md             # NEW
    - Statistics: api-reference/statistics.md
    - Admin: api-reference/admin.md                         # NEW
    - Webhooks: api-reference/webhooks.md
    - GraphQL: api-reference/graphql.md
  
  - SDKs & Integration:
    - Overview: sdks/index.md                               # NEW
    - JavaScript/TypeScript: sdks/javascript.md             # NEW
    - Python: sdks/python.md                                # NEW
    - Rust: sdks/rust.md                                    # NEW
    - PHP: sdks/php.md                                      # NEW
    - Go: sdks/go.md                                        # NEW
    - Webhook Integration: sdks/webhooks.md                 # NEW
  
  - Payment Gateways:
    - Overview: payment-gateways/index.md
    - Stripe: payment-gateways/stripe.md
    - Airwallex: payment-gateways/airwallex.md
    - AliPay: payment-gateways/alipay.md
    - WeChat Pay: payment-gateways/wechatpay.md
    - Webhooks: payment-gateways/webhooks.md
  
  - Guides:
    - Overview: guides/index.md                             # NEW
    - API Keys: guides/api-keys.md
    - Working with Webhooks: guides/webhooks.md             # NEW
    - Error Handling: guides/error-handling.md              # NEW
    - Testing: guides/testing.md                            # NEW
    - Shipping Configuration: guides/shipping.md            # MODIFIED
    - Dunning Management: guides/dunning.md                 # MODIFIED
  
  - Deployment:
    - Overview: deployment/index.md
    - Docker: deployment/docker.md
    - Binary: deployment/binary.md
    - Kubernetes: deployment/kubernetes.md
    - Linux:
      - systemd: deployment/linux/systemd.md
      - Manual: deployment/linux/manual.md
    - FreeBSD:
      - Standalone: deployment/freebsd/standalone.md
      - Jails (iocage): deployment/freebsd/jails.md
      - rc.d: deployment/freebsd/rc.d.md
    - macOS:
      - launchd: deployment/macos/launchd.md
    - Reverse Proxies:
      - Caddy: deployment/reverse-proxies/caddy.md
      - Nginx: deployment/reverse-proxies/nginx.md
      - HAProxy: deployment/reverse-proxies/haproxy.md
      - Traefik: deployment/reverse-proxies/traefik.md
    - Scaling: deployment/scaling.md
    - Monitoring: deployment/monitoring.md                  # MOVED
    - Backups: deployment/backups.md                        # MOVED
    - Security: deployment/security.md                      # MOVED
    - Redis: deployment/redis.md
  
  - Architecture:
    - Overview: architecture/index.md
    - Data Model: architecture/data-model.md
    - Database Abstraction: architecture/database-abstraction.md
    - Order Management: architecture/order-management.md
    - Notifications: architecture/notifications.md
    - Media Storage: architecture/media-storage.md
    - Payment System: architecture/payment-system.md        # NEW
    - Subscription Lifecycle: architecture/subscriptions.md # NEW
  
  - Development:
    - Overview: development/index.md
    - Local Setup: development/local-setup.md
    - Contributing: development/contributing.md
    - Testing: development/testing.md
    - CLI Reference: development/cli-reference.md
  
  - Migration:
    - Overview: migration/index.md
    - Shopify: migration/shopify.md
    - WooCommerce: migration/woocommerce.md
    - Magento: migration/magento.md
    - Medusa: migration/medusa.md
```

---

## 3. File Mapping (Old Location → New Location)

### 3.1 Files to Move

| Current Path | New Path | Action | Notes |
|--------------|----------|--------|-------|
| `guides/shipping.md` | `guides/shipping-configuration.md` | Rename | Keep as guide, remove API content |
| `guides/dunning.md` | `guides/dunning-management.md` | Rename | Keep as guide, remove API content |
| `guides/api-keys.md` | `guides/api-keys.md` | Keep | Already appropriate location |
| `deployment/monitoring/monitoring.md` | `deployment/monitoring.md` | Move | Flatten structure |
| `deployment/backups/backups.md` | `deployment/backups.md` | Move | Flatten structure |
| `deployment/security/security.md` | `deployment/security.md` | Move | Flatten structure |

### 3.2 Files to Create (New API Reference Docs)

| New File | Purpose | Source Material |
|----------|---------|-----------------|
| `api-reference/auth.md` | Auth endpoints | `crates/rcommerce-api/src/routes/auth.rs` |
| `api-reference/admin.md` | Admin endpoints | `crates/rcommerce-api/src/routes/admin.rs` |
| `api-reference/dunning.md` | Dunning API | `crates/rcommerce-api/src/routes/dunning.rs` |
| `api-reference/shipping.md` | Shipping API | Extract from `guides/shipping.md` |
| `api-reference/addresses.md` | Address management | `crates/rcommerce-core/src/models/address.rs` |
| `api-reference/inventory.md` | Inventory API | To be implemented |
| `api-reference/fulfillment.md` | Fulfillment API | To be implemented |
| `api-reference/pagination.md` | Pagination guide | New content |
| `api-reference/rate-limiting.md` | Rate limiting | New content |

### 3.3 Files to Create (New SDK Docs)

| New File | Purpose |
|----------|---------|
| `sdks/index.md` | SDK overview and comparison |
| `sdks/javascript.md` | JavaScript/TypeScript SDK |
| `sdks/python.md` | Python SDK |
| `sdks/rust.md` | Rust SDK |
| `sdks/php.md` | PHP SDK |
| `sdks/go.md` | Go SDK |
| `sdks/webhooks.md` | Webhook integration guide |

### 3.4 Files to Create (New Guide Docs)

| New File | Purpose |
|----------|---------|
| `guides/index.md` | Guides overview |
| `guides/webhooks.md` | Working with webhooks |
| `guides/error-handling.md` | Error handling patterns |
| `guides/testing.md` | Testing your integration |

### 3.5 Files to Create (New Architecture Docs)

| New File | Purpose |
|----------|---------|
| `architecture/payment-system.md` | Payment system architecture |
| `architecture/subscriptions.md` | Subscription lifecycle |

### 3.6 Files to Create (Getting Started)

| New File | Purpose |
|----------|---------|
| `getting-started/index.md` | Getting started overview |
| `getting-started/first-steps.md` | Step-by-step tutorial |

---

## 4. Detailed Content Changes

### 4.1 Shipping Documentation Split

**Current:** `guides/shipping.md` (413 lines) - Mixed guide + API reference

**Proposed Split:**

1. **`guides/shipping-configuration.md`** (Guide - ~250 lines)
   - Overview of shipping system
   - Supported carriers
   - Configuration in config.toml
   - Weight calculations (conceptual)
   - Shipping zones and rules (conceptual)
   - Best practices
   - Troubleshooting

2. **`api-reference/shipping.md`** (API Reference - ~200 lines)
   - `POST /api/v1/shipping/rates` - Get shipping rates
   - `POST /api/v1/shipping/shipments` - Create shipment
   - `GET /api/v1/shipping/tracking/:tracking_number` - Track shipment
   - Request/response schemas
   - Error codes

### 4.2 Dunning Documentation Split

**Current:** `guides/dunning.md` (855 lines) - Mixed guide + API reference

**Proposed Split:**

1. **`guides/dunning-management.md`** (Guide - ~500 lines)
   - Introduction to dunning
   - Configuration options
   - Email template customization
   - Best practices
   - FAQ
   - Example workflows

2. **`api-reference/dunning.md`** (API Reference - ~250 lines)
   - `POST /api/v1/admin/dunning/process` - Process dunning
   - `GET /api/v1/admin/dunning/retries` - List pending retries
   - `GET /api/v1/admin/dunning/retries/due` - List due retries
   - `POST /api/v1/admin/dunning/retry/:invoice_id` - Manual retry
   - `GET /api/v1/admin/dunning/config` - Get config
   - `GET /api/v1/admin/dunning/stats` - Get statistics
   - `GET /api/v1/subscriptions/:id/dunning-history` - Get history
   - `POST /api/v1/subscriptions/:id/reset-dunning` - Reset state
   - Request/response schemas

### 4.3 API Keys Documentation

**Current:** `guides/api-keys.md` (632 lines) - Appropriate location, minor updates needed

**Changes:**
- Keep in `guides/` as it's a how-to guide
- Add link to new `api-reference/auth.md` for endpoint details
- Add SDK examples from new SDK docs

---

## 5. Translation Status Tracking

### 5.1 Files with Existing Translations (Keep)

All existing files have Chinese translations (`.zh.md` counterparts). These should be preserved and updated when the English versions change.

### 5.2 New Files Requiring Translation

| English File | Chinese File | Priority |
|--------------|--------------|----------|
| `api-reference/auth.md` | `api-reference/auth.zh.md` | High |
| `api-reference/admin.md` | `api-reference/admin.zh.md` | High |
| `api-reference/dunning.md` | `api-reference/dunning.zh.md` | High |
| `api-reference/shipping.md` | `api-reference/shipping.zh.md` | High |
| `api-reference/addresses.md` | `api-reference/addresses.zh.md` | Medium |
| `api-reference/inventory.md` | `api-reference/inventory.zh.md` | Medium |
| `api-reference/fulfillment.md` | `api-reference/fulfillment.zh.md` | Medium |
| `api-reference/pagination.md` | `api-reference/pagination.zh.md` | High |
| `api-reference/rate-limiting.md` | `api-reference/rate-limiting.zh.md` | High |
| `sdks/index.md` | `sdks/index.zh.md` | High |
| `sdks/javascript.md` | `sdks/javascript.zh.md` | High |
| `sdks/python.md` | `sdks/python.zh.md` | Medium |
| `sdks/rust.md` | `sdks/rust.zh.md` | Medium |
| `sdks/php.md` | `sdks/php.zh.md` | Low |
| `sdks/go.md` | `sdks/go.zh.md` | Low |
| `sdks/webhooks.md` | `sdks/webhooks.zh.md` | High |
| `guides/index.md` | `guides/index.zh.md` | Medium |
| `guides/webhooks.md` | `guides/webhooks.zh.md` | High |
| `guides/error-handling.md` | `guides/error-handling.zh.md` | Medium |
| `guides/testing.md` | `guides/testing.zh.md` | Low |
| `architecture/payment-system.md` | `architecture/payment-system.zh.md` | Medium |
| `architecture/subscriptions.md` | `architecture/subscriptions.zh.md` | Medium |
| `getting-started/index.md` | `getting-started/index.zh.md` | High |
| `getting-started/first-steps.md` | `getting-started/first-steps.zh.md` | High |

### 5.3 Translation Priority Legend

- **High**: Core functionality, frequently accessed
- **Medium**: Important but less frequently accessed
- **Low**: Nice to have, can be translated later

---

## 6. Updated mkdocs.yml Configuration

```yaml
# Page tree
nav:
  - Home: index.md
  
  - Getting Started:
    - Overview: getting-started/index.md
    - Quick Start: getting-started/quickstart.md
    - Installation: getting-started/installation.md
    - Configuration: getting-started/configuration.md
    - First Steps Tutorial: getting-started/first-steps.md
  
  - API Reference:
    - Overview: api-reference/index.md
    - Authentication: api-reference/authentication.md
    - Scopes & Permissions: api-reference/scopes.md
    - Error Codes: api-reference/errors.md
    - Pagination & Filtering: api-reference/pagination.md
    - Rate Limiting: api-reference/rate-limiting.md
    - Products: api-reference/products.md
    - Orders: api-reference/orders.md
    - Customers: api-reference/customers.md
    - Addresses: api-reference/addresses.md
    - Cart: api-reference/cart.md
    - Coupons: api-reference/coupons.md
    - Payments: api-reference/payments.md
    - Subscriptions: api-reference/subscriptions.md
    - Dunning: api-reference/dunning.md
    - Shipping: api-reference/shipping.md
    - Inventory: api-reference/inventory.md
    - Fulfillment: api-reference/fulfillment.md
    - Statistics: api-reference/statistics.md
    - Admin: api-reference/admin.md
    - Webhooks: api-reference/webhooks.md
    - GraphQL: api-reference/graphql.md
  
  - SDKs & Integration:
    - Overview: sdks/index.md
    - JavaScript/TypeScript: sdks/javascript.md
    - Python: sdks/python.md
    - Rust: sdks/rust.md
    - PHP: sdks/php.md
    - Go: sdks/go.md
    - Webhook Integration: sdks/webhooks.md
  
  - Payment Gateways:
    - Overview: payment-gateways/index.md
    - Stripe: payment-gateways/stripe.md
    - Airwallex: payment-gateways/airwallex.md
    - AliPay: payment-gateways/alipay.md
    - WeChat Pay: payment-gateways/wechatpay.md
    - Webhooks: payment-gateways/webhooks.md
  
  - Guides:
    - Overview: guides/index.md
    - API Keys: guides/api-keys.md
    - Working with Webhooks: guides/webhooks.md
    - Error Handling: guides/error-handling.md
    - Testing: guides/testing.md
    - Shipping Configuration: guides/shipping-configuration.md
    - Dunning Management: guides/dunning-management.md
  
  - Deployment:
    - Overview: deployment/index.md
    - Docker: deployment/docker.md
    - Binary: deployment/binary.md
    - Kubernetes: deployment/kubernetes.md
    - Linux:
      - systemd: deployment/linux/systemd.md
      - Manual: deployment/linux/manual.md
    - FreeBSD:
      - Standalone: deployment/freebsd/standalone.md
      - Jails (iocage): deployment/freebsd/jails.md
      - rc.d: deployment/freebsd/rc.d.md
    - macOS:
      - launchd: deployment/macos/launchd.md
    - Reverse Proxies:
      - Caddy: deployment/reverse-proxies/caddy.md
      - Nginx: deployment/reverse-proxies/nginx.md
      - HAProxy: deployment/reverse-proxies/haproxy.md
      - Traefik: deployment/reverse-proxies/traefik.md
    - Scaling: deployment/scaling.md
    - Monitoring: deployment/monitoring.md
    - Backups: deployment/backups.md
    - Security: deployment/security.md
    - Redis: deployment/redis.md
  
  - Architecture:
    - Overview: architecture/index.md
    - Data Model: architecture/data-model.md
    - Database Abstraction: architecture/database-abstraction.md
    - Order Management: architecture/order-management.md
    - Notifications: architecture/notifications.md
    - Media Storage: architecture/media-storage.md
    - Payment System: architecture/payment-system.md
    - Subscription Lifecycle: architecture/subscriptions.md
  
  - Development:
    - Overview: development/index.md
    - Local Setup: development/local-setup.md
    - Contributing: development/contributing.md
    - Testing: development/testing.md
    - CLI Reference: development/cli-reference.md
  
  - Migration:
    - Overview: migration/index.md
    - Shopify: migration/shopify.md
    - WooCommerce: migration/woocommerce.md
    - Magento: migration/magento.md
    - Medusa: migration/medusa.md
```

### 6.1 Updated i18n Nav Translations

Add these translations to the `nav_translations` section in `mkdocs.yml`:

```yaml
nav_translations:
  # Existing translations...
  
  # New sections
  SDKs & Integration: SDK 与集成
  JavaScript/TypeScript: JavaScript/TypeScript
  Python: Python
  Rust: Rust
  PHP: PHP
  Go: Go
  Webhook Integration: Webhook 集成
  
  # New API Reference pages
  Scopes & Permissions: 权限范围
  Pagination & Filtering: 分页与筛选
  Rate Limiting: 速率限制
  Addresses: 地址
  Dunning: 催缴管理
  Shipping: 运输
  Inventory: 库存
  Fulfillment: 履行
  Admin: 管理
  
  # New Guides
  Working with Webhooks: 使用 Webhooks
  Error Handling: 错误处理
  Shipping Configuration: 运输配置
  Dunning Management: 催缴管理
  
  # New Architecture
  Payment System: 支付系统
  Subscription Lifecycle: 订阅生命周期
  
  # New Getting Started
  First Steps Tutorial: 入门教程
```

### 6.2 Updated Redirects

Add these redirects to handle moved files:

```yaml
redirect_maps:
  # Existing redirects...
  
  # Moved guides
  'guides/shipping.md': 'guides/shipping-configuration.md'
  'guides/dunning.md': 'guides/dunning-management.md'
  
  # Flattened deployment structure
  'deployment/monitoring/monitoring.md': 'deployment/monitoring.md'
  'deployment/backups/backups.md': 'deployment/backups.md'
  'deployment/security/security.md': 'deployment/security.md'
  
  # New getting started
  'getting-started.md': 'getting-started/index.md'
```

---

## 7. Implementation Phases

### Phase 1: Foundation (Week 1)
1. Create new directory structure
2. Move and rename existing files
3. Update `mkdocs.yml` navigation
4. Add redirects for moved files

### Phase 2: API Reference Expansion (Week 2)
1. Create `api-reference/auth.md`
2. Create `api-reference/admin.md`
3. Create `api-reference/dunning.md`
4. Create `api-reference/shipping.md`
5. Create `api-reference/pagination.md`
6. Create `api-reference/rate-limiting.md`

### Phase 3: SDK Documentation (Week 3)
1. Create `sdks/index.md`
2. Create `sdks/javascript.md`
3. Create `sdks/python.md`
4. Create `sdks/rust.md`
5. Create `sdks/webhooks.md`

### Phase 4: Guide Improvements (Week 4)
1. Split `guides/shipping.md` → `guides/shipping-configuration.md` + `api-reference/shipping.md`
2. Split `guides/dunning.md` → `guides/dunning-management.md` + `api-reference/dunning.md`
3. Create `guides/index.md`
4. Create `guides/webhooks.md`
5. Create `guides/error-handling.md`

### Phase 5: Architecture & Getting Started (Week 5)
1. Create `getting-started/index.md`
2. Create `getting-started/first-steps.md`
3. Create `architecture/payment-system.md`
4. Create `architecture/subscriptions.md`

### Phase 6: Translations (Week 6-7)
1. Create all `.zh.md` counterparts for new files
2. Update existing translations where content changed

### Phase 7: Review & Launch (Week 8)
1. Review all new documentation
2. Test all links and navigation
3. Build and deploy

---

## 8. Summary of Changes

### Files to Create: 32
- 9 new API reference docs
- 7 new SDK docs
- 4 new guide docs
- 3 new architecture docs
- 2 new getting started docs
- 7 corresponding `.zh.md` translations (minimum)

### Files to Move: 6
- 3 guides (rename)
- 3 deployment docs (flatten structure)

### Files to Update: 5
- `mkdocs.yml` (navigation, translations, redirects)
- `guides/api-keys.md` (add links to new docs)
- `api-reference/index.md` (add new sections)
- `guides/shipping.md` → split content
- `guides/dunning.md` → split content

### Total New Lines of Documentation: ~3,000-4,000

---

## 9. Success Metrics

After restructuring, the documentation should have:

- [ ] Clear separation between API reference and guides
- [ ] 100% coverage of API endpoints in reference docs
- [ ] SDK documentation for at least 3 languages
- [ ] Complete Chinese translations for all high-priority content
- [ ] No broken links or redirects
- [ ] Consistent navigation structure
- [ ] Improved user feedback (fewer support questions about missing docs)

---

*Document Version: 1.0*
*Created: 2026-02-02*
*Status: Draft - Pending Review*
