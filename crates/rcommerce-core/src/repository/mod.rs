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

/// Database connection wrapper (PostgreSQL)
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
        Ok(())
    }
}

/// Create database pool from configuration
/// Currently supports PostgreSQL only
pub async fn create_pool(config: &crate::config::DatabaseConfig) -> Result<Pool<Postgres>> {
    use sqlx::postgres::PgPoolOptions;
    use crate::config::DatabaseType;
    use crate::Error;
    
    if config.db_type != DatabaseType::Postgres {
        return Err(Error::Config(
            format!("Database type {:?} not yet supported. Use PostgreSQL for now.", config.db_type)
        ));
    }
    
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    );
    
    tracing::info!("Connecting to PostgreSQL database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(config.pool_size)
        .connect(&database_url)
        .await
        .map_err(|e| Error::Database(e))?;
    
    tracing::info!("Database connected successfully");
    Ok(pool)
}
