# Notifications API

The Notifications API manages email, SMS, push, and webhook notifications for your store. It provides endpoints for sending notifications, managing templates, and tracking delivery status.

## Base URL

```
/api/v1/notifications
```

## Authentication

Notification endpoints require authentication. Admin endpoints require secret key.

```http
Authorization: Bearer YOUR_API_KEY
```

## Notification Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440500",
  "channel": "email",
  "recipient": "customer@example.com",
  "subject": "Order Confirmed: ORD-12345",
  "body": "Your order has been confirmed...",
  "html_body": "<html>...</html>",
  "priority": "high",
  "status": "delivered",
  "attempt_count": 1,
  "max_attempts": 3,
  "error_message": null,
  "metadata": {
    "order_id": "550e8400-e29b-41d4-a716-446655440100",
    "type": "order_confirmation"
  },
  "scheduled_at": null,
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:01Z"
}
```

### Notification Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier |
| `channel` | string | `email`, `sms`, `push`, `webhook`, `in_app` |
| `recipient` | string | Recipient address (email, phone, or URL) |
| `subject` | string | Notification subject line |
| `body` | string | Plain text body content |
| `html_body` | string | HTML formatted content (optional) |
| `priority` | string | `low`, `normal`, `high`, `urgent` |
| `status` | string | `pending`, `sent`, `delivered`, `failed`, `bounced` |
| `attempt_count` | integer | Number of delivery attempts made |
| `max_attempts` | integer | Maximum retry attempts |
| `error_message` | string | Error description if failed |
| `metadata` | object | Additional context data |
| `scheduled_at` | datetime | Scheduled send time (optional) |
| `created_at` | datetime | Creation timestamp |
| `updated_at` | datetime | Last update timestamp |

## Endpoints

### List Notifications

```http
GET /api/v1/notifications
```

Retrieve a paginated list of notifications.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |
| `channel` | string | Filter by channel: `email`, `sms`, `push`, `webhook` |
| `status` | string | Filter by status: `pending`, `sent`, `delivered`, `failed` |
| `priority` | string | Filter by priority: `low`, `normal`, `high`, `urgent` |
| `recipient` | string | Filter by recipient address |
| `created_after` | datetime | Created after date |
| `created_before` | datetime | Created before date |
| `sort` | string | `created_at`, `updated_at`, `priority` |
| `order` | string | `asc` or `desc` (default: desc) |

#### Example Request

```http
GET /api/v1/notifications?channel=email&status=failed&sort=created_at&order=desc
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440500",
      "channel": "email",
      "recipient": "customer@example.com",
      "subject": "Order Confirmed: ORD-12345",
      "priority": "high",
      "status": "failed",
      "attempt_count": 3,
      "error_message": "Connection timeout",
      "created_at": "2024-01-15T10:00:00Z",
      "updated_at": "2024-01-15T10:05:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 45,
      "per_page": 20,
      "current_page": 1,
      "total_pages": 3,
      "has_next": true,
      "has_prev": false
    }
  }
}
```

### Send Notification

```http
POST /api/v1/notifications
```

Send a new notification to a recipient.

#### Request Body

```json
{
  "channel": "email",
  "recipient": "customer@example.com",
  "subject": "Welcome to Our Store",
  "body": "Thank you for joining us!",
  "html_body": "<h1>Welcome!</h1><p>Thank you for joining us!</p>",
  "priority": "normal",
  "template_id": "welcome_html",
  "template_variables": {
    "customer_name": "John Doe",
    "company_name": "R Commerce"
  },
  "metadata": {
    "campaign_id": "welcome_series_1"
  },
  "scheduled_at": null
}
```

#### Request Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `channel` | string | Yes | Notification channel |
| `recipient` | string | Yes | Recipient address |
| `subject` | string | Conditional | Subject line (required for email) |
| `body` | string | Conditional | Plain text body (required if no template) |
| `html_body` | string | No | HTML content |
| `priority` | string | No | Priority level (default: `normal`) |
| `template_id` | string | No | Template to use |
| `template_variables` | object | No | Variables for template substitution |
| `metadata` | object | No | Additional context data |
| `scheduled_at` | datetime | No | Schedule for later delivery |

#### Example Response

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440500",
  "channel": "email",
  "recipient": "customer@example.com",
  "subject": "Welcome to Our Store",
  "status": "pending",
  "priority": "normal",
  "attempt_count": 0,
  "max_attempts": 3,
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:00Z"
}
```

### Get Notification

```http
GET /api/v1/notifications/{id}
```

