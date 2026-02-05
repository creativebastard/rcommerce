# Dunning Guide

This guide covers everything you need to know about configuring and using the Dunning System for failed payment recovery in R Commerce.

## Introduction to Dunning

**Dunning** is the automated process of retrying failed subscription payments and communicating with customers to recover revenue. When a customer's payment fails, the dunning system takes over to:

1. **Retry the payment** at strategic intervals
2. **Email the customer** with helpful reminders
3. **Keep the subscription active** during the grace period
4. **Cancel only as a last resort** when all recovery attempts fail

### Why Dunning Matters

Failed payments are a major source of revenue loss for subscription businesses:

- **15-20%** of subscription payments fail each month
- **60-80%** of failed payments can be recovered with proper dunning
- **Involuntary churn** (payment failures) often exceeds voluntary cancellations
- Each recovered payment preserves **customer lifetime value**

### How R Commerce Dunning Works

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

## Configuring Dunning Settings

### Basic Configuration

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

### Configuration Options Explained

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

## Setting Up Dunning Emails

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

## Monitoring Failed Payments

### Dashboard Overview

Access the dunning dashboard at `/admin/dunning`:

```
┌─────────────────────────────────────────────────────────────────────┐
│ DUNNING DASHBOARD                                                   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  RECOVERY METRICS (Last 30 Days)                                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │   Recovery   │  │   Revenue    │  │    Avg       │              │
│  │    Rate      │  │  Recovered   │  │  Attempts    │              │
│  │    72.5%     │  │  $24,500     │  │    1.8       │              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
│                                                                     │
│  ACTIVE DUNNING CASES (12)                                         │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ Customer        │ Product              │ Retry │ Next Retry  │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │ john@email.com  │ Premium Plan         │ 1/3   │ Today       │  │
│  │ jane@email.com  │ Basic Subscription   │ 2/3   │ Tomorrow    │  │
│  │ bob@email.com   │ Enterprise Plan      │ 3/3   │ In 2 days   │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                     │
│  RECOVERY BY ATTEMPT                                               │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ Attempt 1: ████████████████████ 45%                         │  │
│  │ Attempt 2: ██████████████ 28%                               │  │
│  │ Attempt 3: ████████ 15%                                     │  │
│  │ Cancelled: ██████ 12%                                       │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### API Endpoints

#### List Failed Payments

```bash
GET /api/v1/admin/dunning/failed-payments
```

**Response:**
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

#### Get Dunning Metrics

```bash
GET /api/v1/admin/dunning/metrics?period=30d
```

**Response:**
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

#### Get Subscription Dunning History

```bash
GET /api/v1/subscriptions/{id}/dunning-history
```

**Response:**
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

### Webhook Events

Subscribe to dunning-related webhooks:

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

## Best Practices for Payment Recovery

### 1. Optimize Your Retry Schedule

**Test different schedules** to find what works for your business:

| Business Type | Recommended Schedule |
|--------------|---------------------|
| SaaS / Software | [1, 3, 7] - Standard |
| Physical Goods | [3, 7, 14] - More time for customers |
| High-Value B2B | [3, 7, 14, 21] - Extended with more attempts |
| Low-Value / Freemium | [1, 3] - Quick recovery or cancel |

### 2. Make Payment Updates Easy

**Reduce friction** in the payment update process:

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

### 3. Segment Your Approach

**Different strategies for different customers:**

```toml
# High-value customers get extended grace periods
[dunning.segments.premium]
max_retries = 5
retry_intervals_days = [3, 7, 14, 21, 30]
grace_period_days = 45

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

### 4. Monitor and Respond

**Key metrics to track:**

| Metric | Target | Action if Below Target |
|--------|--------|----------------------|
| Recovery Rate | >65% | Review email templates |
| First-Attempt Recovery | >40% | Optimize retry timing |
| Email Open Rate | >50% | Improve subject lines |
| Click Rate | >15% | Better call-to-action |

### 5. Proactive Prevention

**Reduce failures before they happen:**

1. **Expiration Reminders**
   ```
   Email customers 30 days before card expiration
   Include direct link to update payment method
   ```

