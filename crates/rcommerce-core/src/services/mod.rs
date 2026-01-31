pub mod product_service;
pub mod customer_service;
pub mod order_service;
pub mod auth_service;
pub mod cart_service;
pub mod coupon_service;

pub use product_service::ProductService;
pub use customer_service::CustomerService;
pub use order_service::OrderService;
pub use auth_service::AuthService;
pub use auth_service::ApiKey;
pub use auth_service::JwtClaims;
pub use auth_service::AuthenticatedUser;
pub use auth_service::TokenType;
pub use cart_service::CartService;
pub use coupon_service::CouponService;

use crate::Result;

/// Common service trait for dependency injection
#[async_trait::async_trait]
pub trait Service: Send + Sync {
    async fn health_check(&self) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct PaginationParams {
    pub page: i64,
    pub per_page: i64,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.per_page
    }
    
    pub fn limit(&self) -> i64 {
        self.per_page
    }
}

#[derive(Debug, Clone)]
pub struct PaginationInfo {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}