pub mod product_repository;
pub mod customer_repository;
pub mod order_repository;
pub mod cart_repository;
pub mod coupon_repository;

pub use product_repository::ProductRepository;
pub use customer_repository::CustomerRepository;
pub use cart_repository::CartRepository;
pub use coupon_repository::CouponRepository;

use sqlx::{Pool, Postgres};
use crate::Result;

/// Database connection wrapper
#[derive(Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    
    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
    
    /// Run migrations
    pub async fn run_migrations(&self) -> Result<()> {
        // For now, migrations need to be run manually or via CLI
        // This is a placeholder for future migration system integration
        Ok(())
    }
}

/// Helper function to create a database connection pool
pub async fn create_pool(config: &crate::config::DatabaseConfig) -> Result<Pool<Postgres>> {
    use sqlx::postgres::PgPoolOptions;
    use crate::Error;
    
    if config.db_type != crate::config::DatabaseType::Postgres {
        return Err(Error::Config("Only PostgreSQL is currently supported".to_string()));
    }
    
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    );
    
    let pool = PgPoolOptions::new()
        .max_connections(config.pool_size)
        .connect(&database_url)
        .await
        .map_err(|e| Error::Database(e))?;
    
    Ok(pool)
}