2. **Pre-Dunning Emails**
   ```
   3 days before billing: "Your subscription renews soon"
   Include amount and payment method on file
   ```

3. **Account Updater Services**
   ```
   Integrate with Visa/Mastercard account updater
   Automatically update expired card numbers
   ```

### 6. Customer Communication

**Tone progression through dunning:**

| Stage | Tone | Example Opening |
|-------|------|-----------------|
| First Failure | Friendly, helpful | "Don't worry, this happens..." |
| Retry Failure | Urgent, polite | "We need your attention..." |
| Final Notice | Serious, clear | "Action required to keep your subscription..." |
| Cancellation | Professional, open | "We'd love to have you back..." |

### 7. Manual Intervention

**When to manually intervene:**

- High-value customers (enterprise accounts)
- Long-tenure customers (2+ years)
- Customers who contact support
- Technical failures (not payment issues)

**CLI commands for manual dunning:**

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

## Example Dunning Workflow

### Scenario: Monthly Subscription Failure

**Background:**
- Customer: Sarah Johnson
- Subscription: Premium Plan ($49/month)
- Payment method: Visa ending in 4242
- Billing date: 1st of each month

**Timeline:**

**February 1, 2026 - Initial Billing Failure**
```
08:00: Automated billing attempt fails
08:05: Dunning system processes failure
08:10: Subscription status → Past Due
08:15: First Failure email sent to Sarah
08:15: Next retry scheduled for February 2
```

**February 1, 2026 - Email Received**
```
Subject: Payment Failed - Please Update Your Payment Method

Hi Sarah,

We were unable to process your Premium Plan subscription payment.

Amount Due: $49.00
Next Retry: February 2, 2026

Don't worry - your subscription is still active! 
Please update your payment method:

[Update Payment Method]
```

**February 2, 2026 - Retry #1 (Silent)**
```
08:00: Automatic retry with same card
08:01: Card declined again (insufficient funds)
08:05: Next retry scheduled for February 5
```

**February 5, 2026 - Retry #2**
```
08:00: Automatic retry
08:01: Card declined again
08:05: Retry Failure email sent
08:05: Next retry scheduled for February 12
```

**February 5, 2026 - Email Received**
```
Subject: Payment Failed Again - Action Required

Hi Sarah,

Your Premium Plan payment failed again (Attempt 2 of 3).

Amount Due: $49.00
Next Retry: February 12, 2026

To keep your subscription active, please update your payment method:

[Update Payment Method Now]
```

**February 10, 2026 - Customer Action**
```
14:30: Sarah clicks email link
14:35: Updates to new Mastercard
14:40: Payment processed successfully
14:45: Subscription restored to Active
14:50: Payment Recovered email sent
```

**February 10, 2026 - Success Email**
```
Subject: Payment Successful - Subscription Active ✓

Hi Sarah,

Great news! Your payment was successfully processed.

Amount Charged: $49.00
Next Billing: March 1, 2026

Your Premium Plan subscription is now active!
```

**Outcome:** Payment recovered on 2nd retry attempt. Total time to recovery: 9 days.

## FAQ

### General Questions

**Q: What is dunning?**

A: Dunning is the automated process of retrying failed subscription payments and communicating with customers to recover revenue. It includes scheduled retries, email notifications, and eventual cancellation if recovery fails.

**Q: How much revenue can dunning recover?**

A: With proper configuration, dunning typically recovers 60-80% of failed subscription payments. Recovery rates vary by industry, customer segment, and dunning strategy.

**Q: Is dunning automatic?**

A: Yes, the entire dunning process is automated once configured. The system handles retries, emails, and cancellations without manual intervention.

### Configuration Questions

**Q: Can I customize the retry schedule?**

A: Yes, you can configure the number of retries and intervals between them in your `config.toml`:

```toml
[dunning]
max_retries = 3
retry_intervals_days = [1, 3, 7]
```

**Q: What's the recommended retry schedule?**

A: The default schedule `[1, 3, 7]` works well for most businesses:
- Retry 1: 1 day after failure (catches temporary issues)
- Retry 2: 3 days later (allows time for customer action)
- Retry 3: 7 days later (final attempt)

