# Stripe Integration

Stripe provides global payment processing with support for cards, wallets, and subscriptions through R Commerce's server-side processing API.

## Features

- **Cards**: Visa, Mastercard, Amex, Discover, JCB
- **Wallets**: Apple Pay, Google Pay, Link
- **Bank Transfers**: ACH, SEPA, BACS
- **Buy Now Pay Later**: Klarna, Afterpay, Affirm
- **Subscriptions**: Recurring billing with Stripe Billing
- **3D Secure**: Automatic handling of Strong Customer Authentication (SCA)

## Configuration

### Environment Variables

```bash
# Required - Server-side only
STRIPE_API_KEY=sk_live_...
STRIPE_WEBHOOK_SECRET=whsec_...

# Optional - Only if using Stripe.js (legacy)
STRIPE_PUBLISHABLE_KEY=pk_live_...
```

**Note:** With R Commerce's server-side processing, you only need the **Secret Key**. The Publishable Key is not required for the new v2 API.

### Config File

```toml
[payment.stripe]
enabled = true
api_key = "${STRIPE_API_KEY}"
webhook_secret = "${STRIPE_WEBHOOK_SECRET}"
capture_method = "automatic"  # or "manual"
statement_descriptor = "RCOMMERCE"
```

## API Usage (v2 - Server-Side)

### Get Available Payment Methods

```http
POST /api/v2/payments/methods
Content-Type: application/json
Authorization: Bearer <token>

{
  "currency": "USD",
  "amount": "99.99"
}
```

**Response:**

```json
{
  "gateway_id": "stripe",
  "gateway_name": "Stripe",
  "payment_methods": [
    {
      "method_type": "card",
      "display_name": "Credit/Debit Card",
      "requires_redirect": false,
      "supports_3ds": true,
      "required_fields": [
        {
          "name": "number",
          "label": "Card Number",
          "field_type": "card_number",
          "required": true,
          "pattern": "^[\\d\\s]{13,19}$"
        },
        {
          "name": "exp_month",
          "label": "Expiry Month",
          "field_type": "expiry_date",
          "required": true
        },
        {
          "name": "exp_year",
          "label": "Expiry Year",
          "field_type": "expiry_date",
          "required": true
        },
        {
          "name": "cvc",
          "label": "CVC",
          "field_type": "cvc",
          "required": true
        }
      ]
    }
  ]
}
```

### Initiate Payment

Send card data directly to R Commerce API (server-side processing):

```http
POST /api/v2/payments
Content-Type: application/json
Authorization: Bearer <token>

{
  "gateway_id": "stripe",
  "amount": "99.99",
  "currency": "USD",
  "payment_method": {
    "type": "card",
    "card": {
      "number": "4242424242424242",
      "exp_month": 12,
      "exp_year": 2025,
      "cvc": "123",
      "name": "John Doe"
    }
  },
  "order_id": "order_123",
  "customer_email": "customer@example.com",
  "return_url": "https://yoursite.com/checkout/complete"
}
```

**Success Response:**

```json
{
  "type": "success",
  "payment_id": "pay_550e8400-e29b-41d4-a716-446655440000",
  "transaction_id": "pi_3O...",
  "payment_status": "succeeded",
  "payment_method": {
    "method_type": "card",
    "last_four": "4242",
    "card_brand": "visa",
    "exp_month": "12",
    "exp_year": "2025"
  },
  "receipt_url": "https://pay.stripe.com/receipts/..."
}
```

**3D Secure Required Response:**

```json
{
  "type": "requires_action",
  "payment_id": "pay_550e8400-e29b-41d4-a716-446655440000",
  "action_type": "three_d_secure",
  "action_data": {
    "redirect_url": "https://hooks.stripe.com/3d_secure/...",
    "type": "use_stripe_sdk"
  },
  "expires_at": "2026-01-28T11:00:00Z"
}
```

### Complete 3D Secure

After the customer completes 3D Secure authentication:

```http
POST /api/v2/payments/pay_xxx/complete
Content-Type: application/json
Authorization: Bearer <token>

{
  "action_type": "three_d_secure",
  "action_data": {
    "payment_intent": "pi_3O..."
  }
}
```

### Refund Payment

```http
POST /api/v2/payments/pay_xxx/refund
Content-Type: application/json
Authorization: Bearer <token>

{
  "amount": "99.99",
  "reason": "requested_by_customer"
}
```

## Frontend Integration (v2)

### JavaScript Example

