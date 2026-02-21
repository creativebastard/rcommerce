//! Cart API Routes
//!
//! Provides endpoints for shopping cart management including:
//! - Guest cart creation
//! - Customer cart retrieval
//! - Cart item management
//! - Cart merging
//! - Coupon application

use crate::middleware::JwtAuth;
use crate::state::AppState;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use rcommerce_core::{
    models::{AddToCartInput, ApplyCouponInput, CartIdentifier, CartWithItems, UpdateCartItemInput},
    services::cart_service::ProductDetails,
    Error,
};
use serde::Deserialize;

use uuid::Uuid;

/// Create a new guest cart
pub async fn create_guest_cart(
    State(state): State<AppState>,
) -> Result<Json<CartWithItems>, Error> {
    // Generate session token for guest cart
    let session_token = format!("sess_{}", Uuid::new_v4().to_string().replace("-", ""));

    // Get or create cart via service
    let cart = state
        .cart_service
        .get_or_create_cart(CartIdentifier::Session(session_token.clone()), "USD")
        .await?;

    // Return cart with items (empty for new cart)
    let cart_with_items = state.cart_service.get_cart_with_items(cart.id).await?;

    Ok(Json(cart_with_items))
}

/// Get or create customer cart
pub async fn get_customer_cart(
    State(state): State<AppState>,
    Extension(jwt_auth): Extension<JwtAuth>,
) -> Result<Json<CartWithItems>, Error> {
    // Get or create cart for authenticated customer
    let cart = state
        .cart_service
        .get_or_create_cart(CartIdentifier::Customer(jwt_auth.customer_id), "USD")
        .await?;

    // Return cart with items
    let cart_with_items = state.cart_service.get_cart_with_items(cart.id).await?;

    Ok(Json(cart_with_items))
}

/// Get cart by ID
pub async fn get_cart(
    State(state): State<AppState>,
    Path(cart_id): Path<Uuid>,
) -> Result<Json<CartWithItems>, Error> {
    // Get cart with items from service
    let cart_with_items = state.cart_service.get_cart_with_items(cart_id).await?;

    Ok(Json(cart_with_items))
}

/// Request body for adding item to cart
#[derive(Debug, Deserialize)]
pub struct AddItemRequest {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub quantity: i32,
}

/// Add item to cart
pub async fn add_item_to_cart(
    State(state): State<AppState>,
    Path(cart_id): Path<Uuid>,
    Json(request): Json<AddItemRequest>,
) -> Result<Json<rcommerce_core::models::CartItem>, Error> {
    // Validate quantity
    if request.quantity <= 0 {
        return Err(Error::validation("Quantity must be greater than 0"));
    }

    // Get product details
    let product_detail = state
        .product_service
        .get_product(request.product_id)
        .await?
        .ok_or_else(|| Error::not_found("Product not found"))?;

    // Build product details for cart service
    let product_details = if let Some(variant_id) = request.variant_id {
        // Find the variant
        let variant = product_detail
            .variants
            .iter()
            .find(|v| v.id == variant_id)
            .ok_or_else(|| Error::not_found("Product variant not found"))?;

        ProductDetails {
            unit_price: variant.price,
            original_price: variant.compare_at_price.unwrap_or(variant.price),
            sku: variant.sku.clone(),
            title: product_detail.product.title.clone(),
            variant_title: Some(variant.title.clone()),
            image_url: product_detail.images.first().map(|i| i.src.clone()),
            requires_shipping: variant.requires_shipping,
            is_gift_card: false,
        }
    } else {
        // Use product-level details
        ProductDetails {
            unit_price: product_detail.product.price,
            original_price: product_detail
                .product
                .compare_at_price
                .unwrap_or(product_detail.product.price),
            sku: product_detail.product.sku.clone(),
            title: product_detail.product.title.clone(),
            variant_title: None,
            image_url: product_detail.images.first().map(|i| i.src.clone()),
            requires_shipping: product_detail.product.requires_shipping,
            is_gift_card: product_detail.product.product_type
                == rcommerce_core::models::ProductType::Digital,
        }
    };

    // Create input for adding to cart
    let input = AddToCartInput {
        product_id: request.product_id,
        variant_id: request.variant_id,
        quantity: request.quantity,
        custom_attributes: None,
    };

    // Add item to cart via service
    let cart_item = state
        .cart_service
        .add_item(cart_id, input, product_details)
        .await?;

    Ok(Json(cart_item))
}

/// Request body for updating cart item
#[derive(Debug, Deserialize)]
pub struct UpdateItemRequest {
    pub quantity: i32,
}

