# Webhooks API

!!! info "Implementation Status"
    Payment webhooks are fully implemented. Order, product, and customer webhooks are available with basic functionality. Advanced webhook features like delivery logs and retry configuration are planned for v0.2.

The Webhooks API allows you to receive real-time event notifications via HTTP callbacks.

## Overview

Webhooks enable your application to receive push notifications when events occur in your R Commerce store, rather than polling for changes.

## Base URL

```
/api/v1/webhooks
```

## Authentication

Webhook management requires secret API key.

```http
Authorization: Bearer YOUR_SECRET_KEY
```

## Webhook Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440300",
  "url": "https://your-app.com/webhooks/rcommerce",
  "topic": "order.created",
  "include_fields": ["id", "order_number", "total_price", "customer"],
  "metafield_namespaces": ["global"],
  "secret": "whsec_...",
  "api_version": "2024-01",
  "is_active": true,
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:00Z"
}
```

### Webhook Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier |
| `url` | string | HTTPS endpoint URL |
| `topic` | string | Event topic to subscribe to |
| `include_fields` | array | Specific fields to include (optional) |
| `metafield_namespaces` | array | Metafield namespaces to include |
| `secret` | string | Signing secret for verification |
| `api_version` | string | API version for payload format |
| `is_active` | boolean | Whether webhook is active |
| `created_at` | datetime | Creation timestamp |
| `updated_at` | datetime | Last modification |

## Endpoints

### List Webhooks

```http
GET /api/v1/webhooks
```

Retrieve all configured webhooks.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `topic` | string | Filter by event topic |
| `is_active` | boolean | Filter by active status |

### Get Webhook

```http
GET /api/v1/webhooks/{id}
```

Retrieve a specific webhook configuration.

### Create Webhook

```http
POST /api/v1/webhooks
```

Register a new webhook endpoint.

#### Request Body

```json
{
  "url": "https://your-app.com/webhooks/rcommerce",
  "topic": "order.created",
  "include_fields": ["id", "order_number", "total_price"],
  "api_version": "2024-01"
}
```

#### Required Fields

- `url` - HTTPS URL that can receive POST requests
- `topic` - Event type to subscribe to

### Update Webhook

```http
PUT /api/v1/webhooks/{id}
```

Update webhook configuration.

#### Request Body

```json
{
  "url": "https://new-url.com/webhooks",
  "is_active": true,
  "include_fields": ["id", "order_number", "customer"]
}
```

### Delete Webhook

```http
DELETE /api/v1/webhooks/{id}
```

Remove a webhook subscription.

### Test Webhook

```http
POST /api/v1/webhooks/{id}/test
```

Send a test event to the webhook URL.

#### Request Body

```json
{
  "event": "order.created"
}
```

## Webhook Topics

### Orders

| Topic | Description |
|-------|-------------|
| `order.created` | New order placed |
| `order.updated` | Order information changed |
| `order.cancelled` | Order cancelled |
| `order.closed` | Order closed |
| `order.reopened` | Order reopened |
| `order.payment_received` | Payment captured |
| `order.fulfillment_created` | Fulfillment created |
| `order.fulfillment_updated` | Fulfillment updated |
| `order.refund_created` | Refund processed |

### Products

| Topic | Description |
|-------|-------------|
| `product.created` | New product created |
| `product.updated` | Product information changed |
| `product.deleted` | Product removed |
| `product.published` | Product published |
| `product.unpublished` | Product unpublished |
| `product.inventory_changed` | Stock quantity updated |
| `product.low_stock` | Inventory below threshold |
| `product.out_of_stock` | Inventory reached zero |

### Customers

| Topic | Description |
|-------|-------------|
| `customer.created` | New customer account created |
| `customer.updated` | Customer information changed |
| `customer.deleted` | Customer account deleted |
| `customer.address_created` | New address added |

### Payments

| Topic | Description |
|-------|-------------|
| `payment.created` | New payment initiated |
| `payment.succeeded` | Payment completed |
| `payment.failed` | Payment failed |
| `refund.created` | Refund initiated |
| `dispute.created` | Dispute opened |

### Cart

| Topic | Description |
|-------|-------------|
| `cart.created` | New cart created |
| `cart.updated` | Cart updated |
| `cart.converted` | Cart converted to order |

## Webhook Payload

### Delivery Headers

```http
POST /webhooks/rcommerce HTTP/1.1
Host: your-app.com
Content-Type: application/json
X-RCommerce-Topic: order.created
X-RCommerce-Webhook-Id: 550e8400-e29b-41d4-a716-446655440300
X-RCommerce-Event-Id: evt_550e8400e29b41d4a716446655440301
X-RCommerce-Signature: t=1705312800,v1=abc123...
User-Agent: R-Commerce-Webhook/1.0
```

### Payload Format

```json
{
  "id": "evt_550e8400e29b41d4a716446655440301",
  "topic": "order.created",
  "api_version": "2024-01",
  "created_at": "2024-01-15T10:00:00Z",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440100",
    "order_number": "1001",
    "total_price": "59.49",
    "currency": "USD",
    "customer": {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "email": "customer@example.com"
    }
  }
}
```

## Signature Verification

Verify webhook signatures to ensure payloads are from R Commerce.

### Signature Format

```
X-RCommerce-Signature: t=<timestamp>,v1=<signature>
```

### Verification Example (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhook(payload, signature, secret) {
  const elements = signature.split(',');
  const signatureHash = elements.find(e => e.startsWith('v1=')).split('v1=')[1];
  const timestamp = elements.find(e => e.startsWith('t=')).split('t=')[1];
  
  // Check timestamp (prevent replay attacks)
  const now = Math.floor(Date.now() / 1000);
  if (now - parseInt(timestamp) > 300) { // 5 minute tolerance
    throw new Error('Webhook timestamp too old');
  }
  
  // Compute expected signature
  const signedPayload = timestamp + '.' + payload;
  const expectedSignature = crypto
    .createHmac('sha256', secret)
    .update(signedPayload)
    .digest('hex');
  
  // Compare signatures
  return crypto.timingSafeEqual(
    Buffer.from(signatureHash),
    Buffer.from(expectedSignature)
  );
}

// Usage
app.post('/webhooks/rcommerce', express.raw({type: 'application/json'}), (req, res) => {
  const signature = req.headers['x-rcommerce-signature'];
  const secret = process.env.WEBHOOK_SECRET;
  
  if (!verifyWebhook(req.body, signature, secret)) {
    return res.status(401).send('Invalid signature');
  }
  
  const event = JSON.parse(req.body);
  // Process event...
  
  res.status(200).send('OK');
});
```

