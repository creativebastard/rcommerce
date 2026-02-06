# Subscriptions API

The Subscriptions API provides comprehensive recurring billing management, allowing you to create and manage subscription-based products with automated billing cycles, invoicing, and lifecycle management.

## Base URL

```
/api/v1/subscriptions
```

## Authentication

All subscription endpoints require authentication via API key or JWT token.

```http
Authorization: Bearer YOUR_API_KEY
```

## Subscription Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "customer_id": "550e8400-e29b-41d4-a716-446655440001",
  "order_id": "550e8400-e29b-41d4-a716-446655440002",
  "product_id": "550e8400-e29b-41d4-a716-446655440003",
  "variant_id": "550e8400-e29b-41d4-a716-446655440004",
  "status": "active",
  "interval": "monthly",
  "interval_count": 1,
  "currency": "USD",
  "amount": "29.99",
  "setup_fee": "9.99",
  "trial_days": 14,
  "trial_ends_at": "2024-02-15T10:00:00Z",
  "current_cycle": 3,
  "min_cycles": 3,
  "max_cycles": null,
  "starts_at": "2024-01-15T10:00:00Z",
  "next_billing_at": "2024-04-15T10:00:00Z",
  "last_billing_at": "2024-03-15T10:00:00Z",
  "ends_at": null,
  "cancelled_at": null,
  "cancellation_reason": null,
  "payment_method_id": "pm_1234567890",
  "gateway": "stripe",
  "notes": "Premium plan subscription",
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-03-15T10:00:00Z"
}
```

### Subscription Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier |
| `customer_id` | UUID | Customer who owns the subscription |
| `order_id` | UUID | Original order that created the subscription |
| `product_id` | UUID | Subscribed product ID |
| `variant_id` | UUID | Product variant ID (if applicable) |
| `status` | string | `active`, `paused`, `cancelled`, `expired`, `past_due`, `trialing`, `pending` |
| `interval` | string | `daily`, `weekly`, `bi_weekly`, `monthly`, `quarterly`, `bi_annually`, `annually` |
| `interval_count` | integer | Number of intervals between billings (e.g., 3 for every 3 months) |
| `currency` | string | ISO 4217 currency code |
| `amount` | decimal | Amount charged per billing cycle |
| `setup_fee` | decimal | One-time setup fee (optional) |
| `trial_days` | integer | Number of trial days |
| `trial_ends_at` | datetime | When trial period ends |
| `current_cycle` | integer | Current billing cycle number |
| `min_cycles` | integer | Minimum cycles before cancellation allowed |
| `max_cycles` | integer | Maximum cycles (null = unlimited) |
| `starts_at` | datetime | Subscription start date |
| `next_billing_at` | datetime | Next scheduled billing date |
| `last_billing_at` | datetime | Last successful billing date |
| `ends_at` | datetime | When subscription ends (if scheduled) |
| `cancelled_at` | datetime | When subscription was cancelled |
| `cancellation_reason` | string | Reason for cancellation |
| `payment_method_id` | string | Payment gateway method ID |
| `gateway` | string | Payment gateway used (e.g., `stripe`, `airwallex`) |
| `notes` | string | Internal notes |
| `created_at` | datetime | Creation timestamp |
| `updated_at` | datetime | Last modification timestamp |

## Subscription Invoice Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440010",
  "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
  "order_id": "550e8400-e29b-41d4-a716-446655440020",
  "cycle_number": 3,
  "period_start": "2024-03-15T10:00:00Z",
  "period_end": "2024-04-15T10:00:00Z",
  "subtotal": "29.99",
  "tax_total": "2.99",
  "total": "32.98",
  "status": "paid",
  "paid_at": "2024-03-15T10:05:00Z",
  "payment_id": "pi_1234567890",
  "failed_attempts": 0,
  "last_failed_at": null,
  "failure_reason": null,
  "next_retry_at": null,
  "retry_count": 0,
  "created_at": "2024-03-15T10:00:00Z",
  "updated_at": "2024-03-15T10:05:00Z"
}
```

