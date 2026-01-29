use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use rcommerce_core::{Result, Config};

pub async fn run(config: Config) -> Result<()> {
    let addr = SocketAddr::from((
        config.server.host.parse::<std::net::IpAddr>()
            .map_err(|e| rcommerce_core::Error::Config(format!("Invalid host: {}", e)))?,
        config.server.port
    ));
    
    // Build API v1 routes
    let api_v1 = Router::new()
        .merge(crate::routes::product_router())
        .merge(crate::routes::customer_router())
        .merge(crate::routes::order_router())
        .merge(crate::routes::auth_router())
        .merge(crate::routes::cart_router())
        .merge(crate::routes::coupon_router());
    
    // Configure CORS for demo frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Build main router with simplified Phase 1 routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/", get(root))
        .nest("/api/v1", api_v1)
        .layer(cors);
    
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
    
    // Start server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| rcommerce_core::Error::Network(e.to_string()))?;
    
    axum::serve(listener, app)
        .await
        .map_err(|e| rcommerce_core::Error::Network(e.to_string()))?;
    
    Ok(())
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
