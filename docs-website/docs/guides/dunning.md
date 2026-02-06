# Dunning Setup Guide

This guide covers how to configure and optimize the Dunning System for failed payment recovery in R Commerce.

## What is Dunning?

**Dunning** is the automated process of retrying failed subscription payments and communicating with customers to recover revenue. When a customer's payment fails, the dunning system:

1. **Retries the payment** at strategic intervals
2. **Emails the customer** with helpful reminders
3. **Keeps the subscription active** during the grace period
4. **Cancels only as a last resort** when all recovery attempts fail

### Why Dunning Matters

Failed payments are a major source of revenue loss for subscription businesses:

- **15-20%** of subscription payments fail each month
- **60-80%** of failed payments can be recovered with proper dunning
- **Involuntary churn** (payment failures) often exceeds voluntary cancellations
- Each recovered payment preserves **customer lifetime value**

## How Dunning Works

```
Payment Fails
     │
     ▼
┌─────────────────────────────────────────────────────────────┐
│  Day 0: First Failure                                       │
│  • Subscription status → Past Due                           │
│  • Email sent: "Payment Failed - Please Update"             │
│  • Next retry scheduled: Day +1                             │
└─────────────────────────────────────────────────────────────┘
     │
     ▼
┌─────────────────────────────────────────────────────────────┐
│  Day 1: Retry #1 (Silent)                                   │
│  • Automatic retry with stored payment method               │
│  • If success → Subscription restored to Active             │
│  • If failure → Continue to next retry                      │
└─────────────────────────────────────────────────────────────┘
     │
     ▼
┌─────────────────────────────────────────────────────────────┐
│  Day 4: Retry #2                                            │
│  • Email sent: "Payment Failed Again - Action Required"     │
│  • Next retry scheduled: Day +7                             │
└─────────────────────────────────────────────────────────────┘
     │
     ▼
┌─────────────────────────────────────────────────────────────┐
│  Day 11: Retry #3 (Final)                                   │
│  • Email sent: "Final Notice: Cancellation Pending"         │
│  • This is the last attempt                                 │
└─────────────────────────────────────────────────────────────┘
     │
     ▼
┌─────────────────────────────────────────────────────────────┐
│  Day 11+: Cancellation                                      │
│  • Subscription cancelled                                   │
│  • Email sent: "Subscription Cancelled"                     │
│  • Customer can reactivate anytime                          │
└─────────────────────────────────────────────────────────────┘
```

## Step 1: Configure Dunning Settings

Add dunning configuration to your `config.toml`:

```toml
[dunning]
# Number of retry attempts before cancellation (default: 3)
max_retries = 3

# Days between retries (default: [1, 3, 7])
retry_intervals_days = [1, 3, 7]

# Grace period in days (default: 14)
grace_period_days = 14

# Send email on first failure (default: true)
email_on_first_failure = true

# Send email on final failure (default: true)
email_on_final_failure = true
```

### Configuration Options

| Option | Description | Default | Recommended |
|--------|-------------|---------|-------------|
| `max_retries` | Number of payment retry attempts | 3 | 3-5 |
| `retry_intervals_days` | Days to wait between each retry | [1, 3, 7] | [1, 3, 7] or [3, 7, 14] |
| `grace_period_days` | Days subscription stays active during dunning | 14 | 14-30 |
| `email_on_first_failure` | Send email immediately on first failure | true | true |
| `email_on_final_failure` | Send final warning before cancellation | true | true |

### Retry Schedule Examples

#### Standard Schedule (Default)
```toml
[dunning]
max_retries = 3
retry_intervals_days = [1, 3, 7]
```
- Retry 1: 1 day after initial failure
- Retry 2: 3 days after retry 1 (4 days total)
- Retry 3: 7 days after retry 2 (11 days total)

#### Aggressive Recovery
```toml
[dunning]
max_retries = 5
retry_intervals_days = [1, 2, 3, 5, 7]
grace_period_days = 18
```
Faster retries for businesses with tight margins or high competition.

#### Gentle Recovery
```toml
[dunning]
max_retries = 3
retry_intervals_days = [3, 7, 14]
grace_period_days = 24
```
Slower schedule for premium services or B2B subscriptions.

#### Minimal Recovery
```toml
[dunning]
max_retries = 2
retry_intervals_days = [3, 7]
grace_period_days = 10
```
Fewer attempts for low-value subscriptions or free tiers.

### Late Fees (Optional)

You can optionally charge late fees for overdue payments:

```toml
[dunning]
# Apply late fee after the 2nd retry attempt
late_fee_after_retry = 2

# Late fee amount
late_fee_amount = "5.00"
```

**Important considerations:**
- Late fees may increase customer churn
- Check local regulations before implementing
- Clearly communicate late fee policy in terms of service
- Consider waiving late fees for first-time failures

## Step 2: Set Up Dunning Emails

### Default Email Templates

R Commerce includes professionally written email templates for each dunning stage:

