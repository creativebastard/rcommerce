//! Repository module for database access
//!
//! Supports PostgreSQL (default) and SQLite (with "sqlite" feature).

pub mod traits;
pub mod postgres;

// Cart and Coupon repositories (existing)
pub mod cart_repository;
pub mod coupon_repository;

// Re-export cart and coupon traits
pub use cart_repository::CartRepository;
pub use coupon_repository::CouponRepository;

// PostgreSQL exports (default)
pub use postgres::{
    PostgresProductRepository as ProductRepository,
    PostgresCustomerRepository as CustomerRepository,
    PostgresDb as Database,
    create_pool,
};

// SQLite support (optional feature)
#[cfg(feature = "sqlite")]
pub mod sqlite;

#[cfg(feature = "sqlite")]
pub use sqlite::{
    SqliteProductRepository,
    SqliteCustomerRepository,
    SqliteDb,
    create_pool as create_sqlite_pool,
};

// Re-export traits
pub use traits::{ProductRepositoryTrait, CustomerRepositoryTrait};
