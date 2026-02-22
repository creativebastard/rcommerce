//! Checkout Service
//!
//! Orchestrates the complete checkout flow including cart validation,
//! shipping calculation, tax calculation, order creation, and payment processing.
//! This service integrates Cart, Tax, Shipping, Order, and Payment services.

use std::sync::Arc;

use chrono::Utc;
use rust_decimal::Decimal;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{
    Error, Result,
    models::{Cart, CartItem, Currency, Address},
    tax::{
        TaxService, TaxContext, TaxAddress, TaxableItem, CustomerTaxInfo,
        TransactionType, TaxCalculation, VatId,
    },
    shipping::{
        ShippingProviderFactory, ShippingRate, Package, RateOptions,
    },
    order::{
        OrderService, CreateOrderRequest, CreateOrderItem, Order,
    },
    payment::{PaymentGateway, CreatePaymentRequest, PaymentMethod},
    services::CartService,
};

/// Checkout service that orchestrates the complete checkout flow
pub struct CheckoutService {
    cart_service: Arc<CartService>,
    tax_service: Arc<dyn TaxService>,
    order_service: Arc<OrderService>,
    payment_gateway: Arc<dyn PaymentGateway>,
    #[allow(dead_code)]
    shipping_factory: Arc<ShippingProviderFactory>,
    config: CheckoutConfig,
}

/// Checkout configuration
#[derive(Debug, Clone)]
pub struct CheckoutConfig {
    /// Default shipping rate per kg
    pub default_shipping_rate: Decimal,
    /// Base shipping cost
    pub base_shipping_cost: Decimal,
    /// Free shipping threshold (None = no free shipping)
    pub free_shipping_threshold: Option<Decimal>,
    /// Default currency
    pub default_currency: Currency,
    /// Whether to validate VAT IDs during checkout
    pub validate_vat_ids: bool,
    /// Seller country code for OSS determination
    pub seller_country_code: String,
}

impl Default for CheckoutConfig {
    fn default() -> Self {
        use rust_decimal_macros::dec;
        Self {
            default_shipping_rate: dec!(5.00),
            base_shipping_cost: dec!(10.00),
            free_shipping_threshold: Some(dec!(100.00)),
            default_currency: Currency::USD,
            validate_vat_ids: true,
            seller_country_code: "US".to_string(),
        }
    }
}

/// Checkout summary for displaying to customer
#[derive(Debug, Clone)]
pub struct CheckoutSummary {
    pub cart_id: Uuid,
    pub items: Vec<CartItem>,
    pub subtotal: Decimal,
    pub discount_total: Decimal,
    pub shipping_total: Decimal,
    pub shipping_tax: Decimal,
    pub item_tax: Decimal,
    pub tax_total: Decimal,
    pub total: Decimal,
    pub currency: Currency,
    pub available_shipping_rates: Vec<ShippingRate>,
    pub selected_shipping_rate: Option<ShippingRate>,
    pub tax_breakdown: Vec<TaxBreakdownItem>,
    pub vat_id_valid: Option<bool>,
}

/// Tax breakdown item for display
#[derive(Debug, Clone)]
pub struct TaxBreakdownItem {
    pub tax_zone_name: String,
    pub tax_rate_name: String,
    pub rate: Decimal,
    pub taxable_amount: Decimal,
    pub tax_amount: Decimal,
}

/// Checkout initialization request
#[derive(Debug, Clone)]
pub struct InitiateCheckoutRequest {
    pub cart_id: Uuid,
    pub shipping_address: Address,
    pub billing_address: Option<Address>,
    pub vat_id: Option<String>,
    pub customer_id: Option<Uuid>,
    pub currency: Option<Currency>,
}

/// Shipping selection request
#[derive(Debug, Clone)]
pub struct SelectShippingRequest {
    pub cart_id: Uuid,
    pub shipping_rate: ShippingRate,
    pub package: Package,
}

/// Complete checkout request
#[derive(Debug, Clone)]
pub struct CompleteCheckoutRequest {
    pub cart_id: Uuid,
    pub shipping_address: Address,
    pub billing_address: Option<Address>,
    pub payment_method: PaymentMethod,
    pub customer_email: String,
    pub customer_id: Option<Uuid>,
    pub vat_id: Option<String>,
    pub notes: Option<String>,
    pub selected_shipping_rate: ShippingRate,
}

