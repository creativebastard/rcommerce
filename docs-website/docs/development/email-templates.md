# Email Templates

R Commerce uses customizable email templates for all customer and admin notifications. This guide covers available templates, variables, customization, and testing.

## Overview

Email templates are:

- **HTML and text** - Multi-part emails for all clients
- **Templating engine** - Handlebars syntax for dynamic content
- **Multi-language** - Support for localization
- **Customizable** - Override default templates

## Available Email Templates

### Customer Notifications

| Template | Description | Trigger |
|----------|-------------|---------|
| `order_confirmation` | Order placed confirmation | Order created |
| `order_shipped` | Shipping notification | Fulfillment created |
| `order_delivered` | Delivery confirmation | Order delivered |
| `order_cancelled` | Order cancellation | Order cancelled |
| `order_refund` | Refund notification | Refund processed |
| `payment_receipt` | Payment confirmation | Payment captured |
| `payment_failed` | Failed payment alert | Payment failed |
| `account_created` | Welcome email | Customer registered |
| `password_reset` | Password reset link | Reset requested |
| `abandoned_cart` | Cart recovery | Cart abandoned (24h) |

### Admin Notifications

| Template | Description | Trigger |
|----------|-------------|---------|
| `admin_order_notification` | New order alert | Order created |
| `admin_low_stock` | Low inventory alert | Stock below threshold |
| `admin_payment_dispute` | Chargeback/dispute alert | Dispute created |
| `admin_daily_summary` | Daily sales report | Daily cron |

### Subscription Notifications

| Template | Description | Trigger |
|----------|-------------|---------|
| `subscription_created` | Subscription confirmation | Subscription started |
| `subscription_renewal` | Upcoming renewal notice | 3 days before renewal |
| `subscription_payment_failed` | Failed renewal payment | Renewal payment failed |
| `subscription_cancelled` | Cancellation confirmation | Subscription cancelled |
| `subscription_expired` | Subscription ended | Subscription expired |

### Dunning Management

| Template | Description | Trigger |
|----------|-------------|---------|
| `dunning_retry_1` | First payment retry notice | First retry attempt |
| `dunning_retry_2` | Second payment retry notice | Second retry attempt |
| `dunning_final` | Final notice before cancellation | Final retry attempt |

## Template Variables

### Common Variables

All templates have access to:

```handlebars
{{!-- Store information --}}
{{store.name}}
{{store.url}}
{{store.logo_url}}
{{store.support_email}}
{{store.support_phone}}

{{!-- Current date/time --}}
{{now}}
{{formatDate now "YYYY-MM-DD"}}

{{!-- Helper functions --}}
{{formatCurrency amount currency}}
{{formatDate date format}}
{{uppercase string}}
{{lowercase string}}
```

### Order Templates

```handlebars
{{!-- Order details --}}
{{order.order_number}}
{{order.status}}
{{order.created_at}}
{{order.total}}
{{order.subtotal}}
{{order.tax_total}}
{{order.shipping_total}}
{{order.discount_total}}
{{order.currency}}

{{!-- Customer details --}}
{{order.customer.email}}
{{order.customer.first_name}}
{{order.customer.last_name}}
{{order.customer.full_name}}

{{!-- Shipping address --}}
{{order.shipping_address.name}}
{{order.shipping_address.address1}}
{{order.shipping_address.city}}
{{order.shipping_address.country}}

{{!-- Line items --}}
{{#each order.line_items}}
  {{this.name}}
  {{this.sku}}
  {{this.quantity}}
  {{this.price}}
  {{this.total}}
{{/each}}

{{!-- Fulfillment details (shipping template) --}}
{{fulfillment.tracking_number}}
{{fulfillment.tracking_url}}
{{fulfillment.carrier}}
{{fulfillment.shipped_at}}
```

### Customer Templates

```handlebars
{{!-- Customer details --}}
{{customer.email}}
{{customer.first_name}}
{{customer.last_name}}
{{customer.full_name}}
{{customer.phone}}

{{!-- Account details --}}
{{customer.created_at}}
{{customer.orders_count}}
{{customer.total_spent}}

{{!-- Password reset --}}
{{reset_url}}
{{reset_token}}
{{reset_expires_at}}

{{!-- Account creation --}}
{{login_url}}
```

### Payment Templates

