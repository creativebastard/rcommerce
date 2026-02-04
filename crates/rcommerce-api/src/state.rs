use std::sync::Arc;

use rcommerce_core::cache::RedisPool;
use rcommerce_core::repository::{Database, PostgresApiKeyRepository};
use rcommerce_core::services::{AuthService, CustomerService, ProductService};

use crate::middleware::AuthRateLimiter;

#[derive(Clone)]
pub struct AppState {
    pub product_service: ProductService,
    pub customer_service: CustomerService,
    pub auth_service: AuthService,
    pub db: Database,
    pub redis: Option<RedisPool>,
    pub auth_rate_limiter: AuthRateLimiter,
    pub api_key_repository: Arc<PostgresApiKeyRepository>,
}

impl AppState {
    pub fn new(
        product_service: ProductService,
        customer_service: CustomerService,
        auth_service: AuthService,
        db: Database,
        redis: Option<RedisPool>,
        api_key_repository: PostgresApiKeyRepository,
    ) -> Self {
        // Create auth rate limiter: 5 attempts per minute per IP
        let auth_rate_limiter = AuthRateLimiter::new(5, 60);
        
        Self {
            product_service,
            customer_service,
            auth_service,
            db,
            redis,
            auth_rate_limiter,
            api_key_repository: Arc::new(api_key_repository),
        }
    }
}
