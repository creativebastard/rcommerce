//! Checkout routes for order creation
//!
//! Provides endpoints for the complete checkout flow:
//! - POST /checkout/initiate - Start checkout, calculate tax and shipping rates
//! - POST /checkout/shipping - Select shipping method
//! - POST /checkout/complete - Complete checkout, create order and process payment

use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Extension, Json, Router,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;
use crate::middleware::JwtAuth;

// Import core checkout types
use rcommerce_core::services::{
    CheckoutSummary, CheckoutResult,
    InitiateCheckoutRequest, SelectShippingRequest, CompleteCheckoutRequest,
};
use rcommerce_core::models::Address;
use rcommerce_core::payment::{PaymentMethod, CardDetails};

/// Request to initiate checkout
#[derive(Debug, Deserialize)]
pub struct InitiateCheckoutApiRequest {
    pub cart_id: Uuid,
    pub shipping_address: Address,
    pub billing_address: Option<Address>,
    pub vat_id: Option<String>,
    pub currency: Option<String>,
}

/// Request to select shipping
#[derive(Debug, Deserialize)]
pub struct SelectShippingApiRequest {
    pub cart_id: Uuid,
    pub shipping_rate: ShippingRateResponse,
}

/// Request to complete checkout
#[derive(Debug, Deserialize)]
pub struct CompleteCheckoutApiRequest {
    pub cart_id: Uuid,
    pub shipping_address: Address,
    pub billing_address: Option<Address>,
    pub payment_method: PaymentMethodRequest,
    pub customer_email: String,
    pub vat_id: Option<String>,
    pub notes: Option<String>,
    pub selected_shipping_rate: ShippingRateResponse,
}

/// Payment method request
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaymentMethodRequest {
    Card { 
        number: String,
        exp_month: u32,
        exp_year: u32,
        cvc: String,
        name: String,
    },
    GooglePay,
    ApplePay,
    WeChatPay,
    AliPay,
    BankTransfer,
    CashOnDelivery,
}

impl From<PaymentMethodRequest> for PaymentMethod {
    fn from(req: PaymentMethodRequest) -> Self {
        match req {
            PaymentMethodRequest::Card { number, exp_month, exp_year, cvc, name } => {
                PaymentMethod::Card(CardDetails {
                    number,
                    exp_month,
                    exp_year,
                    cvc,
                    name,
                })
            },
            PaymentMethodRequest::GooglePay => PaymentMethod::GooglePay,
            PaymentMethodRequest::ApplePay => PaymentMethod::ApplePay,
            PaymentMethodRequest::WeChatPay => PaymentMethod::WeChatPay,
            PaymentMethodRequest::AliPay => PaymentMethod::AliPay,
            PaymentMethodRequest::BankTransfer => PaymentMethod::BankTransfer,
            PaymentMethodRequest::CashOnDelivery => PaymentMethod::CashOnDelivery,
        }
    }
}

/// Shipping rate response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingRateResponse {
    pub provider_id: String,
    pub carrier: String,
    pub service_code: String,
    pub service_name: String,
    pub rate: Decimal,
    pub currency: String,
    pub delivery_days: Option<i32>,
    pub total_cost: Decimal,
}

impl From<rcommerce_core::shipping::ShippingRate> for ShippingRateResponse {
    fn from(rate: rcommerce_core::shipping::ShippingRate) -> Self {
        Self {
            provider_id: rate.provider_id,
            carrier: rate.carrier,
            service_code: rate.service_code,
            service_name: rate.service_name,
            rate: rate.rate,
            currency: rate.currency,
            delivery_days: rate.delivery_days,
            total_cost: rate.total_cost,
        }
    }
}

impl From<ShippingRateResponse> for rcommerce_core::shipping::ShippingRate {
    fn from(rate: ShippingRateResponse) -> Self {
        Self {
            provider_id: rate.provider_id,
            carrier: rate.carrier,
            service_code: rate.service_code,
            service_name: rate.service_name,
            rate: rate.rate,
            currency: rate.currency,
            delivery_days: rate.delivery_days,
            delivery_date: None,
            estimated: true,
            insurance_fee: None,
            fuel_surcharge: None,
            handling_fee: None,
            other_fees: std::collections::HashMap::new(),
            total_cost: rate.total_cost,
        }
    }
}