```handlebars
{{!-- Payment details --}}
{{payment.id}}
{{payment.amount}}
{{payment.currency}}
{{payment.status}}
{{payment.gateway}}
{{payment.created_at}}

{{!-- Card details (if applicable) --}}
{{payment.card_brand}}
{{payment.card_last_four}}

{{!-- Receipt URL --}}
{{payment.receipt_url}}
```

### Subscription Templates

```handlebars
{{!-- Subscription details --}}
{{subscription.id}}
{{subscription.status}}
{{subscription.plan_name}}
{{subscription.plan_amount}}
{{subscription.plan_interval}}
{{subscription.current_period_start}}
{{subscription.current_period_end}}

{{!-- Renewal details --}}
{{subscription.renewal_date}}
{{subscription.renewal_amount}}

{{!-- Cancellation --}}
{{subscription.cancelled_at}}
{{subscription.cancellation_reason}}
```

## Customizing Templates

### Template Storage

Templates are stored in:

```
/etc/rcommerce/templates/          # System templates
/opt/rcommerce/templates/          # Custom templates (FreeBSD)
/var/lib/rcommerce/templates/      # Custom templates (Linux)
./templates/                       # Development
```

### Template Structure

Each template has three files:

```
templates/
├── order_confirmation/
│   ├── subject.hbs       # Email subject
│   ├── html.hbs          # HTML body
│   └── text.hbs          # Plain text body
```

### Creating Custom Templates

1. **Create template directory:**
   ```bash
   mkdir -p /var/lib/rcommerce/templates/order_confirmation
   ```

2. **Create subject template:**
   ```handlebars
   {{!-- templates/order_confirmation/subject.hbs --}}
   Order {{order.order_number}} Confirmed - {{store.name}}
   ```

3. **Create HTML template:**
   ```handlebars
   {{!-- templates/order_confirmation/html.hbs --}}
   <!DOCTYPE html>
   <html>
   <head>
     <style>
       body { font-family: Arial, sans-serif; }
       .header { background: #f5f5f5; padding: 20px; }
       .content { padding: 20px; }
     </style>
   </head>
   <body>
     <div class="header">
       <h1>Thank you for your order!</h1>
     </div>
     <div class="content">
       <p>Hi {{order.customer.first_name}},</p>
       <p>Your order #{{order.order_number}} has been confirmed.</p>
       
       <h2>Order Summary</h2>
       <table>
         {{#each order.line_items}}
         <tr>
           <td>{{this.name}} x {{this.quantity}}</td>
           <td>{{formatCurrency this.total ../order.currency}}</td>
         </tr>
         {{/each}}
       </table>
       
       <p><strong>Total: {{formatCurrency order.total order.currency}}</strong></p>
       
       <p><a href="{{store.url}}/orders/{{order.order_number}}">View Order</a></p>
     </div>
   </body>
   </html>
   ```

4. **Create text template:**
   ```handlebars
   {{!-- templates/order_confirmation/text.hbs --}}
   Thank you for your order!
   
   Hi {{order.customer.first_name}},
   
   Your order #{{order.order_number}} has been confirmed.
   
   Order Summary:
   {{#each order.line_items}}
   - {{this.name}} x {{this.quantity}}: {{formatCurrency this.total ../order.currency}}
   {{/each}}
   
   Total: {{formatCurrency order.total order.currency}}
   
   View Order: {{store.url}}/orders/{{order.order_number}}
   ```

### Template Configuration

Configure template location in `config.toml`:

```toml
[notifications.email]
template_dir = "/var/lib/rcommerce/templates"
default_from = "noreply@yourstore.com"
reply_to = "support@yourstore.com"

# Enable template auto-reload (development)
auto_reload = false

# Template cache TTL
cache_ttl = 3600
```

### Template Inheritance

Create base templates for common elements:

```handlebars
{{!-- templates/base/html.hbs --}}
<!DOCTYPE html>
<html>
<head>
  <style>
    {{> styles}}
  </style>
</head>
<body>
  <div class="email-wrapper">
    {{> header}}
    
    <div class="content">
      {{{body}}}
    </div>
    
    {{> footer}}
  </div>
</body>
</html>
```

Use partials:

```handlebars
{{!-- templates/order_confirmation/html.hbs --}}
{{#> base}}
  {{#*inline "body"}}
    <h1>Thank you for your order!</h1>
    <p>Your order #{{order.order_number}} has been confirmed.</p>
  {{/inline}}
{{/base}}
```

## Testing Templates

### CLI Testing

Test templates with sample data:

