//! Repository module for database access
//!
//! PostgreSQL is the supported database.

pub mod traits;
pub mod postgres;

// Cart and Coupon repositories
pub mod cart_repository;
pub mod coupon_repository;

// Re-export cart and coupon traits
pub use cart_repository::CartRepository;
pub use coupon_repository::CouponRepository;

// PostgreSQL exports
pub use postgres::{
    PostgresProductRepository as ProductRepository,
    PostgresCustomerRepository as CustomerRepository,
    PostgresDb as Database,
    create_pool,
};

// Re-export traits
pub use traits::{ProductRepositoryTrait, CustomerRepositoryTrait};
