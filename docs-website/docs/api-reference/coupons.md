# Coupons API

The Coupons API provides comprehensive discount management including percentage, fixed amount, free shipping, and buy-X-get-Y promotions. All endpoints are fully implemented and operational.

## Overview

- **Coupon Types**: Percentage, Fixed Amount, Free Shipping, Buy X Get Y
- **Usage Limits**: Per-coupon and per-customer limits
- **Restrictions**: Minimum order amount, product/category exclusions
- **Time-Based**: Validity periods and expiration dates

## Base URL

```
/api/v1/coupons
```

## Authentication

| Endpoint | Authentication | Notes |
|----------|---------------|-------|
| GET /coupons | Optional | Public list of available coupons |
| GET /coupons/{code} | Optional | Validate coupon without applying |
| POST /coupons | Required | Create new coupon (admin) |
| PUT /coupons/{id} | Required | Update coupon (admin) |
| DELETE /coupons/{id} | Required | Delete coupon (admin) |

## Coupon Types

| Type | Description | Example |
|------|-------------|---------|
| `percentage` | Percentage off subtotal | 20% off |
| `fixed_amount` | Fixed amount off subtotal | $10 off |
| `free_shipping` | Waives shipping cost | Free shipping |
| `buy_x_get_y` | Buy X quantity, get Y free | Buy 2 get 1 free |

## Endpoints

### List Coupons

Returns available coupons. Optionally filter by active status.

```http
GET /api/v1/coupons?active=true&page=1&per_page=20
```

**Response (200 OK):**

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "code": "SUMMER20",
      "type": "percentage",
      "value": "20.00",
      "description": "20% off summer collection",
      "minimum_order_amount": "50.00",
      "starts_at": "2026-06-01T00:00:00Z",
      "expires_at": "2026-08-31T23:59:59Z",
      "usage_limit": 1000,
      "usage_count": 245,
      "is_active": true
    }
  ],
  "meta": {
    "total": 15,
    "page": 1,
    "per_page": 20,
    "total_pages": 1
  }
}
```

### Get Coupon

Retrieves details of a specific coupon by code or ID.

```http
GET /api/v1/coupons/SUMMER20
```

**Response (200 OK):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "code": "SUMMER20",
  "type": "percentage",
  "value": "20.00",
  "description": "20% off summer collection",
  "minimum_order_amount": "50.00",
  "maximum_discount_amount": "100.00",
  "starts_at": "2026-06-01T00:00:00Z",
  "expires_at": "2026-08-31T23:59:59Z",
  "usage_limit": 1000,
  "usage_limit_per_customer": 1,
  "usage_count": 245,
  "is_active": true,
  "applies_to": {
    "product_ids": [],
    "collection_ids": ["550e8400-e29b-41d4-a716-446655440001"],
    "exclude_product_ids": ["550e8400-e29b-41d4-a716-446655440002"]
  }
}
```

### Validate Coupon

Validates a coupon against a cart without applying it. Useful for showing discount preview.

```http
POST /api/v1/coupons/validate
Content-Type: application/json
X-Session-Token: <session_token>

{
  "coupon_code": "SUMMER20",
  "cart_id": "550e8400-e29b-41d4-a716-446655440003"
}
```

**Response (200 OK) - Valid:**

```json
{
  "valid": true,
  "coupon": {
    "code": "SUMMER20",
    "type": "percentage",
    "value": "20.00",
    "description": "20% off summer collection"
  },
  "discount": {
    "subtotal": "150.00",
    "discount_amount": "30.00",
    "new_total": "120.00"
  }
}
```

**Response (200 OK) - Invalid:**

```json
{
  "valid": false,
  "error": {
    "code": "COUPON_MINIMUM_NOT_MET",
    "message": "Cart subtotal ($35.00) is below the minimum order amount ($50.00)"
  }
}
```

### Create Coupon

Creates a new coupon. Requires admin authentication.

```http
POST /api/v1/coupons
Content-Type: application/json
Authorization: Bearer <admin_jwt_token>

{
  "code": "WELCOME15",
  "type": "percentage",
  "value": "15.00",
  "description": "15% off for new customers",
  "minimum_order_amount": "0.00",
  "maximum_discount_amount": null,
  "starts_at": "2026-01-01T00:00:00Z",
  "expires_at": null,
  "usage_limit": 10000,
  "usage_limit_per_customer": 1,
  "is_active": true,
  "applies_to": {
    "product_ids": [],
    "collection_ids": [],
    "exclude_product_ids": []
  },
  "customer_eligibility": {
    "new_customers_only": true,
    "customer_ids": [],
    "customer_segments": ["new_subscribers"]
  }
}
```

**Response (201 Created):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "code": "WELCOME15",
  "type": "percentage",
  "value": "15.00",
  "description": "15% off for new customers",
  "minimum_order_amount": "0.00",
  "starts_at": "2026-01-01T00:00:00Z",
  "expires_at": null,
  "usage_limit": 10000,
  "usage_limit_per_customer": 1,
  "usage_count": 0,
  "is_active": true,
  "created_at": "2026-01-28T10:00:00Z"
}
```

### Create Buy X Get Y Coupon

Special format for buy-X-get-Y promotions.

```http
POST /api/v1/coupons
Content-Type: application/json
Authorization: Bearer <admin_jwt_token>

{
  "code": "BUY2GET1",
  "type": "buy_x_get_y",
  "buy_x_get_y": {
    "buy_quantity": 2,
    "get_quantity": 1,
    "buy_product_ids": ["550e8400-e29b-41d4-a716-446655440001"],
    "get_product_ids": ["550e8400-e29b-41d4-a716-446655440001"],
    "get_discount_type": "percentage",
    "get_discount_value": "100.00"
  },
  "description": "Buy 2 get 1 free on selected products",
  "usage_limit": 500,
  "is_active": true
}
```

### Update Coupon

Updates an existing coupon. Partial updates are supported.

```http
PUT /api/v1/coupons/550e8400-e29b-41d4-a716-446655440000
Content-Type: application/json
Authorization: Bearer <admin_jwt_token>