### Verification Example (Python)

```python
import hmac
import hashlib
import time

def verify_webhook(payload: bytes, signature: str, secret: str) -> bool:
    elements = dict(e.split('=') for e in signature.split(','))
    timestamp = elements['t']
    signature_hash = elements['v1']
    
    # Check timestamp
    now = int(time.time())
    if now - int(timestamp) > 300:
        raise ValueError('Webhook timestamp too old')
    
    # Compute expected signature
    signed_payload = f"{timestamp}.{payload.decode()}"
    expected = hmac.new(
        secret.encode(),
        signed_payload.encode(),
        hashlib.sha256
    ).hexdigest()
    
    return hmac.compare_digest(signature_hash, expected)
```

### Verification Example (Rust)

```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

fn verify_webhook(payload: &[u8], signature: &str, secret: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let parts: std::collections::HashMap<_, _> = signature
        .split(',')
        .filter_map(|e| {
            let mut kv = e.splitn(2, '=');
            Some((kv.next()?, kv.next()?))
        })
        .collect();
    
    let timestamp = parts.get("t").ok_or("Missing timestamp")?;
    let signature_hash = parts.get("v1").ok_or("Missing signature")?;
    
    // Check timestamp
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    if now - timestamp.parse::<u64>()? > 300 {
        return Err("Webhook timestamp too old".into());
    }
    
    // Compute signature
    let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())?;
    mac.update(signed_payload.as_bytes());
    let expected = hex::encode(mac.finalize().into_bytes());
    
    Ok(expected == *signature_hash)
}
```

## Delivery Behavior

### Retry Policy

Failed webhook deliveries are retried with exponential backoff:

| Attempt | Delay |
|---------|-------|
| 1 | Immediate |
| 2 | 5 seconds |
| 3 | 25 seconds |
| 4 | 2 minutes |
| 5 | 10 minutes |

Maximum 5 retry attempts over 15 minutes.

### Success Criteria

A webhook delivery is considered successful if your endpoint returns:
- HTTP 200-299 status code
- Response within 30 seconds

### Failed Deliveries

After all retries are exhausted:
- Webhook is automatically disabled
- Email notification sent to store owner
- Failed deliveries logged for 30 days

## Best Practices

### Endpoint Requirements

1. **HTTPS only** - Must use valid SSL certificate
2. **Respond quickly** - Return 200 before processing
3. **Idempotent** - Handle duplicate events gracefully
4. **Verify signatures** - Always validate authenticity

### Handling Events

```javascript
app.post('/webhooks/rcommerce', async (req, res) => {
  // 1. Verify signature
  if (!verifyWebhook(req.body, req.headers['x-rcommerce-signature'], secret)) {
    return res.status(401).send('Invalid signature');
  }
  
  // 2. Acknowledge immediately
  res.status(200).send('OK');
  
  // 3. Process asynchronously
  const event = JSON.parse(req.body);
  
  try {
    switch (event.topic) {
      case 'order.created':
        await handleOrderCreated(event.data);
        break;
      case 'order.cancelled':
        await handleOrderCancelled(event.data);
        break;
      // ...
    }
  } catch (error) {
    // Log error, alert team
    console.error('Webhook processing failed:', error);
  }
});
```

### Idempotency

Use event IDs to prevent duplicate processing:

```javascript
const processedEvents = new Set(); // Use Redis in production

async function handleWebhook(event) {
  if (processedEvents.has(event.id)) {
    return; // Already processed
  }
  
  // Process event...
  
  processedEvents.add(event.id);
}
```

## Delivery Logs

### List Delivery Attempts

```http
GET /api/v1/webhooks/{webhook_id}/deliveries
```

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `status` | string | `success`, `failed` |
| `event_id` | string | Filter by event |

### Delivery Object

```json
{
  "id": "del_550e8400e29b41d4a716446655440302",
  "event_id": "evt_550e8400e29b41d4a716446655440301",
  "webhook_id": "550e8400-e29b-41d4-a716-446655440300",
  "status": "success",
  "http_status": 200,
  "response_body": "OK",
  "attempts": 1,
  "created_at": "2024-01-15T10:00:00Z",
  "completed_at": "2024-01-15T10:00:01Z"
}
```

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `WEBHOOK_NOT_FOUND` | 404 | Webhook does not exist |
| `INVALID_URL` | 400 | URL must be valid HTTPS |
| `INVALID_TOPIC` | 400 | Unknown event topic |
| `URL_UNREACHABLE` | 400 | Test delivery failed |
| `MAX_WEBHOOKS_EXCEEDED` | 429 | Too many webhooks configured |
| `WEBHOOK_DISABLED` | 400 | Webhook is inactive |
