# R Commerce Codebase Review & TODO List

**Review Date:** 2026-02-07  
**Current Status:** ~90% Implemented, ~10% Stubbed/Mock, 212 Compilation Errors

---

## Executive Summary

The R Commerce codebase has a solid foundation with fully implemented core models, repositories, and services. However, there are **212 compilation errors** preventing the project from building, and several API routes return mock data instead of using the real services. The documentation is comprehensive but has gaps where features are documented but not implemented.

### Key Metrics

| Category | Status |
|----------|--------|
| Models | ✅ 100% Complete (7 models) |
| Repositories | ✅ 100% Complete (6 repos) |
| Services | ✅ 100% Complete (9 services) |
| API Routes | 🟡 60% Working, 40% Mock/Stub |
| Database Schema | 🟡 85% Complete (some mismatches) |
| Compilation | ❌ 212 Errors |
| Documentation | 🟡 Comprehensive but some gaps |

---

## Critical Priority (Blocking Release)

### 1. Fix 212 Compilation Errors ⏰ ~2 hours
**Status:** ❌ BLOCKING  
**Files:** Various across crates

**Error Breakdown:**
- E0277: 78 errors (Trait bounds - async Send/Sync)
- E0308: 47 errors (Type mismatches)
- E0599: 22 errors (No method found)
- E0609: 17 errors (Field access issues)
- Others: ~48 errors

**Impact:** Cannot build or run the application.

---

### 2. Fix Database Schema Mismatches
**Status:** ⚠️ HIGH PRIORITY

#### 2.1 Address Table - Default Flags Mismatch
**Issue:** Model expects `is_default_shipping` and `is_default_billing`, table only has `is_default`

**Fix Options:**
- Option A: Add migration to add two boolean columns
- Option B: Update model to use single `is_default` field

**Recommendation:** Option A - supports both shipping and billing defaults

```sql
-- Migration needed
ALTER TABLE addresses ADD COLUMN is_default_shipping BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE addresses ADD COLUMN is_default_billing BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE addresses DROP COLUMN is_default;
```

#### 2.2 Customer Table - Missing Role Column
**Issue:** Model has `role: CustomerRole` but no column in table

**Fix:**
```sql
ALTER TABLE customers ADD COLUMN role VARCHAR(20) NOT NULL DEFAULT 'customer';
```

#### 2.3 Duplicate Migration Numbers
**Issue:** Both `006_customer_fields.sql` and `006_subscriptions.sql` exist

**Fix:** Rename one to `006a_` or `007_`

#### 2.4 Broken Foreign Key Reference
**Issue:** `coupon_applications` references `product_categories(id)` but table doesn't exist

**Fix:** Either create `product_categories` table or remove FK constraint

---

## High Priority (Core Functionality)

### 3. Implement Working Cart API Routes
**Status:** ❌ MOCK DATA  
**File:** `crates/rcommerce-api/src/routes/cart.rs`

**Current State:** All 10 endpoints return hardcoded mock data

**Endpoints to Implement:**
- `POST /carts/guest` - Create guest cart
- `GET /carts/me` - Get customer cart
- `GET /carts/:cart_id` - Get cart by ID
- `POST /carts/:cart_id/items` - Add item to cart
- `PUT /carts/:cart_id/items/:item_id` - Update cart item
- `DELETE /carts/:cart_id/items/:item_id` - Remove cart item
- `DELETE /carts/:cart_id/items` - Clear cart
- `POST /carts/merge` - Merge guest cart to customer cart
- `POST /carts/:cart_id/coupon` - Apply coupon
- `DELETE /carts/:cart_id/coupon` - Remove coupon

**Resources Available:**
- ✅ `CartService` fully implemented
- ✅ `CartRepository` fully implemented
- ✅ `Cart` and `CartItem` models complete

**Implementation:** Replace mock responses with calls to `CartService`

---

### 4. Implement Working Coupon API Routes
**Status:** ❌ MOCK DATA  
**File:** `crates/rcommerce-api/src/routes/coupon.rs`

**Current State:** All 7 endpoints return hardcoded mock data

**Endpoints to Implement:**
- `GET /coupons` - List coupons
- `POST /coupons` - Create coupon
- `GET /coupons/:coupon_id` - Get coupon
- `PUT /coupons/:coupon_id` - Update coupon
- `DELETE /coupons/:coupon_id` - Delete coupon
- `POST /coupons/validate` - Validate coupon code
- `GET /coupons/:coupon_id/stats` - Get coupon statistics

**Resources Available:**
- ✅ `CouponService` fully implemented
- ✅ `CouponRepository` fully implemented
- ✅ `Coupon` model complete

---

### 5. Implement Working Payment API Routes
**Status:** ❌ MOCK DATA  
**File:** `crates/rcommerce-api/src/routes/payment.rs`

**Current State:** All 8 endpoints return mock data with comments "In a real implementation..."

