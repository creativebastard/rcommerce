# Orders API

The Orders API manages the complete order lifecycle from creation through fulfillment.

## Base URL

```
/api/v1/orders
```

## Authentication

All order endpoints require authentication. Admin endpoints require secret key.

```http
Authorization: Bearer YOUR_API_KEY
```

## Order Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440100",
  "order_number": "1001",
  "customer_id": "550e8400-e29b-41d4-a716-446655440001",
  "email": "customer@example.com",
  "phone": "+1-555-0123",
  "status": "open",
  "financial_status": "paid",
  "fulfillment_status": "unfulfilled",
  "currency": "USD",
  "subtotal_price": "49.99",
  "total_tax": "4.50",
  "total_shipping": "5.00",
  "total_discounts": "0.00",
  "total_price": "59.49",
  "total_weight": "1.0",
  "total_items": 2,
  "line_items": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440101",
      "product_id": "550e8400-e29b-41d4-a716-446655440000",
      "variant_id": "550e8400-e29b-41d4-a716-446655440001",
      "title": "Premium Cotton T-Shirt",
      "variant_title": "Medium / Black",
      "sku": "TSH-M-BLK",
      "quantity": 2,
      "price": "24.99",
      "total": "49.98",
      "requires_shipping": true,
      "is_gift_card": false
    }
  ],
  "shipping_address": {
    "first_name": "John",
    "last_name": "Doe",
    "company": "Acme Inc",
    "phone": "+1-555-0123",
    "address1": "123 Main St",
    "address2": "Apt 4B",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001"
  },
  "billing_address": {
    "first_name": "John",
    "last_name": "Doe",
    "company": "Acme Inc",
    "phone": "+1-555-0123",
    "address1": "123 Main St",
    "address2": "Apt 4B",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001"
  },
  "shipping_lines": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440102",
      "title": "Standard Shipping",
      "price": "5.00",
      "code": "standard",
      "source": "shopify"
    }
  ],
  "tax_lines": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440103",
      "title": "NY State Tax",
      "rate": "0.09",
      "price": "4.50"
    }
  ],
  "discount_codes": [],
  "note": "Please gift wrap",
  "note_attributes": {
    "gift_message": "Happy Birthday!"
  },
  "cart_token": "cart_abc123",
  "checkout_token": "checkout_def456",
  "referring_site": "https://google.com",
  "landing_site": "/products/t-shirt",
  "source_name": "web",
  "client_details": {
    "browser_ip": "192.168.1.1",
    "user_agent": "Mozilla/5.0...",
    "session_hash": "sess_xyz789"
  },
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:05:00Z",
  "processed_at": "2024-01-15T10:01:00Z",
  "closed_at": null,
  "cancelled_at": null,
  "cancel_reason": null
}
```

### Order Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier |
| `order_number` | string | Human-readable order number |
| `customer_id` | UUID | Associated customer (null for guest) |
| `email` | string | Customer email address |
| `phone` | string | Customer phone number |
| `status` | string | `open`, `closed`, `cancelled` |
| `financial_status` | string | `pending`, `authorized`, `paid`, `partially_paid`, `refunded`, `partially_refunded`, `voided` |
| `fulfillment_status` | string | `unfulfilled`, `partial`, `fulfilled` |
| `currency` | string | ISO 4217 currency code |
| `subtotal_price` | decimal | Sum of line items |
| `total_tax` | decimal | Total tax amount |
| `total_shipping` | decimal | Total shipping cost |
| `total_discounts` | decimal | Total discounts applied |
| `total_price` | decimal | Final order total |
| `total_weight` | decimal | Total weight in kg |
| `total_items` | integer | Total quantity of items |
| `line_items` | array | Products in the order |
| `shipping_address` | object | Delivery address |
| `billing_address` | object | Billing address |
| `shipping_lines` | array | Selected shipping methods |
| `tax_lines` | array | Applied taxes |
| `discount_codes` | array | Applied discount codes |
| `note` | string | Customer note |
| `note_attributes` | object | Custom key-value data |
| `cart_token` | string | Reference to original cart |
| `checkout_token` | string | Reference to checkout session |
| `referring_site` | string | Traffic source URL |
| `landing_site` | string | First page visited |
| `source_name` | string | Order source (web, pos, api) |
| `client_details` | object | Browser/IP information |
| `created_at` | datetime | Order creation time |
| `updated_at` | datetime | Last modification |
| `processed_at` | datetime | When order was processed |
| `closed_at` | datetime | When order was closed |
| `cancelled_at` | datetime | When order was cancelled |
| `cancel_reason` | string | Reason for cancellation |

## Endpoints

### List Orders

```http
GET /api/v1/orders
```

Retrieve a paginated list of orders.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |
| `status` | string | Filter by order status |
| `financial_status` | string | Filter by payment status |
| `fulfillment_status` | string | Filter by fulfillment status |
| `customer_id` | UUID | Filter by customer |
| `email` | string | Filter by customer email |
| `order_number` | string | Search by order number |
| `min_total` | decimal | Minimum order total |
| `max_total` | decimal | Maximum order total |
| `created_after` | datetime | Created after date |
| `created_before` | datetime | Created before date |
| `updated_after` | datetime | Updated after date |
| `sort` | string | `created_at`, `updated_at`, `total_price` |
| `order` | string | `asc` or `desc` (default: desc) |

#### Example Request

```http
GET /api/v1/orders?status=open&financial_status=paid&sort=created_at&order=desc
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440100",
      "order_number": "1001",
      "email": "customer@example.com",
      "total_price": "59.49",
      "currency": "USD",
      "status": "open",
      "financial_status": "paid",
      "fulfillment_status": "unfulfilled",
      "created_at": "2024-01-15T10:00:00Z",
      "updated_at": "2024-01-15T10:05:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 45,
      "per_page": 20,
      "current_page": 1,
      "total_pages": 3,
      "has_next": true,
      "has_prev": false
    }
  }
}
```

### Get Order

```http
GET /api/v1/orders/{id}
```

Retrieve a single order by ID or order number.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | string | Order UUID or order number |

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `include` | string | Related data: `fulfillments`, `refunds`, `transactions`, `notes` |

#### Example Request

```http
GET /api/v1/orders/1001?include=fulfillments,transactions
Authorization: Bearer sk_live_xxx
```

### Create Order (Admin)

```http
POST /api/v1/orders
```

Create a new order manually (admin only).

#### Request Body

```json
{
  "email": "customer@example.com",
  "line_items": [
    {
      "variant_id": "550e8400-e29b-41d4-a716-446655440001",
      "quantity": 2,
      "price": "24.99"
    }
  ],
  "shipping_address": {
    "first_name": "John",
    "last_name": "Doe",
    "address1": "123 Main St",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001"
  },
  "billing_address": {
    "first_name": "John",
    "last_name": "Doe",
    "address1": "123 Main St",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001"
  },
  "shipping_lines": [
    {
      "title": "Standard Shipping",
      "price": "5.00"
    }
  ],
  "note": "Please gift wrap",
  "send_receipt": true,
  "inventory_behaviour": "decrement_obeying_policy"
}
```

### Update Order

```http
PUT /api/v1/orders/{id}
```

Update order details (limited fields after creation).

#### Request Body

```json
{
  "email": "newemail@example.com",
  "note": "Updated note",
  "tags": ["vip", "wholesale"]
}
```

### Cancel Order

```http
POST /api/v1/orders/{id}/cancel
```

Cancel an open order.

#### Request Body

```json
{
  "reason": "customer_request",
  "restock": true,
  "notify_customer": true
}
```

#### Cancel Reasons

- `customer_request` - Customer requested cancellation
- `fraudulent` - Order flagged as fraudulent
- `inventory` - Item out of stock
- `other` - Other reason

### Close Order

```http
POST /api/v1/orders/{id}/close
```

Close a fulfilled order.

### Reopen Order

```http
POST /api/v1/orders/{id}/reopen
```

Reopen a closed or cancelled order.

## Line Items

### Add Line Item

```http
POST /api/v1/orders/{order_id}/line_items
```

Add a product to an existing order.

#### Request Body

```json
{
  "variant_id": "550e8400-e29b-41d4-a716-446655440001",
  "quantity": 1,
  "price": "24.99"
}
```

### Update Line Item

```http
PUT /api/v1/orders/{order_id}/line_items/{line_item_id}
```

#### Request Body

```json
{
  "quantity": 3,
  "price": "22.99"
}
```

### Remove Line Item

```http
DELETE /api/v1/orders/{order_id}/line_items/{line_item_id}
```

## Fulfillments

### List Fulfillments

```http
GET /api/v1/orders/{order_id}/fulfillments
```

### Create Fulfillment

```http
POST /api/v1/orders/{order_id}/fulfillments
```

#### Request Body

```json
{
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "line_items": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440101",
      "quantity": 2
    }
  ],
  "tracking_number": "1Z999AA10123456784",
  "tracking_company": "UPS",
  "tracking_url": "https://www.ups.com/track?tracknum=1Z999AA10123456784",
  "notify_customer": true
}
```

### Update Fulfillment

```http
PUT /api/v1/orders/{order_id}/fulfillments/{fulfillment_id}
```

### Cancel Fulfillment

```http
POST /api/v1/orders/{order_id}/fulfillments/{fulfillment_id}/cancel
```

## Order Notes

### List Notes

```http
GET /api/v1/orders/{order_id}/notes
```

### Create Note

```http
POST /api/v1/orders/{order_id}/notes
```

#### Request Body

```json
{
  "body": "Customer called about delivery time",
  "author": "John Smith",
  "notify_customer": false
}
```

### Delete Note

```http
DELETE /api/v1/orders/{order_id}/notes/{note_id}
```

## Refunds

### Calculate Refund

```http
POST /api/v1/orders/{order_id}/refunds/calculate
```

Calculate refund amounts before processing.

#### Request Body

```json
{
  "shipping": "full",
  "refund_line_items": [
    {
      "line_item_id": "550e8400-e29b-41d4-a716-446655440101",
      "quantity": 1,
      "restock": true
    }
  ]
}
```

### Create Refund

```http
POST /api/v1/orders/{order_id}/refunds
```

#### Request Body

```json
{
  "note": "Customer unhappy with product",
  "notify_customer": true,
  "shipping": {
    "full_refund": true
  },
  "refund_line_items": [
    {
      "line_item_id": "550e8400-e29b-41d4-a716-446655440101",
      "quantity": 1,
      "restock": true
    }
  ],
  "transactions": [
    {
      "parent_id": "550e8400-e29b-41d4-a716-446655440200",
      "amount": "24.99",
      "kind": "refund",
      "gateway": "stripe"
    }
  ]
}
```

## Risk Assessment

### Get Order Risks

```http
GET /api/v1/orders/{order_id}/risks
```

Returns fraud risk assessment for the order.

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `ORDER_NOT_FOUND` | 404 | Order does not exist |
| `ORDER_ALREADY_CANCELLED` | 409 | Order already cancelled |
| `ORDER_ALREADY_CLOSED` | 409 | Order already closed |
| `INVALID_STATUS_TRANSITION` | 400 | Cannot change to requested status |
| `LINE_ITEM_NOT_FOUND` | 404 | Line item does not exist |
| `INVALID_QUANTITY` | 400 | Quantity must be positive |
| `INSUFFICIENT_INVENTORY` | 400 | Not enough stock available |
| `FULFILLMENT_NOT_FOUND` | 404 | Fulfillment does not exist |
| `REFUND_EXCEEDS_TOTAL` | 400 | Refund amount too high |
| `PAYMENT_REQUIRED` | 402 | Order requires payment |

## Webhooks

| Event | Description |
|-------|-------------|
| `order.created` | New order placed |
| `order.updated` | Order information changed |
| `order.cancelled` | Order cancelled |
| `order.closed` | Order closed |
| `order.reopened` | Order reopened |
| `order.payment_received` | Payment captured |
| `order.fulfillment_created` | Fulfillment created |
| `order.fulfillment_updated` | Fulfillment updated |
| `order.fulfillment_cancelled` | Fulfillment cancelled |
| `order.refund_created` | Refund processed |
| `order.note_created` | Note added to order |