/// Checkout result
#[derive(Debug, Clone)]
pub struct CheckoutResult {
    pub order: Order,
    pub payment_id: String,
    pub total_charged: Decimal,
    pub currency: Currency,
}

/// Tax calculation result with shipping
#[derive(Debug, Clone)]
pub struct TaxCalculationWithShipping {
    pub item_tax: Decimal,
    pub shipping_tax: Decimal,
    pub total_tax: Decimal,
    pub calculation: TaxCalculation,
}

impl CheckoutService {
    /// Create a new checkout service
    pub fn new(
        cart_service: Arc<CartService>,
        tax_service: Arc<dyn TaxService>,
        order_service: Arc<OrderService>,
        payment_gateway: Arc<dyn PaymentGateway>,
        shipping_factory: Arc<ShippingProviderFactory>,
        config: CheckoutConfig,
    ) -> Self {
        Self {
            cart_service,
            tax_service,
            order_service,
            payment_gateway,
            shipping_factory,
            config,
        }
    }

    /// Initiate checkout - calculate totals, tax, and available shipping rates
    pub async fn initiate_checkout(
        &self,
        request: InitiateCheckoutRequest,
    ) -> Result<CheckoutSummary> {
        info!("Initiating checkout for cart {}", request.cart_id);

        // Get cart with items
        let cart_with_items = self.cart_service.get_cart_with_items(request.cart_id).await?;
        let cart = cart_with_items.cart;
        let items = cart_with_items.items;

        // Validate cart
        self.validate_cart(&cart, &items).await?;

        // Calculate subtotal (already in cart)
        let subtotal = cart.subtotal;
        let discount_total = cart.discount_total;

        // Validate VAT ID if provided
        let vat_id_valid = if let Some(ref vat_id) = request.vat_id {
            if self.config.validate_vat_ids {
                match self.validate_vat_id(vat_id).await {
                    Ok(valid) => Some(valid),
                    Err(e) => {
                        warn!("VAT ID validation failed: {}", e);
                        Some(false)
                    }
                }
            } else {
                None
            }
        } else {
            None
        };

        // Calculate tax
        let tax_result = self.calculate_tax(
            &items,
            &request.shipping_address,
            request.billing_address.as_ref(),
            request.vat_id.as_deref(),
            request.customer_id,
            cart.currency,
        ).await?;

        // Get available shipping rates
        let package = self.estimate_package(&items).await?;
        let shipping_rates = self.get_shipping_rates(
            &request.shipping_address,
            &package,
        ).await?;

        // Calculate initial shipping cost (will be updated when customer selects rate)
        let shipping_total = self.estimate_shipping_cost(subtotal);
        let shipping_tax = self.calculate_shipping_tax(
            shipping_total,
            &request.shipping_address,
        ).await?;

        let tax_total = tax_result.total_tax + shipping_tax;
        let total = subtotal - discount_total + shipping_total + tax_total;

        // Build tax breakdown
        let tax_breakdown = tax_result.calculation.tax_breakdown.iter().map(|tb| {
            TaxBreakdownItem {
                tax_zone_name: tb.tax_zone_name.clone(),
                tax_rate_name: tb.tax_rate_name.clone(),
                rate: tb.rate,
                taxable_amount: tb.taxable_amount,
                tax_amount: tb.tax_amount,
            }
        }).collect();

        let summary = CheckoutSummary {
            cart_id: cart.id,
            items,
            subtotal,
            discount_total,
            shipping_total,
            shipping_tax,
            item_tax: tax_result.item_tax,
            tax_total,
            total,
            currency: cart.currency,
            available_shipping_rates: shipping_rates,
            selected_shipping_rate: None,
            tax_breakdown,
            vat_id_valid,
        };

        debug!("Checkout summary: subtotal={}, tax={}, total={}", 
            summary.subtotal, summary.tax_total, summary.total);

        Ok(summary)
    }