```bash
# Test with default sample data
rcommerce template test order_confirmation

# Test with custom data file
rcommerce template test order_confirmation --data test_order.json

# Output to file
rcommerce template test order_confirmation --output test_email.html

# Send test email
rcommerce template test order_confirmation --send-to admin@example.com
```

### Sample Data File

```json
{
  "order": {
    "order_number": "1001",
    "status": "confirmed",
    "total": "99.99",
    "currency": "USD",
    "customer": {
      "first_name": "John",
      "last_name": "Doe",
      "email": "john@example.com"
    },
    "line_items": [
      {
        "name": "Premium T-Shirt",
        "quantity": 2,
        "price": "49.99",
        "total": "99.98"
      }
    ]
  },
  "store": {
    "name": "My Store",
    "url": "https://mystore.com"
  }
}
```

### Preview Server

Run a local preview server:

```bash
# Start preview server
rcommerce template preview

# Access at http://localhost:3001/preview/order_confirmation
```

### Email Testing Tools

Test email rendering:

```bash
# Send to Litmus/Email on Acid
rcommerce template test order_confirmation --litmus

# Check spam score
rcommerce template test order_confirmation --spam-check

# Validate HTML
rcommerce template test order_confirmation --validate
```

## Template Helpers

### Built-in Helpers

```handlebars
{{!-- Format currency --}}
{{formatCurrency 99.99 "USD"}}  {{!-- $99.99 --}}

{{!-- Format date --}}
{{formatDate order.created_at "MMM DD, YYYY"}}  {{!-- Jan 15, 2024 --}}

{{!-- Conditional --}}
{{#if order.discount_total}}
  Discount: {{formatCurrency order.discount_total order.currency}}
{{/if}}

{{!-- Each with index --}}
{{#each order.line_items}}
  {{@index}}. {{this.name}}
{{/each}}

{{!-- Comparison --}}
{{#eq order.status "completed"}}
  Your order is complete!
{{/eq}}

{{!-- Unless (inverse if) --}}
{{#unless order.paid}}
  Payment pending
{{/unless}}
```

### Custom Helpers

Register custom helpers in configuration:

```toml
[notifications.email.helpers]
# Define custom helpers
loyalty_tier = "{{#if (gte customer.total_spent 1000)}}Gold{{else}}Silver{{/if}}"
```

## Localization

### Multi-language Templates

Create language-specific templates:

```
templates/
├── order_confirmation/
│   ├── en/
│   │   ├── subject.hbs
│   │   ├── html.hbs
│   │   └── text.hbs
│   ├── de/
│   │   ├── subject.hbs
│   │   ├── html.hbs
│   │   └── text.hbs
│   └── zh/
│       ├── subject.hbs
│       ├── html.hbs
│       └── text.hbs
```

### Language Selection

Language is determined by:

1. Customer's preferred language
2. Order's locale
3. Store default language

```toml
[notifications.email]
default_language = "en"
supported_languages = ["en", "de", "zh", "fr", "es"]
```

## Best Practices

### Email Design

1. **Use inline styles** - Many clients block `<style>` tags
2. **Table-based layouts** - Better email client support
3. **600px max width** - Standard email width
4. **Alt text for images** - Accessibility and image blocking
5. **Plain text version** - Always include text template

### Template Maintenance

1. **Version control** - Track template changes in Git
2. **Test before deploy** - Always test template changes
3. **Monitor deliverability** - Track bounce/spam rates
4. **A/B testing** - Test different template versions

### Security

1. **Escape variables** - Prevent XSS in emails
2. **Validate URLs** - Ensure all links are valid
3. **No sensitive data** - Don't include passwords or tokens

## Troubleshooting

### Template Not Found

```
ERROR: Template 'order_confirmation' not found
```

**Solutions:**

1. Check template directory path
2. Verify template files exist
3. Check file permissions

### Variable Not Rendering

```
{{order.unknown_field}} renders as empty
```

**Solutions:**

1. Check available variables in documentation
2. Use `{{log order}}` to debug
3. Verify data is passed to template

### Email Rendering Issues

**Solutions:**

1. Test with multiple email clients
2. Use email testing service (Litmus, Email on Acid)
3. Check inline CSS
4. Validate HTML

## Related Documentation

- [Notifications Guide](../guides/notifications.md)
- [Dunning Management](../guides/dunning.md)
- [Configuration](../getting-started/configuration.md)
