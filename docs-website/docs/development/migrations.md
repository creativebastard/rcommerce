# Database Migrations

Database migrations manage schema changes in R Commerce, allowing you to version control your database structure and apply changes consistently across environments.

## Overview

R Commerce uses SQL-based migrations that are:

- **Version controlled** - Track schema changes in Git
- **Reversible** - Rollback capability for each migration
- **Atomic** - Each migration runs in a transaction
- **Idempotent** - Safe to run multiple times

## Running Migrations on Startup

### Automatic Migration

Enable automatic migrations in your configuration:

```toml
[database]
url = "postgres://user:pass@localhost/rcommerce"

# Run pending migrations on startup
run_migrations_on_startup = true

# Migration settings
[migrations]
# Directory containing migration files
path = "./migrations"

# Fail startup if migrations fail
strict = true

# Log migration execution
verbose = true
```

### Manual Migration

Run migrations manually using the CLI:

```bash
# Run all pending migrations
rcommerce db migrate

# Run migrations with specific config
rcommerce db migrate -c /path/to/config.toml

# Check migration status
rcommerce db status

# View pending migrations
rcommerce db pending
```

## Creating Custom Migrations

### Migration File Naming

Migration files follow this naming convention:

```
{version}_{description}.sql
```

Examples:

```
001_initial_schema.sql
002_add_user_preferences.sql
003_create_inventory_table.sql
```

### Migration File Structure

Each migration file contains up and down migrations:

```sql
-- Up migration (apply changes)
-- 004_add_product_tags.sql

CREATE TABLE product_tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    tag VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(product_id, tag)
);

CREATE INDEX idx_product_tags_product_id ON product_tags(product_id);
CREATE INDEX idx_product_tags_tag ON product_tags(tag);

-- Down migration (rollback changes)
-- @DOWN

DROP INDEX IF EXISTS idx_product_tags_tag;
DROP INDEX IF EXISTS idx_product_tags_product_id;
DROP TABLE IF EXISTS product_tags;
```

### Migration Template

Use the CLI to generate a new migration:

```bash
# Create a new migration file
rcommerce db create-migration add_customer_loyalty_points

# Output: Created migrations/005_add_customer_loyalty_points.sql
```

Generated template:

```sql
-- Migration: add_customer_loyalty_points
-- Created at: 2024-01-15T10:30:00Z

-- Up migration


-- @DOWN

-- Down migration

```

### Best Practices for Migrations

#### 1. Keep Migrations Small

Split large changes into multiple migrations:

```sql
-- 005_create_order_items.sql
CREATE TABLE order_items (...);

-- 006_add_order_item_indexes.sql
CREATE INDEX idx_order_items_order_id ON order_items(order_id);
CREATE INDEX idx_order_items_product_id ON order_items(product_id);
```

#### 2. Make Migrations Reversible

Always provide down migrations:

```sql
-- Up
ALTER TABLE customers ADD COLUMN phone VARCHAR(20);

-- @DOWN
ALTER TABLE customers DROP COLUMN IF EXISTS phone;
```

#### 3. Handle Existing Data

Consider data migration in schema changes:

```sql
-- Add new column with default
ALTER TABLE products ADD COLUMN status VARCHAR(20) DEFAULT 'active';

-- Update existing records
UPDATE products SET status = 'active' WHERE status IS NULL;

-- Make column non-nullable
ALTER TABLE products ALTER COLUMN status SET NOT NULL;
```

#### 4. Use Transactions

Migrations run in transactions by default. For operations that can't run in transactions (like creating indexes concurrently), use:

```sql
-- @NO_TRANSACTION

CREATE INDEX CONCURRENTLY idx_products_name ON products(name);
```

#### 5. Avoid Destructive Changes

Instead of dropping columns, consider:

```sql
-- Instead of: ALTER TABLE products DROP COLUMN old_field;

-- 1. Add deprecation notice
COMMENT ON COLUMN products.old_field IS 'DEPRECATED: Will be removed in v2.0';

-- 2. Make nullable
ALTER TABLE products ALTER COLUMN old_field DROP NOT NULL;

-- 3. Remove in later migration
```

## Database Reset Procedures

### Development Reset

Reset database to initial state:

```bash
# Reset database (drops and recreates)
rcommerce db reset

# Reset with fresh migrations
rcommerce db reset --with-migrations

# Reset and seed with test data
rcommerce db reset --seed
```

### Migration Rollback

Rollback specific migrations:

```bash
# Rollback last migration
rcommerce db rollback

# Rollback specific number of migrations
rcommerce db rollback --steps 3

# Rollback to specific version
rcommerce db rollback --to 003

# Rollback specific migration
rcommerce db rollback --migration 005_add_feature
```

### Force Migration State

In rare cases, you may need to force the migration state:

```bash
# Mark a migration as applied (without running it)
rcommerce db force --version 005

# Mark a migration as pending
rcommerce db unforce --version 005

# Reset migration tracking (use with caution!)
rcommerce db reset-tracking
```

