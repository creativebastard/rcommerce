# Database Abstraction Refactoring Plan

## Status: In Progress

This document tracks the progress of refactoring raw SQL queries into proper repository pattern implementations.

## Completed ✅

### 1. Tax System Integration
- **CartService** - Integrated with TaxService for real-time tax calculation
- **OrderService** - Integrated with TaxService for order tax calculation
- **CheckoutService** - New orchestrator service for checkout flow
- **Documentation** - Updated in both English and Chinese

### 2. OrderRepository
- **Location**: `crates/rcommerce-core/src/repository/order_repository.rs`
- **Trait**: `OrderRepository` with full CRUD operations
- **Implementation**: `PostgresOrderRepository`
- **Features**:
  - Find by ID, order number
  - List with filtering (customer, status, date range)
  - Create/update/delete orders
  - Order item management
  - Status updates (order, payment, fulfillment)

## Remaining Work 📋

### Priority 1: High-Impact Services

#### 1. InventoryRepository
**File**: `crates/rcommerce-core/src/inventory/service.rs` (13 queries)

**Needed Operations**:
- Stock level management
- Stock reservation CRUD
- Stock movement tracking
- Inventory location management
- Low stock alerts

#### 2. DigitalProductRepository
**File**: `crates/rcommerce-core/src/services/digital_product_service.rs` (17 queries)

**Needed Operations**:
- Download management
- License key management
- Activation tracking
- Download log recording

#### 3. BundleRepository
**File**: `crates/rcommerce-core/src/services/bundle_service.rs` (14 queries)

**Needed Operations**:
- Bundle component management
- Bundle pricing
- Component validation

### Priority 2: Medium-Impact Services

#### 4. FulfillmentRepository
**File**: `crates/rcommerce-core/src/order/fulfillment.rs` (6 queries)

**Needed Operations**:
- Shipment/fulfillment CRUD
- Tracking information
- Return processing

#### 5. NotificationRepository
**File**: `crates/rcommerce-core/src/notification/service.rs` (4 queries)

**Needed Operations**:
- Notification CRUD
- Template management
- Delivery tracking

### Priority 3: Refactor Services to Use Repositories

Once repositories are created, refactor these services:

1. **OrderService** - Use `OrderRepository` instead of raw SQL
2. **InventoryService** - Use `InventoryRepository`
3. **DigitalProductService** - Use `DigitalProductRepository`
4. **BundleService** - Use `BundleRepository`
5. **FulfillmentService** - Use `FulfillmentRepository`
6. **NotificationService** - Use `NotificationRepository`

## Architecture Benefits

### Current State (Mixed)
```
Services → Raw SQL (❌)
Services → Repositories (✅) - Partial
```

### Target State (Clean Architecture)
```
API Layer → Services → Repositories → Database
                    ↓
              Business Logic
                    ↓
              Domain Models
```

### Benefits
1. **Testability** - Easy to mock repositories for unit tests
2. **Maintainability** - Database changes isolated to repositories
3. **Flexibility** - Can swap database implementations (PostgreSQL, MySQL, etc.)
4. **Code Reuse** - Common queries centralized
5. **Type Safety** - Repository methods return strongly-typed models

## Implementation Guidelines

### Repository Pattern Structure
```rust
// 1. Define the trait
#[async_trait]
pub trait OrderRepository: Send + Sync + 'static {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Order>>;
    async fn create(&self, order: &Order) -> Result<Order>;
    // ...
}

// 2. Implement for PostgreSQL
pub struct PostgresOrderRepository {
    db: sqlx::PgPool,
}

#[async_trait]
impl OrderRepository for PostgresOrderRepository {
    // Implementation
}
```

### Service Refactoring Pattern
```rust
// Before
pub struct OrderService {
    db: Database,
}

// After
pub struct OrderService {
    order_repo: Box<dyn OrderRepository>,
    inventory_repo: Box<dyn InventoryRepository>,
}
```

## Query Count by File

| File | Query Count | Priority | Status |
|------|-------------|----------|--------|
| `order/service.rs` | 19 | High | ✅ Repository Created |
| `services/digital_product_service.rs` | 17 | High | ⏳ Pending |
| `services/bundle_service.rs` | 14 | High | ⏳ Pending |
| `inventory/service.rs` | 13 | High | ⏳ Pending |
| `order/fulfillment.rs` | 6 | Medium | ⏳ Pending |
| `notification/service.rs` | 4 | Medium | ⏳ Pending |

## Next Steps

1. Create `InventoryRepository`
2. Create `DigitalProductRepository`
3. Create `BundleRepository`
4. Refactor services to use repositories
5. Add comprehensive tests for repositories

## Notes

- The tax system integration is complete and functional
- OrderRepository is ready for use in OrderService refactoring
- All new code follows the existing repository pattern
- Documentation has been updated in both English and Chinese