/// Tax breakdown item
#[derive(Debug, Clone, Serialize)]
pub struct TaxBreakdownResponse {
    pub tax_zone_name: String,
    pub tax_rate_name: String,
    pub rate: Decimal,
    pub taxable_amount: Decimal,
    pub tax_amount: Decimal,
}

/// Cart item in checkout response
#[derive(Debug, Clone, Serialize)]
pub struct CheckoutItemResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub title: String,
    pub sku: Option<String>,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub total: Decimal,
}

/// Checkout summary response
#[derive(Debug, Clone, Serialize)]
pub struct CheckoutSummaryResponse {
    pub cart_id: Uuid,
    pub items: Vec<CheckoutItemResponse>,
    pub subtotal: Decimal,
    pub discount_total: Decimal,
    pub shipping_total: Decimal,
    pub shipping_tax: Decimal,
    pub item_tax: Decimal,
    pub tax_total: Decimal,
    pub total: Decimal,
    pub currency: String,
    pub available_shipping_rates: Vec<ShippingRateResponse>,
    pub selected_shipping_rate: Option<ShippingRateResponse>,
    pub tax_breakdown: Vec<TaxBreakdownResponse>,
    pub vat_id_valid: Option<bool>,
}

impl From<CheckoutSummary> for CheckoutSummaryResponse {
    fn from(summary: CheckoutSummary) -> Self {
        Self {
            cart_id: summary.cart_id,
            items: summary.items.into_iter().map(|item| CheckoutItemResponse {
                id: item.id,
                product_id: item.product_id,
                variant_id: item.variant_id,
                title: item.title,
                sku: item.sku,
                quantity: item.quantity,
                unit_price: item.unit_price,
                total: item.total,
            }).collect(),
            subtotal: summary.subtotal,
            discount_total: summary.discount_total,
            shipping_total: summary.shipping_total,
            shipping_tax: summary.shipping_tax,
            item_tax: summary.item_tax,
            tax_total: summary.tax_total,
            total: summary.total,
            currency: format!("{:?}", summary.currency),
            available_shipping_rates: summary.available_shipping_rates.into_iter().map(Into::into).collect(),
            selected_shipping_rate: summary.selected_shipping_rate.map(Into::into),
            tax_breakdown: summary.tax_breakdown.into_iter().map(|tb| TaxBreakdownResponse {
                tax_zone_name: tb.tax_zone_name,
                tax_rate_name: tb.tax_rate_name,
                rate: tb.rate,
                taxable_amount: tb.taxable_amount,
                tax_amount: tb.tax_amount,
            }).collect(),
            vat_id_valid: summary.vat_id_valid,
        }
    }
}

/// Order item response
#[derive(Debug, Clone, Serialize)]
pub struct OrderItemResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub title: String,
    pub sku: Option<String>,
    pub quantity: i32,
    pub price: Decimal,
    pub total: Decimal,
    pub tax_amount: Decimal,
}

/// Order response
#[derive(Debug, Clone, Serialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub order_number: String,
    pub customer_id: Option<Uuid>,
    pub customer_email: String,
    pub status: String,
    pub payment_status: String,
    pub fulfillment_status: String,
    pub currency: String,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub shipping_total: Decimal,
    pub discount_total: Decimal,
    pub total: Decimal,
    pub items: Vec<OrderItemResponse>,
    pub created_at: String,
    pub metadata: serde_json::Value,
}

/// Checkout result response
#[derive(Debug, Clone, Serialize)]
pub struct CheckoutResultResponse {
    pub order: OrderResponse,
    pub payment_id: String,
    pub total_charged: Decimal,
    pub currency: String,
}

impl From<CheckoutResult> for CheckoutResultResponse {
    fn from(result: CheckoutResult) -> Self {
        Self {
            order: OrderResponse {
                id: result.order.id,
                order_number: result.order.order_number,
                customer_id: result.order.customer_id,
                customer_email: result.order.customer_email.clone(),
                status: format!("{:?}", result.order.status).to_lowercase(),
                payment_status: format!("{:?}", result.order.payment_status).to_lowercase(),
                fulfillment_status: format!("{:?}", result.order.fulfillment_status).to_lowercase(),
                currency: result.order.currency.to_string(),
                subtotal: result.order.subtotal,
                tax_total: result.order.tax_total,
                shipping_total: result.order.shipping_total,
                discount_total: result.order.discount_total,
                total: result.order.total,
                items: vec![], // Order doesn't have items directly - they need to be fetched separately
                created_at: result.order.created_at.to_rfc3339(),
                metadata: serde_json::json!({}),
            },
            payment_id: result.payment_id,
            total_charged: result.total_charged,
            currency: format!("{:?}", result.currency),
        }
    }
}

