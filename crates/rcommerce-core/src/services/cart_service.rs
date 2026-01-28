//! Cart Service
//!
//! Handles all cart-related business logic.

use std::sync::Arc;

use chrono::{Duration, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    Error, Result,
    models::{
        Cart, CartItem, CartWithItems, CartIdentifier, AddToCartInput, 
        UpdateCartItemInput, ApplyCouponInput,
    },
    repository::{CartRepository, CouponRepository},
    services::CouponService,
};

/// Cart service for managing shopping carts
pub struct CartService {
    cart_repo: Arc<dyn CartRepository>,
    coupon_repo: Arc<dyn CouponRepository>,
    coupon_service: Arc<CouponService>,
}

impl CartService {
    /// Create a new cart service
    pub fn new(
        cart_repo: Arc<dyn CartRepository>,
        coupon_repo: Arc<dyn CouponRepository>,
        coupon_service: Arc<CouponService>,
    ) -> Self {
        Self {
            cart_repo,
            coupon_repo,
            coupon_service,
        }
    }

    /// Get or create a cart for a customer or session
    pub async fn get_or_create_cart(
        &self,
        identifier: CartIdentifier,
        currency: &str,
    ) -> Result<Cart> {
        // Try to find existing cart
        let existing_cart = match &identifier {
            CartIdentifier::Customer(customer_id) => {
                self.cart_repo.find_active_by_customer(*customer_id).await?
            }
            CartIdentifier::Session(session_token) => {
                self.cart_repo.find_active_by_session(session_token).await?
            }
            CartIdentifier::CartId(cart_id) => {
                self.cart_repo.find_by_id(*cart_id).await?
            }
        };

        if let Some(cart) = existing_cart {
            // Update expiration
            self.cart_repo.update_expiration(cart.id, Utc::now() + Duration::days(30)).await?;
            return Ok(cart);
        }

        // Create new cart
        let cart = self.create_cart(identifier, currency).await?;
        Ok(cart)
    }

    /// Create a new cart
    async fn create_cart(&self, identifier: CartIdentifier, currency: &str) -> Result<Cart> {
        let (customer_id, session_token) = match identifier {
            CartIdentifier::Customer(id) => (Some(id), None),
            CartIdentifier::Session(token) => (None, Some(token)),
            CartIdentifier::CartId(_) => (None, None),
        };

        let cart = Cart {
            id: Uuid::new_v4(),
            customer_id,
            session_token,
            currency: currency.parse().unwrap_or(crate::models::Currency::USD),
            subtotal: Decimal::ZERO,
            discount_total: Decimal::ZERO,
            tax_total: Decimal::ZERO,
            shipping_total: Decimal::ZERO,
            total: Decimal::ZERO,
            coupon_code: None,
            email: None,
            shipping_address_id: None,
            billing_address_id: None,
            shipping_method: None,
            notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            expires_at: Some(Utc::now() + Duration::days(30)),
            converted_to_order: false,
            order_id: None,
        };

        self.cart_repo.create(&cart).await?;
        Ok(cart)
    }

    /// Get cart with items
    pub async fn get_cart_with_items(&self, cart_id: Uuid) -> Result<CartWithItems> {
        let cart = self.cart_repo
            .find_by_id(cart_id)
            .await?
            .ok_or_else(|| Error::not_found("Cart not found"))?;

        let items = self.cart_repo.get_items(cart_id).await?;

        Ok(CartWithItems { cart, items })
    }

