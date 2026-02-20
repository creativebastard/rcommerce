# Webhooks API

!!! info "Implementation Status"
    Webhook management API is fully implemented. All endpoints support listing, creating, updating, deleting, testing webhooks, and viewing delivery history. HMAC-SHA256 signature verification is included for security.

The Webhooks API allows you to receive real-time event notifications via HTTP callbacks.

## Overview

Webhooks enable your application to receive push notifications when events occur in your R Commerce store, rather than polling for changes.

## Base URL

```
/api/v1/webhooks
```

## Authentication

Webhook management requires API key authentication.

```http
Authorization: Bearer YOUR_API_KEY
```

## Webhook Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440300",
  "name": "Order Notifications",
  "url": "https://your-app.com/webhooks/rcommerce",
  "events": ["order.created", "order.paid", "order.shipped"],
  "is_active": true,
  "last_triggered_at": "2024-01-15T10:00:00Z",
  "created_at": "2024-01-15T10:00:00Z"
}
```

### Webhook Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier |
| `name` | string | Webhook name for identification |
| `url` | string | HTTPS endpoint URL |
| `events` | array | Event types to subscribe to |
| `is_active` | boolean | Whether webhook is active |
| `last_triggered_at` | datetime | Last successful delivery |
| `created_at` | datetime | Creation timestamp |

### Delivery History

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440301",
  "event_type": "order.created",
  "status": 200,
  "delivered_at": "2024-01-15T10:00:00Z",
  "created_at": "2024-01-15T10:00:00Z"
}
```

## Endpoints

### List Webhooks

```http
GET /api/v1/webhooks
```

Retrieve all configured webhooks.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `is_active` | boolean | Filter by active status |
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |

#### Response

```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440300",
    "name": "Order Notifications",
    "url": "https://your-app.com/webhooks/rcommerce",
    "events": ["order.created", "order.paid"],
    "is_active": true,
    "last_triggered_at": "2024-01-15T10:00:00Z",
    "created_at": "2024-01-15T10:00:00Z"
  }
]
```

### Get Webhook

```http
GET /api/v1/webhooks/{id}
```

Retrieve a specific webhook configuration.

#### Response

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440300",
  "name": "Order Notifications",
  "url": "https://your-app.com/webhooks/rcommerce",
  "events": ["order.created", "order.paid", "order.shipped"],
  "is_active": true,
  "last_triggered_at": "2024-01-15T10:00:00Z",
  "created_at": "2024-01-15T10:00:00Z"
}
```

### Create Webhook

```http
POST /api/v1/webhooks
```

Register a new webhook endpoint.

#### Request Body

```json
{
  "name": "Order Notifications",
  "url": "https://your-app.com/webhooks/rcommerce",
  "events": ["order.created", "order.paid", "order.shipped"],
  "secret": "optional-custom-secret"
}
```

#### Required Fields

- `name` - Webhook name for identification
- `url` - HTTPS URL that can receive POST requests
- `events` - Array of event types to subscribe to

#### Response

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440300",
  "name": "Order Notifications",
  "url": "https://your-app.com/webhooks/rcommerce",
  "events": ["order.created", "order.paid", "order.shipped"],
  "is_active": true,
  "created_at": "2024-01-15T10:00:00Z"
}
```

### Update Webhook

```http
PUT /api/v1/webhooks/{id}
```

Update webhook configuration.

#### Request Body

```json
{
  "name": "Updated Name",
  "url": "https://new-url.com/webhooks",
  "events": ["order.created", "order.paid"],
  "is_active": true
}
```

All fields are optional. Only provided fields will be updated.

### Delete Webhook

```http
DELETE /api/v1/webhooks/{id}
```

Remove a webhook subscription.

#### Response

```json
{
  "success": true,
  "message": "Webhook deleted successfully"
}
```

### Test Webhook

```http
POST /api/v1/webhooks/{id}/test
```

Send a test event to the webhook URL.

#### Request Body

```json
{
  "event_type": "order.created",
  "payload": {
    "custom": "data"
  }
}
```

