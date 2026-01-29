use axum::{Json, Router, routing::post};
use crate::state::AppState;

/// Login endpoint - Basic Phase 1 implementation
pub async fn login(Json(_payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test_token",
        "token_type": "Bearer",
        "expires_in": 3600
    }))
}

/// Register endpoint - Basic Phase 1 implementation
pub async fn register(Json(_payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "customer": {
            "id": "123e4567-e89b-12d3-a456-426614174001",
            "email": "demo@rcommerce.app",
            "first_name": "Demo",
            "last_name": "User",
            "created_at": "2024-01-01T00:00:00Z"
        }
    }))
}

/// Router for auth routes
pub fn router() -> Router<AppState> {
    axum::Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
}
