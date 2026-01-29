# Payment Webhooks

Webhooks enable real-time notifications when payment events occur across all supported gateways.

## Configuration

Configure webhook endpoints in your R Commerce settings and each payment gateway's dashboard.

### R Commerce Endpoints

| Gateway | Endpoint URL |
|---------|-------------|
| Stripe | `https://api.yoursite.com/api/v1/webhooks/payments/stripe` |
| Airwallex | `https://api.yoursite.com/api/v1/webhooks/payments/airwallex` |
| Alipay | `https://api.yoursite.com/api/v1/webhooks/payments/alipay` |
| WeChat Pay | `https://api.yoursite.com/api/v1/webhooks/payments/wechatpay` |

### Webhook Events

R Commerce normalizes events across all gateways:

| Event | Description | Stripe | Airwallex | Alipay | WeChat |
|-------|-------------|--------|-----------|--------|--------|
| `payment.created` | Payment intent created | ✓ | ✓ | ✓ | ✓ |
| `payment.pending` | Awaiting customer action | ✓ | ✓ | ✓ | ✓ |
| `payment.processing` | Payment being processed | ✓ | ✓ | - | - |
| `payment.success` | Payment completed | ✓ | ✓ | ✓ | ✓ |
| `payment.failed` | Payment failed | ✓ | ✓ | ✓ | ✓ |
| `payment.cancelled` | Payment cancelled | ✓ | ✓ | ✓ | ✓ |
| `payment.refunded` | Refund processed | ✓ | ✓ | ✓ | ✓ |
| `payment.disputed` | Chargeback/dispute opened | ✓ | - | ✓ | ✓ |

## Webhook Payload

Standard webhook payload format:

```json
{
  "id": "evt_550e8400-e29b-41d4-a716-446655440000",
  "type": "payment.success",
  "api_version": "v1",
  "created_at": "2026-01-28T10:30:00Z",
  "data": {
    "payment": {
      "id": "pay_550e8400-e29b-41d4-a716-446655440001",
      "gateway": "stripe",
      "gateway_payment_id": "pi_3O...",
      "amount": "99.99",
      "currency": "USD",
      "status": "succeeded",
      "order_id": "ord_550e8400-e29b-41d4-a716-446655440002"
    }
  }
}
```

## Security

### Signature Verification

All webhooks include a signature header for verification:

```http
X-Webhook-Signature: t=1706448000,v1=sha256=...
```

Verify signatures to ensure webhooks are from legitimate sources:

```rust
use rcommerce_core::webhook::verify_signature;

let signature = headers.get("X-Webhook-Signature").unwrap();
let body = request.body();
let secret = std::env::var("WEBHOOK_SECRET").unwrap();

if !verify_signature(signature, body, &secret) {
    return Err(Error::unauthorized("Invalid signature"));
}
```

### Replay Protection

- Check `created_at` timestamp (reject if > 5 minutes old)
- Track processed event IDs to prevent duplicates
- Use idempotency keys for webhook handlers

## Handling Webhooks

### Best Practices

1. **Acknowledge Quickly**: Return 200 OK immediately, process asynchronously
2. **Idempotency**: Handle duplicate webhooks gracefully
3. **Retries**: Expect and handle retries (exponential backoff)
4. **Logging**: Log all webhook events for debugging
5. **Ordering**: Don't assume webhooks arrive in order

### Example Handler

```rust
async fn handle_webhook(
    headers: HeaderMap,
    body: Bytes,
    state: AppState,
) -> Result<impl IntoResponse, Error> {
    // Verify signature
    let signature = headers.get("X-Webhook-Signature")
        .ok_or(Error::bad_request("Missing signature"))?;
    
    verify_webhook_signature(&body, signature)?;
    
    // Parse event
    let event: WebhookEvent = serde_json::from_slice(&body)?;
    
    // Queue for async processing
    state.job_queue.enqueue(ProcessWebhookJob {
        event_id: event.id.clone(),
        event_type: event.type_.clone(),
        payload: body.to_vec(),
    }).await?;
    
    // Return immediately
    Ok(StatusCode::OK)
}
```

### Async Processing

```rust
async fn process_webhook(event: WebhookEvent) -> Result<(), Error> {
    // Check for duplicates
    if is_event_processed(&event.id).await? {
        return Ok(());
    }
    
    match event.type_.as_str() {
        "payment.success" => {
            let payment = event.data.payment;
            update_order_status(&payment.order_id, "paid").await?;
            send_confirmation_email(&payment.order_id).await?;
            update_inventory(&payment.order_id).await?;
        }
        "payment.failed" => {
            let payment = event.data.payment;
            update_order_status(&payment.order_id, "payment_failed").await?;
            send_payment_failed_email(&payment.order_id).await?;
        }
        "payment.refunded" => {
            let refund = event.data.refund;
            process_refund(&refund).await?;
        }
        _ => {
            log::warn!("Unhandled webhook event: {}", event.type_);
        }
    }
    
    // Mark as processed
    mark_event_processed(&event.id).await?;
    
    Ok(())
}
```

## Retry Behavior

| Gateway | Retry Strategy | Max Retries |
|---------|---------------|-------------|
| Stripe | Exponential: 1min, 5min, 25min, 125min, 625min | 3 days |
| Airwallex | Exponential: 5s, 25s, 125s, 625s, 3125s | 24 hours |
| Alipay | Fixed: 4min, 10min, 10min, 1hr, 2hr, 6hr, 15hr | 25 hours |
| WeChat Pay | Exponential: 8s, 64s, 512s, 4096s, 32768s | 48 hours |

## Testing Webhooks

### Local Development

Use webhook forwarding tools:

```bash
# Stripe CLI
stripe listen --forward-to localhost:8080/webhooks/stripe

# ngrok for other gateways
ngrok http 8080
```

### Test Events

Trigger test events via API:

```http
POST /api/v1/admin/webhooks/test
Authorization: Bearer <admin_token>

{
  "gateway": "stripe",
  "event_type": "payment.success",
  "payload": {
    "payment_id": "pay_test_123"
  }
}
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Webhooks not received | Check endpoint URL, firewall rules |
| Signature verification fails | Verify secret key matches gateway |
| Duplicate processing | Implement idempotency checks |
| Timeouts | Process asynchronously, return 200 quickly |
| Missing events | Check gateway dashboard for delivery status |

## Dashboard

View webhook delivery status in R Commerce admin:

```
Admin → Settings → Webhooks → Logs
```

Shows:
- Event ID and type
- Delivery status
- Response code
- Retry count
- Payload preview
