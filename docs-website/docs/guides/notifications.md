# Notifications Setup Guide

This guide covers how to configure email and SMS notifications in R Commerce, including setting up templates and testing your notification system.

## Overview

R Commerce provides a comprehensive notification system that supports:

- **Email Notifications**: SMTP, SendGrid, AWS SES, Mailgun
- **SMS Notifications**: Twilio, AWS SNS
- **Webhook Notifications**: HTTP callbacks for custom integrations
- **Template System**: HTML and text templates with variable substitution
- **Multi-language Support**: Localized notifications

## Step 1: Configure Email Provider

### SMTP Configuration

For most use cases, SMTP is the simplest option:

```toml
[notifications.email]
provider = "smtp"
enabled = true
from_address = "noreply@yourstore.com"
from_name = "Your Store"

[notifications.email.smtp]
host = "smtp.gmail.com"
port = 587
username = "your-email@gmail.com"
password = "your-app-password"
encryption = "starttls"  # Options: none, ssl, starttls
```

### SendGrid Configuration

For high-volume sending, SendGrid is recommended:

```toml
[notifications.email]
provider = "sendgrid"
enabled = true
from_address = "noreply@yourstore.com"
from_name = "Your Store"

[notifications.email.sendgrid]
api_key = "SG.your-api-key"
```

### AWS SES Configuration

For AWS users, SES provides cost-effective email delivery:

```toml
[notifications.email]
provider = "ses"
enabled = true
from_address = "noreply@yourstore.com"
from_name = "Your Store"

[notifications.email.ses]
access_key_id = "AKIA..."
secret_access_key = "..."
region = "us-east-1"
```

### Mailgun Configuration

```toml
[notifications.email]
provider = "mailgun"
enabled = true
from_address = "noreply@yourstore.com"
from_name = "Your Store"

[notifications.email.mailgun]
api_key = "key-..."
domain = "mg.yourstore.com"
```

## Step 2: Configure SMS Provider (Optional)

### Twilio Configuration

```toml
[notifications.sms]
provider = "twilio"
enabled = true
from_number = "+1234567890"

[notifications.sms.twilio]
account_sid = "AC..."
auth_token = "..."
```

### AWS SNS Configuration

```toml
[notifications.sms]
provider = "sns"
enabled = true

[notifications.sms.sns]
access_key_id = "AKIA..."
secret_access_key = "..."
region = "us-east-1"
sender_id = "YourStore"
```

## Step 3: Set Up Email Templates

### Template Directory Structure

Create the following directory structure for your templates:

```
templates/
├── emails/
│   ├── order/
│   │   ├── confirmation.html
│   │   ├── confirmation.txt
│   │   ├── shipped.html
│   │   ├── shipped.txt
│   │   ├── delivered.html
│   │   └── delivered.txt
│   ├── customer/
│   │   ├── welcome.html
│   │   ├── welcome.txt
│   │   ├── password_reset.html
│   │   └── password_reset.txt
│   ├── payment/
│   │   ├── receipt.html
│   │   ├── receipt.txt
│   │   ├── failed.html
│   │   └── failed.txt
│   └── subscription/
│       ├── created.html
│       ├── created.txt
│       ├── renewal_reminder.html
│       └── renewal_reminder.txt
└── sms/
    ├── order_shipped.txt
    └── payment_failed.txt
```

### Order Confirmation Email Template

```html
<!-- templates/emails/order/confirmation.html -->
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Order Confirmation - {{order_number}}</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 600px;
            margin: 0 auto;
            padding: 20px;
        }
        .header {
            background: #007bff;
            color: white;
            padding: 20px;
            text-align: center;
        }
        .content {
            background: #f8f9fa;
            padding: 20px;
            margin: 20px 0;
        }
        .order-details {
            background: white;
            padding: 15px;
            margin: 15px 0;
            border: 1px solid #dee2e6;
        }
        .item {
            padding: 10px 0;
            border-bottom: 1px solid #eee;
        }
        .totals {
            text-align: right;
            margin-top: 15px;
        }
        .button {
            display: inline-block;
            background: #007bff;
            color: white;
            padding: 12px 24px;
            text-decoration: none;
            border-radius: 4px;
            margin: 20px 0;
        }
        .footer {
            text-align: center;
            color: #6c757d;
            font-size: 12px;
            margin-top: 30px;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>Thank You for Your Order!</h1>
    </div>
    
    <div class="content">
        <p>Hi {{customer_name}},</p>
        
        <p>We've received your order and are preparing it for shipment.</p>
        
        <div class="order-details">
            <h2>Order #{{order_number}}</h2>
            <p><strong>Order Date:</strong> {{order_date}}<br>
            <strong>Status:</strong> {{order_status}}</p>
            
            <h3>Items Ordered</h3>
            {{#each items}}
            <div class="item">
                <strong>{{name}}</strong><br>
                Quantity: {{quantity}} × {{price}} = {{total}}
            </div>
            {{/each}}
            
            <div class="totals">
                <p>Subtotal: {{subtotal}}<br>
                Shipping: {{shipping_cost}}<br>
                Tax: {{tax}}<br>
                <strong>Total: {{total}}</strong></p>
            </div>
        </div>
        
        <div class="order-details">
            <h3>Shipping Address</h3>
            <p>{{shipping_address.name}}<br>
            {{shipping_address.line1}}<br>
            {{#if shipping_address.line2}}{{shipping_address.line2}}<br>{{/if}}
            {{shipping_address.city}}, {{shipping_address.state}} {{shipping_address.zip}}<br>
            {{shipping_address.country}}</p>
        </div>
        
        <p style="text-align: center;">
            <a href="{{order_tracking_url}}" class="button">Track Your Order</a>
        </p>
        
        <p>We'll send you another email when your order ships.</p>
        
        <p>Questions? Contact us at <a href="mailto:{{support_email}}">{{support_email}}</a></p>
    </div>
    
    <div class="footer">
        <p>{{company_name}}<br>
        {{company_address}}<br>
        <a href="{{unsubscribe_url}}">Unsubscribe</a></p>
    </div>
</body>
</html>
```

