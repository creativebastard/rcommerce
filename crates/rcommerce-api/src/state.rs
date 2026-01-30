use rcommerce_core::services::{ProductService, CustomerService, OrderService, AuthService};
use rcommerce_core::cache::RedisPool;

#[derive(Clone)]
pub struct AppState {
    pub product_service: ProductService,
    pub customer_service: CustomerService,
    pub order_service: OrderService,
    pub auth_service: AuthService,
    pub redis: Option<RedisPool>,
}

impl AppState {
    pub fn new(
        product_service: ProductService,
        customer_service: CustomerService,
        order_service: OrderService,
        auth_service: AuthService,
        redis: Option<RedisPool>,
    ) -> Self {
        Self {
            product_service,
            customer_service,
            order_service,
            auth_service,
            redis,
        }
    }
}