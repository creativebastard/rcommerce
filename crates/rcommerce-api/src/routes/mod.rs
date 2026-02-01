pub mod admin;
pub mod auth;
pub mod cart;
pub mod coupon;
pub mod customer;
pub mod order;
pub mod payment;
pub mod product;

pub use admin::router as admin_router;
pub use auth::router as auth_router;
pub use cart::router as cart_router;
pub use coupon::router as coupon_router;
pub use customer::router as customer_router;
pub use order::router as order_router;
pub use payment::router as payment_router;
pub use product::router as product_router;

use crate::state::AppState;
use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

/// Create the main API router with all routes
pub fn create_router(app_state: AppState) -> Router<AppState> {
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health_check))
        .route("/", get(api_info))
        .nest("/api/v1", api_v1_routes())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}

/// API v1 routes
fn api_v1_routes() -> Router<AppState> {
    Router::new()
        .merge(product_router())
        .merge(customer_router())
        .merge(order_router())
        .merge(auth_router())
        .merge(cart_router())
        .merge(coupon_router())
        .merge(payment_router())
        .merge(admin_router())
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

/// API info endpoint
async fn api_info() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "name": "R Commerce API",
        "version": "1.0.0",
        "status": "operational"
    }))
}