    /// Select shipping method and recalculate totals
    pub async fn select_shipping(
        &self,
        request: SelectShippingRequest,
    ) -> Result<CheckoutSummary> {
        info!("Selecting shipping for cart {}", request.cart_id);

        // Get cart
        let cart_with_items = self.cart_service.get_cart_with_items(request.cart_id).await?;
        let cart = cart_with_items.cart;
        let items = cart_with_items.items;

        // Get shipping address from cart (stored during initiate_checkout)
        let shipping_address = self.get_cart_shipping_address(request.cart_id).await?
            .ok_or_else(|| Error::validation("Shipping address not set"))?;

        // Calculate tax with selected shipping
        let shipping_total = request.shipping_rate.total_cost;
        let shipping_tax = self.calculate_shipping_tax(
            shipping_total,
            &shipping_address,
        ).await?;

        // Recalculate totals
        let subtotal = cart.subtotal;
        let discount_total = cart.discount_total;
        
        // Get item tax (recalculate to ensure consistency)
        let tax_result = self.calculate_tax(
            &items,
            &shipping_address,
            None, // billing address
            None, // vat_id
            cart.customer_id,
            cart.currency,
        ).await?;

        let tax_total = tax_result.total_tax + shipping_tax;
        let total = subtotal - discount_total + shipping_total + tax_total;

        // Get available shipping rates
        let shipping_rates = self.get_shipping_rates(
            &shipping_address,
            &request.package,
        ).await?;

        let summary = CheckoutSummary {
            cart_id: cart.id,
            items,
            subtotal,
            discount_total,
            shipping_total,
            shipping_tax,
            item_tax: tax_result.item_tax,
            tax_total,
            total,
            currency: cart.currency,
            available_shipping_rates: shipping_rates,
            selected_shipping_rate: Some(request.shipping_rate),
            tax_breakdown: vec![], // TODO: Rebuild breakdown
            vat_id_valid: None,
        };

        Ok(summary)
    }

    /// Complete checkout - create order and process payment
    pub async fn complete_checkout(
        &self,
        request: CompleteCheckoutRequest,
    ) -> Result<CheckoutResult> {
        info!("Completing checkout for cart {}", request.cart_id);

        // Get cart with items
        let cart_with_items = self.cart_service.get_cart_with_items(request.cart_id).await?;
        let cart = cart_with_items.cart;
        let items = cart_with_items.items;

        // Validate cart one final time
        self.validate_cart(&cart, &items).await?;

        // Calculate final tax
        let tax_result = self.calculate_tax(
            &items,
            &request.shipping_address,
            request.billing_address.as_ref(),
            request.vat_id.as_deref(),
            request.customer_id,
            cart.currency,
        ).await?;

        // Calculate shipping tax
        let shipping_total = request.selected_shipping_rate.total_cost;
        let shipping_tax = self.calculate_shipping_tax(
            shipping_total,
            &request.shipping_address,
        ).await?;

        let tax_total = tax_result.total_tax + shipping_tax;
        let total = cart.subtotal - cart.discount_total + shipping_total + tax_total;

        // Create order items with tax
        let order_items: Vec<CreateOrderItem> = items.iter().map(|item| {
            // Find tax for this item
            let item_tax = tax_result.calculation.line_items.iter()
                .find(|li| li.item_id == item.id)
                .map(|li| li.tax_amount)
                .unwrap_or_default();

            CreateOrderItem {
                product_id: item.product_id,
                variant_id: item.variant_id,
                quantity: item.quantity,
                price: item.unit_price,
                tax_amount: item_tax,
            }
        }).collect();

        // Create order
        let create_order_request = CreateOrderRequest {
            customer_id: request.customer_id,
            customer_email: request.customer_email.clone(),
            billing_address_id: None, // TODO: Save address and get ID
            shipping_address_id: None, // TODO: Save address and get ID
            items: order_items,
            currency: cart.currency.to_string(),
            subtotal: cart.subtotal,
            tax_total,
            shipping_total,
            discount_total: cart.discount_total,
            total,
            notes: request.notes,
            tags: None,
            metadata: serde_json::json!({
                "cart_id": cart.id.to_string(),
                "vat_id": request.vat_id,
                "shipping_carrier": request.selected_shipping_rate.carrier,
                "shipping_service": request.selected_shipping_rate.service_code,
            }),
        };

        let order = self.order_service.create_order(create_order_request).await?;

        // Record tax transaction for reporting
        self.tax_service.record_tax_transaction(order.id, &tax_result.calculation).await?;

        // Process payment
        let payment_request = CreatePaymentRequest {
            amount: total,
            currency: cart.currency.to_string(),
            order_id: order.id,
            customer_id: request.customer_id,
            customer_email: request.customer_email.clone(),
            payment_method: request.payment_method,
            billing_address: request.billing_address.clone(),
            metadata: serde_json::json!({
                "order_id": order.id.to_string(),
                "order_number": order.order_number,
            }),
        };

        let payment = self.payment_gateway.create_payment(payment_request).await?;
        let confirmed_payment = self.payment_gateway.confirm_payment(&payment.id).await?;

        // Mark cart as converted
        self.cart_service.mark_converted(cart.id, Some(order.id)).await?;

        info!("Checkout complete: order={}, payment={}", order.id, payment.id);

        Ok(CheckoutResult {
            order,
            payment_id: confirmed_payment.id,
            total_charged: confirmed_payment.amount,
            currency: cart.currency,
        })
    }

