//! Repository module for database access
//!
//! PostgreSQL is the supported database.

pub mod traits;
pub mod postgres;

// Cart, Coupon, API Key, Statistics, Order, Inventory, Fulfillment, Notification, and Category repositories
pub mod cart_repository;
pub mod coupon_repository;
pub mod api_key_repository;
pub mod subscription_repository;
pub mod statistics_repository;
pub mod order_repository;
pub mod inventory_repository;
pub mod fulfillment_repository;
pub mod notification_repository;
pub mod category_repository;
pub mod tag_repository;

// Re-export cart, coupon, api_key, subscription, statistics, order, inventory, and fulfillment traits
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
pub use order_repository::{OrderRepository, PostgresOrderRepository, OrderFilter};
pub use inventory_repository::{InventoryRepository, PostgresInventoryRepository};
pub use fulfillment_repository::{FulfillmentRepository, PostgresFulfillmentRepository};
pub use notification_repository::{NotificationRepository, PostgresNotificationRepository};
pub use category_repository::{CategoryRepository, CategoryTreeNode, PostgresCategoryRepository};
pub use tag_repository::{TagRepository, PostgresTagRepository};

// PostgreSQL exports
pub use postgres::{
    PostgresProductRepository as ProductRepository,
    PostgresCustomerRepository as CustomerRepository,
    PostgresDb as Database,
    create_pool,
};

// Re-export traits
pub use traits::{ProductRepositoryTrait, CustomerRepositoryTrait};
