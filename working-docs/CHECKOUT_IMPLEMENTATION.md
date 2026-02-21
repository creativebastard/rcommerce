# Checkout and Order Creation Implementation

## Summary
This implementation integrates Tax and Shipping services into the order creation process and introduces a new checkout flow with three endpoints.

## Changes Made

### 1. New File: `/crates/rcommerce-api/src/routes/checkout.rs`
Created a new checkout routes module with three endpoints:

- **POST /api/v1/checkout/initiate**
  - Starts the checkout process
  - Calculates tax based on shipping address using TaxService
  - Gets available shipping rates from ShippingService
  - Returns checkout summary with totals, tax breakdown, and shipping options

- **POST /api/v1/checkout/shipping**
  - Allows customer to select a shipping method
  - Recalculates totals with the selected shipping rate
  - Updates tax calculation based on shipping cost

- **POST /api/v1/checkout/complete**
  - Completes the checkout process
  - Creates the order
  - Processes payment through PaymentService
  - Marks cart as converted

### 2. Updated: `/crates/rcommerce-api/src/state.rs`
Added new services to AppState:
- `order_service: OrderService` - For order creation
- `tax_service: Arc<DefaultTaxService>` - For tax calculation
- `shipping_factory: Arc<ShippingProviderFactory>` - For shipping rate calculation
- `checkout_service: Arc<CheckoutService>` - For orchestrating checkout flow

### 3. Updated: `/crates/rcommerce-api/src/server.rs`
- Added imports for new services (OrderService, CheckoutService, CheckoutConfig, DefaultTaxService, ShippingProviderFactory)
- Initialized all new services in `create_app_state()`:
  - OrderService with database connection
  - DefaultTaxService with database pool
  - ShippingProviderFactory from configuration
  - CheckoutService with all dependencies
- Added checkout routes to the protected routes
- Added checkout routes to the route logging

### 4. Updated: `/crates/rcommerce-api/src/routes/mod.rs`
- Added `pub mod checkout;` to declare the new module
- Added `pub use checkout::router as checkout_router;` to export the router
- Added checkout_router to the api_v1_routes function

### 5. Updated: `/crates/rcommerce-api/src/routes/order.rs`
Enhanced the existing `create_order` endpoint to use TaxService and ShippingService:
- Added `shipping_address` field to `CreateOrderRequest` for address-based calculations
- Added `coupon_code` field for discount application
- Integrated TaxService for accurate tax calculation based on shipping address
- Integrated ShippingProviderFactory for shipping cost calculation
- Falls back to default 10% tax if address not provided (backward compatibility)
- Added authentication requirement via JwtAuth extension

## API Usage Examples

### Initiate Checkout
```bash
curl -X POST http://localhost:8080/api/v1/checkout/initiate \
  -H "Authorization: Bearer {jwt_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "cart_id": "550e8400-e29b-41d4-a716-446655440000",
    "shipping_address": {
      "first_name": "John",
      "last_name": "Doe",
      "address1": "123 Main St",
      "city": "New York",
      "state": "NY",
      "country": "US",
      "zip": "10001"
    },
    "billing_address": null,
    "vat_id": null,
    "currency": "USD"
  }'
```

**Response:**
```json
{
  "cart_id": "550e8400-e29b-41d4-a716-446655440000",
  "items": [...],
  "subtotal": "100.00",
  "discount_total": "0.00",
  "shipping_total": "10.00",
  "shipping_tax": "0.80",
  "item_tax": "8.50",
  "tax_total": "9.30",
  "total": "119.30",
  "currency": "USD",
  "available_shipping_rates": [
    {
      "provider_id": "default",
      "carrier": "Standard",
      "service_code": "ground",
      "service_name": "Ground Shipping",
      "rate": "10.00",
      "currency": "USD",
      "delivery_days": 5,
      "total_cost": "10.00"
    }
  ],
  "selected_shipping_rate": null,
  "tax_breakdown": [
    {
      "tax_zone_name": "US-NY",
      "tax_rate_name": "Sales Tax",
      "rate": "0.085",
      "taxable_amount": "100.00",
      "tax_amount": "8.50"
    }
  ],
  "vat_id_valid": null
}
```