## Migration Best Practices

### Development Workflow

1. **Create migration before code changes**
   ```bash
   rcommerce db create-migration add_feature_x
   ```

2. **Write migration first**
   - Define schema changes
   - Test migration locally

3. **Write application code**
   - Implement features using new schema

4. **Test migration**
   ```bash
   rcommerce db reset --with-migrations
   ```

5. **Commit both migration and code**
   ```bash
   git add migrations/ src/
   git commit -m "Add feature X with migration"
   ```

### Team Collaboration

#### Migration Conflicts

When two developers create migrations with the same version:

```bash
# Check migration status
rcommerce db status

# If conflict exists, renumber migration
mv migrations/005_add_feature.sql migrations/006_add_feature.sql

# Update migration tracking
rcommerce db force --version 005
rcommerce db migrate
```

#### Code Review Checklist

- [ ] Migration has up and down sections
- [ ] Migration is reversible
- [ ] No destructive changes without deprecation
- [ ] Indexes are added for foreign keys
- [ ] Data migration is handled (if needed)
- [ ] Migration has been tested locally

### Production Deployment

#### Pre-Deployment

1. **Backup database**
   ```bash
   pg_dump -Fc rcommerce > backup_$(date +%Y%m%d).dump
   ```

2. **Test migrations on staging**
   ```bash
   rcommerce db migrate --dry-run
   ```

3. **Check migration duration**
   ```sql
   -- Estimate time for large table changes
   EXPLAIN ANALYZE ALTER TABLE large_table ADD COLUMN new_col VARCHAR(100);
   ```

#### Deployment Steps

1. **Deploy application** (with migrations disabled)
2. **Run migrations manually**
   ```bash
   rcommerce db migrate
   ```
3. **Verify migration success**
   ```bash
   rcommerce db status
   ```
4. **Enable application traffic**

#### Zero-Downtime Migrations

For large tables, use non-blocking migrations:

```sql
-- @NO_TRANSACTION

-- Create index without locking table
CREATE INDEX CONCURRENTLY idx_orders_created_at ON orders(created_at);
```

Add columns without defaults (fast):

```sql
-- Step 1: Add nullable column (fast)
ALTER TABLE products ADD COLUMN new_feature BOOLEAN;

-- Step 2: Set default in application code

-- Step 3: Backfill data in batches

-- Step 4: Make non-nullable (later migration)
```

## Troubleshooting

### Migration Failures

**"Migration already applied"**

```bash
# Check current state
rcommerce db status

# Force mark as applied if manually fixed
rcommerce db force --version 005
```

**"Migration checksum mismatch"**

```bash
# If migration was modified after application
rcommerce db verify --fix

# Or reset and reapply
rcommerce db rollback --to 004
rcommerce db migrate
```

**"Lock timeout"**

```sql
-- Check for locks
SELECT * FROM pg_locks WHERE NOT granted;

-- Kill blocking process
SELECT pg_terminate_backend(pid);
```

### Common Issues

**Failed migration leaves database in inconsistent state:**

```bash
# Check migration status
rcommerce db status

# Rollback failed migration
rcommerce db rollback

# Fix migration file
vim migrations/005_failed_migration.sql

# Retry
rcommerce db migrate
```

**Long-running migration:**

```bash
# Monitor progress
watch -n 5 'rcommerce db status'

# Check PostgreSQL activity
psql -c "SELECT * FROM pg_stat_activity WHERE state = 'active';"
```

### Migration Debugging

Enable verbose logging:

```bash
# Debug mode
RUST_LOG=debug rcommerce db migrate

# SQL logging
RUST_LOG=sqlx=debug rcommerce db migrate
```

View migration history:

```bash
# List applied migrations
rcommerce db history

# Show specific migration details
rcommerce db show 005
```

## Migration Reference

### CLI Commands

| Command | Description |
|---------|-------------|
| `rcommerce db migrate` | Run pending migrations |
| `rcommerce db rollback` | Rollback migrations |
| `rcommerce db status` | Show migration status |
| `rcommerce db pending` | List pending migrations |
| `rcommerce db history` | Show applied migrations |
| `rcommerce db create-migration` | Create new migration file |
| `rcommerce db reset` | Reset database |
| `rcommerce db force` | Force migration state |
| `rcommerce db verify` | Verify migration integrity |

### Migration Metadata Table

R Commerce tracks migrations in the `_migrations` table:

```sql
SELECT * FROM _migrations;

-- Output:
-- version | name                    | applied_at
-- ---------+-------------------------+------------------------
-- 001      | initial_schema          | 2024-01-01 00:00:00+00
-- 002      | add_user_preferences    | 2024-01-02 00:00:00+00
-- 003      | create_inventory_table  | 2024-01-03 00:00:00+00
```

## Related Documentation

- [Database Configuration](../getting-started/configuration.md)
- [Local Development Setup](./local-setup.md)
- [Production Deployment](../deployment/binary.md)