### Invoice Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier |
| `subscription_id` | UUID | Associated subscription ID |
| `order_id` | UUID | Generated order ID (if applicable) |
| `cycle_number` | integer | Billing cycle number |
| `period_start` | datetime | Billing period start |
| `period_end` | datetime | Billing period end |
| `subtotal` | decimal | Amount before tax |
| `tax_total` | decimal | Tax amount |
| `total` | decimal | Total amount due |
| `status` | string | `pending`, `billed`, `paid`, `failed`, `past_due`, `cancelled` |
| `paid_at` | datetime | When payment was received |
| `payment_id` | string | Gateway payment ID |
| `failed_attempts` | integer | Number of failed payment attempts |
| `failure_reason` | string | Reason for last failure |
| `next_retry_at` | datetime | Next scheduled retry date |
| `retry_count` | integer | Number of retry attempts |

## Endpoints

### List Subscriptions

```http
GET /api/v1/subscriptions
```

Retrieve a paginated list of subscriptions for the authenticated customer.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `status` | string | Filter by status: `active`, `paused`, `cancelled`, `expired`, `past_due`, `trialing`, `pending` |
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |

#### Example Request

```http
GET /api/v1/subscriptions?status=active&page=1&per_page=20
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "subscriptions": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "customer_id": "550e8400-e29b-41d4-a716-446655440001",
      "product_id": "550e8400-e29b-41d4-a716-446655440003",
      "status": "active",
      "interval": "monthly",
      "interval_count": 1,
      "currency": "USD",
      "amount": "29.99",
      "next_billing_at": "2024-04-15T10:00:00Z",
      "created_at": "2024-01-15T10:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 15,
    "total_pages": 1
  }
}
```

### Create Subscription

```http
POST /api/v1/subscriptions
```

Create a new subscription for a customer.

#### Request Body

```json
{
  "customer_id": "550e8400-e29b-41d4-a716-446655440001",
  "order_id": "550e8400-e29b-41d4-a716-446655440002",
  "product_id": "550e8400-e29b-41d4-a716-446655440003",
  "variant_id": "550e8400-e29b-41d4-a716-446655440004",
  "interval": "monthly",
  "interval_count": 1,
  "currency": "USD",
  "amount": "29.99",
  "setup_fee": "9.99",
  "trial_days": 14,
  "min_cycles": 3,
  "max_cycles": null,
  "payment_method_id": "pm_1234567890",
  "gateway": "stripe",
  "notes": "Premium plan subscription"
}
```

#### Required Fields

- `customer_id` - Customer UUID
- `order_id` - Original order UUID
- `product_id` - Product UUID
- `interval` - Billing interval
- `interval_count` - Number of intervals (1-12)
- `currency` - ISO 4217 currency code
- `amount` - Subscription amount
- `payment_method_id` - Payment method ID from gateway
- `gateway` - Payment gateway identifier

#### Example Response

```json
{
  "success": true,
  "subscription": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "customer_id": "550e8400-e29b-41d4-a716-446655440001",
    "product_id": "550e8400-e29b-41d4-a716-446655440003",
    "status": "trialing",
    "interval": "monthly",
    "interval_count": 1,
    "currency": "USD",
    "amount": "29.99",
    "trial_days": 14,
    "trial_ends_at": "2024-01-29T10:00:00Z",
    "next_billing_at": "2024-01-29T10:00:00Z",
    "created_at": "2024-01-15T10:00:00Z",
    "updated_at": "2024-01-15T10:00:00Z"
  }
}
```

### Get Subscription

```http
GET /api/v1/subscriptions/{id}
```

Retrieve a single subscription by ID.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | UUID | Subscription ID |

#### Example Request

