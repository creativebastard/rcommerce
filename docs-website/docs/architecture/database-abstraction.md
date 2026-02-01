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
