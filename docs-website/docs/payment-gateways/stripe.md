# Stripe Integration

Stripe provides global payment processing with support for cards, wallets, and subscriptions.

## Features

- **Cards**: Visa, Mastercard, Amex, Discover, JCB
- **Wallets**: Apple Pay, Google Pay, Link
- **Bank Transfers**: ACH, SEPA, BACS
- **Buy Now Pay Later**: Klarna, Afterpay, Affirm
- **Subscriptions**: Recurring billing with Stripe Billing

## Configuration

### Environment Variables

```bash
STRIPE_API_KEY=sk_live_...
STRIPE_WEBHOOK_SECRET=whsec_...
STRIPE_PUBLISHABLE_KEY=pk_live_...
```

### Config File

```toml
[payment.stripe]
enabled = true
api_key = "${STRIPE_API_KEY}"
webhook_secret = "${STRIPE_WEBHOOK_SECRET}"
publishable_key = "${STRIPE_PUBLISHABLE_KEY}"
capture_method = "automatic"  # or "manual"
```

## API Usage

### Create Payment Intent

```http
POST /api/v1/payments
Content-Type: application/json
Authorization: Bearer <token>

{
  "gateway": "stripe",
  "amount": "99.99",
  "currency": "USD",
  "payment_method_types": ["card"],
  "metadata": {
    "order_id": "order_123"
  }
}
```

**Response:**

```json
{
  "id": "pi_3O...",
  "client_secret": "pi_3O..._secret_...",
  "status": "requires_payment_method",
  "amount": 9999,
  "currency": "USD"
}
```

Use the `client_secret` with Stripe.js to complete the payment on the frontend.

### Capture Payment (Manual)

If using manual capture:

```http
POST /api/v1/payments/pi_3O.../capture
Authorization: Bearer <token>

{
  "amount": "99.99"
}
```

### Refund Payment

```http
POST /api/v1/payments/pi_3O.../refund
Authorization: Bearer <token>

{
  "amount": "99.99",
  "reason": "requested_by_customer"
}
```

## Webhook Events

Configure webhook endpoint: `https://api.yoursite.com/webhooks/stripe`

| Event | Description |
|-------|-------------|
| `payment_intent.succeeded` | Payment completed |
| `payment_intent.payment_failed` | Payment failed |
| `payment_intent.canceled` | Payment canceled |
| `charge.refunded` | Refund processed |
| `invoice.paid` | Subscription payment received |
| `invoice.payment_failed` | Subscription payment failed |

## Frontend Integration

### Stripe.js Example

```javascript
import { loadStripe } from '@stripe/stripe-js';

const stripe = await loadStripe('pk_live_...');

// Create payment intent on backend
const { client_secret } = await fetch('/api/v1/payments', {
  method: 'POST',
  body: JSON.stringify({ amount: 99.99, gateway: 'stripe' })
}).then(r => r.json());

// Confirm payment
const result = await stripe.confirmCardPayment(client_secret, {
  payment_method: {
    card: cardElement,
    billing_details: { name: 'Customer Name' }
  }
});

if (result.error) {
  // Show error
} else {
  // Payment successful
}
```

## Testing

Use Stripe test cards:

| Card Number | Scenario |
|-------------|----------|
| 4242 4242 4242 4242 | Success |
| 4000 0000 0000 0002 | Declined |
| 4000 0000 0000 3220 | 3D Secure required |

## Best Practices

1. **Idempotency**: Always use idempotency keys for retries
2. **Webhooks**: Verify webhook signatures
3. **Logging**: Log all payment events for audit
4. **Error Handling**: Handle declines gracefully with retry options
