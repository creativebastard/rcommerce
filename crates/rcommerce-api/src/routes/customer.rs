use crate::state::AppState;
use axum::{extract::State, routing::get, Json, Router};
use rcommerce_core::Error;

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

/// Get current customer profile (requires auth)
pub async fn get_current_customer(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, Error> {
    // Extract token from Authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::unauthorized("Missing authorization header"))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| Error::unauthorized("Invalid authorization header format"))?;

    // Verify token and get customer ID
    let claims = state.auth_service.verify_token(token)?;
    let customer_id = claims.sub;

    // Fetch customer from database
    let customer = state
        .customer_service
        .get_customer(customer_id)
        .await?
        .ok_or_else(|| Error::not_found("Customer not found"))?;

    Ok(Json(serde_json::json!({
        "customer": {
            "id": customer.customer.id,
            "email": customer.customer.email,
            "first_name": customer.customer.first_name,
            "last_name": customer.customer.last_name,
            "phone": customer.customer.phone,
            "accepts_marketing": customer.customer.accepts_marketing,
            "tax_exempt": customer.customer.tax_exempt,
            "currency": customer.customer.currency.to_string(),
            "created_at": customer.customer.created_at,
            "updated_at": customer.customer.updated_at,
        }
    })))
}

/// Router for customer routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/customers", get(list_customers))
        .route("/customers/me", get(get_current_customer))
        .route("/customers/:id", get(get_customer))
}