Retrieve a single notification by ID.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | UUID | Notification ID |

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `include` | string | Related data: `attempts`, `template` |

#### Example Response

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440500",
  "channel": "email",
  "recipient": "customer@example.com",
  "subject": "Order Confirmed: ORD-12345",
  "body": "Your order has been confirmed...",
  "html_body": "<html>...</html>",
  "priority": "high",
  "status": "delivered",
  "attempt_count": 1,
  "max_attempts": 3,
  "error_message": null,
  "metadata": {
    "order_id": "550e8400-e29b-41d4-a716-446655440100",
    "type": "order_confirmation"
  },
  "scheduled_at": null,
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:01Z",
  "attempts": [
    {
      "channel": "email",
      "attempted_at": "2024-01-15T10:00:01Z",
      "status": "delivered",
      "error": null
    }
  ]
}
```

### Retry Failed Notification

```http
POST /api/v1/notifications/{id}/retry
```

Retry a failed notification delivery.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | UUID | Notification ID |

#### Example Request

```http
POST /api/v1/notifications/550e8400-e29b-41d4-a716-446655440500/retry
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440500",
  "status": "pending",
  "attempt_count": 3,
  "message": "Notification queued for retry"
}
```

### Cancel Scheduled Notification

```http
POST /api/v1/notifications/{id}/cancel
```

Cancel a notification that has been scheduled but not yet sent.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | UUID | Notification ID |

#### Example Response

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440500",
  "status": "cancelled",
  "message": "Scheduled notification cancelled"
}
```

## Templates

### List Templates

```http
GET /api/v1/notifications/templates
```

Retrieve all available notification templates.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `channel` | string | Filter by channel |
| `category` | string | Filter by category |

#### Example Response

