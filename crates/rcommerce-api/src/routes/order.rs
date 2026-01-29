use axum::{Json, Router, routing::get};
use crate::state::AppState;

/// List orders - Basic Phase 1 implementation
pub async fn list_orders() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "orders": [
            {
                "id": "123e4567-e89b-12d3-a456-426614174010",
                "order_number": "1001",
                "customer_id": "123e4567-e89b-12d3-a456-426614174001",
                "email": "demo@rcommerce.app",
                "total_price": 59.98,
                "currency": "USD",
                "financial_status": "paid",
                "fulfillment_status": "unfulfilled",
                "status": "open",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z"
            }
        ],
        "meta": {
            "total": 1,
            "page": 1,
            "per_page": 20,
            "total_pages": 1
        }
    }))
}

/// Get order by ID - Basic Phase 1 implementation
pub async fn get_order(path: axum::extract::Path<String>) -> Json<serde_json::Value> {
    let _id = path.0; // Extract ID from path but ignore for now
    
    Json(serde_json::json!({
        "order": {
            "id": "123e4567-e89b-12d3-a456-426614174010",
            "order_number": "1001",
            "customer_id": "123e4567-e89b-12d3-a456-426614174001",
            "email": "demo@rcommerce.app",
            "total_price": 59.98,
            "subtotal_price": 49.99,
            "total_tax": 4.99,
            "total_shipping": 5.00,
            "currency": "USD",
            "financial_status": "paid",
            "fulfillment_status": "unfulfilled",
            "status": "open",
            "line_items": [
                {
                    "id": "123e4567-e89b-12d3-a456-426614174020",
                    "product_id": "123e4567-e89b-12d3-a456-426614174000",
                    "title": "Sample Product 1",
                    "quantity": 2,
                    "price": 29.99,
                    "total": 59.98
                }
            ],
            "shipping_address": {
                "first_name": "Demo",
                "last_name": "User",
                "address1": "123 Main St",
                "city": "New York",
                "state": "NY",
                "country": "US",
                "zip": "10001"
            },
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }
    }))
}

/// Router for order routes
pub fn router() -> Router<AppState> {
    axum::Router::new()
        .route("/orders", get(list_orders))
        .route("/orders/:id", get(get_order))
}