| Stage | Email Type | Purpose |
|-------|------------|---------|
| Day 0 | First Failure | Friendly reminder to update payment method |
| Day 4 | Retry Failure | Urgent notice with attempt count |
| Day 11 | Final Notice | Last warning before cancellation |
| Day 11+ | Cancellation Notice | Professional closure with reactivation link |
| Any | Payment Recovered | Success confirmation |

### Customizing Email Templates

Email templates are stored in the `templates/dunning/` directory:

```
templates/
└── dunning/
    ├── first_failure.html
    ├── first_failure.txt
    ├── retry_failure.html
    ├── retry_failure.txt
    ├── final_notice.html
    ├── final_notice.txt
    ├── cancellation_notice.html
    ├── cancellation_notice.txt
    ├── payment_recovered.html
    └── payment_recovered.txt
```

### Available Template Variables

Use these variables in your custom templates:

| Variable | Description | Example |
|----------|-------------|---------|
| `{{customer_name}}` | Customer's first name | "John" |
| `{{subscription_id}}` | Subscription ID | "550e8400-e29b-41d4-a716-446655440000" |
| `{{product_name}}` | Subscription product name | "Premium Coffee Subscription" |
| `{{amount}}` | Amount due | "$29.99" |
| `{{currency}}` | Currency code | "USD" |
| `{{attempt_number}}` | Current retry attempt | "2" |
| `{{max_attempts}}` | Maximum retry attempts | "3" |
| `{{next_retry_date}}` | Next scheduled retry | "January 15, 2026" |
| `{{grace_period_end}}` | End of grace period | "January 25, 2026" |
| `{{update_payment_url}}` | Payment update link | "https://shop.example.com/payment/update" |
| `{{account_url}}` | Customer account portal | "https://shop.example.com/account" |
| `{{support_url}}` | Support contact page | "https://shop.example.com/support" |
| `{{company_name}}` | Your business name | "Acme Inc" |

### Example Custom Template

```html
<!-- templates/dunning/first_failure.html -->
<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .button { background: #007bff; color: white; padding: 12px 24px; 
                  text-decoration: none; border-radius: 4px; display: inline-block; }
        .alert { background: #fff3cd; border: 1px solid #ffeaa7; 
                 padding: 15px; border-radius: 4px; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Hi {{customer_name}},</h1>
        
        <div class="alert">
            <strong>We couldn't process your payment</strong>
        </div>
        
        <p>Your subscription to <strong>{{product_name}}</strong> is still active, 
        but we need you to update your payment method.</p>
        
        <p><strong>Amount Due:</strong> {{amount}}<br>
        <strong>Next Retry:</strong> {{next_retry_date}}</p>
        
        <p style="text-align: center; margin: 30px 0;">
            <a href="{{update_payment_url}}" class="button">Update Payment Method</a>
        </p>
        
        <p>Common reasons payments fail:</p>
        <ul>
            <li>Expired credit card</li>
            <li>Insufficient funds</li>
            <li>Bank security block</li>
        </ul>
        
        <p>Questions? <a href="{{support_url}}">Contact our support team</a>.</p>
        
        <p>Thanks,<br>{{company_name}}</p>
    </div>
</body>
</html>
```

### Email Best Practices

**Do:**
- ✅ Keep subject lines clear and actionable
- ✅ Include a prominent call-to-action button
- ✅ Explain why the payment failed (when known)
- ✅ Maintain a helpful, non-threatening tone
- ✅ Provide multiple contact options
- ✅ Test emails on mobile devices

**Don't:**
- ❌ Use aggressive or threatening language
- ❌ Hide the payment update link
- ❌ Send too many emails
- ❌ Use all caps or excessive punctuation
- ❌ Include unnecessary technical details

## Step 3: Configure Customer Segments

Different customer segments may need different dunning strategies:

```toml
# High-value customers get extended grace periods
[dunning.segments.premium]
max_retries = 5
retry_intervals_days = [3, 7, 14, 21, 30]
grace_period_days = 45
email_template_prefix = "premium_"

# Standard customers use default
[dunning.segments.standard]
max_retries = 3
retry_intervals_days = [1, 3, 7]
grace_period_days = 14

# Low-value or trial customers
[dunning.segments.basic]
max_retries = 2
retry_intervals_days = [1, 3]
grace_period_days = 7
```

Assign segments to customers via tags or metadata:

```bash
# Set customer segment via CLI
rcommerce customer set-segment <customer-id> --segment premium
```

## Step 4: Set Up the Payment Update Page

Create a customer-facing page for updating payment methods:

### Configuration

```toml
[dunning.payment_update]
page_url = "https://shop.example.com/payment/update"
require_login = false  # Allow token-based access from email
session_duration_minutes = 30
success_redirect = "https://shop.example.com/account"
```

### Page Requirements

The payment update page should:
- Pre-populate customer information
- Support multiple payment methods
- Be mobile-optimized
- Show the amount due and retry schedule
- Confirm successful updates immediately

## Step 5: Monitor and Optimize

### Key Metrics to Track

| Metric | Target | Action if Below Target |
|--------|--------|----------------------|
| Recovery Rate | >65% | Review email templates |
| First-Attempt Recovery | >40% | Optimize retry timing |
| Email Open Rate | >50% | Improve subject lines |
| Click Rate | >15% | Better call-to-action |

