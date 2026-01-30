use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use rcommerce_core::{Result, Config};
use rcommerce_core::repository::{
    Database, create_pool,
    ProductRepository, CustomerRepository,
};
use rcommerce_core::services::{ProductService, CustomerService, OrderService, AuthService};
use rcommerce_core::cache::RedisPool;
use crate::state::AppState;

pub async fn run(config: Config) -> Result<()> {
    let addr = SocketAddr::from((
        config.server.host.parse::<std::net::IpAddr>()
            .map_err(|e| rcommerce_core::Error::Config(format!("Invalid host: {}", e)))?,
        config.server.port
    ));
    
    // Initialize database connection
    info!("Connecting to PostgreSQL database...");
    let pool = create_pool(
        &config.database.host,
        config.database.port,
        &config.database.database,
        &config.database.username,
        &config.database.password,
        config.database.pool_size,
    ).await?;
    let db = Database::new(pool);
    
    // Initialize repositories
    let product_repo = ProductRepository::new(db.clone());
    let customer_repo = CustomerRepository::new(db);
    
    // Initialize services
    let product_service = ProductService::new(product_repo);
    let customer_service = CustomerService::new(customer_repo);
    let order_service = OrderService::new();
    let auth_service = AuthService::new(config.clone());
    
    // Initialize Redis (optional)
    let redis = init_redis(&config).await;
    
    // Create app state
    let app_state = AppState::new(
        product_service,
        customer_service,
        order_service,
        auth_service,
        redis,
    );
    
    // Configure CORS for demo frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Build main router with API v1 and v2 routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/", get(root))
        .nest("/api/v1", api_routes())
        .nest("/api/v2", payment_v2_routes())
        .layer(cors)
        .with_state(app_state);
    
    info!("R Commerce API server listening on {}", addr);
    info!("Available routes:");
    info!("  GET  /health                      - Health check");
    info!("  GET  /                            - API info");
    info!("  GET  /api/v1/products            - List products");
    info!("  GET  /api/v1/products/:id        - Get product");
    info!("  GET  /api/v1/customers           - List customers");
    info!("  GET  /api/v1/customers/:id       - Get customer");
    info!("  GET  /api/v1/orders              - List orders");
    info!("  GET  /api/v1/orders/:id          - Get order");
    info!("  POST /api/v1/auth/login          - Login");
    info!("  POST /api/v1/auth/register       - Register");
    info!("  POST /api/v1/carts/guest         - Create guest cart");
    info!("  GET  /api/v1/carts/me            - Get customer cart");
    info!("  POST /api/v1/carts/merge         - Merge carts");
    info!("  GET  /api/v1/coupons             - List coupons");
    info!("  POST /api/v1/coupons             - Create coupon");
    info!("  GET  /api/v1/payments/config     - Stripe config");
    info!("  POST /api/v1/payments/intent     - Create payment intent");
    info!("  POST /api/v1/payments/confirm    - Confirm payment");
    
    // Start server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| rcommerce_core::Error::Network(e.to_string()))?;
    
    axum::serve(listener, app)
        .await
        .map_err(|e| rcommerce_core::Error::Network(e.to_string()))?;
    
    Ok(())
}

/// Initialize Redis connection pool (optional)
async fn init_redis(config: &Config) -> Option<RedisPool> {
    // Check if Redis URL is configured
    let redis_url = config.cache.redis_url.clone()
        .or_else(|| std::env::var("REDIS_URL").ok());
    
    if let Some(url) = redis_url {
        info!("Connecting to Redis at {}...", url);
        match RedisPool::new(rcommerce_core::cache::RedisConfig {
            url,
            pool_size: config.cache.redis_pool_size as usize,
            ..Default::default()
        }).await {
            Ok(pool) => {
                info!("Redis connected successfully");
                Some(pool)
            }
            Err(e) => {
                tracing::warn!("Failed to connect to Redis: {}. Continuing without cache.", e);
                None
            }
        }
    } else {
        info!("Redis not configured. Running without cache.");
        None
    }
}

/// API v1 routes
fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(crate::routes::product_router())
        .merge(crate::routes::customer_router())
        .merge(crate::routes::order_router())
        .merge(crate::routes::auth_router())
        .merge(crate::routes::cart_router())
        .merge(crate::routes::coupon_router())
        .merge(crate::routes::payment_router())
}

/// API v2 routes (payment agnostic API)
fn payment_v2_routes() -> Router<AppState> {
    crate::routes::payment_v2_router()
}

async fn health_check() -> &'static str {
    "OK"
}

async fn root() -> &'static str {
    "R Commerce API v0.1.0 - Phase 1 MVP"
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response, "OK");
    }
}