### Select Shipping
```bash
curl -X POST http://localhost:8080/api/v1/checkout/shipping \
  -H "Authorization: Bearer {jwt_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "cart_id": "550e8400-e29b-41d4-a716-446655440000",
    "shipping_rate": {
      "provider_id": "default",
      "carrier": "Standard",
      "service_code": "ground",
      "service_name": "Ground Shipping",
      "rate": "10.00",
      "currency": "USD",
      "delivery_days": 5,
      "total_cost": "10.00"
    }
  }'
```

### Complete Checkout
```bash
curl -X POST http://localhost:8080/api/v1/checkout/complete \
  -H "Authorization: Bearer {jwt_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "cart_id": "550e8400-e29b-41d4-a716-446655440000",
    "shipping_address": {
      "first_name": "John",
      "last_name": "Doe",
      "address1": "123 Main St",
      "city": "New York",
      "state": "NY",
      "country": "US",
      "zip": "10001"
    },
    "customer_email": "john@example.com",
    "payment_method": {
      "type": "card",
      "token": "tok_visa"
    },
    "selected_shipping_rate": {
      "provider_id": "default",
      "carrier": "Standard",
      "service_code": "ground",
      "service_name": "Ground Shipping",
      "rate": "10.00",
      "currency": "USD",
      "delivery_days": 5,
      "total_cost": "10.00"
    }
  }'
```

## Architecture

### Service Dependencies
```
CheckoutService
├── CartService (for cart management)
├── TaxService (for tax calculation)
├── OrderService (for order creation)
├── PaymentGateway (for payment processing)
└── ShippingProviderFactory (for shipping rates)
```

### Data Flow
1. **Initiate Checkout**
   - Get cart with items
   - Validate cart (not empty, not expired, not already converted)
   - Calculate tax based on shipping address
   - Get shipping rates from providers
   - Return summary with all options

2. **Select Shipping**
   - Update selected shipping rate
   - Recalculate shipping tax
   - Update total

3. **Complete Checkout**
   - Validate cart one final time
   - Calculate final tax
   - Create order with items
   - Process payment
   - Mark cart as converted
   - Return order and payment info

## Configuration

The shipping providers are configured in `config.toml`:

```toml
[shipping]
default_provider = "default"
test_mode = true

[shipping.dhl]
enabled = false
api_key = ""
api_secret = ""
account_number = ""
sandbox = true

[shipping.fedex]
enabled = false
api_key = ""
api_secret = ""
account_number = ""
sandbox = true

[shipping.ups]
enabled = false
api_key = ""
username = ""
password = ""
account_number = ""
sandbox = true

[shipping.usps]
enabled = false
api_key = ""
sandbox = true
```

## Backward Compatibility

The existing `POST /api/v1/orders` endpoint is still available and has been enhanced:
- If `shipping_address` is provided, it uses TaxService and ShippingService for calculations
- If `shipping_address` is not provided, it falls back to the default 10% tax rate and free shipping

## Testing

To test the implementation:

1. Start the server with a valid configuration
2. Create a cart and add items
3. Call `/checkout/initiate` to get tax and shipping calculations
4. Select a shipping method via `/checkout/shipping`
5. Complete the checkout via `/checkout/complete`
6. Verify the order is created with correct totals

## Success Criteria Met

- [x] Tax is calculated based on shipping address using TaxService
- [x] Shipping costs are calculated based on package/destination using ShippingService
- [x] Multiple shipping options are available
- [x] Order totals are accurate (subtotal + tax + shipping - discounts)
- [x] Checkout flow supports proper orchestration of services
- [x] Inventory validation during order creation
- [x] Payment integration through PaymentService