### Using the Dashboard

Access the dunning dashboard at `/admin/dunning` to view:

- **Recovery Metrics**: Recovery rate, revenue recovered, average attempts
- **Active Cases**: Current dunning cases with retry status
- **Recovery by Attempt**: Which retry attempts are most successful
- **Trend Analysis**: Recovery rate trends over time

### CLI Commands for Monitoring

```bash
# View dunning metrics
rcommerce dunning metrics --period 30d

# List active dunning cases
rcommerce dunning list --status active

# Get subscription dunning history
rcommerce subscription dunning-status <subscription-id>
```

## Step 6: Manual Intervention

Sometimes manual intervention is needed for high-value customers:

### When to Intervene Manually

- High-value customers (enterprise accounts)
- Long-tenure customers (2+ years)
- Customers who contact support
- Technical failures (not payment issues)

### CLI Commands

```bash
# View subscription dunning status
rcommerce subscription dunning-status <subscription-id>

# Manually trigger a retry
rcommerce subscription retry-payment <subscription-id>

# Extend grace period
rcommerce subscription extend-grace <subscription-id> --days 7 --reason "Customer contacted support"

# Cancel subscription immediately
rcommerce subscription cancel <subscription-id> --reason "payment_failed" --immediate
```

## Best Practices

### 1. Optimize Your Retry Schedule

Test different schedules to find what works for your business:

| Business Type | Recommended Schedule |
|--------------|---------------------|
| SaaS / Software | [1, 3, 7] - Standard |
| Physical Goods | [3, 7, 14] - More time for customers |
| High-Value B2B | [3, 7, 14, 21] - Extended with more attempts |
| Low-Value / Freemium | [1, 3] - Quick recovery or cancel |

### 2. Make Payment Updates Easy

Reduce friction in the payment update process:

```
✅ DO:
• Single-click payment update links in emails
• Pre-populate customer information
• Support multiple payment methods
• Mobile-optimized payment forms
• Save payment methods for future use

❌ DON'T:
• Require login to update payment
• Redirect through multiple pages
• Ask for information you already have
• Use expired or broken links
```

### 3. Proactive Prevention

Reduce failures before they happen:

**Expiration Reminders:**
```toml
[notifications.expiration]
enabled = true
days_before_expiration = 30
template = "card_expiring_soon"
```

**Pre-Dunning Emails:**
```toml
[notifications.pre_dunning]
enabled = true
days_before_billing = 3
template = "upcoming_billing"
```

**Account Updater Services:**
```toml
[payment.account_updater]
enabled = true
providers = ["visa", "mastercard"]
```

### 4. Customer Communication Tone

Progress the tone through dunning stages:

| Stage | Tone | Example Opening |
|-------|------|-----------------|
| First Failure | Friendly, helpful | "Don't worry, this happens..." |
| Retry Failure | Urgent, polite | "We need your attention..." |
| Final Notice | Serious, clear | "Action required to keep your subscription..." |
| Cancellation | Professional, open | "We'd love to have you back..." |

## Troubleshooting

### Common Issues

**Low Recovery Rate**
- Review email subject lines
- Check payment update page usability
- Verify retry schedule timing
- Test different email templates

**High Cancellation Rate**
- Extend grace period
- Add more retry attempts
- Improve email messaging
- Offer payment plan options

**Payment Update Page Not Working**
- Check token validation
- Verify SSL certificate
- Test on mobile devices
- Review error logging

**Emails Not Sending**
- Verify email provider configuration
- Check template syntax
- Review spam folder placement
- Test email deliverability

### Debug Mode

Enable debug logging for dunning:

```toml
[dunning.debug]
log_emails = true
log_retries = true
log_webhooks = true
```

## FAQ

**Q: What is dunning?**

A: Dunning is the automated process of retrying failed subscription payments and communicating with customers to recover revenue. It includes scheduled retries, email notifications, and eventual cancellation if recovery fails.

**Q: How much revenue can dunning recover?**

A: With proper configuration, dunning typically recovers 60-80% of failed subscription payments. Recovery rates vary by industry, customer segment, and dunning strategy.

**Q: Is dunning automatic?**

A: Yes, the entire dunning process is automated once configured. The system handles retries, emails, and cancellations without manual intervention.

**Q: Do customers lose access during dunning?**

A: No, customers retain access during the grace period (default: 14 days). The subscription status changes to "Past Due" but service continues until cancellation.

**Q: Can customers update their payment method during dunning?**

A: Yes, customers can update their payment method at any time during the dunning process. The next scheduled retry will use the new payment method.

**Q: What's a good recovery rate?**

A: Industry benchmarks:
- Excellent: >75%
- Good: 60-75%
- Average: 45-60%
- Needs improvement: <45%

## Next Steps

- [Configure Notifications](../guides/notifications.md)
- [Set Up Payment Gateways](../payment-gateways/index.md)
- [API Reference: Subscriptions](../api-reference/subscriptions.md)
- [Webhooks Reference](../api-reference/webhooks.md)