```http
GET /api/v1/subscriptions/550e8400-e29b-41d4-a716-446655440000
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "subscription": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "customer_id": "550e8400-e29b-41d4-a716-446655440001",
    "order_id": "550e8400-e29b-41d4-a716-446655440002",
    "product_id": "550e8400-e29b-41d4-a716-446655440003",
    "status": "active",
    "interval": "monthly",
    "interval_count": 1,
    "currency": "USD",
    "amount": "29.99",
    "current_cycle": 3,
    "next_billing_at": "2024-04-15T10:00:00Z",
    "created_at": "2024-01-15T10:00:00Z",
    "updated_at": "2024-03-15T10:00:00Z"
  }
}
```

### Update Subscription

```http
PUT /api/v1/subscriptions/{id}
```

Update an existing subscription.

#### Request Body

```json
{
  "interval": "quarterly",
  "interval_count": 1,
  "amount": "79.99",
  "next_billing_at": "2024-04-15T10:00:00Z",
  "max_cycles": 12,
  "payment_method_id": "pm_newpaymentmethod",
  "notes": "Upgraded to quarterly billing"
}
```

#### Example Response

```json
{
  "success": true,
  "subscription": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "interval": "quarterly",
    "interval_count": 1,
    "amount": "79.99",
    "next_billing_at": "2024-04-15T10:00:00Z",
    "max_cycles": 12,
    "updated_at": "2024-03-20T10:00:00Z"
  }
}
```

### Cancel Subscription

```http
POST /api/v1/subscriptions/{id}/cancel
```

Cancel a subscription.

#### Request Body

```json
{
  "reason": "too_expensive",
  "reason_details": "Found a better alternative",
  "cancel_at_end": true
}
```

#### Cancellation Reasons