```json
{
  "data": [
    {
      "id": "order_confirmation_html",
      "name": "Order Confirmation HTML",
      "channel": "email",
      "subject": "Order Confirmed: {{ order_number }}",
      "variables": [
        "order_number",
        "order_date",
        "customer_name",
        "order_total"
      ],
      "has_html": true,
      "created_at": "2024-01-01T00:00:00Z"
    },
    {
      "id": "welcome_html",
      "name": "Welcome HTML",
      "channel": "email",
      "subject": "Welcome to {{ company_name }}!",
      "variables": [
        "customer_name",
        "company_name",
        "login_url"
      ],
      "has_html": true,
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

### Get Template

```http
GET /api/v1/notifications/templates/{id}
```

Retrieve a specific template.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | string | Template ID |

#### Example Response

```json
{
  "id": "order_confirmation_html",
  "name": "Order Confirmation HTML",
  "channel": "email",
  "subject": "Order Confirmed: {{ order_number }}",
  "body": "Hello {{ customer_name }}, thank you for your order...",
  "html_body": "<!DOCTYPE html>...</html>",
  "variables": [
    "order_number",
    "order_date",
    "order_total",
    "customer_name",
    "customer_email",
    "shipping_address",
    "billing_address",
    "subtotal",
    "shipping_cost",
    "tax",
    "items",
    "company_name",
    "support_email"
  ],
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

### Create Template

```http
POST /api/v1/notifications/templates
```

Create a new notification template (admin only).

#### Request Body

```json
{
  "id": "custom_promotion",
  "name": "Custom Promotion",
  "channel": "email",
  "subject": "Special Offer: {{ discount_percent }}% Off!",
  "body": "Hi {{ customer_name }}, get {{ discount_percent }}% off your next order!",
  "html_body": "<h1>Special Offer!</h1><p>Hi {{ customer_name }}...</p>",
  "variables": [
    "customer_name",
    "discount_percent",
    "promo_code",
    "expiry_date"
  ]
}
```

#### Request Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | Yes | Unique template identifier |
| `name` | string | Yes | Human-readable name |
| `channel` | string | Yes | Target channel |
| `subject` | string | Conditional | Subject template (email only) |
| `body` | string | Yes | Body template with placeholders |
| `html_body` | string | No | HTML template |
| `variables` | array | Yes | List of required variable names |

#### Example Response

```json
{
  "id": "custom_promotion",
  "name": "Custom Promotion",
  "channel": "email",
  "subject": "Special Offer: {{ discount_percent }}% Off!",
  "variables": [
    "customer_name",
    "discount_percent",
    "promo_code",
    "expiry_date"
  ],
  "created_at": "2024-01-15T10:00:00Z"
}
```

### Update Template

```http
PUT /api/v1/notifications/templates/{id}
```

Update an existing template.

#### Request Body

```json
{
  "name": "Updated Promotion Name",
  "subject": "Updated Subject: {{ discount_percent }}% Off!",
  "body": "Updated body content...",
  "html_body": "<h1>Updated HTML</h1>...",
  "variables": [
    "customer_name",
    "discount_percent",
    "promo_code"
  ]
}
```

### Delete Template

```http
DELETE /api/v1/notifications/templates/{id}
```

Delete a custom template. System templates cannot be deleted.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | string | Template ID |

#### Example Response

```json
{
  "deleted": true,
  "id": "custom_promotion"
}
```

### Preview Template

```http
POST /api/v1/notifications/templates/{id}/preview
```

Preview a template with sample variables.

#### Request Body

```json
{
  "variables": {
    "customer_name": "John Doe",
    "order_number": "ORD-12345",
    "order_total": "99.99"
  }
}
```

#### Example Response

```json
{
  "subject": "Order Confirmed: ORD-12345",
  "body": "Hello John Doe, thank you for your order...",
  "html_body": "<!DOCTYPE html>...",
  "rendered_at": "2024-01-15T10:00:00Z"
}
```

## Built-in Templates

### Order Templates

| Template ID | Channel | Description |
|-------------|---------|-------------|
| `order_confirmation` | email | Plain text order confirmation |
| `order_confirmation_html` | email | HTML order confirmation with invoice |
| `order_shipped` | email | Plain text shipping notification |
| `order_shipped_html` | email | HTML shipping notification |
| `order_cancelled_html` | email | Order cancellation notice |

### Payment Templates

| Template ID | Channel | Description |
|-------------|---------|-------------|
| `payment_successful_html` | email | Payment confirmation |
| `payment_failed_html` | email | Payment failure notice |
| `refund_processed_html` | email | Refund confirmation |

### Subscription Templates

| Template ID | Channel | Description |
|-------------|---------|-------------|
| `subscription_created_html` | email | New subscription welcome |
| `subscription_renewal_html` | email | Renewal confirmation |
| `subscription_cancelled_html` | email | Cancellation notice |

### Dunning Templates

| Template ID | Channel | Description |
|-------------|---------|-------------|
| `dunning_first_html` | email | First payment retry notice |
| `dunning_retry_html` | email | Subsequent retry notice |
| `dunning_final_html` | email | Final payment notice |

### Customer Templates

| Template ID | Channel | Description |
|-------------|---------|-------------|
| `welcome_html` | email | New customer welcome |
| `password_reset_html` | email | Password reset instructions |
| `abandoned_cart_html` | email | Cart recovery email |

### Inventory Templates

| Template ID | Channel | Description |
|-------------|---------|-------------|
| `low_stock_alert` | email | Low stock warning |

## Bulk Operations

### Send Bulk Notifications

```http
POST /api/v1/notifications/bulk
```

Send notifications to multiple recipients.

#### Request Body

```json
{
  "template_id": "promotion_announcement",
  "channel": "email",
  "recipients": [
    {
      "email": "customer1@example.com",
      "variables": {
        "customer_name": "John Doe",
        "discount_code": "SAVE20"
      }
    },
    {
      "email": "customer2@example.com",
      "variables": {
        "customer_name": "Jane Smith",
        "discount_code": "SAVE20"
      }
    }
  ],
  "priority": "normal",
  "metadata": {
    "campaign_id": "summer_sale_2024"
  }
}
```

#### Example Response

```json
{
  "batch_id": "batch_550e8400e29b41d4a716446655440600",
  "total": 2,
  "queued": 2,
  "failed": 0,
  "notifications": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440501",
      "recipient": "customer1@example.com",
      "status": "pending"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440502",
      "recipient": "customer2@example.com",
      "status": "pending"
    }
  ]
}
```

## Statistics

### Get Delivery Statistics

```http
GET /api/v1/notifications/stats
```

Retrieve notification delivery statistics.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `channel` | string | Filter by channel |
| `since` | datetime | Statistics since date |

#### Example Response

```json
{
  "period": {
    "start": "2024-01-01T00:00:00Z",
    "end": "2024-01-15T23:59:59Z"
  },
  "overall": {
    "sent": 10000,
    "delivered": 9500,
    "failed": 450,
    "bounced": 50,
    "delivery_rate": 0.95,
    "failure_rate": 0.045
  },
  "by_channel": {
    "email": {
      "sent": 8000,
      "delivered": 7600,
      "failed": 360,
      "bounced": 40
    },
    "sms": {
      "sent": 1500,
      "delivered": 1425,
      "failed": 75,
      "bounced": 0
    },
    "push": {
      "sent": 500,
      "delivered": 475,
      "failed": 15,
      "bounced": 10
    }
  }
}
```

## Webhook Endpoints

### Incoming Webhooks

Receive notification status updates via webhooks.

#### Delivery Status Webhook

```http
POST /webhooks/notifications/delivery
```

R Commerce sends webhook events when notification status changes.

#### Payload Format

```json
{
  "id": "evt_550e8400e29b41d4a716446655440700",
  "topic": "notification.delivered",
  "created_at": "2024-01-15T10:00:01Z",
  "data": {
    "notification_id": "550e8400-e29b-41d4-a716-446655440500",
    "channel": "email",
    "recipient": "customer@example.com",
    "status": "delivered",
    "delivered_at": "2024-01-15T10:00:01Z",
    "message_id": "msg_abc123"
  }
}
```

### Webhook Events

| Event | Description |
|-------|-------------|
| `notification.created` | Notification queued for sending |
| `notification.sent` | Notification sent to provider |
| `notification.delivered` | Notification successfully delivered |
| `notification.failed` | Delivery failed |
| `notification.bounced` | Notification bounced |
| `notification.opened` | Email opened (email only) |
| `notification.clicked` | Link clicked (email only) |

## Channels

### Email

Send email notifications using SMTP or email service providers.

**Recipient format:** Valid email address

```json
{
  "channel": "email",
  "recipient": "customer@example.com",
  "subject": "Order Confirmation",
  "body": "...",
  "html_body": "..."
}
```

### SMS

Send SMS notifications via Twilio or other providers.

**Recipient format:** E.164 phone number (+1234567890)

```json
{
  "channel": "sms",
  "recipient": "+1234567890",
  "body": "Your order ORD-12345 has shipped!"
}
```

### Push

Send push notifications to mobile devices.

**Recipient format:** Device token

```json
{
  "channel": "push",
  "recipient": "device_token_abc123",
  "subject": "Order Shipped",
  "body": "Your order has shipped!"
}
```

### Webhook

Send HTTP webhooks to external endpoints.

**Recipient format:** HTTPS URL

```json
{
  "channel": "webhook",
  "recipient": "https://your-app.com/webhooks/notifications",
  "body": "{\"event\": \"order_created\", \"data\": {...}}"
}
```

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `NOTIFICATION_NOT_FOUND` | 404 | Notification does not exist |
| `TEMPLATE_NOT_FOUND` | 404 | Template does not exist |
| `INVALID_CHANNEL` | 400 | Unsupported notification channel |
| `INVALID_RECIPIENT` | 400 | Invalid recipient format for channel |
| `MISSING_SUBJECT` | 400 | Subject required for email channel |
| `MISSING_TEMPLATE_VARIABLES` | 400 | Required template variables not provided |
| `TEMPLATE_RENDER_ERROR` | 400 | Failed to render template |
| `NOTIFICATION_ALREADY_SENT` | 409 | Cannot retry non-failed notification |
| `NOTIFICATION_CANCELLED` | 409 | Notification was cancelled |
| `CANNOT_CANCEL_SENT` | 400 | Cannot cancel already sent notification |
| `BULK_LIMIT_EXCEEDED` | 400 | Too many recipients in bulk request |
| `RATE_LIMIT_EXCEEDED` | 429 | Too many notification requests |
| `CHANNEL_DISABLED` | 400 | Notification channel is disabled |
| `PROVIDER_ERROR` | 502 | External provider error |

## Rate Limits

| Endpoint | Limit |
|----------|-------|
| `POST /notifications` | 100/minute |
| `POST /notifications/bulk` | 10/minute, max 1000 recipients |
| `GET /notifications` | 1000/minute |
| Other endpoints | 100/minute |

## Best Practices

### Template Variables

Use consistent variable naming:

```json
{
  "customer_name": "John Doe",
  "order_number": "ORD-12345",
  "order_total": "99.99",
  "company_name": "R Commerce"
}
```

### Retry Strategy

Failed notifications are automatically retried with exponential backoff:

| Attempt | Delay |
|---------|-------|
| 1 | Immediate |
| 2 | 5 seconds |
| 3 | 25 seconds |

### Scheduling Notifications

Schedule notifications for optimal delivery times:

```json
{
  "channel": "email",
  "recipient": "customer@example.com",
  "template_id": "promotion",
  "scheduled_at": "2024-01-20T09:00:00Z"
}
```

### Testing

Use the preview endpoint to test templates before sending:

```http
POST /api/v1/notifications/templates/welcome_html/preview
{
  "variables": {
    "customer_name": "Test User",
    "company_name": "Test Company"
  }
}
```
