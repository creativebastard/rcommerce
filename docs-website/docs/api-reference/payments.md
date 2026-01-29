# Payments API

The Payments API handles payment processing, transactions, refunds, and payment method management.

## Base URL

```
/api/v1/payments
```

## Authentication

Payment endpoints require secret API key for processing. Read-only access available with restricted keys.

```http
Authorization: Bearer YOUR_SECRET_KEY
```

## Payment Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440200",
  "order_id": "550e8400-e29b-41d4-a716-446655440100",
  "amount": "59.49",
  "currency": "USD",
  "status": "succeeded",
  "gateway": "stripe",
  "gateway_payment_id": "pi_3O...",
  "payment_method": {
    "type": "card",
    "card": {
      "brand": "visa",
      "last4": "4242",
      "exp_month": 12,
      "exp_year": 2025,
      "fingerprint": "fp_..."
    }
  },
  "description": "Order #1001",
  "receipt_email": "customer@example.com",
  "receipt_url": "https://pay.stripe.com/receipts/...",
  "captured": true,
  "capture_method": "automatic",
  "confirmation_method": "automatic",
  "customer_id": "550e8400-e29b-41d4-a716-446655440001",
  "refunded_amount": "0.00",
  "refunds": [],
  "dispute": null,
  "metadata": {
    "order_number": "1001",
    "customer_name": "John Doe"
  },
  "created_at": "2024-01-15T10:01:00Z",
  "updated_at": "2024-01-15T10:01:30Z",
  "captured_at": "2024-01-15T10:01:30Z"
}
```

### Payment Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier |
| `order_id` | UUID | Associated order |
| `amount` | decimal | Payment amount |
| `currency` | string | ISO 4217 currency code |
| `status` | string | `pending`, `processing`, `succeeded`, `failed`, `canceled`, `refunded` |
| `gateway` | string | Payment gateway used |
| `gateway_payment_id` | string | Gateway's transaction ID |
| `payment_method` | object | Payment method details |
| `description` | string | Payment description |
| `receipt_email` | string | Email for receipt |
| `receipt_url` | string | URL to view receipt |
| `captured` | boolean | Funds captured |
| `capture_method` | string | `automatic` or `manual` |
| `customer_id` | UUID | Saved customer (if applicable) |
| `refunded_amount` | decimal | Total amount refunded |
| `refunds` | array | List of refunds |
| `dispute` | object | Dispute information |
| `metadata` | object | Custom key-value data |
| `created_at` | datetime | Creation timestamp |
| `updated_at` | datetime | Last modification |
| `captured_at` | datetime | Capture timestamp |

## Endpoints

### List Payments

```http
GET /api/v1/payments
```

Retrieve a paginated list of payments.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |
| `order_id` | UUID | Filter by order |
| `customer_id` | UUID | Filter by customer |
| `gateway` | string | Filter by payment gateway |
| `status` | string | Filter by status |
| `min_amount` | decimal | Minimum amount |
| `max_amount` | decimal | Maximum amount |
| `created_after` | datetime | Created after date |
| `created_before` | datetime | Created before date |
| `sort` | string | `created_at`, `amount` |
| `order` | string | `asc` or `desc` |

### Get Payment

```http
GET /api/v1/payments/{id}
```

Retrieve a single payment by ID.

### Create Payment

```http
POST /api/v1/payments
```

Create a new payment for an order.

#### Request Body

```json
{
  "order_id": "550e8400-e29b-41d4-a716-446655440100",
  "amount": "59.49",
  "currency": "USD",
  "gateway": "stripe",
  "payment_method": {
    "type": "card",
    "card": {
      "number": "4242424242424242",
      "exp_month": 12,
      "exp_year": 2025,
      "cvc": "123"
    }
  },
  "capture_method": "automatic",
  "receipt_email": "customer@example.com",
  "description": "Order #1001",
  "metadata": {
    "order_number": "1001"
  }
}
```

### Capture Payment

```http
POST /api/v1/payments/{id}/capture
```

Capture an authorized (uncaptured) payment.

#### Request Body

```json
{
  "amount": "59.49"
}
```

### Cancel Payment

```http
POST /api/v1/payments/{id}/cancel
```

Cancel an uncaptured payment.

## Refunds

### Create Refund

```http
POST /api/v1/payments/{id}/refunds
```

Refund a captured payment.

#### Request Body

```json
{
  "amount": "59.49",
  "reason": "requested_by_customer",
  "metadata": {
    "note": "Customer unhappy with product"
  }
}
```

#### Refund Reasons

- `duplicate` - Duplicate charge
- `fraudulent` - Fraudulent transaction
- `requested_by_customer` - Customer request

### Get Refund

```http
GET /api/v1/payments/{payment_id}/refunds/{refund_id}
```

### List Refunds

```http
GET /api/v1/payments/{payment_id}/refunds
```

## Payment Methods

### List Customer Payment Methods

```http
GET /api/v1/customers/{customer_id}/payment_methods
```

### Create Payment Method

```http
POST /api/v1/customers/{customer_id}/payment_methods
```

#### Request Body

```json
{
  "type": "card",
  "card": {
    "number": "4242424242424242",
    "exp_month": 12,
    "exp_year": 2025,
    "cvc": "123"
  },
  "set_as_default": true
}
```

### Delete Payment Method

```http
DELETE /api/v1/customers/{customer_id}/payment_methods/{payment_method_id}
```

### Set Default Payment Method

```http
POST /api/v1/customers/{customer_id}/payment_methods/{payment_method_id}/default
```

## Payment Intents

Payment intents are used for complex payment flows with 3D Secure.

### Create Payment Intent

```http
POST /api/v1/payment_intents
```

#### Request Body

```json
{
  "amount": "59.49",
  "currency": "USD",
  "customer_id": "550e8400-e29b-41d4-a716-446655440001",
  "payment_method": "pm_...",
  "confirmation_method": "manual",
  "capture_method": "automatic",
  "setup_future_usage": "off_session",
  "metadata": {
    "order_id": "550e8400-e29b-41d4-a716-446655440100"
  }
}
```

### Confirm Payment Intent

```http
POST /api/v1/payment_intents/{id}/confirm
```

### Capture Payment Intent

```http
POST /api/v1/payment_intents/{id}/capture
```

### Cancel Payment Intent

```http
POST /api/v1/payment_intents/{id}/cancel
```

## Disputes

### List Disputes

```http
GET /api/v1/disputes
```

### Get Dispute

```http
GET /api/v1/disputes/{id}
```

### Submit Evidence

```http
POST /api/v1/disputes/{id}/evidence
```

#### Request Body

```json
{
  "product_description": "Premium cotton t-shirt",
  "customer_email": "customer@example.com",
  "shipping_date": "2024-01-16",
  "shipping_carrier": "UPS",
  "shipping_tracking_number": "1Z999...",
  "access_activity_log": "Customer accessed digital download 3 times",
  "uncategorized_text": "Additional context...",
  "uncategorized_file": "file_..."
}
```

## Payouts

### List Payouts

```http
GET /api/v1/payouts
```

Retrieve payouts to your bank account.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `status` | string | `pending`, `in_transit`, `paid`, `failed`, `canceled` |
| `date_after` | date | Payouts after date (YYYY-MM-DD) |
| `date_before` | date | Payouts before date |

### Get Payout

```http
GET /api/v1/payouts/{id}
```

## Balance

### Get Balance

```http
GET /api/v1/balance
```

Retrieve current account balance.

#### Example Response

```json
{
  "available": [
    {
      "currency": "USD",
      "amount": "12500.00"
    }
  ],
  "pending": [
    {
      "currency": "USD",
      "amount": "3500.00"
    }
  ],
  "instant_available": [
    {
      "currency": "USD",
      "amount": "5000.00"
    }
  ]
}
```

### Get Balance Transactions

```http
GET /api/v1/balance/transactions
```

Retrieve detailed balance transaction history.

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `PAYMENT_NOT_FOUND` | 404 | Payment does not exist |
| `INVALID_AMOUNT` | 400 | Invalid payment amount |
| `INVALID_CURRENCY` | 400 | Unsupported currency |
| `INVALID_PAYMENT_METHOD` | 400 | Invalid card or bank details |
| `CARD_DECLINED` | 402 | Card was declined |
| `INSUFFICIENT_FUNDS` | 402 | Card has insufficient funds |
| `EXPIRED_CARD` | 402 | Card has expired |
| `INCORRECT_CVC` | 402 | CVC check failed |
| `PROCESSING_ERROR` | 402 | Gateway processing error |
| `ALREADY_CAPTURED` | 409 | Payment already captured |
| `ALREADY_REFUNDED` | 409 | Payment already fully refunded |
| `REFUND_AMOUNT_INVALID` | 400 | Refund exceeds payment amount |
| `DISPUTE_NOT_FOUND` | 404 | Dispute does not exist |

## Webhooks

| Event | Description |
|-------|-------------|
| `payment.created` | New payment initiated |
| `payment.succeeded` | Payment completed successfully |
| `payment.failed` | Payment failed |
| `payment.captured` | Authorized payment captured |
| `payment.canceled` | Payment canceled |
| `refund.created` | Refund initiated |
| `refund.succeeded` | Refund completed |
| `refund.failed` | Refund failed |
| `dispute.created` | Dispute/chargeback opened |
| `dispute.updated` | Dispute status changed |
| `dispute.closed` | Dispute resolved |
| `payout.created` | Payout initiated |
| `payout.paid` | Payout deposited |
| `payout.failed` | Payout failed |
