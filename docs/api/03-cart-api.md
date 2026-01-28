# Cart API Documentation

The Cart API provides a complete shopping cart system that supports both guest users (via session tokens) and authenticated customers. Carts persist for 30 days by default and can be seamlessly merged when a guest user logs in.

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
| GET /carts/guest | None (uses session token) |
| GET /carts/me | Required (JWT or API Key) |
| All other endpoints | Optional (session token or JWT) |

## Session Management

For guest users, include the session token in the header:

```http
X-Session-Token: <session_token>
```

The session token is returned when creating a guest cart and should be stored client-side (localStorage, cookies, etc.).

## Endpoints

### Create Guest Cart

Creates a new cart for a guest user.

```http
POST /api/v1/carts/guest
Content-Type: application/json

{
  "currency": "USD"
}
```

**Response (201 Created):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "session_token": "sess_abc123xyz",
  "currency": "USD",
  "subtotal": "0.00",
  "discount_total": "0.00",
  "tax_total": "0.00",
  "shipping_total": "0.00",
  "total": "0.00",
  "item_count": 0,
  "items": [],
  "expires_at": "2026-02-27T10:00:00Z"
}
```

**Important:** Store the `session_token` client-side for subsequent requests.

### Get or Create Customer Cart

Returns the current customer's active cart or creates a new one.

```http
GET /api/v1/carts/me
Authorization: Bearer <jwt_token>
```

**Response (200 OK):**

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
  "expires_at": "2026-02-27T10:00:00Z"
}
```

### Get Cart by ID

Retrieves a specific cart with all items.

```http
GET /api/v1/carts/{cart_id}
X-Session-Token: <session_token>
```

### Add Item to Cart

Adds a product to the cart. If the item already exists, quantities are merged.

```http
POST /api/v1/carts/{cart_id}/items
Content-Type: application/json
X-Session-Token: <session_token>

{
  "product_id": "550e8400-e29b-41d4-a716-446655440004",
  "variant_id": "550e8400-e29b-41d4-a716-446655440005",
  "quantity": 2,
  "custom_attributes": {
    "engraving": "Happy Birthday"
  }
}
```

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
  "is_gift_card": false,
  "custom_attributes": {
    "engraving": "Happy Birthday"
  }
}
```

### Update Cart Item

Updates the quantity or custom attributes of a cart item.

```http
PUT /api/v1/carts/{cart_id}/items/{item_id}
Content-Type: application/json
X-Session-Token: <session_token>

{
  "quantity": 3,
  "custom_attributes": {
    "engraving": "Happy Anniversary"
  }
}
```

**Response (200 OK):**

Returns the updated cart item.

### Remove Item from Cart

Removes an item from the cart.

```http
DELETE /api/v1/carts/{cart_id}/items/{item_id}
X-Session-Token: <session_token>
```

**Response (204 No Content)**

### Clear Cart

Removes all items from the cart.

```http
DELETE /api/v1/carts/{cart_id}/items
X-Session-Token: <session_token>
```

**Response (204 No Content)**

### Apply Coupon

Applies a discount coupon to the cart.

```http
POST /api/v1/carts/{cart_id}/coupon
Content-Type: application/json
X-Session-Token: <session_token>

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

Removes the applied coupon from the cart.

```http
DELETE /api/v1/carts/{cart_id}/coupon
X-Session-Token: <session_token>
```

**Response (200 OK):**

Returns the updated cart without the coupon.

### Update Cart Details

Updates cart-level information like email, shipping method, or notes.

```http
PUT /api/v1/carts/{cart_id}
Content-Type: application/json
X-Session-Token: <session_token>

{
  "email": "customer@example.com",
  "shipping_method": "express",
  "notes": "Please gift wrap"
}
```

**Response (200 OK):**

Returns the updated cart.

### Merge Guest Cart

Merges a guest cart into the authenticated customer's cart. Call this after user login.

```http
POST /api/v1/carts/merge
Content-Type: application/json
Authorization: Bearer <jwt_token>

{
  "session_token": "sess_abc123xyz"
}
```

**Response (200 OK):**

Returns the merged cart with all items from both carts combined.

**Behavior:**
- If both carts have the same item, quantities are summed
- If both carts have coupons, the customer's coupon is kept
- The guest cart is marked as converted

### Delete Cart

Permanently deletes a cart and all its items.

```http
DELETE /api/v1/carts/{cart_id}
X-Session-Token: <session_token>
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

| Code | Description |
|------|-------------|
| `CART_NOT_FOUND` | Cart ID does not exist |
| `CART_EXPIRED` | Cart has expired |
| `CART_CONVERTED` | Cart was already converted to an order |
| `ITEM_NOT_FOUND` | Cart item ID does not exist |
| `INVALID_QUANTITY` | Quantity must be between 1 and 9999 |
| `PRODUCT_NOT_AVAILABLE` | Product is inactive or out of stock |
| `COUPON_INVALID` | Coupon code is invalid or expired |
| `COUPON_MINIMUM_NOT_MET` | Cart subtotal below coupon minimum |
| `COUPON_USAGE_LIMIT` | Coupon usage limit reached |

## Webhooks

The Cart system emits the following webhook events:

| Event | Description |
|-------|-------------|
| `cart.created` | New cart created |
| `cart.updated` | Cart details updated |
| `cart.item_added` | Item added to cart |
| `cart.item_updated` | Item quantity/attributes updated |
| `cart.item_removed` | Item removed from cart |
| `cart.coupon_applied` | Coupon applied to cart |
| `cart.coupon_removed` | Coupon removed from cart |
| `cart.merged` | Guest cart merged into customer cart |
| `cart.converted` | Cart converted to order |

## Best Practices

1. **Session Token Storage**: Store session tokens in localStorage or cookies with appropriate security settings
2. **Cart Persistence**: Always check for existing carts before creating new ones
3. **Quantity Validation**: Validate product availability before adding to cart
4. **Error Handling**: Handle cart expiration gracefully by creating new carts
5. **Merge on Login**: Always call the merge endpoint when a guest user logs in

## Example Flow: Guest to Customer

```javascript
// 1. Guest adds items to cart
const guestCart = await fetch('/api/v1/carts/guest', {
  method: 'POST',
  body: JSON.stringify({ currency: 'USD' })
});
localStorage.setItem('cart_session', guestCart.session_token);

// 2. Guest adds item
await fetch(`/api/v1/carts/${guestCart.id}/items`, {
  method: 'POST',
  headers: { 'X-Session-Token': guestCart.session_token },
  body: JSON.stringify({ product_id: '...', quantity: 2 })
});

// 3. Guest logs in
const login = await fetch('/api/v1/auth/login', { ... });

// 4. Merge guest cart into customer cart
await fetch('/api/v1/carts/merge', {
  method: 'POST',
  headers: { 'Authorization': `Bearer ${login.token}` },
  body: JSON.stringify({ session_token: guestCart.session_token })
});

// 5. Clear guest session
localStorage.removeItem('cart_session');
```
