# Checkout API

The Checkout API orchestrates the complete checkout flow including tax calculation, shipping rates, and payment processing.

## Overview

The checkout process follows a three-step flow:

1. **Initiate Checkout** - Calculate tax and get shipping rates
2. **Select Shipping** - Choose shipping method
3. **Complete Checkout** - Process payment and create order

## Base URL

```
/api/v1/checkout
```

## Authentication

All checkout endpoints require authentication via JWT token.

```http
Authorization: Bearer <jwt_token>
```

## Endpoints

### Initiate Checkout

Calculates totals, tax, and available shipping rates for the customer's cart.

```http
POST /api/v1/checkout/initiate
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "cart_id": "550e8400-e29b-41d4-a716-446655440000",
  "shipping_address": {
    "first_name": "John",
    "last_name": "Doe",
    "address1": "123 Main St",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001",
    "phone": "+1-555-0123"
  },
  "billing_address": {
    "first_name": "John",
    "last_name": "Doe",
    "address1": "123 Main St",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001",
    "phone": "+1-555-0123"
  },
  "vat_id": null,
  "currency": "USD"
}
```

**Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cart_id` | UUID | Yes | ID of the cart to checkout |
| `shipping_address` | Address | Yes | Shipping address object |
| `billing_address` | Address | No | Billing address (defaults to shipping) |
| `vat_id` | string | No | VAT ID for tax exemption |
| `currency` | string | No | Currency code (e.g., "USD", "EUR") |

**Address Object:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `first_name` | string | Yes | First name |
| `last_name` | string | Yes | Last name |
| `company` | string | No | Company name |
| `address1` | string | Yes | Street address line 1 |
| `address2` | string | No | Street address line 2 |
| `city` | string | Yes | City name |
| `state` | string | Yes | State or province |
| `country` | string | Yes | Two-letter country code (ISO 3166-1 alpha-2) |
| `zip` | string | Yes | Postal/ZIP code |
| `phone` | string | No | Phone number |

**Response (200 OK):**

```json
{
  "cart_id": "550e8400-e29b-41d4-a716-446655440000",
  "items": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "product_id": "550e8400-e29b-41d4-a716-446655440002",
      "variant_id": null,
      "title": "Premium T-Shirt",
      "sku": "PROD-001",
      "quantity": 2,
      "unit_price": "75.00",
      "total": "150.00"
    }
  ],
  "subtotal": "150.00",
  "discount_total": "15.00",
  "shipping_total": "10.00",
  "shipping_tax": "0.90",
  "item_tax": "13.50",
  "tax_total": "14.40",
  "total": "159.40",
  "currency": "USD",
  "available_shipping_rates": [
    {
      "provider_id": "ups",
      "carrier": "UPS",
      "service_code": "ground",
      "service_name": "UPS Ground",
      "rate": "10.00",
      "currency": "USD",
      "delivery_days": 5,
      "total_cost": "10.00"
    },
    {
      "provider_id": "ups",
      "carrier": "UPS",
      "service_code": "express",
      "service_name": "UPS Express",
      "rate": "25.00",
      "currency": "USD",
      "delivery_days": 2,
      "total_cost": "25.00"
    }
  ],
  "selected_shipping_rate": null,
  "tax_breakdown": [
    {
      "tax_zone_name": "New York",
      "tax_rate_name": "Sales Tax",
      "rate": "0.08",
      "taxable_amount": "150.00",
      "tax_amount": "12.00"
    },
    {
      "tax_zone_name": "New York",
      "tax_rate_name": "Local Tax",
      "rate": "0.01",
      "taxable_amount": "150.00",
      "tax_amount": "1.50"
    }
  ],
  "vat_id_valid": null
}
```

### Select Shipping

Updates the checkout with the selected shipping method and recalculates totals.

```http
POST /api/v1/checkout/shipping
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "cart_id": "550e8400-e29b-41d4-a716-446655440000",
  "shipping_rate": {
    "provider_id": "ups",
    "carrier": "UPS",
    "service_code": "ground",
    "service_name": "UPS Ground",
    "rate": "10.00",
    "currency": "USD",
    "delivery_days": 5,
    "total_cost": "10.00"
  }
}
```

**Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cart_id` | UUID | Yes | ID of the cart |
| `shipping_rate` | ShippingRate | Yes | Selected shipping rate |

**Shipping Rate Object:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `provider_id` | string | Yes | Shipping provider ID |
| `carrier` | string | Yes | Carrier name (e.g., "UPS", "FedEx") |
| `service_code` | string | Yes | Service code identifier |
| `service_name` | string | Yes | Human-readable service name |
| `rate` | decimal | Yes | Base shipping rate |
| `currency` | string | Yes | Currency code |
| `delivery_days` | integer | No | Estimated delivery days |
| `total_cost` | decimal | Yes | Total cost including fees |

**Response (200 OK):**

Returns the updated checkout summary with the selected shipping rate applied.

### Complete Checkout

Finalizes the checkout, creates an order, and processes payment.

