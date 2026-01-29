# Airwallex Integration

Airwallex provides multi-currency payment processing with competitive FX rates and global coverage.

## Features

- **Multi-Currency**: Accept 60+ currencies, settle in your base currency
- **FX Optimization**: Real-time exchange rates with low margins
- **Payment Methods**: Cards, bank transfers, local payment methods
- **Global Coverage**: Strong presence in APAC, expanding globally
- **Payouts**: Send payments to suppliers and partners worldwide

## Configuration

### Environment Variables

```bash
AIRWALLEX_API_KEY=your_api_key
AIRWALLEX_CLIENT_ID=your_client_id
AIRWALLEX_WEBHOOK_SECRET=your_webhook_secret
```

### Config File

```toml
[payment.airwallex]
enabled = true
api_key = "${AIRWALLEX_API_KEY}"
client_id = "${AIRWALLEX_CLIENT_ID}"
webhook_secret = "${AIRWALLEX_WEBHOOK_SECRET}"
sandbox = false
```

## API Usage

### Create Payment Intent

```http
POST /api/v1/payments
Content-Type: application/json
Authorization: Bearer <token>

{
  "gateway": "airwallex",
  "amount": "150.00",
  "currency": "AUD",
  "customer_email": "customer@example.com",
  "payment_method_types": ["card", "alipaycn"]
}
```

**Response:**

```json
{
  "id": "int_...",
  "client_secret": "int_..._secret_...",
  "status": "requires_payment_method",
  "amount": 150.00,
  "currency": "AUD",
  "available_payment_methods": ["card", "alipaycn"]
}
```

### Multi-Currency Example

```http
POST /api/v1/payments
Content-Type: application/json

{
  "gateway": "airwallex",
  "amount": "10000",
  "currency": "JPY",
  "settlement_currency": "USD"
}
```

## Webhook Events

Configure webhook endpoint: `https://api.yoursite.com/webhooks/airwallex`

| Event | Description |
|-------|-------------|
| `payment_intent.created` | Payment intent created |
| `payment_intent.requires_payment_method` | Awaiting payment method |
| `payment_intent.requires_capture` | Authorized, awaiting capture |
| `payment_intent.succeeded` | Payment completed |
| `payment_intent.cancelled` | Payment cancelled |
| `refund.succeeded` | Refund processed |

## FX and Multi-Currency

### Supported Currencies

Major currencies: USD, EUR, GBP, AUD, CAD, JPY, CNY, HKD, SGD, NZD

### FX Rate Locking

Request rate lock for price certainty:

```http
POST /api/v1/payments
{
  "gateway": "airwallex",
  "amount": "1000.00",
  "currency": "EUR",
  "lock_fx_rate": true,
  "fx_rate_valid_until": "2026-01-29T10:00:00Z"
}
```

## Testing

Use Airwallex sandbox environment:

```toml
[payment.airwallex]
sandbox = true
```

Test cards:

| Card Number | Result |
|-------------|--------|
| 4111 1111 1111 1111 | Success |
| 4000 0000 0000 0002 | Declined |

## Best Practices

1. **Currency Display**: Show prices in customer's local currency
2. **FX Transparency**: Display exchange rate and fees upfront
3. **Settlement**: Choose settlement currency based on your costs
4. **Webhook Handling**: Process webhooks idempotently