/// Update cart item
pub async fn update_cart_item(
    State(state): State<AppState>,
    Path((cart_id, item_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateItemRequest>,
) -> Result<Json<rcommerce_core::models::CartItem>, Error> {
    // Validate quantity (0 is allowed - removes item)
    if request.quantity < 0 {
        return Err(Error::validation("Quantity cannot be negative"));
    }

    // Create update input
    let input = UpdateCartItemInput {
        quantity: request.quantity,
        custom_attributes: None,
    };

    // Update item via service
    let cart_item = state
        .cart_service
        .update_item(cart_id, item_id, input)
        .await?;

    Ok(Json(cart_item))
}

/// Remove item from cart
pub async fn remove_cart_item(
    State(state): State<AppState>,
    Path((cart_id, item_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, Error> {
    // Remove item via service
    state.cart_service.remove_item(cart_id, item_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Clear all items from cart
pub async fn clear_cart(
    State(state): State<AppState>,
    Path(cart_id): Path<Uuid>,
) -> Result<StatusCode, Error> {
    // Clear cart via service
    state.cart_service.clear_cart(cart_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Request body for merging carts
#[derive(Debug, Deserialize)]
pub struct MergeCartRequest {
    pub session_token: String,
}

/// Merge guest cart into customer cart
pub async fn merge_carts(
    State(state): State<AppState>,
    Extension(jwt_auth): Extension<JwtAuth>,
    Json(request): Json<MergeCartRequest>,
) -> Result<Json<CartWithItems>, Error> {
    // Merge carts via service
    let cart = state
        .cart_service
        .merge_carts(&request.session_token, jwt_auth.customer_id)
        .await?;

    // Return merged cart with items
    let cart_with_items = state.cart_service.get_cart_with_items(cart.id).await?;

    Ok(Json(cart_with_items))
}

/// Request body for applying coupon
#[derive(Debug, Deserialize)]
pub struct ApplyCouponRequest {
    pub coupon_code: String,
}

/// Apply coupon to cart
pub async fn apply_coupon(
    State(state): State<AppState>,
    Path(cart_id): Path<Uuid>,
    Json(request): Json<ApplyCouponRequest>,
) -> Result<Json<CartWithItems>, Error> {
    // Validate coupon code is not empty
    if request.coupon_code.trim().is_empty() {
        return Err(Error::validation("Coupon code is required"));
    }

    // Apply coupon via service
    let input = ApplyCouponInput {
        coupon_code: request.coupon_code.trim().to_uppercase(),
    };

    let cart = state.cart_service.apply_coupon(cart_id, input).await?;

    // Return cart with items
    let cart_with_items = state.cart_service.get_cart_with_items(cart.id).await?;

    Ok(Json(cart_with_items))
}

/// Remove coupon from cart
pub async fn remove_coupon(
    State(state): State<AppState>,
    Path(cart_id): Path<Uuid>,
) -> Result<Json<CartWithItems>, Error> {
    // Remove coupon via service
    let cart = state.cart_service.remove_coupon(cart_id).await?;

    // Return cart with items
    let cart_with_items = state.cart_service.get_cart_with_items(cart.id).await?;

    Ok(Json(cart_with_items))
}

/// Delete cart
pub async fn delete_cart(
    State(state): State<AppState>,
    Path(cart_id): Path<Uuid>,
) -> Result<StatusCode, Error> {
    // Delete cart via service
    state.cart_service.delete_cart(cart_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Router for public cart routes (no auth required)
pub fn public_router() -> Router<AppState> {
    Router::new()
        // Guest cart - no auth required
        .route("/carts/guest", post(create_guest_cart))
        // Get cart by ID - no auth required (identified by cart_id)
        .route("/carts/:cart_id", get(get_cart))
}

/// Router for protected cart routes (auth required)
pub fn protected_router() -> Router<AppState> {
    Router::new()
        // Customer cart - requires auth
        .route("/carts/me", get(get_customer_cart))
        // Cart modification - requires auth
        .route(
            "/carts/:cart_id",
            delete(delete_cart),
        )
        // Cart items - requires auth
        .route(
            "/carts/:cart_id/items",
            post(add_item_to_cart).delete(clear_cart),
        )
        .route(
            "/carts/:cart_id/items/:item_id",
            put(update_cart_item).delete(remove_cart_item),
        )
        // Cart merge - requires auth
        .route("/carts/merge", post(merge_carts))
        // Coupons - requires auth
        .route(
            "/carts/:cart_id/coupon",
            post(apply_coupon).delete(remove_coupon),
        )
}

/// Router for cart routes (combines public and protected - used for backward compatibility)
pub fn router() -> Router<AppState> {
    // Note: This router is kept for backward compatibility
    // The server should use public_router() and protected_router() separately
    // with appropriate middleware
    public_router().merge(protected_router())
}
