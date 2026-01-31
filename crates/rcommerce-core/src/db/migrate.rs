//! Database migration system for R Commerce
//! 
//! This module provides automatic database schema management:
//! - Runs migrations on startup
//! - Tracks applied migrations
//! - Supports seeding demo data

use sqlx::{PgPool, Row};
use tracing::{info, warn, error};

use crate::{Error, Result};

/// Migration record tracking applied migrations
#[derive(Debug, Clone)]
pub struct Migration {
    pub version: i64,
    pub name: String,
    pub applied_at: chrono::DateTime<chrono::Utc>,
}

/// Database migration manager
pub struct Migrator {
    pool: PgPool,
}

impl Migrator {
    /// Create a new migrator instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Initialize migration tracking table
    async fn init_migration_table(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS _migrations (
                version BIGINT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(())
    }

    /// Get list of applied migrations
    async fn get_applied_migrations(&self) -> Result<Vec<Migration>> {
        let rows = sqlx::query(
            r#"SELECT version, name, applied_at FROM _migrations ORDER BY version"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        let migrations = rows
            .into_iter()
            .map(|row| Migration {
                version: row.get("version"),
                name: row.get("name"),
                applied_at: row.get("applied_at"),
            })
            .collect();

        Ok(migrations)
    }

    /// Record a migration as applied
    async fn record_migration(&self, version: i64, name: &str) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO _migrations (version, name) VALUES ($1, $2) ON CONFLICT DO NOTHING"#
        )
        .bind(version)
        .bind(name)
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(())
    }

    /// Run all pending migrations
    pub async fn migrate(&self) -> Result<()> {
        info!("Initializing migration system...");
        self.init_migration_table().await?;

        let applied = self.get_applied_migrations().await?;
        info!("Found {} applied migrations", applied.len());

        // Define all migrations
        let migrations = vec![
            (1, "initial_schema", include_str!("../../migrations/001_initial_schema.sql")),
            (2, "carts_and_coupons", include_str!("../../migrations/002_carts_and_coupons.sql")),
            (3, "demo_products", include_str!("../../migrations/003_demo_products.sql")),
            (4, "fix_product_schema", include_str!("../../migrations/004_fix_product_schema.sql")),
            (5, "customer_fields", include_str!("../../migrations/006_customer_fields.sql")),
            (6, "api_keys", include_str!("../../migrations/005_api_keys.sql")),
            (7, "fix_currency_type", include_str!("../../migrations/007_fix_currency_type.sql")),
        ];

        for (version, name, sql) in migrations {
            if applied.iter().any(|m| m.version == version) {
                info!("Migration {} ({}) already applied, skipping", version, name);
                continue;
            }

            info!("Applying migration {} ({})...", version, name);
            
            // Execute the entire migration SQL as a single batch
            // This is necessary because splitting by semicolons breaks DO blocks
            sqlx::raw_sql(sql)
                .execute(&self.pool)
                .await
                .map_err(|e| {
                    error!("Failed to execute migration {}: {}", version, e);
                    Error::Database(e)
                })?;
            
            // Record migration
            self.record_migration(version, name).await?;
            info!("Migration {} ({}) applied successfully", version, name);
        }

        info!("All migrations completed successfully!");
        Ok(())
    }

    /// Reset database (drop all tables and re-run migrations)
    pub async fn reset(&self) -> Result<()> {
        warn!("RESETTING DATABASE - This will delete all data!");
        
        // Drop all tables
        sqlx::query(
            r#"
            DO $$ DECLARE
                r RECORD;
            BEGIN
                FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
                    EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
                END LOOP;
            END $$;
            "#
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        // Drop all types
        sqlx::query(
            r#"
            DO $$ DECLARE
                r RECORD;
            BEGIN
                FOR r IN (SELECT typname FROM pg_type WHERE typtype = 'e' AND typnamespace = 'public'::regnamespace) LOOP
                    EXECUTE 'DROP TYPE IF EXISTS ' || quote_ident(r.typname) || ' CASCADE';
                END LOOP;
            END $$;
            "#
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        info!("Database reset complete. Re-running migrations...");
        self.migrate().await?;
        
        Ok(())
    }

    /// Seed database with demo data
    pub async fn seed(&self) -> Result<()> {
        info!("Seeding database with demo data...");
        
        // Check if products already exist
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products")
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        if count > 0 {
            warn!("Database already contains {} products, skipping seed", count);
            return Ok(());
        }

        // Demo data is already included in migration 003
        // If we need additional seed data, add it here
        
        info!("Demo data seeded successfully!");
        Ok(())
    }

    /// Get database status
    pub async fn status(&self) -> Result<DbStatus> {
        self.init_migration_table().await?;
        
        let applied = self.get_applied_migrations().await?;
        
        // Get table counts
        let product_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products")
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        let customer_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM customers")
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        let order_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM orders")
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        Ok(DbStatus {
            applied_migrations: applied.len() as i64,
            product_count,
            customer_count,
            order_count,
        })
    }
}

/// Database status information
#[derive(Debug, Clone)]
pub struct DbStatus {
    pub applied_migrations: i64,
    pub product_count: i64,
    pub customer_count: i64,
    pub order_count: i64,
}

/// Run migrations automatically on server start
pub async fn auto_migrate(pool: &PgPool) -> Result<()> {
    let migrator = Migrator::new(pool.clone());
    migrator.migrate().await?;
    Ok(())
}
