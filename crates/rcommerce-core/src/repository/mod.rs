//! Repository module for database access
//!
//! PostgreSQL is the supported database.

pub mod traits;
pub mod postgres;

// Cart, Coupon, and API Key repositories
pub mod cart_repository;
pub mod coupon_repository;
pub mod api_key_repository;

// Re-export cart, coupon, and api_key traits
pub use cart_repository::CartRepository;
pub use coupon_repository::CouponRepository;
pub use api_key_repository::{ApiKeyRepository, ApiKeyRecord, CreateApiKeyRequest, PostgresApiKeyRepository};

// PostgreSQL exports
pub use postgres::{
    PostgresProductRepository as ProductRepository,
    PostgresCustomerRepository as CustomerRepository,
    PostgresDb as Database,
    create_pool,
};

// Re-export traits
pub use traits::{ProductRepositoryTrait, CustomerRepositoryTrait};
