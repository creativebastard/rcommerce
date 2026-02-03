use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::middleware::{admin_middleware, auth_middleware};

use crate::state::AppState;
use rcommerce_core::cache::RedisPool;
use rcommerce_core::repository::{create_pool, CustomerRepository, Database, ProductRepository};
use rcommerce_core::services::{AuthService, CustomerService, ProductService};
use rcommerce_core::{Config, Result};

pub async fn run(config: Config) -> Result<()> {
    let addr = SocketAddr::from((
        config
            .server
            .host
            .parse::<std::net::IpAddr>()
            .map_err(|e| rcommerce_core::Error::Config(format!("Invalid host: {}", e)))?,
        config.server.port,
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
    )
    .await?;
    let db = Database::new(pool);

    // Initialize repositories
    let product_repo = ProductRepository::new(db.clone());
    let customer_repo = CustomerRepository::new(db.clone());

    // Initialize services
    let product_service = ProductService::new(product_repo);
    let customer_service = CustomerService::new(customer_repo);
    // Order service requires payment gateway - initialized per-request for now
    let auth_service = AuthService::new(config.clone());

    // Initialize Redis (optional)
    let redis = init_redis(&config).await;

    // Create app state
    let app_state = AppState::new(product_service, customer_service, auth_service, db, redis);

    // Configure CORS for demo frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build main router with API v1 routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/", get(root))
        .nest("/api/v1", api_routes(app_state.clone()))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    info!("R Commerce API server listening on {}", addr);
    info!("Available routes:");
    info!("  GET  /health                      - Health check");
    info!("  GET  /                            - API info");
    info!("  GET  /api/v1/products             - List products");
    info!("  GET  /api/v1/products/:id         - Get product");
    info!("  GET  /api/v1/customers            - List customers");
    info!("  GET  /api/v1/customers/:id        - Get customer");
    info!("  GET  /api/v1/orders               - List orders");
    info!("  GET  /api/v1/orders/:id           - Get order");
    info!("  POST /api/v1/auth/login           - Login");
    info!("  POST /api/v1/auth/register        - Register");
    info!("  POST /api/v1/carts/guest          - Create guest cart");
    info!("  GET  /api/v1/carts/me             - Get customer cart");
    info!("  POST /api/v1/carts/merge          - Merge carts");
    info!("  GET  /api/v1/coupons              - List coupons");
    info!("  POST /api/v1/coupons              - Create coupon");
    info!("  POST /api/v1/payments/methods     - Get payment methods");
    info!("  POST /api/v1/payments             - Create payment");
    info!("  GET  /api/v1/payments/:id         - Get payment status");
    info!("  POST /api/v1/payments/:id/complete - Complete payment");
    info!("  POST /api/v1/payments/:id/refund  - Refund payment");

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
    let redis_url = config
        .cache
        .redis_url
        .clone()
        .or_else(|| std::env::var("REDIS_URL").ok());

    if let Some(url) = redis_url {
        info!("Connecting to Redis at {}...", url);
        match RedisPool::new(rcommerce_core::cache::RedisConfig {
            url,
            pool_size: config.cache.redis_pool_size as usize,
            ..Default::default()
        })
        .await
        {
            Ok(pool) => {
                info!("Redis connected successfully");
                Some(pool)
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to connect to Redis: {}. Continuing without cache.",
                    e
                );
                None
            }
        }
    } else {
        info!("Redis not configured. Running without cache.");
        None
    }
}

/// API v1 routes
fn api_routes(app_state: AppState) -> Router<AppState> {
    // Public routes (no auth required)
    let public_routes = Router::new()
        .merge(crate::routes::auth_router())
        // Webhooks are public (signature verification handles security)
        .route(
            "/webhooks/:gateway_id",
            post(crate::routes::payment::handle_webhook),
        );

    // Protected routes (auth required) - middleware applied to each router individually
    let protected_routes = Router::new()
        .merge(crate::routes::product_router())
        .merge(crate::routes::customer_router())
        .merge(crate::routes::order_router())
        .merge(crate::routes::cart_router())
        .merge(crate::routes::coupon_router())
        // Payment routes except webhooks
        .merge(crate::routes::payment::payment_routes())
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    // Admin routes (admin auth required)
    let admin_routes = Router::new()
        .nest("/admin", crate::routes::admin_router())
        .route_layer(middleware::from_fn_with_state(app_state, admin_middleware));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes)
}

async fn health_check() -> &'static str {
    "OK"
}

async fn root() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "name": "R Commerce API",
        "version": "0.1.0",
        "status": "operational",
        "phase": "Phase 1 MVP"
    }))
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
