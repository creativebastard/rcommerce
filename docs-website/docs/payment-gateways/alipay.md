# Alipay Integration

Alipay is China's leading digital payment platform with over 1 billion users.

## Features

- **QR Code Payments**: Scan to pay in-store
- **In-App Payments**: Pay within your mobile app
- **Website Payments**: Redirect to Alipay for payment
- **Express Checkout**: One-touch payment for returning users
- **Multi-Currency**: Support for CNY and major foreign currencies

## Prerequisites

- Alipay merchant account
- Completed business verification
- API credentials from Alipay Developer Center

## Configuration

### Environment Variables

```bash
ALIPAY_APP_ID=your_app_id
ALIPAY_PRIVATE_KEY=your_private_key
ALIPAY_PUBLIC_KEY=alipay_public_key
ALIPAY_GATEWAY_URL=https://openapi.alipay.com/gateway.do
```

### Config File

```toml
[payment.alipay]
enabled = true
app_id = "${ALIPAY_APP_ID}"
private_key = "${ALIPAY_PRIVATE_KEY}"
public_key = "${ALIPAY_PUBLIC_KEY}"
gateway_url = "https://openapi.alipay.com/gateway.do"
sandbox = false
```

## API Usage

### Create Payment (Website)

```http
POST /api/v1/payments
Content-Type: application/json
Authorization: Bearer <token>

{
  "gateway": "alipay",
  "amount": "299.00",
  "currency": "CNY",
  "payment_method": "web",
  "subject": "Product Purchase",
  "body": "Order #12345",
  "return_url": "https://yoursite.com/payment/success",
  "notify_url": "https://yoursite.com/webhooks/alipay"
}
```

**Response:**

```json
{
  "id": "2026...",
  "payment_url": "https://mapi.alipay.com/...",
  "status": "pending"
}
```

Redirect the customer to `payment_url` to complete payment.

### Create Payment (Mobile App)

```http
POST /api/v1/payments
{
  "gateway": "alipay",
  "amount": "299.00",
  "currency": "CNY",
  "payment_method": "app",
  "subject": "Product Purchase"
}
```

**Response:**

```json
{
  "id": "2026...",
  "order_string": "app_id=...&biz_content=...&sign=...",
  "status": "pending"
}
```

Pass `order_string` to your mobile app to initiate Alipay SDK payment.

### Query Payment Status

```http
GET /api/v1/payments/{payment_id}
Authorization: Bearer <token>
```

## Webhook Notifications

Alipay sends async notifications to your `notify_url`:

```
POST https://yoursite.com/webhooks/alipay
Content-Type: application/x-www-form-urlencoded

trade_status=TRADE_SUCCESS
out_trade_no=your_order_id
trade_no=alipay_trade_id
total_amount=299.00
...
```

Verify the notification signature before processing.

## Payment Status

| Status | Description |
|--------|-------------|
| `WAIT_BUYER_PAY` | Awaiting customer payment |
| `TRADE_CLOSED` | Payment window expired or refunded |
| `TRADE_SUCCESS` | Payment completed |
| `TRADE_FINISHED` | Transaction complete, no refunds allowed |

## Refunds

```http
POST /api/v1/payments/{payment_id}/refund
{
  "amount": "299.00",
  "reason": "Customer request"
}
```

## Testing

Use Alipay sandbox:

```toml
[payment.alipay]
sandbox = true
gateway_url = "https://openapi.alipaydev.com/gateway.do"
```

Sandbox buyer account: `sandbox_buyer@alipay.com`

## Best Practices

1. **Signature Verification**: Always verify Alipay signatures
2. **Idempotency**: Handle duplicate notifications gracefully
3. **Timeout Handling**: Alipay payments timeout after 15 minutes
4. **Mobile Optimization**: Ensure mobile-friendly payment flow