### Plain Text Version

```
<!-- templates/emails/order/confirmation.txt -->
Thank You for Your Order!

Hi {{customer_name}},

We've received your order and are preparing it for shipment.

Order #{{order_number}}
Order Date: {{order_date}}
Status: {{order_status}}

Items Ordered:
{{#each items}}
- {{name}}
  Quantity: {{quantity}} × {{price}} = {{total}}
{{/each}}

Subtotal: {{subtotal}}
Shipping: {{shipping_cost}}
Tax: {{tax}}
Total: {{total}}

Shipping Address:
{{shipping_address.name}}
{{shipping_address.line1}}
{{#if shipping_address.line2}}{{shipping_address.line2}}{{/if}}
{{shipping_address.city}}, {{shipping_address.state}} {{shipping_address.zip}}
{{shipping_address.country}}

Track your order: {{order_tracking_url}}

We'll send you another email when your order ships.

Questions? Contact us at {{support_email}}

---
{{company_name}}
{{company_address}}
Unsubscribe: {{unsubscribe_url}}
```

## Step 4: Configure Notification Events

Define which events trigger notifications:

```toml
[notifications.events]

# Order Events
order_created = ["email"]
order_paid = ["email"]
order_shipped = ["email", "sms"]
order_delivered = ["email"]
order_cancelled = ["email"]
order_refunded = ["email"]

# Customer Events
customer_registered = ["email"]
customer_password_reset = ["email"]
customer_welcome = ["email"]

# Payment Events
payment_success = ["email"]
payment_failed = ["email", "sms"]
payment_refunded = ["email"]

# Subscription Events
subscription_created = ["email"]
subscription_renewal_reminder = ["email"]
subscription_payment_failed = ["email", "sms"]
subscription_cancelled = ["email"]

# Shipping Events
shipping_label_created = ["email"]
shipping_exception = ["email"]
```

## Step 5: Test Your Notifications

### Using the CLI

```bash
# Test email configuration
rcommerce notifications test-email \
  --to "test@example.com" \
  --template "order/confirmation"

# Test SMS configuration
rcommerce notifications test-sms \
  --to "+1234567890" \
  --template "order_shipped"

# Send a test order confirmation
rcommerce notifications send-test \
  --event order_created \
  --to "test@example.com"
```

### Test Checklist

Before going live, test these scenarios:

- [ ] Order confirmation email renders correctly
- [ ] Order shipped email includes tracking link
- [ ] Password reset email works end-to-end
- [ ] Welcome email triggers on registration
- [ ] Payment receipt includes correct totals
- [ ] SMS notifications are received (if enabled)
- [ ] Unsubscribe links work
- [ ] Email opens and clicks are tracked (if enabled)

## Step 6: Configure Advanced Options

### Email Tracking

```toml
[notifications.email.tracking]
enabled = true
track_opens = true
track_clicks = true
webhook_url = "https://yourstore.com/webhooks/email-tracking"
```

### Bounce Handling

```toml
[notifications.email.bounce_handling]
enabled = true
soft_bounce_threshold = 3
auto_unsubscribe_hard_bounces = true
notification_email = "admin@yourstore.com"
```

### Rate Limiting

```toml
[notifications.rate_limiting]
enabled = true
max_emails_per_hour = 1000
max_sms_per_hour = 100
burst_allowance = 100
```

### Retry Logic

```toml
[notifications.retry]
enabled = true
max_retries = 3
retry_delays = [60, 300, 900]  # seconds
```