| Reason | Description |
|--------|-------------|
| `customer_requested` | Customer initiated cancellation |
| `payment_failed` | Repeated payment failures |
| `fraudulent` | Fraudulent activity detected |
| `too_expensive` | Price concern |
| `not_useful` | Product no longer needed |
| `other` | Other reason |

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cancel_at_end` | boolean | If true, cancel at end of current period; if false, cancel immediately |

#### Example Response

```json
{
  "success": true,
  "message": "Subscription cancelled successfully",
  "subscription": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "status": "cancelled",
    "cancelled_at": "2024-03-20T10:00:00Z",
    "cancellation_reason": "too_expensive",
    "updated_at": "2024-03-20T10:00:00Z"
  }
}
```

### Pause Subscription

```http
POST /api/v1/subscriptions/{id}/pause
```

Temporarily pause an active subscription. Billing will be suspended until resumed.

#### Example Request

```http
POST /api/v1/subscriptions/550e8400-e29b-41d4-a716-446655440000/pause
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "success": true,
  "message": "Subscription paused successfully",
  "subscription": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "status": "paused",
    "updated_at": "2024-03-20T10:00:00Z"
  }
}
```

### Resume Subscription

```http
POST /api/v1/subscriptions/{id}/resume
```

Resume a paused subscription.

#### Example Request

```http
POST /api/v1/subscriptions/550e8400-e29b-41d4-a716-446655440000/resume
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "success": true,
  "message": "Subscription resumed successfully",
  "subscription": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "status": "active",
    "next_billing_at": "2024-04-15T10:00:00Z",
    "updated_at": "2024-03-20T10:00:00Z"
  }
}
```

### Get Subscription Invoices

```http
GET /api/v1/subscriptions/{id}/invoices
```

Retrieve all invoices for a subscription.

#### Example Request

```http
GET /api/v1/subscriptions/550e8400-e29b-41d4-a716-446655440000/invoices
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "invoices": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440010",
      "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
      "cycle_number": 1,
      "period_start": "2024-01-29T10:00:00Z",
      "period_end": "2024-02-29T10:00:00Z",
      "subtotal": "29.99",
      "tax_total": "2.99",
      "total": "32.98",
      "status": "paid",
      "paid_at": "2024-01-29T10:05:00Z",
      "created_at": "2024-01-29T10:00:00Z"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440011",
      "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
      "cycle_number": 2,
      "period_start": "2024-02-29T10:00:00Z",
      "period_end": "2024-03-29T10:00:00Z",
      "subtotal": "29.99",
      "tax_total": "2.99",
      "total": "32.98",
      "status": "paid",
      "paid_at": "2024-02-29T10:05:00Z",
      "created_at": "2024-02-29T10:00:00Z"
    }
  ]
}
```

## Dunning Endpoints

### List Failed Payments

```http
GET /api/v1/admin/dunning/failed-payments
```

Retrieve a list of subscriptions with failed payments currently in dunning.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `status` | string | Filter by status: `past_due`, `in_dunning` |
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |

#### Example Response

```json
{
  "data": [
    {
      "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
      "customer": {
        "id": "550e8400-e29b-41d4-a716-446655440001",
        "email": "customer@example.com",
        "name": "John Doe"
      },
      "product_name": "Premium Subscription",
      "amount": "29.99",
      "currency": "USD",
      "failed_attempts": 2,
      "max_attempts": 3,
      "next_retry_at": "2026-01-15T10:00:00Z",
      "status": "past_due",
      "first_failed_at": "2026-01-10T08:30:00Z"
    }
  ],
  "meta": {
    "total": 12,
    "page": 1,
    "per_page": 20
  }
}
```

### Get Dunning Metrics

```http
GET /api/v1/admin/dunning/metrics?period=30d
```

Retrieve dunning performance metrics.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `period` | string | Time period: `7d`, `30d`, `90d`, `1y` (default: 30d) |

#### Example Response

```json
{
  "period": "30d",
  "total_failures": 156,
  "total_recoveries": 113,
  "recovery_rate": 72.44,
  "recovered_revenue": "24500.00",
  "lost_revenue": "3200.00",
  "recovery_by_attempt": [
    { "attempt": 1, "recoveries": 70, "rate": 44.87 },
    { "attempt": 2, "recoveries": 28, "rate": 17.95 },
    { "attempt": 3, "recoveries": 15, "rate": 9.62 }
  ],
  "average_recovery_time_hours": 72.5
}
```

### Get Subscription Dunning History

```http
GET /api/v1/subscriptions/{id}/dunning-history
```

Retrieve the complete dunning history for a specific subscription.

#### Example Response

```json
{
  "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "active",
  "retry_attempts": [
    {
      "attempt_number": 1,
      "attempted_at": "2026-01-10T08:30:00Z",
      "succeeded": false,
      "error_message": "Card declined",
      "error_code": "insufficient_funds"
    },
    {
      "attempt_number": 2,
      "attempted_at": "2026-01-13T08:30:00Z",
      "succeeded": true,
      "payment_id": "pi_1234567890"
    }
  ],
  "emails_sent": [
    {
      "type": "first_failure",
      "sent_at": "2026-01-10T08:30:00Z",
      "opened_at": "2026-01-10T09:15:00Z",
      "clicked_at": "2026-01-10T09:16:00Z"
    }
  ]
}
```

### Retry Payment Manually

```http
POST /api/v1/subscriptions/{id}/retry-payment
```

Manually trigger a payment retry for a subscription in dunning.

#### Example Response

```json
{
  "success": true,
  "message": "Payment retry initiated",
  "payment_id": "pi_1234567890",
  "status": "processing"
}
```

### Extend Grace Period

```http
POST /api/v1/subscriptions/{id}/extend-grace
```

Extend the grace period for a subscription in dunning.

#### Request Body

```json
{
  "days": 7,
  "reason": "Customer contacted support"
}
```

#### Example Response

```json
{
  "success": true,
  "message": "Grace period extended by 7 days",
  "new_grace_period_end": "2026-01-25T10:00:00Z"
}
```

## Dunning Webhook Events

Subscribe to these webhook events for dunning notifications:

| Event | Description |
|-------|-------------|
| `dunning.payment_failed` | A payment failed and entered dunning |
| `dunning.payment_recovered` | A failed payment was successfully recovered |
| `dunning.subscription_cancelled` | Subscription cancelled after failed dunning |
| `dunning.retry_attempted` | A retry attempt was made |
| `dunning.email_sent` | A dunning email was sent |
| `dunning.grace_period_extended` | Grace period was manually extended |

### Webhook Payload Examples

**dunning.payment_failed:**
```json
{
  "event": "dunning.payment_failed",
  "data": {
    "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
    "customer_id": "550e8400-e29b-41d4-a716-446655440001",
    "invoice_id": "550e8400-e29b-41d4-a716-446655440002",
    "attempt_number": 1,
    "max_attempts": 3,
    "next_retry_at": "2026-01-15T10:00:00Z",
    "error_message": "Card declined",
    "amount": "29.99",
    "currency": "USD"
  }
}
```

**dunning.payment_recovered:**
```json
{
  "event": "dunning.payment_recovered",
  "data": {
    "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
    "customer_id": "550e8400-e29b-41d4-a716-446655440001",
    "invoice_id": "550e8400-e29b-41d4-a716-446655440002",
    "attempt_number": 2,
    "payment_id": "pi_1234567890",
    "amount": "29.99",
    "currency": "USD"
  }
}
```

**dunning.subscription_cancelled:**
```json
{
  "event": "dunning.subscription_cancelled",
  "data": {
    "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
    "customer_id": "550e8400-e29b-41d4-a716-446655440001",
    "reason": "payment_failed",
    "total_attempts": 3,
    "cancelled_at": "2026-01-20T10:00:00Z"
  }
}
```

## Admin Endpoints

### List All Subscriptions (Admin)

```http
GET /api/v1/admin/subscriptions
```

Retrieve all subscriptions across all customers (admin only).

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `status` | string | Filter by status |
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |

### Get Subscription Summary (Admin)

```http
GET /api/v1/admin/subscriptions/summary
```

Get aggregated subscription statistics.

#### Example Response

```json
{
  "summary": {
    "total_active": 1250,
    "total_cancelled": 320,
    "total_expired": 45,
    "total_past_due": 12,
    "monthly_recurring_revenue": "45250.00",
    "annual_recurring_revenue": "543000.00"
  }
}
```

### Process Billing (Admin)

```http
POST /api/v1/admin/subscriptions/process-billing
```

Trigger billing run for all due subscriptions.

#### Example Response

```json
{
  "success": true,
  "message": "Processed 45 subscriptions",
  "invoices_created": 45
}
```

## Code Examples

### cURL

```bash
# List subscriptions
curl -X GET "https://api.rcommerce.app/api/v1/subscriptions?status=active" \
  -H "Authorization: Bearer sk_live_xxx"

