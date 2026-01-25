use axum::{Router, routing::get};
use std::net::SocketAddr;
use tracing::info;

use rcommerce_core::{Result, Config};

pub async fn run(config: Config) -> Result<()> {
    let addr = SocketAddr::from((
        config.server.host.parse::<std::net::IpAddr>()
            .map_err(|e| rcommerce_core::Error::Config(format!("Invalid host: {}", e)))?,
        config.server.port
    ));
    
    // Build router with simplified Phase 1 routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/", get(root))
        .nest("/api/v1", crate::routes::product::router())
        .nest("/api/v1", crate::routes::customer::router());
    
    info!("R Commerce API server listening on {}", addr);
    info!("Available routes:");
    info!("  GET  /health                      - Health check");
    info!("  GET  /                            - API info");
    info!("  GET  /api/v1/products            - List products");
    info!("  GET  /api/v1/products/:id        - Get product");
    info!("  GET  /api/v1/customers           - List customers");
    info!("  GET  /api/v1/customers/:id       - Get customer");
    
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