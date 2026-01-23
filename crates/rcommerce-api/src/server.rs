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
    
    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/", get(root));
    
    info!("R Commerce API server listening on {}", addr);
    info!("Configuration: {:?}", config.server);
    
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
    "R Commerce API"
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