# Create subscription
curl -X POST "https://api.rcommerce.app/api/v1/subscriptions" \
  -H "Authorization: Bearer sk_live_xxx" \
  -H "Content-Type: application/json" \
  -d '{
    "customer_id": "550e8400-e29b-41d4-a716-446655440001",
    "order_id": "550e8400-e29b-41d4-a716-446655440002",
    "product_id": "550e8400-e29b-41d4-a716-446655440003",
    "interval": "monthly",
    "interval_count": 1,
    "currency": "USD",
    "amount": "29.99",
    "payment_method_id": "pm_1234567890",
    "gateway": "stripe"
  }'

# Cancel subscription
curl -X POST "https://api.rcommerce.app/api/v1/subscriptions/550e8400-e29b-41d4-a716-446655440000/cancel" \
  -H "Authorization: Bearer sk_live_xxx" \
  -H "Content-Type: application/json" \
  -d '{
    "reason": "too_expensive",
    "cancel_at_end": true
  }'
```

### JavaScript

```javascript
// List subscriptions
const response = await fetch('https://api.rcommerce.app/api/v1/subscriptions?status=active', {
  headers: {
    'Authorization': 'Bearer sk_live_xxx'
  }
});
const data = await response.json();
console.log(data.subscriptions);

// Create subscription
const createResponse = await fetch('https://api.rcommerce.app/api/v1/subscriptions', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer sk_live_xxx',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    customer_id: '550e8400-e29b-41d4-a716-446655440001',
    order_id: '550e8400-e29b-41d4-a716-446655440002',
    product_id: '550e8400-e29b-41d4-a716-446655440003',
    interval: 'monthly',
    interval_count: 1,
    currency: 'USD',
    amount: '29.99',
    payment_method_id: 'pm_1234567890',
    gateway: 'stripe'
  })
});
const newSubscription = await createResponse.json();

