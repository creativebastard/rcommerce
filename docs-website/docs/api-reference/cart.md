# Cart API

The Cart API allows you to manage shopping carts for both guest and authenticated customers.

## Overview

- **Guest Carts**: Identified by session token, no authentication required
- **Customer Carts**: Tied to authenticated customer accounts
- **Cart Merging**: Automatic merging when guest logs in
- **Expiration**: Carts expire after 30 days of inactivity

## Base URL

```
/api/v1/carts
```

## Authentication

| Endpoint | Authentication |
|----------|---------------|
| `POST /carts/guest` | None |
| `GET /carts/:id` | None |
| `GET /carts/me` | Required (JWT) |
| All other endpoints | Required (JWT) |

## Guest Carts

Guest carts are identified by a session token and do not require authentication.

### Create Guest Cart

```http
POST /api/v1/carts/guest
```

Creates a new guest cart with a session token.

**Response:**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "session_token": "sess_abc123...",
  "currency": "USD",
  "subtotal": "0.00",
  "discount_total": "0.00",
  "tax_total": "0.00",
  "shipping_total": "0.00",
  "total": "0.00",
  "item_count": 0,
  "items": [],
  "expires_at": "2026-03-23T10:00:00Z"
}
```

**Important:** Store the `session_token` client-side for subsequent requests.

### Get Cart by ID

Retrieves a specific cart with all items.

```http
GET /api/v1/carts/{cart_id}
```

**Response:**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "session_token": "sess_abc123...",
  "currency": "USD",
  "subtotal": "150.00",
  "discount_total": "15.00",
  "tax_total": "13.50",
  "shipping_total": "10.00",
  "total": "158.50",
  "coupon_code": "SUMMER10",
  "item_count": 3,
  "items": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440003",
      "product_id": "550e8400-e29b-41d4-a716-446655440004",
      "variant_id": "550e8400-e29b-41d4-a716-446655440005",
      "quantity": 2,
      "unit_price": "50.00",
      "original_price": "50.00",
      "subtotal": "100.00",
      "discount_amount": "10.00",
      "total": "90.00",
      "sku": "PROD-001-L",
      "title": "Premium T-Shirt",
      "variant_title": "Large / Blue",
      "image_url": "https://cdn.example.com/products/001.jpg",
      "requires_shipping": true,
      "is_gift_card": false
    }
  ],
  "expires_at": "2026-03-23T10:00:00Z"
}
```

## Customer Carts

Authenticated customers can retrieve their cart using the `/carts/me` endpoint.

### Get Customer Cart

Returns the current customer's active cart or creates a new one.

```http
GET /api/v1/carts/me
Authorization: Bearer <jwt_token>
```

**Response:**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "customer_id": "550e8400-e29b-41d4-a716-446655440002",
  "currency": "USD",
  "subtotal": "150.00",
  "discount_total": "15.00",
  "tax_total": "13.50",
  "shipping_total": "10.00",
  "total": "158.50",
  "coupon_code": "SUMMER10",
  "item_count": 3,
  "items": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440003",
      "product_id": "550e8400-e29b-41d4-a716-446655440004",
      "variant_id": "550e8400-e29b-41d4-a716-446655440005",
      "quantity": 2,
      "unit_price": "50.00",
      "original_price": "50.00",
      "subtotal": "100.00",
      "discount_amount": "10.00",
      "total": "90.00",
      "sku": "PROD-001-L",
      "title": "Premium T-Shirt",
      "variant_title": "Large / Blue",
      "image_url": "https://cdn.example.com/products/001.jpg",
      "requires_shipping": true,
      "is_gift_card": false
    }
  ],
  "expires_at": "2026-03-23T10:00:00Z"
}
```

## Cart Items

### Add Item to Cart

```http
POST /api/v1/carts/{cart_id}/items
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "product_id": "550e8400-e29b-41d4-a716-446655440001",
  "variant_id": null,
  "quantity": 2
}
```

**Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `product_id` | UUID | Yes | Product ID |
| `variant_id` | UUID | No | Product variant ID |
| `quantity` | integer | Yes | Quantity to add (1-9999) |

**Response (201 Created):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440003",
  "product_id": "550e8400-e29b-41d4-a716-446655440004",
  "variant_id": "550e8400-e29b-41d4-a716-446655440005",
  "quantity": 2,
  "unit_price": "50.00",
  "original_price": "50.00",
  "subtotal": "100.00",
  "discount_amount": "10.00",
  "total": "90.00",
  "sku": "PROD-001-L",
  "title": "Premium T-Shirt",
  "variant_title": "Large / Blue",
  "image_url": "https://cdn.example.com/products/001.jpg",
  "requires_shipping": true,
  "is_gift_card": false
}
```

### Update Cart Item

