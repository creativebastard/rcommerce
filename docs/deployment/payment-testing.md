# Payment Gateway Testing Guide

This guide covers testing the payment gateway integrations for Stripe and Airwallex.

## Overview

R Commerce supports multiple payment gateways:
- **Stripe** - Global card payments and digital wallets
- **Airwallex** - Multi-currency global payments
- **WeChat Pay** - China's leading mobile payment
- **AliPay** - Alibaba's global payment platform
- **Mock Gateway** - For development and testing

## Prerequisites

### Test API Keys

#### Stripe Test Credentials
1. Create a Stripe account at https://stripe.com
2. Go to Developers → API Keys
3. Copy your **Test Secret Key** (starts with `sk_test_`)
4. Set up a **Webhook Endpoint** and copy the signing secret (starts with `whsec_`)

```bash
export STRIPE_TEST_SECRET_KEY="sk_test_..."
export STRIPE_TEST_WEBHOOK_SECRET="whsec_..."
```

#### Airwallex Test Credentials
1. Create an Airwallex account at https://www.airwallex.com
2. Go to Developer → API Keys
3. Create a new API key pair
4. Copy your **Client ID** and **API Key**
5. Set up a **Webhook Endpoint** and copy the webhook secret

```bash
export AIRWALLEX_TEST_CLIENT_ID="your_client_id"
export AIRWALLEX_TEST_API_KEY="your_api_key"
export AIRWALLEX_TEST_WEBHOOK_SECRET="your_webhook_secret"
```

## Running Tests

### Quick Test (Mock Gateway)

```bash
cd /Users/jeremy/Development/gokart
cargo test -p rcommerce-core --lib payment::gateways::tests
```

### Stripe Integration Tests

```bash
export STRIPE_TEST_SECRET_KEY="sk_test_..."
export STRIPE_TEST_WEBHOOK_SECRET="whsec_..."

cargo test -p rcommerce-core --lib payment::gateways::integration_tests::stripe_tests -- --test-threads=1
```

### Airwallex Integration Tests

```bash
export AIRWALLEX_TEST_CLIENT_ID="..."
export AIRWALLEX_TEST_API_KEY="..."
export AIRWALLEX_TEST_WEBHOOK_SECRET="..."

cargo test -p rcommerce-core --lib payment::gateways::integration_tests::airwallex_tests -- --test-threads=1
```

### All Payment Tests

```bash
cargo test -p rcommerce-core --lib payment::gateways -- --test-threads=1
```

## Test Scenarios

### 1. Create Payment Intent

Tests that a payment intent can be created successfully.

**Expected Result:**
- Payment intent ID returned
- Client secret generated
- Correct amount and currency

### 2. Confirm Payment

Tests confirming a payment (requires test payment method).

**Stripe Test Cards:**
| Card Number | Scenario |
|-------------|----------|
| `4242424242424242` | Success |
| `4000000000000002` | Card declined |
| `4000000000009995` | Insufficient funds |
| `4000002500003155` | Requires 3D Secure |

### 3. Capture Payment

Tests capturing an authorized payment.

### 4. Refund Payment

Tests refunding a captured payment.

### 5. Get Payment

Tests retrieving payment details.

### 6. Webhook Handling

Tests webhook signature verification and event parsing.

## Manual Testing with cURL

### Stripe

```bash
# Create payment intent
curl -X POST https://api.stripe.com/v1/payment_intents \
  -u "$STRIPE_TEST_SECRET_KEY:" \
  -d amount=2000 \
  -d currency=usd \
  -d "metadata[order_id]"="test-order-123"
```

### Airwallex

```bash
# Get access token
curl -X POST https://api.airwallex.com/api/v1/authentication/login \
  -H "Content-Type: application/json" \
  -H "x-client-id: $AIRWALLEX_TEST_CLIENT_ID" \
  -H "x-api-key: $AIRWALLEX_TEST_API_KEY"

# Create payment intent
curl -X POST https://api.airwallex.com/api/v1/pa/payment_intents/create \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "request_id": "test-'$(uuidgen)'",
    "amount": 2000,
    "currency": "USD",
    "descriptor": "Test Order"
  }'
```

## Configuration

Add to your `config.toml`:

```toml
[payment]
default_gateway = "stripe"

[payment.stripe]
enabled = true
secret_key = "${STRIPE_SECRET_KEY}"
webhook_secret = "${STRIPE_WEBHOOK_SECRET}"

[payment.airwallex]
enabled = true
client_id = "${AIRWALLEX_CLIENT_ID}"
api_key = "${AIRWALLEX_API_KEY}"
webhook_secret = "${AIRWALLEX_WEBHOOK_SECRET}"
```

## Webhook Testing

### Local Webhook Testing with Stripe CLI

```bash
# Install Stripe CLI
brew install stripe/stripe-cli/stripe

# Login
stripe login

# Forward webhooks to local server
stripe listen --forward-to localhost:8080/webhooks/stripe
```

### Trigger Test Events

```bash
# Trigger payment success
stripe trigger payment_intent.succeeded

# Trigger payment failure
stripe trigger payment_intent.payment_failed
```

## Troubleshooting

### Common Issues

**Authentication Errors:**
- Verify API keys are correct
- Check that you're using test keys (not production)
- Ensure environment variables are set

**Network Errors:**
- Check internet connectivity
- Verify firewall settings
- Some corporate networks block payment APIs

**Webhook Errors:**
- Verify webhook secret is correct
- Check payload is not modified
- Ensure correct signature algorithm

### Debug Logging

Enable debug logging:

```bash
RUST_LOG=debug cargo test -p rcommerce-core --lib payment::gateways
```

## Production Checklist

Before going live:

- [ ] Switch to production API keys
- [ ] Configure production webhook endpoints
- [ ] Enable 3D Secure for card payments
- [ ] Set up webhook signature verification
- [ ] Configure fraud rules
- [ ] Test all payment flows end-to-end
- [ ] Set up monitoring and alerts
- [ ] Configure backup payment gateway

## Security Notes

1. **Never commit API keys** - Use environment variables
2. **Verify webhooks** - Always verify signatures
3. **Use HTTPS** - Never send payment data over HTTP
4. **PCI Compliance** - Don't store raw card data
5. **Rate limiting** - Implement rate limits on payment endpoints

## Additional Resources

- [Stripe Testing Documentation](https://stripe.com/docs/testing)
- [Airwallex API Documentation](https://www.airwallex.com/docs/api)
- [Stripe Test Card Numbers](https://stripe.com/docs/testing#cards)