```javascript
// checkout.js - No Stripe.js required!

async function processPayment() {
  // 1. Collect card data from form
  const cardData = {
    number: document.getElementById('cardNumber').value,
    exp_month: parseInt(document.getElementById('expMonth').value),
    exp_year: parseInt(document.getElementById('expYear').value),
    cvc: document.getElementById('cvc').value,
    name: document.getElementById('cardName').value
  };
  
  // 2. Send to R Commerce API
  const response = await fetch('/api/v2/payments', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${API_KEY}`
    },
    body: JSON.stringify({
      gateway_id: 'stripe',
      amount: '99.99',
      currency: 'USD',
      payment_method: {
        type: 'card',
        card: cardData
      },
      order_id: orderId,
      customer_email: customerEmail,
      return_url: window.location.origin + '/checkout/complete'
    })
  });
  
  const result = await response.json();
  
  // 3. Handle response
  switch (result.type) {
    case 'success':
      // Payment complete
      window.location.href = '/checkout/success';
      break;
      
    case 'requires_action':
      // Handle 3D Secure
      if (result.action_type === 'three_d_secure') {
        // Option 1: Redirect to Stripe's 3DS page
        window.location.href = result.action_data.redirect_url;
        
        // Option 2: Use Stripe.js for embedded 3DS (optional)
        // const stripe = await loadStripe('pk_...');
        // await stripe.handleCardAction(result.action_data.client_secret);
      }
      break;
      
    case 'failed':
      showError(result.error_message);
      break;
  }
}

// Called when customer returns from 3DS redirect
async function handle3DReturn() {
  const urlParams = new URLSearchParams(window.location.search);
  const paymentIntent = urlParams.get('payment_intent');
  const paymentId = sessionStorage.getItem('pending_payment_id');
  
  const response = await fetch(`/api/v2/payments/${paymentId}/complete`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${API_KEY}`
    },
    body: JSON.stringify({
      action_type: 'three_d_secure',
      action_data: { payment_intent: paymentIntent }
    })
  });
  
  const result = await response.json();
  
  if (result.type === 'success') {
    window.location.href = '/checkout/success';
  } else {
    showError(result.error_message);
  }
}
```

## Webhook Events

Configure webhook endpoint: `https://api.yoursite.com/api/v2/webhooks/stripe`

### Required Events

| Event | Description |
|-------|-------------|
| `payment_intent.succeeded` | Payment completed successfully |
| `payment_intent.payment_failed` | Payment failed |
| `payment_intent.canceled` | Payment canceled |
| `charge.refunded` | Refund processed |
| `invoice.paid` | Subscription payment received |
| `invoice.payment_failed` | Subscription payment failed |

## Testing

### Test Cards

| Card Number | Scenario |
|-------------|----------|
| `4242424242424242` | Success |
| `4000000000000002` | Card declined |
| `4000000000009995` | Insufficient funds |
| `4000002500003155` | Requires 3D Secure |
| `4000000000003220` | 3D Secure 2 frictionless |
| `4000008400001629` | 3D Secure 2 challenge |
| `4000000000000127` | Incorrect CVC |
| `4000000000000069` | Expired card |

### Test with cURL

```bash
# Initiate payment
curl -X POST http://localhost:8080/api/v2/payments \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "gateway_id": "stripe",
    "amount": "99.99",
    "currency": "USD",
    "payment_method": {
      "type": "card",
      "card": {
        "number": "4242424242424242",
        "exp_month": 12,
        "exp_year": 2025,
        "cvc": "123",
        "name": "Test User"
      }
    },
    "order_id": "test_order_123"
  }'
```

### Local Webhook Testing

```bash
# Install Stripe CLI
brew install stripe/stripe-cli/stripe

# Login
stripe login

# Forward webhooks to local server
stripe listen --forward-to localhost:8080/api/v2/webhooks/stripe

# Trigger test events
stripe trigger payment_intent.succeeded
stripe trigger payment_intent.payment_failed
```

## Legacy Integration (v1)

If you're using the legacy v1 API with Stripe.js:

```javascript
import { loadStripe } from '@stripe/stripe-js';

const stripe = await loadStripe('pk_live_...');
const { client_secret } = await fetch('/api/v1/payments').then(r => r.json());
const result = await stripe.confirmCardPayment(client_secret, {
  payment_method: { card: cardElement }
});
```

**We recommend migrating to v2** for better security and simpler integration.

## Best Practices

1. **Idempotency**: Always use idempotency keys for retries
   ```json
   {
     "idempotency_key": "unique-key-per-attempt"
   }
   ```

2. **Error Handling**: Handle all response types (success, requires_action, failed)

3. **3D Secure**: Always provide `return_url` for cards that may require 3DS

4. **Webhooks**: Verify webhook signatures in production

5. **Logging**: Log all payment events for audit

6. **Testing**: Use test cards before going live

## Troubleshooting

### Common Issues

**"Gateway not found" error:**
- Check that Stripe is enabled in config
- Verify `api_key` is set correctly

**"Card declined" errors:**
- Check that you're using test keys in development
- Verify card number is correct

**3D Secure not working:**
- Ensure `return_url` is provided
- Check that the URL is publicly accessible

**Webhook errors:**
- Verify webhook secret is correct
- Check that endpoint URL is correct (`/api/v2/webhooks/stripe`)

## Additional Resources

- [Stripe Testing Documentation](https://stripe.com/docs/testing)
- [Stripe Test Card Numbers](https://stripe.com/docs/testing#cards)
- [3D Secure Guide](https://stripe.com/docs/payments/3d-secure)
- [Stripe API Reference](https://stripe.com/docs/api)