    /// Add item to cart with product details
    /// Note: Product validation should be done by the caller (API layer)
    pub async fn add_item(
        &self, 
        cart_id: Uuid, 
        input: AddToCartInput,
        product_details: ProductDetails,
    ) -> Result<CartItem> {
        // Verify cart exists and is not converted
        let cart = self.cart_repo
            .find_by_id(cart_id)
            .await?
            .ok_or_else(|| Error::not_found("Cart not found"))?;

        if cart.converted_to_order {
            return Err(Error::validation("Cart has already been converted to an order"));
        }

        // Check if item already exists in cart
        if let Some(mut existing_item) = self.cart_repo.find_item(cart_id, input.product_id, input.variant_id).await? {
            // Update quantity
            existing_item.quantity += input.quantity;
            existing_item.calculate_totals();
            self.cart_repo.update_item(&existing_item).await?;
            
            // Recalculate cart totals
            self.recalculate_cart(cart_id).await?;
            
            return Ok(existing_item);
        }

        // Create new cart item
        let mut item = CartItem {
            id: Uuid::new_v4(),
            cart_id,
            product_id: input.product_id,
            variant_id: input.variant_id,
            quantity: input.quantity,
            unit_price: product_details.unit_price,
            original_price: product_details.original_price,
            subtotal: Decimal::ZERO,
            discount_amount: Decimal::ZERO,
            total: Decimal::ZERO,
            sku: product_details.sku,
            title: product_details.title,
            variant_title: product_details.variant_title,
            image_url: product_details.image_url,
            requires_shipping: product_details.requires_shipping,
            is_gift_card: product_details.is_gift_card,
            custom_attributes: input.custom_attributes,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        item.calculate_totals();
        self.cart_repo.add_item(&item).await?;

        // Recalculate cart totals
        self.recalculate_cart(cart_id).await?;

        Ok(item)
    }

    /// Update cart item quantity
    pub async fn update_item(&self, cart_id: Uuid, item_id: Uuid, input: UpdateCartItemInput) -> Result<CartItem> {
        let mut item = self.cart_repo
            .find_item_by_id(item_id)
            .await?
            .ok_or_else(|| Error::not_found("Cart item not found"))?;

        if item.cart_id != cart_id {
            return Err(Error::validation("Item does not belong to cart"));
        }

        if input.quantity == 0 {
            // Remove item
            self.cart_repo.remove_item(item_id).await?;
            self.recalculate_cart(cart_id).await?;
            return Ok(item);
        } else {
            item.quantity = input.quantity;
            item.custom_attributes = input.custom_attributes;
            item.calculate_totals();
            self.cart_repo.update_item(&item).await?;
            self.recalculate_cart(cart_id).await?;
            return Ok(item);
        }
    }

    /// Remove item from cart
    pub async fn remove_item(&self, cart_id: Uuid, item_id: Uuid) -> Result<()> {
        let item = self.cart_repo
            .find_item_by_id(item_id)
            .await?
            .ok_or_else(|| Error::not_found("Cart item not found"))?;

        if item.cart_id != cart_id {
            return Err(Error::validation("Item does not belong to cart"));
        }

        self.cart_repo.remove_item(item_id).await?;
        self.recalculate_cart(cart_id).await?;
        Ok(())
    }

    /// Apply coupon to cart
    pub async fn apply_coupon(&self, cart_id: Uuid, input: ApplyCouponInput) -> Result<Cart> {
        let cart = self.cart_repo
            .find_by_id(cart_id)
            .await?
            .ok_or_else(|| Error::not_found("Cart not found"))?;

        if cart.converted_to_order {
            return Err(Error::validation("Cart has already been converted to an order"));
        }

        // Validate coupon
        let validation = self.coupon_service.validate_coupon(&input.coupon_code, cart_id).await?;
        
        if !validation.is_valid() {
            return Err(Error::validation(
                validation.error_message().unwrap_or_else(|| "Invalid coupon".to_string())
            ));
        }

        // Apply coupon
        let mut updated_cart = cart;
        updated_cart.coupon_code = Some(input.coupon_code);
        self.cart_repo.update(&updated_cart).await?;

        // Recalculate with discount
        self.recalculate_cart(cart_id).await?;

        // Get updated cart
        self.cart_repo
            .find_by_id(cart_id)
            .await?
            .ok_or_else(|| Error::not_found("Cart not found"))
    }

    /// Remove coupon from cart
    pub async fn remove_coupon(&self, cart_id: Uuid) -> Result<Cart> {
        let mut cart = self.cart_repo
            .find_by_id(cart_id)
            .await?
            .ok_or_else(|| Error::not_found("Cart not found"))?;

        cart.coupon_code = None;
        self.cart_repo.update(&cart).await?;

        // Recalculate without discount
        self.recalculate_cart(cart_id).await?;

        // Get updated cart
        self.cart_repo
            .find_by_id(cart_id)
            .await?
            .ok_or_else(|| Error::not_found("Cart not found"))
    }

    /// Recalculate cart totals
    async fn recalculate_cart(&self, cart_id: Uuid) -> Result<()> {
        let items = self.cart_repo.get_items(cart_id).await?;
        let mut cart = self.cart_repo
            .find_by_id(cart_id)
            .await?
            .ok_or_else(|| Error::not_found("Cart not found"))?;

        // Calculate subtotals
        cart.subtotal = items.iter().map(|i| i.subtotal).sum();
        
        // Calculate discount if coupon applied
        cart.discount_total = if let Some(ref coupon_code) = cart.coupon_code {
            if let Ok(coupon) = self.coupon_repo.find_by_code(coupon_code).await {
                if let Some(coupon) = coupon {
                    self.coupon_service.calculate_discount(&coupon, cart.subtotal, &items).await?
                } else {
                    Decimal::ZERO
                }
            } else {
                Decimal::ZERO
            }
        } else {
            Decimal::ZERO
        };

        // Apply discount to items proportionally
        for mut item in items {
            let item_ratio = if cart.subtotal > Decimal::ZERO {
                item.subtotal / cart.subtotal
            } else {
                Decimal::ZERO
            };
            item.discount_amount = (cart.discount_total * item_ratio).round_dp(2);
            item.total = item.subtotal - item.discount_amount;
            self.cart_repo.update_item(&item).await?;
        }

        // Calculate final total
        cart.total = cart.subtotal - cart.discount_total + cart.tax_total + cart.shipping_total;
        cart.updated_at = Utc::now();

        self.cart_repo.update(&cart).await?;
        Ok(())
    }

    /// Merge guest cart into customer cart
    pub async fn merge_carts(&self, session_token: &str, customer_id: Uuid) -> Result<Cart> {
        let guest_cart = self.cart_repo
            .find_active_by_session(session_token)
            .await?;

        let customer_cart = self.cart_repo
            .find_active_by_customer(customer_id)
            .await?;

        match (guest_cart, customer_cart) {
            (Some(guest), Some(customer)) => {
                // Merge items from guest cart into customer cart
                let guest_items = self.cart_repo.get_items(guest.id).await?;
                
                for item in guest_items {
                    // Add item to customer cart
                    let new_item = CartItem {
                        id: Uuid::new_v4(),
                        cart_id: customer.id,
                        product_id: item.product_id,
                        variant_id: item.variant_id,
                        quantity: item.quantity,
                        unit_price: item.unit_price,
                        original_price: item.original_price,
                        subtotal: item.subtotal,
                        discount_amount: item.discount_amount,
                        total: item.total,
                        sku: item.sku,
                        title: item.title,
                        variant_title: item.variant_title,
                        image_url: item.image_url,
                        requires_shipping: item.requires_shipping,
                        is_gift_card: item.is_gift_card,
                        custom_attributes: item.custom_attributes,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    };
                    self.cart_repo.add_item(&new_item).await?;
                }

                // Mark guest cart as converted
                self.cart_repo.mark_converted(guest.id, None).await?;

                // Apply coupon from guest cart if customer cart doesn't have one
                if guest.coupon_code.is_some() && customer.coupon_code.is_none() {
                    if let Some(coupon_code) = guest.coupon_code {
                        let _ = self.apply_coupon(customer.id, ApplyCouponInput { coupon_code }).await;
                    }
                }

                self.recalculate_cart(customer.id).await?;
                
                self.cart_repo
                    .find_by_id(customer.id)
                    .await?
                    .ok_or_else(|| Error::not_found("Cart not found"))
            }
            (Some(guest), None) => {
                // Transfer guest cart to customer
                self.cart_repo.assign_customer(guest.id, customer_id).await?;
                
                self.cart_repo
                    .find_by_id(guest.id)
                    .await?
                    .ok_or_else(|| Error::not_found("Cart not found"))
            }
            (None, Some(customer)) => Ok(customer),
            (None, None) => {
                // Create new cart for customer
                self.get_or_create_cart(CartIdentifier::Customer(customer_id), "USD").await
            }
        }
    }

    /// Clear cart (remove all items)
    pub async fn clear_cart(&self, cart_id: Uuid) -> Result<()> {
        self.cart_repo.clear_items(cart_id).await?;
        self.recalculate_cart(cart_id).await?;
        Ok(())
    }

    /// Delete cart
    pub async fn delete_cart(&self, cart_id: Uuid) -> Result<()> {
        self.cart_repo.delete(cart_id).await?;
        Ok(())
    }
}

/// Product details needed to add item to cart
#[derive(Debug, Clone)]
pub struct ProductDetails {
    pub unit_price: Decimal,
    pub original_price: Decimal,
    pub sku: Option<String>,
    pub title: String,
    pub variant_title: Option<String>,
    pub image_url: Option<String>,
    pub requires_shipping: bool,
    pub is_gift_card: bool,
}
