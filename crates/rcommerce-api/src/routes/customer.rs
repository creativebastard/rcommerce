use axum::{Json, Router, routing::get};
use crate::state::AppState;

/// List customers - Basic Phase 1 implementation
pub async fn list_customers() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "customers": [
            {
                "id": "123e4567-e89b-12d3-a456-426614174001",
                "email": "demo@rcommerce.app",
                "first_name": "Demo",
                "last_name": "User",
                "phone": "+1-555-0123",
                "accepts_marketing": true,
                "tax_exempt": false,
                "currency": "USD",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
                "confirmed_at": "2024-01-01T00:00:00Z"
            },
            {
                "id": "123e4567-e89b-12d3-a456-426614174002",
                "email": "test@example.com",
                "first_name": "Test",
                "last_name": "Customer",
                "phone": null,
                "accepts_marketing": false,
                "tax_exempt": false,
                "currency": "USD",
                "created_at": "2024-01-02T00:00:00Z",
                "updated_at": "2024-01-02T00:00:00Z",
                "confirmed_at": null
            }
        ],
        "meta": {
            "total": 2,
            "page": 1,
            "per_page": 20,
            "total_pages": 1
        }
    }))
}

/// Get customer by ID - Basic Phase 1 implementation
pub async fn get_customer(path: axum::extract::Path<String>) -> Json<serde_json::Value> {
    let _id = path.0; // Extract ID from path but ignore for now
    
    Json(serde_json::json!({
        "customer": {
            "id": "123e4567-e89b-12d3-a456-426614174001",
            "email": "demo@rcommerce.app",
            "first_name": "Demo",
            "last_name": "User",
            "phone": "+1-555-0123",
            "accepts_marketing": true,
            "tax_exempt": false,
            "currency": "USD",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "confirmed_at": "2024-01-01T00:00:00Z"
        },
        "addresses": [
            {
                "id": "123e4567-e89b-12d3-a456-426614174003",
                "first_name": "Demo",
                "last_name": "User",
                "company": null,
                "phone": "+1-555-0123",
                "address1": "123 Main St",
                "address2": "Apt 4B",
                "city": "New York",
                "state": "NY",
                "country": "US",
                "zip": "10001",
                "is_default_shipping": true,
                "is_default_billing": true,
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z"
            }
        ]
    }))
}

/// Router for customer routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/customers", get(list_customers))
        .route("/customers/:id", get(get_customer))
}