//! Repository module for database access
//!
//! PostgreSQL is the supported database.

pub mod traits;
pub mod postgres;

// Cart, Coupon, and API Key repositories
pub mod cart_repository;
pub mod coupon_repository;
pub mod api_key_repository;
pub mod subscription_repository;

// Re-export cart, coupon, api_key, and subscription traits
pub use cart_repository::CartRepository;
pub use coupon_repository::CouponRepository;
pub use api_key_repository::{ApiKeyRepository, ApiKeyRecord, CreateApiKeyRequest, PostgresApiKeyRepository};
pub use subscription_repository::{SubscriptionRepository, PostgresSubscriptionRepository};

// PostgreSQL exports
pub use postgres::{
    PostgresProductRepository as ProductRepository,
    PostgresCustomerRepository as CustomerRepository,
    PostgresDb as Database,
    create_pool,
};

// Re-export traits
pub use traits::{ProductRepositoryTrait, CustomerRepositoryTrait};