```http
POST /api/v1/checkout/complete
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "cart_id": "550e8400-e29b-41d4-a716-446655440000",
  "shipping_address": {
    "first_name": "John",
    "last_name": "Doe",
    "address1": "123 Main St",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001",
    "phone": "+1-555-0123"
  },
  "billing_address": {
    "first_name": "John",
    "last_name": "Doe",
    "address1": "123 Main St",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001",
    "phone": "+1-555-0123"
  },
  "payment_method": {
    "type": "card",
    "token": "tok_visa"
  },
  "customer_email": "john.doe@example.com",
  "vat_id": null,
  "notes": "Please leave at front door",
  "selected_shipping_rate": {
    "provider_id": "ups",
    "carrier": "UPS",
    "service_code": "ground",
    "service_name": "UPS Ground",
    "rate": "10.00",
    "currency": "USD",
    "delivery_days": 5,
    "total_cost": "10.00"
  }
}
```

**Payment Method Types:**

| Type | Fields | Description |
|------|--------|-------------|
| `card` | `token` | Credit/debit card token |
| `bank_transfer` | `account_number`, `routing_number` | Bank transfer |
| `digital_wallet` | `provider`, `token` | Digital wallet (Apple Pay, Google Pay) |
| `buy_now_pay_later` | `provider` | BNPL provider (Klarna, Afterpay) |
| `cash_on_delivery` | none | Cash on delivery |

**Response (201 Created):**

```json
{
  "order": {
    "id": "550e8400-e29b-41d4-a716-446655440010",
    "order_number": "1001",
    "customer_id": "550e8400-e29b-41d4-a716-446655440005",
    "customer_email": "john.doe@example.com",
    "status": "pending",
    "payment_status": "paid",
    "fulfillment_status": "unfulfilled",
    "currency": "USD",
    "subtotal": "150.00",
    "tax_total": "14.40",
    "shipping_total": "10.00",
    "discount_total": "15.00",
    "total": "159.40",
    "items": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440011",
        "product_id": "550e8400-e29b-41d4-a716-446655440002",
        "variant_id": null,
        "title": "Premium T-Shirt",
        "sku": "PROD-001",
        "quantity": 2,
        "price": "75.00",
        "total": "150.00",
        "tax_amount": "12.00"
      }
    ],
    "created_at": "2026-02-21T06:30:00Z",
    "metadata": {}
  },
  "payment_id": "pay_1234567890",
  "total_charged": "159.40",
  "currency": "USD"
}
```

## Complete Checkout Flow Example

### Step 1: Initiate Checkout

```bash
curl -X POST http://localhost:8080/api/v1/checkout/initiate \
  -H "Authorization: Bearer $JWT_TOKEN" \
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
    "customer_email": "john.doe@example.com"
  }' | jq
```

### Step 2: Select Shipping

```bash
curl -X POST http://localhost:8080/api/v1/checkout/shipping \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "cart_id": "550e8400-e29b-41d4-a716-446655440000",
    "shipping_rate": {
      "provider_id": "ups",
      "carrier": "UPS",
      "service_code": "ground",
      "service_name": "UPS Ground",
      "rate": "10.00",
      "currency": "USD",
      "delivery_days": 5,
      "total_cost": "10.00"
    }
  }' | jq
```

### Step 3: Complete Checkout

```bash
curl -X POST http://localhost:8080/api/v1/checkout/complete \
  -H "Authorization: Bearer $JWT_TOKEN" \
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
    "payment_method": {
      "type": "card",
      "token": "tok_visa"
    },
    "customer_email": "john.doe@example.com",
    "selected_shipping_rate": {
      "provider_id": "ups",
      "carrier": "UPS",
      "service_code": "ground",
      "service_name": "UPS Ground",
      "rate": "10.00",
      "currency": "USD",
      "delivery_days": 5,
      "total_cost": "10.00"
    }
  }' | jq
```

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `CART_NOT_FOUND` | 404 | Cart ID does not exist |
| `CART_EMPTY` | 400 | Cannot checkout with empty cart |
| `INVALID_ADDRESS` | 400 | Address validation failed |
| `INVALID_PAYMENT_METHOD` | 400 | Payment method not supported |
| `PAYMENT_FAILED` | 400 | Payment processing failed |
| `SHIPPING_UNAVAILABLE` | 400 | Shipping not available for address |
| `TAX_CALCULATION_ERROR` | 500 | Failed to calculate taxes |

## Webhooks

The Checkout system emits the following webhook events:

| Event | Description |
|-------|-------------|
| `checkout.initiated` | Checkout process started |
| `checkout.shipping_selected` | Shipping method selected |
| `checkout.completed` | Checkout completed successfully |
| `checkout.failed` | Checkout failed |
| `order.created` | New order created from checkout |
| `payment.processed` | Payment successfully processed |
| `payment.failed` | Payment processing failed |

## Related Topics

- [Cart API](cart.md) - Manage shopping carts
- [Orders API](orders.md) - Order management
- [Payments API](payments.md) - Payment processing
- [Shipping API](shipping.md) - Shipping configuration