    /// Validate cart before checkout
    async fn validate_cart(&self, cart: &Cart, items: &[CartItem]) -> Result<()> {
        if cart.converted_to_order {
            return Err(Error::validation("Cart has already been converted to an order"));
        }

        if items.is_empty() {
            return Err(Error::validation("Cart is empty"));
        }

        if cart.expires_at.map(|exp| exp < Utc::now()).unwrap_or(false) {
            return Err(Error::validation("Cart has expired"));
        }

        // TODO: Validate inventory availability
        // TODO: Validate product prices haven't changed

        Ok(())
    }

    /// Calculate tax for cart items
    async fn calculate_tax(
        &self,
        items: &[CartItem],
        shipping_address: &Address,
        billing_address: Option<&Address>,
        vat_id: Option<&str>,
        customer_id: Option<Uuid>,
        currency: Currency,
    ) -> Result<TaxCalculationWithShipping> {
        // Convert cart items to taxable items
        let taxable_items: Vec<TaxableItem> = items.iter().map(|item| TaxableItem {
            id: item.id,
            product_id: item.product_id,
            quantity: item.quantity,
            unit_price: item.unit_price,
            total_price: item.total,
            tax_category_id: None, // TODO: Get from product
            is_digital: false, // TODO: Get from product
            title: item.title.clone(),
            sku: item.sku.clone(),
        }).collect();

        // Build tax context
        let tax_context = TaxContext {
            customer: CustomerTaxInfo {
                customer_id,
                is_tax_exempt: false, // TODO: Check customer exemptions
                vat_id: vat_id.and_then(|v| VatId::parse(v).ok()),
                exemptions: vec![],
            },
            shipping_address: address_to_tax_address(shipping_address),
            billing_address: billing_address.map(address_to_tax_address)
                .unwrap_or_else(|| address_to_tax_address(shipping_address)),
            currency,
            transaction_type: if vat_id.is_some() { 
                TransactionType::B2B 
            } else { 
                TransactionType::B2C 
            },
        };

        // Calculate tax
        let calculation = self.tax_service.calculate_tax(&taxable_items, &tax_context).await?;
        
        let item_tax: Decimal = calculation.line_items.iter().map(|li| li.tax_amount).sum();
        let total_tax = item_tax + calculation.shipping_tax;

        Ok(TaxCalculationWithShipping {
            item_tax,
            shipping_tax: calculation.shipping_tax,
            total_tax,
            calculation,
        })
    }

    /// Calculate tax on shipping cost
    async fn calculate_shipping_tax(
        &self,
        shipping_cost: Decimal,
        destination: &Address,
    ) -> Result<Decimal> {
        let tax_address = address_to_tax_address(destination);
        
        // Use TaxCalculator directly for shipping tax
        let _calculator = crate::tax::TaxCalculator::new(
            vec![], // rates will be fetched by tax service
            vec![], // zones will be fetched by tax service
            vec![], // categories
        );
        
        // For now, use tax service to get rate and calculate
        let rates = self.tax_service.get_tax_rates(
            &tax_address.country_code,
            tax_address.region_code.as_deref(),
            tax_address.postal_code.as_deref(),
        ).await?;

        if let Some(rate) = rates.first() {
            Ok((shipping_cost * rate.rate).round_dp(2))
        } else {
            Ok(Decimal::ZERO)
        }
    }

