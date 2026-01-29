//! SQLite repository implementations

pub mod product_repository;
pub mod customer_repository;

pub use product_repository::SqliteProductRepository;
pub use customer_repository::SqliteCustomerRepository;

use sqlx::{Pool, Sqlite};

/// SQLite database handle
#[derive(Clone)]
pub struct SqliteDb {
    pool: Pool<Sqlite>,
}

impl SqliteDb {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}

/// Create SQLite connection pool
pub async fn create_pool(path: &str) -> crate::Result<Pool<Sqlite>> {
    use sqlx::sqlite::SqlitePoolOptions;
    
    let database_url = format!("sqlite:{}", path);
    
    tracing::info!("Connecting to SQLite database at {}...", path);
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| crate::Error::Database(e))?;
    
    // Enable foreign keys
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;
    
    tracing::info!("SQLite connected successfully");
    Ok(pool)
}
