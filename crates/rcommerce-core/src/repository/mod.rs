//! Repository module for database access
//!
//! Currently supports PostgreSQL. SQLite and MySQL support planned.

pub mod traits;
pub mod postgres;

// Cart and Coupon repositories (existing)
pub mod cart_repository;
pub mod coupon_repository;

// Re-export cart and coupon traits
pub use cart_repository::CartRepository;
pub use coupon_repository::CouponRepository;

// Exports for PostgreSQL
pub use postgres::{
    PostgresProductRepository as ProductRepository,
    PostgresCustomerRepository as CustomerRepository,
    PostgresDb as Database,
    create_pool,
};

// Re-export traits
pub use traits::{ProductRepositoryTrait, CustomerRepositoryTrait};
