//! Repository module for database access
//!
//! PostgreSQL is the supported database.

pub mod traits;
pub mod postgres;

// Cart, Coupon, API Key, and Statistics repositories
pub mod cart_repository;
pub mod coupon_repository;
pub mod api_key_repository;
pub mod subscription_repository;
pub mod statistics_repository;

// Re-export cart, coupon, api_key, subscription, and statistics traits
pub use cart_repository::{CartRepository, PgCartRepository};
pub use coupon_repository::{CouponRepository, PgCouponRepository};
pub use api_key_repository::{ApiKeyRepository, ApiKeyRecord, CreateApiKeyRequest, PostgresApiKeyRepository};
pub use subscription_repository::{SubscriptionRepository, PostgresSubscriptionRepository};
pub use statistics_repository::{
    StatisticsRepository, PgStatisticsRepository, Period,
    SalesSummary, OrderStatistics, ProductPerformance, CustomerStatistics,
    DashboardMetrics, RevenueDataPoint, PeriodComparison, TrendComparison, TrendDirection,
    StatusCount,
};

// PostgreSQL exports
pub use postgres::{
    PostgresProductRepository as ProductRepository,
    PostgresCustomerRepository as CustomerRepository,
    PostgresDb as Database,
    create_pool,
};

// Re-export traits
pub use traits::{ProductRepositoryTrait, CustomerRepositoryTrait};
