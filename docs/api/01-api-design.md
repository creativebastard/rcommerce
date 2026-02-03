# API Design Specification

## API Philosophy

R commerce follows an **API-first design** philosophy where every feature is accessible through programmatic interfaces. This enables complete headless operation and seamless integration with any frontend or external system.

## API Types

### 1. REST API (Primary)
- **Format**: JSON
- **Auth**: API Keys + Bearer Tokens
- **Versioning**: URL path (`/api/v1/`)
- **Standards**: JSON:API specification where applicable

### 2. GraphQL API (Secondary)
- **Endpoint**: `/graphql`
- **Auth**: Same as REST
- **Use Case**: Frontend data fetching with precise queries
- **Features**: Subscriptions for real-time updates

### 3. Webhooks (Event-Driven)
- **Format**: REST callbacks to configured URLs
- **Payload**: JSON with event type and data
- **Security**: HMAC-SHA256 signatures
- **Retry**: Exponential backoff with jitter

## Base URL & Versioning

```
Production: https://api.yourstore.com/v1
Staging: https://api.staging.yourstore.com/v1
Development: http://localhost:8080/v1
```

**Versioning Strategy:**
- URL-based versioning: `/api/v1/`, `/api/v2/`
- Minor updates: Non-breaking additions only
- Major updates: Breaking changes, new major version
- Deprecation: 6-month notice before endpoint removal

## Authentication & Authorization

R Commerce supports two authentication methods:

### 1. JWT Authentication (User Sessions)

For customer and admin user sessions.

**Login:**
```bash
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Response:**
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

**Using the token:**
```http
GET /api/v1/customers
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Token expiration:**
- Access token: 24 hours (configurable)
- Refresh token: 7 days (configurable)

### 2. API Key Authentication (Service-to-Service)

For server-to-server authentication. Managed via CLI.

```http
GET /api/v1/orders
Authorization: Bearer <prefix>.<secret>
```

**Creating API keys:**
```bash
rcommerce api-key create --name "Production Backend" --scopes "read,write"
```

**API Key Features:**
- Prefix + secret format (e.g., `aB3dEfGh.sEcReTkEy123`)
- Configurable scopes: `read`, `write`
- Optional expiration
- Revocable via CLI

### Permission Scopes

**JWT Tokens:**
- Inherit permissions from user role
- Default: `["read", "write"]` for authenticated users

**API Keys:**
- Explicitly assigned at creation
- Simple scope model: `read`, `write`, or both

### Protected vs Public Routes

**Public (no auth required):**
- `POST /api/v1/auth/login`
- `POST /api/v1/auth/register`
- `POST /api/v1/auth/refresh`
- `POST /api/v1/webhooks/*` (HMAC signature verified)

**Protected (JWT or API key required):**
- `GET/POST /api/v1/products` - *Authentication required to prevent unauthorized scraping*
- `GET /api/v1/products/:id`
- `GET/POST /api/v1/customers`
- `GET/POST /api/v1/orders`
- All `/api/v1/carts/*` endpoints
- All `/api/v1/payments/*` endpoints

> **Note:** Product endpoints require authentication to protect against data scraping and unauthorized access to product information.

## Common Response Formats

### Success Response

```json
{
  "data": {
    "id": "ord_123456",
    "status": "processing",
    ...
  },
  "meta": {
    "request_id": "req_abc123",
    "timestamp": "2024-01-23T14:13:35Z"
  }
}
```

### List Response (Paginated)

```json
{
  "data": [
    { "id": "ord_123", ... },
    { "id": "ord_124", ... }
  ],
  "meta": {
    "request_id": "req_abc123",
    "timestamp": "2024-01-23T14:13:35Z",
    "pagination": {
      "total": 150,
      "per_page": 20,
      "current_page": 1,
      "total_pages": 8,
      "has_next": true,
      "has_prev": false
    }
  }
}
```

### Error Response

