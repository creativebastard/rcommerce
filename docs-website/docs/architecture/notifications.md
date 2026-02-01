# Notifications

## Overview

R Commerce provides a flexible notification system for email, SMS, and webhook-based notifications.

## Notification Types

### Email Notifications

Built-in email templates for:
- Order confirmation
- Shipping notification
- Delivery confirmation
- Password reset
- Welcome email
- Abandoned cart recovery

### SMS Notifications

Optional SMS support for:
- Order confirmations
- Shipping updates
- Delivery notifications

### Webhooks

Real-time HTTP callbacks for:
- Order events
- Payment events
- Customer events
- Inventory events

## Email Configuration

```toml
[notifications.email]
enabled = true
provider = "smtp"  # or "sendgrid", "mailgun"
from_address = "store@example.com"
from_name = "My Store"

[notifications.email.smtp]
host = "smtp.example.com"
port = 587
username = "user"
password = "secret"
secure = true
```

## Webhook Configuration

```toml
[[webhooks]]
url = "https://example.com/webhooks/orders"
events = ["order.created", "order.paid", "order.shipped"]
secret = "whsec_xxx"
```

## Template System

Email templates use Handlebars:

```handlebars
<!-- order_confirmation.hbs -->
<h1>Thank you for your order, {{customer.first_name}}!</h1>

<p>Order #{{order.order_number}}</p>

<ul>
{{#each order.items}}
  <li>{{name}} x {{quantity}} - {{unit_price}}</li>
{{/each}}
</ul>

<p>Total: {{order.total}}</p>
```

## Notification Queue

Notifications are queued for reliable delivery:
- Background job processing
- Retry on failure
- Delivery tracking
- Failed notification alerts

## See Also

- [Configuration](../getting-started/configuration.md)
- [Webhooks](../api-reference/webhooks.md)