If `payload` is not provided, a default test payload will be used.

#### Response

```json
{
  "success": true,
  "status_code": 200,
  "response_body": "OK",
  "duration_ms": 150,
  "message": "Webhook test successful"
}
```

### Get Delivery History

```http
GET /api/v1/webhooks/{id}/deliveries
```

Retrieve delivery history for a webhook.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |

#### Response

```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440301",
    "event_type": "order.created",
    "status": 200,
    "delivered_at": "2024-01-15T10:00:00Z",
    "created_at": "2024-01-15T10:00:00Z"
  }
]
```

## Webhook Events

### Orders

| Event | Description |
|-------|-------------|
| `order.created` | New order placed |
| `order.paid` | Order payment received |
| `order.shipped` | Order fulfillment started |
| `order.delivered` | Order delivered |
| `order.cancelled` | Order cancelled |
| `order.refunded` | Order refunded |

### Products

| Event | Description |
|-------|-------------|
| `product.created` | New product created |
| `product.updated` | Product information changed |
| `product.deleted` | Product removed |
| `product.low_stock` | Inventory below threshold |
| `product.out_of_stock` | Inventory reached zero |

### Customers

| Event | Description |
|-------|-------------|
| `customer.created` | New customer account created |
| `customer.updated` | Customer information changed |

### Payments

| Event | Description |
|-------|-------------|
| `payment.succeeded` | Payment completed |
| `payment.failed` | Payment failed |
| `refund.created` | Refund initiated |

### Subscriptions

| Event | Description |
|-------|-------------|
| `subscription.created` | New subscription |
| `subscription.renewed` | Subscription renewed |
| `subscription.cancelled` | Subscription cancelled |
| `subscription.payment_failed` | Subscription payment failed |

## Webhook Payload

When an event occurs, R Commerce sends a POST request to your webhook URL:

```http
POST /your-webhook-endpoint
Content-Type: application/json
X-Webhook-Signature: sha256=abc123...
X-Webhook-Test: true (for test deliveries)

{
  "event": "order.created",
  "timestamp": "2024-01-23T14:13:35Z",
  "data": {
    "order_id": "ord_123456",
    "order_number": "ORD-2024-001",
    "customer_id": "cus_789",
    "total": "99.99",
    "currency": "USD"
  }
}
```

## Security

### Signature Verification

Webhooks are signed with HMAC-SHA256 for security. Verify the signature to ensure the webhook came from R Commerce:

```python
import hmac
import hashlib

def verify_webhook(payload, signature, secret):
    expected = hmac.new(
        secret.encode('utf-8'),
        payload.encode('utf-8'),
        hashlib.sha256
    ).hexdigest()
    return hmac.compare_digest(f"sha256={expected}", signature)
```

### Best Practices

1. **Always use HTTPS** - Webhooks require secure endpoints
2. **Verify signatures** - Validate the HMAC signature on every request
3. **Respond quickly** - Return a 2xx response within 30 seconds
4. **Handle retries** - Webhooks may be retried if delivery fails
5. **Idempotency** - Handle duplicate events gracefully using the event ID

## Retry Policy

If your endpoint returns a non-2xx status code or times out:

- First retry: 1 second
- Second retry: 3 seconds
- Third retry: 7 seconds
- Maximum: 3 retries

After all retries fail, the webhook will be marked as failed and can be retried manually via the API.

## Testing Webhooks

### Using the Test Endpoint

Use the test endpoint to verify your webhook integration:

```bash
curl -X POST https://api.rcommerce.app/v1/webhooks/{id}/test \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "event_type": "order.created"
  }'
```

### Local Development with ngrok

For local development, use ngrok to expose your local server:

```bash
# Start your local server
npm run dev

# In another terminal, expose it via ngrok
ngrok http 3000

# Use the ngrok HTTPS URL when creating the webhook
curl -X POST https://api.rcommerce.app/v1/webhooks \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Local Dev",
    "url": "https://abc123.ngrok.io/webhooks",
    "events": ["order.created"]
  }'
```