    /// Get available shipping rates
    async fn get_shipping_rates(
        &self,
        destination: &Address,
        package: &Package,
    ) -> Result<Vec<ShippingRate>> {
        // TODO: Get origin address from configuration
        let origin = Address {
            id: Uuid::new_v4(),
            customer_id: Uuid::nil(),
            first_name: "Store".to_string(),
            last_name: "Warehouse".to_string(),
            company: None,
            phone: None,
            address1: "123 Warehouse St".to_string(),
            address2: None,
            city: "Los Angeles".to_string(),
            state: Some("CA".to_string()),
            country: "US".to_string(),
            zip: "90210".to_string(),
            is_default_shipping: false,
            is_default_billing: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let options = RateOptions {
            carriers: None,
            services: None,
            include_insurance: false,
            insurance_value: None,
            signature_confirmation: false,
            adult_signature: false,
            saturday_delivery: false,
            hold_for_pickup: false,
            currency: Some("USD".to_string()),
        };

        // Create a new factory instance for this call
        // Note: In production, consider implementing Clone for ShippingProviderFactory
        // or restructuring to avoid the need for cloning
        let factory = ShippingProviderFactory::new();
        let aggregator = crate::shipping::ShippingRateAggregator::new(factory);
        let rates = aggregator.get_all_rates(&origin, destination, package, &options).await?;

        Ok(rates)
    }

    /// Estimate package dimensions from cart items
    async fn estimate_package(&self, items: &[CartItem]) -> Result<Package> {
        // TODO: Get actual product dimensions
        let total_weight: Decimal = items.iter()
            .map(|item| Decimal::from(item.quantity) * dec!(0.5)) // Assume 0.5kg per item
            .sum();

        Ok(Package {
            weight: total_weight,
            weight_unit: "kg".to_string(),
            length: Some(dec!(30.0)),
            width: Some(dec!(20.0)),
            height: Some(dec!(15.0)),
            dimension_unit: Some("cm".to_string()),
            predefined_package: None,
        })
    }

    /// Estimate shipping cost before rate selection
    fn estimate_shipping_cost(&self, subtotal: Decimal) -> Decimal {
        // Check free shipping threshold
        if let Some(threshold) = self.config.free_shipping_threshold {
            if subtotal >= threshold {
                return Decimal::ZERO;
            }
        }

        // Default estimate
        self.config.base_shipping_cost
    }

    /// Validate VAT ID
    async fn validate_vat_id(&self, vat_id: &str) -> Result<bool> {
        let result = self.tax_service.validate_vat_id(vat_id).await?;
        Ok(result.is_valid)
    }

    /// Get shipping address stored for cart
    async fn get_cart_shipping_address(&self, _cart_id: Uuid) -> Result<Option<Address>> {
        // TODO: Implement address storage/retrieval for carts
        // For now, return None - should be stored during initiate_checkout
        Ok(None)
    }
}

/// Convert Address to TaxAddress
fn address_to_tax_address(address: &Address) -> TaxAddress {
    TaxAddress {
        country_code: address.country.clone(),
        region_code: address.state.clone(),
        postal_code: Some(address.zip.clone()),
        city: Some(address.city.clone()),
    }
}

use rust_decimal_macros::dec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkout_config_default() {
        let config = CheckoutConfig::default();
        assert_eq!(config.default_shipping_rate, dec!(5.00));
        assert_eq!(config.base_shipping_cost, dec!(10.00));
        assert_eq!(config.free_shipping_threshold, Some(dec!(100.00)));
    }

    #[test]
    fn test_address_conversion() {
        let address = Address {
            id: Uuid::new_v4(),
            customer_id: Uuid::new_v4(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            company: None,
            phone: None,
            address1: "123 Main St".to_string(),
            address2: None,
            city: "Berlin".to_string(),
            state: Some("BE".to_string()),
            country: "DE".to_string(),
            zip: "10115".to_string(),
            is_default_shipping: true,
            is_default_billing: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let tax_addr = address_to_tax_address(&address);
        assert_eq!(tax_addr.country_code, "DE");
        assert_eq!(tax_addr.region_code, Some("BE".to_string()));
        assert_eq!(tax_addr.postal_code, Some("10115".to_string()));
        assert_eq!(tax_addr.city, Some("Berlin".to_string()));
    }
}
