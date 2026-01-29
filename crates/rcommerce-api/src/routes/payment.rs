//! Payment API Routes
//!
//! Provides endpoints for payment processing with Stripe

use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};

/// Request to create a payment intent
#[derive(Debug, Deserialize)]
pub struct CreatePaymentIntentRequest {
    pub amount: f64,
    pub currency: String,
    pub order_id: Option<String>,
}

/// Payment intent response
#[derive(Debug, Serialize)]
pub struct PaymentIntentResponse {
    pub client_secret: String,
    pub payment_intent_id: String,
    pub amount: f64,
    pub currency: String,
}

/// Create a Stripe payment intent (demo mode)
pub async fn create_payment_intent(
    Json(request): Json<CreatePaymentIntentRequest>,
) -> Json<serde_json::Value> {
    // Demo mode - return a mock client secret
    // In production, this would call Stripe API
    let payment_intent_id = format!("pi_{}", uuid::Uuid::new_v4().to_string().replace("-", "").get(0..24).unwrap_or("demo123"));
    let client_secret = format!("{}_secret_{}", payment_intent_id, uuid::Uuid::new_v4().to_string().replace("-", "").get(0..24).unwrap_or("demo456"));
    
    Json(serde_json::json!({
        "client_secret": client_secret,
        "payment_intent_id": payment_intent_id,
        "amount": request.amount,
        "currency": request.currency,
        "status": "requires_confirmation"
    }))
}

/// Confirm a payment (demo mode)
#[derive(Debug, Deserialize)]
pub struct ConfirmPaymentRequest {
    pub payment_intent_id: String,
    pub payment_method_id: Option<String>,
}

pub async fn confirm_payment(
    Json(request): Json<ConfirmPaymentRequest>,
) -> Json<serde_json::Value> {
    // Demo mode - always succeed
    Json(serde_json::json!({
        "success": true,
        "payment_intent_id": request.payment_intent_id,
        "status": "succeeded",
        "message": "Payment confirmed (demo mode)"
    }))
}

/// Get Stripe publishable key (demo key)
pub async fn get_stripe_config() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "publishable_key": "pk_test_demo_key_for_rcommerce",
        "demo_mode": true
    }))
}

/// Router for payment routes
pub fn router() -> Router {
    Router::new()
        .route("/payments/config", axum::routing::get(get_stripe_config))
        .route("/payments/intent", post(create_payment_intent))
        .route("/payments/confirm", post(confirm_payment))
}