**Q: Can I disable dunning?**

A: While not recommended, you can effectively disable dunning by setting:

```toml
[dunning]
max_retries = 0
```

This will cancel subscriptions immediately on payment failure.

### Email Questions

**Q: Can I customize dunning emails?**

A: Yes, email templates are stored in `templates/dunning/` and can be fully customized using HTML and template variables.

**Q: What template variables are available?**

A: Common variables include `{{customer_name}}`, `{{amount}}`, `{{next_retry_date}}`, `{{update_payment_url}}`, and more. See the [Setting Up Dunning Emails](#setting-up-dunning-emails) section for the complete list.

**Q: Can I send SMS notifications instead of emails?**

A: SMS notifications are planned for a future release. Currently, dunning supports email notifications only.

### Customer Experience Questions

**Q: Do customers lose access during dunning?**

A: No, customers retain access during the grace period (default: 14 days). The subscription status changes to "Past Due" but service continues until cancellation.

**Q: Can customers update their payment method during dunning?**

A: Yes, customers can update their payment method at any time during the dunning process. The next scheduled retry will use the new payment method.

**Q: What happens if a customer updates their card?**

A: If a customer updates their payment method, the next scheduled retry will use the new card. You can also trigger an immediate retry via the API or CLI.

### Billing Questions

**Q: Can I charge late fees for overdue payments?**

A: Yes, you can configure late fees in your dunning settings:

```toml
[dunning]
late_fee_after_retry = 2
late_fee_amount = "5.00"
```

**Q: Are late fees added to the original invoice or a new one?**

A: Late fees are added to the original invoice total. When payment is eventually processed, it includes both the subscription amount and any accumulated late fees.

**Q: Can customers be charged for multiple months if dunning takes a while?**

A: No, dunning only attempts to collect the current failed invoice. New billing cycles are paused until the past due invoice is resolved.

### Recovery Questions

**Q: Can I manually retry a payment?**

A: Yes, you can manually trigger a retry using the CLI:

```bash
rcommerce subscription retry-payment <subscription-id>
```

Or via the API:

```bash
POST /api/v1/subscriptions/{id}/retry-payment
```

**Q: Can I extend the grace period for a specific customer?**

A: Yes, you can extend the grace period:

```bash
rcommerce subscription extend-grace <subscription-id> --days 7 --reason "Customer contacted support"
```

**Q: What happens after all retries fail?**

A: The subscription is cancelled with the reason "payment_failed". The customer receives a cancellation notice email and can reactivate their subscription by placing a new order.

### Analytics Questions

**Q: How do I track dunning performance?**

A: Use the dunning dashboard at `/admin/dunning` or the metrics API:

```bash
GET /api/v1/admin/dunning/metrics
```

**Q: What's a good recovery rate?**

A: Industry benchmarks:
- Excellent: >75%
- Good: 60-75%
- Average: 45-60%
- Needs improvement: <45%

**Q: How can I improve my recovery rate?**

A: Key strategies:
1. Optimize email subject lines for higher open rates
2. Make payment updates frictionless
3. Test different retry schedules
4. Segment customers by value/tenure
5. Send pre-dunning expiration reminders

### Integration Questions

**Q: Does dunning work with all payment gateways?**

A: Yes, dunning works with all supported payment gateways (Stripe, Airwallex, Alipay, WeChat Pay). The system uses the standard payment gateway interface for retries.

**Q: Can I receive webhooks for dunning events?**

A: Yes, subscribe to these webhook events:
- `dunning.payment_failed`
- `dunning.payment_recovered`
- `dunning.subscription_cancelled`

**Q: Does dunning handle 3D Secure / SCA?**

A: For payment methods requiring authentication, the system sends an email with an authentication link instead of attempting an automatic retry.

---

## Related Documentation

- [Payments](../api-reference/payments.md)
- [Payment Gateways](../payment-gateways/index.md)
- [Webhooks](../api-reference/webhooks.md)
- [CLI Reference](../development/cli-reference.md)