**Endpoints to Implement:**
- `POST /payments/methods` - Get available payment methods
- `POST /payments` - Initiate payment
- `GET /payments/:payment_id` - Get payment status
- `POST /payments/:payment_id/complete` - Complete payment action
- `POST /payments/:payment_id/refund` - Refund payment
- `POST /payment-methods` - Save payment method
- `GET /customers/:customer_id/payment-methods` - List saved methods
- `DELETE /payment-methods/:token` - Delete saved method

**Resources Available:**
- ✅ `PaymentService` (agnostic interface) implemented
- ✅ `StripeGateway` fully implemented
- ✅ Payment types and traits complete

**Note:** Webhook endpoint at `POST /webhooks/:gateway_id` is public and should verify signatures

---

### 6. Complete Customer List/Get Endpoints
**Status:** ⚠️ PARTIAL (only `/me` works)
**File:** `crates/rcommerce-api/src/routes/customer.rs`

**Current State:**
- ✅ `GET /customers/me` - Working (uses JWT, fetches real data)
- ❌ `GET /customers` - Returns mock data (2 hardcoded customers)
- ❌ `GET /customers/:id` - Returns mock data

**Fix:** Replace mock responses with `CustomerService.list_customers()` and `get_customer()`

---

## Medium Priority (Features)

### 7. Implement Webhook Management API
**Status:** ❌ DOCUMENTED BUT MISSING  
**Documentation:** `docs-website/docs/api-reference/webhooks.md`

**Documented Endpoints (Not Implemented):**
- `GET /webhooks` - List webhooks
- `POST /webhooks` - Create webhook
- `GET /webhooks/:id` - Get webhook
- `PUT /webhooks/:id` - Update webhook
- `DELETE /webhooks/:id` - Delete webhook
- `POST /webhooks/:id/test` - Test webhook
- `GET /webhooks/:id/deliveries` - Get delivery history

**Note:** `POST /webhooks/:gateway_id` exists for provider webhooks (Stripe, etc.)

---

### 8. Complete Shipping Carrier Integrations
**Status:** 🟡 STUBS ONLY  
**Files:** `crates/rcommerce-core/src/shipping/carriers/`

**Current State:**
- ✅ Shipping service structure exists
- ✅ Rate calculation framework
- ❌ DHL integration - stub only
- ❌ FedEx integration - stub only
- ❌ UPS integration - stub only
- ❌ USPS integration - stub only

**Documentation:** Comprehensive docs exist in `docs-website/docs/guides/shipping.md`

---

### 9. Implement Missing Payment Gateways
**Status:** 🟡 STUBS ONLY  
**Files:** `crates/rcommerce-core/src/payment/gateways/`

**Current State:**
- ✅ MockPaymentGateway - Complete
- ✅ StripeGateway - Complete
- ✅ StripeAgnosticGateway - Complete
- ⚠️ WeChatPayGateway - Basic structure only
- ⚠️ AliPayGateway - Basic structure only
- ⚠️ AirwallexGateway - Basic structure only

---

### 10. Create Missing Database Tables
**Status:** ❌ MODELS EXIST, NO TABLES

**Missing Tables:**

| Table | Model Location | Priority |
|-------|---------------|----------|
| `fulfillments` | `order.rs` | High |
| `order_notes` | `order.rs` | Medium |
| `subscription_items` | `subscription.rs` | High |
| `collections` | `product.rs` | Medium |
| `product_categories` | Referenced in migration | Medium |
| `product_tags` | `mod.rs` | Low |
| `audit_logs` | `common.rs` | Low |

---

## Lower Priority (Nice to Have)

### 11. GraphQL API
**Status:** ❌ DOCUMENTED BUT NOT IMPLEMENTED  
**Documentation:** `docs-website/docs/api-reference/graphql.md`

**Options:**
1. Implement GraphQL API using `async-graphql`
2. Remove documentation until implemented
3. Mark docs as "Coming Soon"

**Recommendation:** Option 3 - Mark as "Coming Soon" for now

---

### 12. WebSocket Support
**Status:** 🟡 STUBBED (Intentionally)  
**File:** `crates/rcommerce-core/src/websocket/`

**Note:** Code comments state this is intentionally stubbed for initial release as core functionality works without it.

**Recommendation:** Keep stubbed for MVP, implement post-launch

---

### 13. Digital Products Support
**Status:** ❌ DOCUMENTED BUT NOT IMPLEMENTED  
**Documentation:** `docs/architecture/09-product-types-and-subscriptions.md`

**Schema Support:** `product_type` enum includes `digital`

**Missing:**
- File upload/download handling
- License key management
- Download limits/expiration

---

### 14. Bundle Products Support
**Status:** ❌ DOCUMENTED BUT NOT IMPLEMENTED  
**Documentation:** `docs/architecture/09-product-types-and-subscriptions.md`

**Schema Support:** `product_type` enum includes `bundle`

**Missing:**
- Bundle component management
- Bundle pricing logic
- Inventory coordination for bundles

---

## Documentation Gaps

### Missing User-Facing Documentation