Updates the quantity of a cart item.

```http
PUT /api/v1/carts/{cart_id}/items/{item_id}
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "quantity": 3
}
```

**Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `quantity` | integer | Yes | New quantity (0 to remove) |

**Response (200 OK):**

Returns the updated cart item.

### Remove Item from Cart

```http
DELETE /api/v1/carts/{cart_id}/items/{item_id}
Authorization: Bearer <jwt_token>
```

**Response (204 No Content)**

### Clear Cart

Removes all items from the cart.

```http
DELETE /api/v1/carts/{cart_id}/items
Authorization: Bearer <jwt_token>
```

**Response (204 No Content)**

## Cart Merging

When a customer logs in, their guest cart can be merged with their customer cart.

### Merge Carts

```http
POST /api/v1/carts/merge
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "session_token": "sess_abc123..."
}
```

**Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session_token` | string | Yes | Guest cart session token |

**Response (200 OK):**

Returns the merged cart with all items from both carts combined.

**Behavior:**

- If both carts have the same item, quantities are summed
- If both carts have coupons, the customer's coupon is kept
- The guest cart is marked as converted

## Coupons

### Apply Coupon

```http
POST /api/v1/carts/{cart_id}/coupon
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "coupon_code": "SUMMER20"
}
```

**Response (200 OK):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "coupon_code": "SUMMER20",
  "subtotal": "150.00",
  "discount_total": "30.00",
  "tax_total": "12.00",
  "shipping_total": "10.00",
  "total": "142.00"
}
```

**Error Responses:**

- `400 Bad Request` - Invalid coupon code
- `409 Conflict` - Coupon cannot be combined with existing discount

### Remove Coupon

```http
DELETE /api/v1/carts/{cart_id}/coupon
Authorization: Bearer <jwt_token>
```

**Response (200 OK):**

Returns the updated cart without the coupon.

## Delete Cart

Permanently deletes a cart and all its items.

```http
DELETE /api/v1/carts/{cart_id}
Authorization: Bearer <jwt_token>
```

**Response (204 No Content)**

## Cart Calculations

The cart automatically calculates:

```
subtotal = sum(item.quantity * item.unit_price)
discount_total = sum(item.discount_amount) [from coupons]
tax_total = calculated based on shipping address
shipping_total = calculated based on shipping method
total = subtotal - discount_total + tax_total + shipping_total
```

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `CART_NOT_FOUND` | 404 | Cart ID does not exist |
| `CART_EXPIRED` | 400 | Cart has expired |
| `CART_CONVERTED` | 400 | Cart was already converted to an order |
| `ITEM_NOT_FOUND` | 404 | Cart item ID does not exist |
| `INVALID_QUANTITY` | 400 | Quantity must be between 1 and 9999 |
| `PRODUCT_NOT_AVAILABLE` | 400 | Product is inactive or out of stock |
| `COUPON_INVALID` | 400 | Coupon code is invalid or expired |
| `COUPON_MINIMUM_NOT_MET` | 400 | Cart subtotal below coupon minimum |
| `COUPON_USAGE_LIMIT` | 400 | Coupon usage limit reached |

## Webhooks

The Cart system emits the following webhook events:

| Event | Description |
|-------|-------------|
| `cart.created` | New cart created |
| `cart.updated` | Cart details updated |
| `cart.item_added` | Item added to cart |
| `cart.item_updated` | Item quantity updated |
| `cart.item_removed` | Item removed from cart |
| `cart.coupon_applied` | Coupon applied to cart |
| `cart.coupon_removed` | Coupon removed from cart |
| `cart.merged` | Guest cart merged into customer cart |
| `cart.converted` | Cart converted to order |

## Example Flow: Guest to Customer

```javascript
// 1. Guest creates cart
const guestCart = await fetch('/api/v1/carts/guest', {
  method: 'POST'
});
const cartData = await guestCart.json();
localStorage.setItem('cart_session', cartData.session_token);

// 2. Guest adds item
await fetch(`/api/v1/carts/${cartData.id}/items`, {
  method: 'POST',
  headers: { 
    'Authorization': `Bearer ${jwtToken}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({ product_id: '...', quantity: 2 })
});

// 3. Guest logs in
const login = await fetch('/api/v1/auth/login', { ... });
const loginData = await login.json();

// 4. Merge guest cart into customer cart
await fetch('/api/v1/carts/merge', {
  method: 'POST',
  headers: { 
    'Authorization': `Bearer ${loginData.access_token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({ session_token: cartData.session_token })
});

// 5. Clear guest session
localStorage.removeItem('cart_session');
```

## Related Topics

- [Checkout API](checkout.md) - Complete the purchase flow
- [Products API](products.md) - Product catalog
- [Coupons API](coupons.md) - Discount management
