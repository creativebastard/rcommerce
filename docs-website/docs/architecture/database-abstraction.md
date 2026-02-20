# Database Abstraction

## Overview

R Commerce uses a repository pattern for database access, providing a clean separation between business logic and data persistence. The current implementation is optimized for PostgreSQL 14+.

## Repository Pattern

The repository pattern abstracts database operations behind trait interfaces:

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

## Available Repositories

R Commerce provides the following repositories for database operations:

| Repository | Purpose | Key Methods |
|------------|---------|-------------|
| `ProductRepository` | Product catalog management | `find_by_slug`, `update_inventory`, `list_with_filter` |
| `CustomerRepository` | Customer account management | `find_by_email`, `update_password`, `add_address` |
| `OrderRepository` | Order lifecycle management | `find_by_order_number`, `update_status`, `list_with_items` |
| `CartRepository` | Shopping cart operations | `find_by_customer`, `add_item`, `apply_coupon` |
| `CouponRepository` | Discount code management | `validate_code`, `increment_usage`, `find_active` |
| `SubscriptionRepository` | Subscription billing | `find_active_by_customer`, `renew`, `cancel` |
| `ApiKeyRepository` | API key management | `validate_key`, `revoke_key`, `list_by_customer` |
| `InventoryRepository` | Stock tracking & reservations | `get_inventory_level`, `create_reservation`, `adjust_stock` |
| `FulfillmentRepository` | Order fulfillment (shipping) | `create`, `update_tracking`, `mark_shipped`, `mark_delivered` |
| `NotificationRepository` | Notification delivery tracking | `create`, `get_pending`, `mark_delivered`, `get_retryable` |
| `CategoryRepository` | Product category management | `get_tree`, `assign_product`, `get_children` |
| `TagRepository` | Product tag management | `get_or_create`, `bulk_assign_to_product`, `get_popular` |
| `StatisticsRepository` | Analytics & reporting | `get_sales_summary`, `get_dashboard_metrics` |

### Repository Examples

#### Inventory Repository

```rust
#[async_trait]
pub trait InventoryRepository: Send + Sync {
    /// Get inventory level for a product
    async fn get_inventory_level(
        &self,
        product_id: Uuid,
        location_id: Option<Uuid>,
    ) -> Result<Option<InventoryLevel>>;
    
    /// Update inventory level
    async fn update_inventory_level(&self, level: &InventoryLevel) -> Result<InventoryLevel>;
    
    /// Create inventory reservation for an order
    async fn create_reservation(&self, reservation: &StockReservation) -> Result<StockReservation>;
    
    /// Release a reservation
    async fn release_reservation(&self, id: Uuid) -> Result<StockReservation>;
    
    /// Adjust stock quantity
    async fn adjust_stock(
        &self,
        product_id: Uuid,
        location_id: Uuid,
        adjustment: i32,
        reason: &str,
    ) -> Result<InventoryLevel>;
}
```

#### Category Repository

```rust
#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<ProductCategory>>;
    async fn get_by_slug(&self, slug: &str) -> Result<Option<ProductCategory>>;
    async fn create(&self, category: &ProductCategory) -> Result<ProductCategory>;
    async fn update(&self, category: &ProductCategory) -> Result<ProductCategory>;
    async fn delete(&self, id: Uuid) -> Result<bool>;
    
    /// Get hierarchical category tree
    async fn get_tree(&self, root_id: Option<Uuid>) -> Result<Vec<CategoryTreeNode>>;
    
    /// Get child categories
    async fn get_children(&self, parent_id: Uuid) -> Result<Vec<ProductCategory>>;
    
    /// Assign product to category
    async fn assign_product(&self, product_id: Uuid, category_id: Uuid) -> Result<()>;
    
    /// Get products in category
    async fn get_products(&self, category_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Product>>;
}
```

#### Notification Repository

```rust
#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn create(&self, notification: &Notification) -> Result<Notification>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Notification>>;
    async fn update_status(&self, id: Uuid, status: DeliveryStatus) -> Result<Notification>;
    
    /// Get pending notifications ready to be sent
    async fn get_pending(&self, limit: i64) -> Result<Vec<Notification>>;
    
    /// Get scheduled notifications that are due
    async fn get_due(&self, limit: i64) -> Result<Vec<Notification>>;
    
    /// Get failed notifications that should be retried
    async fn get_retryable(&self, limit: i64) -> Result<Vec<Notification>>;
    
    /// Mark notification as delivered
    async fn mark_delivered(&self, id: Uuid) -> Result<Notification>;
    
    /// Mark notification as failed
    async fn mark_failed(&self, id: Uuid, error: &str) -> Result<Notification>;
    
    /// Clean up old delivered notifications
    async fn cleanup_old(&self, before: DateTime<Utc>) -> Result<u64>;
}
```

## Technology Stack

### SQLx

R Commerce uses SQLx for database access:

- **Compile-time query checking**: Queries are verified against the database schema at compile time
- **Async-first design**: Built on Tokio for high-performance async operations
- **Type-safe results**: Query results are automatically mapped to Rust structs
- **Connection pooling**: Built-in connection pool management

**Example:**
```rust
// Compile-time checked query
let product = sqlx::query_as!(Product,
    "SELECT * FROM products WHERE id = $1",
    product_id
)
.fetch_optional(&pool)
.await?;
```

## PostgreSQL Features

R Commerce leverages PostgreSQL-specific features:

### Enums
```sql
CREATE TYPE order_status AS ENUM (
    'pending', 'confirmed', 'processing', 
    'shipped', 'completed', 'cancelled'
);
```

### JSONB for Flexible Data
```sql
-- Metadata stored as JSONB
metadata JSONB NOT NULL DEFAULT '{}'

-- Query JSONB fields
SELECT * FROM products WHERE metadata->>'brand' = 'Nike';
```

### Full-Text Search
```sql
-- Full-text search index
CREATE INDEX idx_products_search ON products 
USING gin(to_tsvector('english', title || ' ' || description));
```

### Arrays
```sql
-- Array fields for tags
tags TEXT[]

-- Query arrays
SELECT * FROM products WHERE tags @> ARRAY['sale'];
```

## Connection Pooling

Connection pool configuration in `config.toml`:

```toml
[database]
host = "localhost"
port = 5432
database = "rcommerce"
username = "rcommerce"
password = "secret"
pool_size = 20          # Max connections in pool
min_connections = 5     # Min idle connections
max_lifetime = 1800     # Connection max lifetime (seconds)
```

## Migrations

Database migrations are managed via the CLI:

```bash
# Run pending migrations
rcommerce db migrate -c config.toml

# Check migration status
rcommerce db status -c config.toml

# Reset database (development only)
rcommerce db reset -c config.toml
```

Migration files are stored in:
```
crates/rcommerce-core/migrations/
├── 001_initial_schema.sql
├── 002_add_api_keys.sql
└── ...
```

## Error Handling

Database errors are mapped to application errors:

```rust
pub enum Error {
    Database(sqlx::Error),
    NotFound(String),
    Validation(String),
    // ...
}
```

## Best Practices

1. **Use compile-time checked queries** when possible for type safety
2. **Use transactions** for multi-step operations
3. **Index strategically** based on query patterns
4. **Use connection pooling** to manage database connections
5. **Monitor slow queries** and optimize as needed

## See Also

- [Data Model](./data-model.md) - Entity relationships
- [Order Management](./order-management.md) - Order lifecycle