/// Initiate checkout endpoint
/// 
/// Calculates totals, tax, and available shipping rates for the customer's cart
pub async fn initiate_checkout(
    State(state): State<AppState>,
    Extension(auth): Extension<JwtAuth>,
    Json(request): Json<InitiateCheckoutApiRequest>,
) -> Result<Json<CheckoutSummaryResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Build the core request
    let currency = request.currency
        .and_then(|c| c.parse().ok())
        .unwrap_or_default();
    
    let core_request = InitiateCheckoutRequest {
        cart_id: request.cart_id,
        shipping_address: request.shipping_address,
        billing_address: request.billing_address,
        vat_id: request.vat_id,
        customer_id: Some(auth.customer_id),
        currency: Some(currency),
    };

    // Call checkout service
    match state.checkout_service.initiate_checkout(core_request).await {
        Ok(summary) => {
            let response: CheckoutSummaryResponse = summary.into();
            Ok(Json(response))
        }
        Err(e) => {
            tracing::error!("Failed to initiate checkout: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": e.to_string()})),
            ))
        }
    }
}

/// Select shipping endpoint
/// 
/// Updates the checkout with the selected shipping method and recalculates totals
pub async fn select_shipping(
    State(state): State<AppState>,
    Extension(_auth): Extension<JwtAuth>,
    Json(request): Json<SelectShippingApiRequest>,
) -> Result<Json<CheckoutSummaryResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Convert shipping rate and create package estimate
    let shipping_rate: rcommerce_core::shipping::ShippingRate = request.shipping_rate.into();
    
    // Estimate package dimensions (simplified - in production, calculate from cart items)
    let package = rcommerce_core::shipping::Package {
        weight: Decimal::from(1), // Default weight, should be calculated
        weight_unit: "kg".to_string(),
        length: Some(Decimal::from(30)),
        width: Some(Decimal::from(20)),
        height: Some(Decimal::from(15)),
        dimension_unit: Some("cm".to_string()),
        predefined_package: None,
    };

    let core_request = SelectShippingRequest {
        cart_id: request.cart_id,
        shipping_rate,
        package,
    };

    // Call checkout service
    match state.checkout_service.select_shipping(core_request).await {
        Ok(summary) => {
            let response: CheckoutSummaryResponse = summary.into();
            Ok(Json(response))
        }
        Err(e) => {
            tracing::error!("Failed to select shipping: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": e.to_string()})),
            ))
        }
    }
}

/// Complete checkout endpoint
/// 
/// Finalizes the checkout, creates an order, and processes payment
pub async fn complete_checkout(
    State(state): State<AppState>,
    Extension(auth): Extension<JwtAuth>,
    Json(request): Json<CompleteCheckoutApiRequest>,
) -> Result<(StatusCode, Json<CheckoutResultResponse>), (StatusCode, Json<serde_json::Value>)> {
    // Validate email
    if request.customer_email.is_empty() || !request.customer_email.contains('@') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid email address"})),
        ));
    }

    // Build the core request
    let core_request = CompleteCheckoutRequest {
        cart_id: request.cart_id,
        shipping_address: request.shipping_address,
        billing_address: request.billing_address,
        payment_method: request.payment_method.into(),
        customer_email: request.customer_email,
        customer_id: Some(auth.customer_id),
        vat_id: request.vat_id,
        notes: request.notes,
        selected_shipping_rate: request.selected_shipping_rate.into(),
    };

    // Call checkout service
    match state.checkout_service.complete_checkout(core_request).await {
        Ok(result) => {
            let response: CheckoutResultResponse = result.into();
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to complete checkout: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": e.to_string()})),
            ))
        }
    }
}

/// Router for checkout routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/checkout/initiate", post(initiate_checkout))
        .route("/checkout/shipping", post(select_shipping))
        .route("/checkout/complete", post(complete_checkout))
}