## Step 7: Set Up Webhook Notifications

For custom integrations, configure webhook notifications:

```toml
[notifications.webhooks]
enabled = true

[[notifications.webhooks.endpoints]]
url = "https://your-crm.com/webhooks/rcommerce"
events = ["order.created", "customer.registered"]
secret = "your-webhook-secret"
headers = { "X-Custom-Header" = "value" }

[[notifications.webhooks.endpoints]]
url = "https://your-analytics.com/events"
events = ["*"]  # All events
secret = "another-secret"
```

## Template Variables Reference

### Order Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `{{order_number}}` | Order number | "ORD-2026-001234" |
| `{{order_date}}` | Order creation date | "January 15, 2026" |
| `{{order_status}}` | Current order status | "Processing" |
| `{{subtotal}}` | Order subtotal | "$99.99" |
| `{{shipping_cost}}` | Shipping cost | "$10.00" |
| `{{tax}}` | Tax amount | "$8.50" |
| `{{total}}` | Order total | "$118.49" |
| `{{items}}` | Array of order items | See template |
| `{{shipping_address}}` | Shipping address object | See template |
| `{{billing_address}}` | Billing address object | See template |

### Customer Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `{{customer_name}}` | Customer full name | "John Doe" |
| `{{customer_first_name}}` | Customer first name | "John" |
| `{{customer_email}}` | Customer email | "john@example.com" |
| `{{customer_id}}` | Customer ID | "550e8400..." |

### Store Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `{{company_name}}` | Store name | "Acme Inc" |
| `{{company_address}}` | Store address | "123 Main St..." |
| `{{support_email}}` | Support email | "support@store.com" |
| `{{store_url}}` | Store URL | "https://store.com" |
| `{{logo_url}}` | Logo URL | "https://store.com/logo.png" |

### Link Variables

| Variable | Description |
|----------|-------------|
| `{{order_tracking_url}}` | Order tracking page URL |
| `{{account_url}}` | Customer account page URL |
| `{{password_reset_url}}` | Password reset URL |
| `{{unsubscribe_url}}` | Email unsubscribe URL |

## Best Practices

### 1. Use Both HTML and Text Templates

Always provide both HTML and plain text versions:

- HTML for rich formatting and branding
- Text for accessibility and email clients that don't render HTML

### 2. Keep Templates Simple

- Use inline CSS (many email clients block external stylesheets)
- Test on mobile devices
- Keep email width under 600px
- Use web-safe fonts

### 3. Personalize Content

Use customer data to personalize:

```html
<p>Hi {{customer_first_name}},</p>
<p>Thank you for being a customer since {{customer_since}}!</p>
```

### 4. Include Clear CTAs

Make action buttons prominent:

```html
<a href="{{order_tracking_url}}" 
   style="background: #007bff; color: white; padding: 12px 24px; 
          text-decoration: none; border-radius: 4px; display: inline-block;">
   Track Your Order
</a>
```

### 5. Monitor Deliverability

Track these metrics:

| Metric | Good | Poor |
|--------|------|------|
| Delivery Rate | >95% | <90% |
| Open Rate | >20% | <10% |
| Click Rate | >3% | <1% |
| Bounce Rate | <2% | >5% |
| Spam Complaint Rate | <0.1% | >0.5% |

### 6. Handle Unsubscribes

Always include unsubscribe links and honor them promptly:

```html
<p style="font-size: 12px; color: #6c757d;">
  You're receiving this because you made a purchase at {{company_name}}.
  <a href="{{unsubscribe_url}}">Unsubscribe</a>
</p>
```

## Troubleshooting

### Emails Not Sending

**Checklist:**
- [ ] Email provider credentials are correct
- [ ] `enabled = true` in configuration
- [ ] Template files exist and are readable
- [ ] No syntax errors in templates
- [ ] Rate limits not exceeded

**Debug Commands:**

```bash
# Check email configuration
rcommerce config get notifications.email

# View notification logs
rcommerce logs --category notifications

# Test with verbose output
rcommerce notifications test-email --to "test@example.com" --verbose
```

### Emails Going to Spam

**Solutions:**
1. Set up SPF, DKIM, and DMARC records
2. Use a dedicated sending domain
3. Warm up your IP address gradually
4. Monitor sender reputation
5. Keep complaint rates low

### SMS Not Delivering

**Checklist:**
- [ ] Phone number format is correct (E.164)
- [ ] SMS provider account has balance
- [ ] Message length under carrier limits
- [ ] Not on carrier blacklist

## Next Steps

- [Configure Dunning Emails](../guides/dunning.md)
- [Set Up Shipping Notifications](../guides/shipping.md)
- [API Reference: Webhooks](../api-reference/webhooks.md)
- [Customize Email Templates](../development/custom-templates.md)