// Pause subscription
await fetch('https://api.rcommerce.app/api/v1/subscriptions/550e8400-e29b-41d4-a716-446655440000/pause', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer sk_live_xxx'
  }
});
```

### Python

```python
import requests

headers = {
    'Authorization': 'Bearer sk_live_xxx',
    'Content-Type': 'application/json'
}

# List subscriptions
response = requests.get(
    'https://api.rcommerce.app/api/v1/subscriptions',
    headers=headers,
    params={'status': 'active'}
)
subscriptions = response.json()['subscriptions']

# Create subscription
subscription_data = {
    'customer_id': '550e8400-e29b-41d4-a716-446655440001',
    'order_id': '550e8400-e29b-41d4-a716-446655440002',
    'product_id': '550e8400-e29b-41d4-a716-446655440003',
    'interval': 'monthly',
    'interval_count': 1,
    'currency': 'USD',
    'amount': '29.99',
    'payment_method_id': 'pm_1234567890',
    'gateway': 'stripe'
}
response = requests.post(
    'https://api.rcommerce.app/api/v1/subscriptions',
    headers=headers,
    json=subscription_data
)
new_subscription = response.json()['subscription']

# Get invoices
response = requests.get(
    f'https://api.rcommerce.app/api/v1/subscriptions/{new_subscription["id"]}/invoices',
    headers=headers
)
invoices = response.json()['invoices']
```

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `SUBSCRIPTION_NOT_FOUND` | 404 | Subscription does not exist |
| `INVALID_INTERVAL` | 400 | Invalid billing interval |
| `INVALID_CURRENCY` | 400 | Invalid currency code |
| `INVALID_AMOUNT` | 400 | Invalid subscription amount |
| `PAYMENT_METHOD_REQUIRED` | 400 | Payment method ID is required |
| `SUBSCRIPTION_ALREADY_CANCELLED` | 409 | Subscription is already cancelled |
| `SUBSCRIPTION_NOT_ACTIVE` | 409 | Subscription is not in active status |
| `MIN_CYCLES_NOT_MET` | 409 | Minimum billing cycles not yet reached |
| `CUSTOMER_NOT_FOUND` | 404 | Customer does not exist |
| `PRODUCT_NOT_FOUND` | 404 | Product does not exist |
| `INVALID_TRIAL_DAYS` | 400 | Invalid trial period specified |

## Webhooks

The Subscriptions API emits the following webhook events:

| Event | Description |
|-------|-------------|
| `subscription.created` | New subscription created |
| `subscription.updated` | Subscription details changed |
| `subscription.cancelled` | Subscription cancelled |
| `subscription.paused` | Subscription paused |
| `subscription.resumed` | Subscription resumed |
| `subscription.trial_ended` | Trial period ended |
| `subscription.payment_succeeded` | Recurring payment successful |
| `subscription.payment_failed` | Recurring payment failed |
| `subscription.past_due` | Subscription entered past due status |
| `subscription.expired` | Subscription reached max cycles |
| `invoice.created` | New invoice generated |
| `invoice.paid` | Invoice paid successfully |
| `invoice.payment_failed` | Invoice payment failed |
| `invoice.past_due` | Invoice became past due |