```json
{
  "error": {
    "code": "order_not_found",
    "message": "Order with ID 'ord_999' was not found",
    "details": {
      "order_id": "ord_999",
      "request_id": "req_xyz789"
    },
    "documentation": "https://docs.rcommerce.app/errors/order_not_found"
  },
  "meta": {
    "request_id": "req_xyz789",
    "timestamp": "2024-01-23T14:13:35Z"
  }
}
```

**Error Codes:**
- `400` - Bad Request (validation errors)
- `401` - Unauthorized (invalid API key)
- `403` - Forbidden (insufficient permissions)
- `404` - Not Found (resource doesn't exist)
- `409` - Conflict (resource state conflict)
- `422` - Unprocessable Entity (business rule violation)
- `429` - Too Many Requests (rate limit exceeded)
- `500` - Internal Server Error
- `502` - Bad Gateway (external service error)
- `503` - Service Unavailable

## Core API Endpoints

### Products

```
GET    /v1/products                 # List products (paginated)
GET    /v1/products/:id             # Get product by ID
POST   /v1/products                 # Create product
PUT    /v1/products/:id             # Update product
PATCH  /v1/products/:id             # Partial update
DELETE /v1/products/:id             # Delete product

GET    /v1/products/:id/variants    # List product variants
GET    /v1/products/:id/images      # List product images

GET    /v1/categories               # List categories
GET    /v1/categories/:id           # Get category
```

**Query Parameters:**
- `status` - Filter by status (active, draft, archived)
- `category_id` - Filter by category
- `ids` - Comma-separated list of IDs
- `created_after`, `created_before` - Date filters
- `sort` - Sort field and direction (e.g., `price:desc`, `created_at:asc`)
- `q` - Search query
- `page`, `per_page` - Pagination

### Orders

```
GET    /v1/orders                   # List orders
GET    /v1/orders/:id               # Get order
POST   /v1/orders                   # Create order (manual/admin)
PUT    /v1/orders/:id               # Update order
PATCH  /v1/orders/:id               # Partial update
DELETE /v1/orders/:id               # Delete order (with constraints)

POST   /v1/orders/:id/payments      # Process payment
POST   /v1/orders/:id/fulfillments  # Create fulfillment
POST   /v1/orders/:id/cancel        # Cancel order
POST   /v1/orders/:id/refund        # Process refund

GET    /v1/orders/:id/notes         # List order notes
POST   /v1/orders/:id/notes         # Add note
```

**Order Statuses:**
pending, confirmed, processing, on_hold, completed, cancelled, refunded, fraud_review

**Query Parameters:**
- `status` - Filter by status
- `customer_id` - Filter by customer
- `payment_status` - Filter by payment status
- `fulfillment_status` - Filter by fulfillment status
- `created_after`, `created_before` - Date filters
- `total_min`, `total_max` - Total amount filters

### Customers

```
GET    /v1/customers                # List customers
GET    /v1/customers/:id            # Get customer
POST   /v1/customers                # Create customer
PUT    /v1/customers/:id            # Update customer
PATCH  /v1/customers/:id            # Partial update
DELETE /v1/customers/:id            # Delete customer (GDPR)

GET    /v1/customers/:id/orders     # Customer order history
GET    /v1/customers/:id/addresses  # Customer addresses
POST   /v1/customers/:id/addresses  # Add address
```

### Cart & Checkout

```
POST   /v1/cart                     # Create cart session
GET    /v1/cart/:token              # Get cart
PUT    /v1/cart/:token              # Update cart
DELETE /v1/cart/:token              # Clear cart

POST   /v1/cart/:token/items        # Add item to cart
PUT    /v1/cart/:token/items/:id    # Update item quantity
DELETE /v1/cart/:token/items/:id    # Remove item

POST   /v1/checkout                 # Create checkout session
GET    /v1/checkout/:session_id     # Get checkout
POST   /v1/checkout/:session_id/payments # Process payment
```

### Payments (v1 - Legacy)

```
GET    /v1/payments                 # List payments
GET    /v1/payments/:id             # Get payment
POST   /v1/payments                 # Create payment (manual refund, etc)
PATCH  /v1/payments/:id             # Update payment (e.g., status)

GET    /v1/payment-methods          # List available payment methods
GET    /v1/payment-gateways         # List configured gateways
```

### Payments (v2 - Agnostic)

The v2 Payments API provides a provider-agnostic interface where all payment processing happens server-side. The frontend sends card data to R Commerce, which then communicates with payment providers.

```
POST   /v2/payments/methods         # Get available payment methods for checkout
POST   /v2/payments                 # Initiate payment (server-side processing)
GET    /v2/payments/:id             # Get payment status
POST   /v2/payments/:id/complete    # Complete 3DS/redirect action
POST   /v2/payments/:id/refund      # Process refund

POST   /v2/payment-methods          # Save payment method for customer
GET    /v2/customers/:id/payment-methods  # List saved payment methods
DELETE /v2/payment-methods/:token   # Delete saved payment method

POST   /v2/webhooks/:gateway       # Receive webhooks from payment providers
```

**Key Differences from v1:**
- **Server-side processing**: Card data is sent to R Commerce API, not directly to Stripe
- **Unified interface**: Same API structure works for all gateways (Stripe, Airwallex, WeChat Pay, etc.)
- **3D Secure handling**: Backend returns `requires_action` response; frontend handles redirect/iframe
- **No provider SDK required**: Frontend doesn't need Stripe.js or other provider SDKs

**Example v2 Payment Flow:**

```javascript
// 1. Get available payment methods
const methods = await fetch('/api/v2/payments/methods', {
  method: 'POST',
  body: JSON.stringify({ currency: 'USD', amount: '99.99' })
});

// 2. Initiate payment with card data
const result = await fetch('/api/v2/payments', {
  method: 'POST',
  body: JSON.stringify({
    gateway_id: 'stripe',
    amount: '99.99',
    currency: 'USD',
    payment_method: {
      type: 'card',
      card: {
        number: '4242424242424242',
        exp_month: 12,
        exp_year: 2025,
        cvc: '123'
      }
    }
  })
});

// 3. Handle response
if (result.type === 'success') {
  // Payment complete
} else if (result.type === 'requires_action') {
  // Handle 3D Secure or redirect
  window.location.href = result.action_data.redirect_url;
}
```

### Shipping

```
POST   /v1/shipping/rates           # Calculate shipping rates
GET    /v1/shipping/zones           # List shipping zones
POST   /v1/shipping/labels          # Generate shipping label
GET    /v1/shipping/tracking/:id    # Get tracking info
```

### Webhooks

```
GET    /v1/webhooks                 # List webhook endpoints
GET    /v1/webhooks/:id             # Get webhook endpoint
POST   /v1/webhooks                 # Create webhook endpoint
PUT    /v1/webhooks/:id             # Update webhook
PATCH  /v1/webhooks/:id             # Partial update
DELETE /v1/webhooks/:id             # Delete webhook

GET    /v1/webhooks/:id/deliveries  # List delivery attempts
POST   /v1/webhooks/:id/test        # Send test webhook
```

## Webhook Events

### Order Events
- `order.created` - New order placed
- `order.updated` - Order information changed
- `order.cancelled` - Order cancelled
- `order.completed` - Order fulfilled and completed
- `order.payment_failed` - Payment attempt failed
- `order.fraud_detected` - Fraud rule triggered

### Payment Events
- `payment.created` - Payment initiated
- `payment.succeeded` - Payment successful
- `payment.failed` - Payment failed
- `payment.refunded` - Payment refunded
- `payment.dispute.created` - Chargeback/dispute opened

### Fulfillment Events
- `fulfillment.created` - Fulfillment created
- `fulfillment.tracking_updated` - Tracking number added
- `fulfillment.delivered` - Order marked delivered

### Customer Events
- `customer.created` - New customer registered
- `customer.updated` - Customer info updated

### Product Events
- `product.created` - New product added
- `product.updated` - Product information changed
- `product.deleted` - Product removed
- `product.inventory_changed` - Stock quantity changed
- `product.back_in_stock` - Item back in stock

## GraphQL API Example

### Query

```graphql
query GetOrder($id: ID!) {
  order(id: $id) {
    id
    orderNumber
    status
    total {
      amount
      currency
    }
    customer {
      id
      email
      firstName
      lastName
    }
    lineItems {
      id
      quantity
      product {
        name
        sku
      }
      total
    }
    fulfillments {
      id
      status
      trackingNumber
      trackingUrl
    }
  }
}
```

### Mutation

```graphql
mutation CreateOrder($input: OrderInput!) {
  createOrder(input: $input) {
    order {
      id
      status
      total
    }
    errors {
      field
      message
    }
  }
}
```

### Subscription

```graphql
subscription OrderUpdates($orderId: ID!) {
  orderUpdated(orderId: $orderId) {
    id
    status
    updatedAt
  }
}
```

## Rate Limiting

**Limits:**
- Public endpoints: 100 requests/minute
- Authenticated endpoints: 1000 requests/minute
- Admin endpoints: 5000 requests/minute
- Webhook deliveries: 10 attempts with exponential backoff

**Headers:**
```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1706014415
```

## Error Handling Best Practices

### Client-Side
```javascript
// Example error handling
try {
  const response = await fetch('/v1/orders', {
    headers: { 'Authorization': 'Bearer sk_xxx' }
  });
  
  if (!response.ok) {
    const error = await response.json();
    
    if (response.status === 429) {
      // Retry with exponential backoff
      await sleep(Math.pow(2, attempt) * 1000);
      return retry();
    }
    
    if (response.status === 409) {
      // Handle conflict (e.g., inventory changed)
      throw new OrderConflictError(error);
    }
    
    throw new APIError(error);
  }
  
  return await response.json();
} catch (error) {
  // Log request_id for support
  console.error('Request failed:', error.request_id);
}
```

## SDK Examples

### Rust SDK
```rust
use rcommerce::Client;

let client = Client::new("sk_live_xxx");
let order = client.orders().create(&CreateOrder {
    customer_id: "cus_123".to_string(),
    line_items: vec![LineItem {
        product_id: "prod_456".to_string(),
        quantity: 2,
        ..Default::default()
    }],
    ..Default::default()
}).await?;
```

### JavaScript SDK
```javascript
import { RCommerce } from '@rcommerce/sdk';

const client = new RCommerce('sk_live_xxx');

const order = await client.orders.create({
  customer_id: 'cus_123',
  line_items: [{
    product_id: 'prod_456',
    quantity: 2
  }]
});
```

## OpenAPI Specification

Full OpenAPI 3.0 specification available at:
- JSON: `https://api.rcommerce.app/v1/openapi.json`
- YAML: `https://api.rcommerce.app/v1/openapi.yaml`

Interactive documentation (Swagger UI) available at:
- `https://api.rcommerce.app/docs`

## Pagination

All list endpoints support pagination:

**Request:**
```http
GET /v1/orders?page=2&per_page=50
```

**Default Values:**
- `page`: 1
- `per_page`: 20 (min: 1, max: 100)

**Response Meta:**
```json
{
  "meta": {
    "pagination": {
      "total": 153,
      "per_page": 50,
      "current_page": 2,
      "total_pages": 4,
      "has_next": true,
      "has_prev": true
    }
  }
}
```

## Request IDs

Every request returns a unique request ID for debugging:

**Header:**
```
X-Request-ID: req_abc123xyz
```

**Include in support requests** to help trace issues through logs.

---

Next: [02-error-codes.md](02-error-codes.md) - Complete error code reference