| Feature | Implementation | Missing Docs |
|---------|---------------|--------------|
| Rate Limiting | ✅ Complete | No user docs |
| TLS/SSL Configuration | ✅ Complete | No user docs |
| API Key Scopes | ✅ Complete | Detailed scope reference |
| Import System | ✅ Structure | No documentation |
| Database Migrations | ✅ Complete | Developer guide |
| Email Templates | ✅ Complete | Template development guide |

---

## Implementation Status Matrix

### API Routes

| Module | Endpoints | Status | Notes |
|--------|-----------|--------|-------|
| Auth | 3 | ✅ Working | JWT, login, register, refresh |
| Product | 2 | ✅ Working | List, detail with variants |
| Customer | 3 | ⚠️ Partial | Only `/me` works |
| Order | 3 | ✅ Working | Full lifecycle |
| Cart | 10 | ❌ Mock | All mock data |
| Coupon | 7 | ❌ Mock | All mock data |
| Payment | 8 | ❌ Mock | All mock data |
| Subscription | 11 | ✅ Working | Full implementation |
| Dunning | 10 | ✅ Working | Retry logic, notifications |
| Statistics | 7 | ✅ Working | Dashboard, reports |
| Admin | 2 | ❌ Stub | Empty/static data |
| Webhooks | 7 | ❌ Missing | Documented only |

### Services & Repositories

| Service | Repository | Status |
|---------|-----------|--------|
| AuthService | - | ✅ Complete |
| CustomerService | CustomerRepository | ✅ Complete |
| ProductService | ProductRepository | ✅ Complete |
| OrderService | - (direct SQL) | ✅ Complete |
| CartService | CartRepository | ✅ Complete |
| CouponService | CouponRepository | ✅ Complete |
| SubscriptionService | SubscriptionRepository | ✅ Complete |
| DunningService | - | ✅ Complete |
| StatisticsService | PgStatisticsRepository | ✅ Complete |
| PaymentService | - | ✅ Complete |
| InventoryService | - | ✅ Complete |
| NotificationService | - | ✅ Complete |

### Payment Gateways

| Gateway | Status |
|---------|--------|
| Mock (Test) | ✅ Complete |
| Stripe | ✅ Complete |
| Stripe (Agnostic) | ✅ Complete |
| WeChat Pay | ⚠️ Stub |
| Alipay | ⚠️ Stub |
| Airwallex | ⚠️ Stub |

---

## Recommended Action Plan

### Phase 1: Unblock (Week 1)
1. **Fix compilation errors** - 2 hours
2. **Fix database schema mismatches** - 2 hours
3. **Create missing tables** - 4 hours

### Phase 2: Core API (Week 2)
4. **Implement Cart API** - 8 hours
5. **Implement Coupon API** - 6 hours
6. **Complete Customer API** - 2 hours

### Phase 3: Payments (Week 3)
7. **Implement Payment API** - 12 hours
8. **Webhook Management API** - 6 hours

### Phase 4: Shipping & Extras (Week 4)
9. **Shipping carriers** - 16 hours
10. **Additional payment gateways** - 16 hours

### Phase 5: Polish (Week 5)
11. **Documentation gaps** - 8 hours
12. **Testing & bug fixes** - 16 hours

**Total Estimated Time:** ~4-5 weeks for full MVP

---

## Quick Wins (Can do immediately)

1. **Fix customer list/get endpoints** - 30 minutes
2. **Mark GraphQL as "Coming Soon"** - 15 minutes
3. **Add missing database columns** - 1 hour
4. **Fix migration numbering** - 15 minutes

---

## Files to Review

### Critical
- `crates/rcommerce-api/src/routes/cart.rs` - Mock data
- `crates/rcommerce-api/src/routes/coupon.rs` - Mock data
- `crates/rcommerce-api/src/routes/payment.rs` - Mock data
- `crates/rcommerce-api/src/routes/customer.rs` - Partial mock

### Schema
- `crates/rcommerce-core/migrations/001_initial_schema.sql`
- `crates/rcommerce-core/migrations/006_customer_fields.sql`
- `crates/rcommerce-core/migrations/006_subscriptions.sql` (duplicate number)

### Documentation
- `docs-website/docs/api-reference/graphql.md` - Not implemented
- `docs-website/docs/api-reference/webhooks.md` - Not implemented

---

## Summary

The R Commerce platform has a **solid foundation** with fully implemented:
- ✅ Data models
- ✅ Database repositories
- ✅ Business logic services
- ✅ Authentication & subscriptions
- ✅ Statistics & dunning

**What's blocking release:**
- ❌ 212 compilation errors
- ❌ Cart, Coupon, Payment APIs return mock data
- ❌ Some database schema mismatches

**What's documented but missing:**
- ❌ GraphQL API
- ❌ Webhook Management API
- ❌ Advanced shipping carriers
- ❌ Some payment gateways

**Recommendation:** Focus on Phase 1 & 2 to get a working MVP, then iterate on additional features.