{
  "usage_limit": 2000,
  "expires_at": "2026-12-31T23:59:59Z"
}
```

**Response (200 OK):**

Returns the updated coupon.

### Delete Coupon

Permanently deletes a coupon. This does not affect orders that already used the coupon.

```http
DELETE /api/v1/coupons/550e8400-e29b-41d4-a716-446655440000
Authorization: Bearer <admin_jwt_token>
```

**Response (204 No Content)**

### Get Coupon Usage

Returns usage statistics for a coupon.

```http
GET /api/v1/coupons/550e8400-e29b-41d4-a716-446655440000/usage
Authorization: Bearer <admin_jwt_token>
```

**Response (200 OK):**

```json
{
  "coupon_id": "550e8400-e29b-41d4-a716-446655440000",
  "code": "SUMMER20",
  "usage_limit": 1000,
  "usage_count": 245,
  "remaining": 755,
  "total_discount_amount": "4895.50",
  "orders_count": 245,
  "average_order_value": "142.30",
  "usage_by_day": [
    {
      "date": "2026-01-27",
      "usage_count": 12,
      "discount_amount": "245.00"
    }
  ]
}
```

## Coupon Application Rules

### Discount Calculation

**Percentage:**
```
discount = subtotal * (value / 100)
if maximum_discount_amount and discount > maximum_discount_amount:
    discount = maximum_discount_amount
```

**Fixed Amount:**
```
discount = value
if discount > subtotal:
    discount = subtotal
```

**Free Shipping:**
```
discount = shipping_total
```

**Buy X Get Y:**
```
eligible_sets = floor(buy_quantity_in_cart / buy_quantity)
discount_items = eligible_sets * get_quantity
discount = sum(discount_items * applicable_product_prices * discount_percentage)
```

### Application Restrictions

1. **Minimum Order Amount**: Cart subtotal must meet or exceed this value
2. **Maximum Discount**: Caps the discount amount for percentage coupons
3. **Product Restrictions**: Only applies to specified products/collections
4. **Exclusions**: Never applies to excluded products
5. **Customer Eligibility**: May be limited to new customers or segments
6. **Time Window**: Must be within starts_at and expires_at
7. **Usage Limits**: Cannot exceed total or per-customer limits

### Stacking Rules

By default, coupons do not stack. The following rules apply:

- Only one coupon per cart
- Coupons cannot be combined with automatic discounts
- Free shipping can stack with percentage/fixed coupons if configured

## Error Codes

| Code | Description |
|------|-------------|
| `COUPON_NOT_FOUND` | Coupon code does not exist |
| `COUPON_EXPIRED` | Coupon has passed its expiration date |
| `COUPON_NOT_STARTED` | Coupon valid period has not started |
| `COUPON_INACTIVE` | Coupon has been deactivated |
| `COUPON_USAGE_LIMIT` | Total usage limit reached |
| `COUPON_CUSTOMER_LIMIT` | Customer has already used this coupon |
| `COUPON_MINIMUM_NOT_MET` | Cart subtotal below minimum requirement |
| `COUPON_PRODUCT_NOT_ELIGIBLE` | No eligible products in cart |
| `COUPON_ALREADY_APPLIED` | Coupon already applied to this cart |
| `COUPON_CANNOT_COMBINE` | Cannot combine with existing discount |
| `COUPON_NEW_CUSTOMERS_ONLY` | Only valid for first-time customers |
| `COUPON_CODE_EXISTS` | Coupon code already in use |

## Webhooks

| Event | Description |
|-------|-------------|
| `coupon.created` | New coupon created |
| `coupon.updated` | Coupon details updated |
| `coupon.deleted` | Coupon deleted |
| `coupon.applied` | Coupon applied to cart |
| `coupon.removed` | Coupon removed from cart |

## Best Practices

1. **Code Format**: Use uppercase alphanumeric codes (e.g., `SUMMER20`, `WELCOME15`)
2. **Expiration**: Always set expiration dates for time-sensitive promotions
3. **Usage Limits**: Set reasonable limits to prevent abuse
4. **Minimum Orders**: Use minimum order amounts to maintain margins
5. **Testing**: Validate coupons in staging before production
6. **Monitoring**: Track usage statistics to measure promotion effectiveness

## Example: Complete Coupon Campaign

```javascript
// Create a summer sale coupon
const coupon = await fetch('/api/v1/coupons', {
  method: 'POST',
  headers: { 'Authorization': `Bearer ${adminToken}` },
  body: JSON.stringify({
    code: 'SUMMER2026',
    type: 'percentage',
    value: '25.00',
    description: 'Summer Sale 2026 - 25% off everything',
    minimum_order_amount: '0.00',
    maximum_discount_amount: '200.00',
    starts_at: '2026-06-01T00:00:00Z',
    expires_at: '2026-08-31T23:59:59Z',
    usage_limit: 5000,
    usage_limit_per_customer: 2,
    is_active: true
  })
});

// Customer validates before checkout
const validation = await fetch('/api/v1/coupons/validate', {
  method: 'POST',
  headers: { 'X-Session-Token': sessionToken },
  body: JSON.stringify({
    coupon_code: 'SUMMER2026',
    cart_id: cartId
  })
});

// Apply to cart
await fetch(`/api/v1/carts/${cartId}/coupon`, {
  method: 'POST',
  headers: { 'X-Session-Token': sessionToken },
  body: JSON.stringify({ coupon_code: 'SUMMER2026' })
});
```
