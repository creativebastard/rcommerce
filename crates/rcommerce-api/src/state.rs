use rcommerce_core::cache::RedisPool;
use rcommerce_core::repository::Database;
use rcommerce_core::services::{AuthService, CustomerService, ProductService};

#[derive(Clone)]
pub struct AppState {
    pub product_service: ProductService,
    pub customer_service: CustomerService,
    pub auth_service: AuthService,
    pub db: Database,
    pub redis: Option<RedisPool>,
}

impl AppState {
    pub fn new(
        product_service: ProductService,
        customer_service: CustomerService,
        auth_service: AuthService,
        db: Database,
        redis: Option<RedisPool>,
    ) -> Self {
        Self {
            product_service,
            customer_service,
            auth_service,
            db,
            redis,
        }
    }
}
