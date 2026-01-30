# Payment Integration Architecture

## Overview

The payment system is designed to be **provider-agnostic** with a unified interface for processing payments across multiple gateways. This enables merchants to accept payments through various methods (credit cards, bank transfers, digital wallets, etc.) while maintaining a consistent integration experience.

**Key Design Principle:** All payment flows are handled server-side. The frontend sends card data to the R Commerce API, which then communicates with payment providers. This approach provides better security (no API keys in JavaScript) and a unified interface across all gateways.

**Supported Gateways:**
- Stripe (Global cards and digital wallets)
- WeChat Pay (China's leading mobile payment)
- AliPay (Alibaba's global payment platform)
- Airwallex (Multi-currency/global)
- Braintree (PayPal-owned)
- PayPal
- Manual/Bank Transfer

## Payment Flow Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         Frontend (Browser/App)                           │
│  - Collect card data from form                                          │
│  - Send to R Commerce API                                               │
│  - Handle 3D Secure / redirect if required                              │
└───────────────────────────┬─────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                      R Commerce API (Axum Server)                        │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  Payment v2 Routes (Agnostic API)                                │   │
│  │  POST /v2/payments/methods    - Get available payment methods   │   │
│  │  POST /v2/payments            - Initiate payment                │   │
│  │  POST /v2/payments/:id/complete - Complete 3DS/redirect         │   │
│  │  POST /v2/payments/:id/refund  - Process refund                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                              │                                          │
│                              ▼                                          │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │              AgnosticPaymentGateway Trait                        │   │
│  │  - initiate_payment()      - Start payment flow                 │   │
│  │  - complete_payment_action() - Complete 3DS/redirect            │   │
│  │  - get_payment_status()    - Check payment status               │   │
│  │  - refund_payment()        - Process refunds                    │   │
│  │  - handle_webhook()        - Receive provider webhooks          │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└───────────────────────────┬─────────────────────────────────────────────┘
                            │
                ┌───────────┴───────────┐
                │                       │
                ▼                       ▼
┌──────────────────────────┐  ┌──────────────────────────┐
│   StripeAgnosticGateway   │  │   AirwallexAgnosticGateway│
│                          │  │                          │
│ - Server-to-server API   │  │ - Server-to-server API   │
│ - Payment intent create  │  │ - Payment intent create  │
│ - 3D Secure handling     │  │ - 3D Secure handling     │
│ - Webhook verification   │  │ - Webhook verification   │
└───────────┬──────────────┘  └───────────┬──────────────┘
            │                             │
            └──────────────┬──────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                   Payment Provider APIs (Server-Side)                    │
│              (Stripe API / Airwallex API / WeChat Pay API)               │
└─────────────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. AgnosticPaymentGateway Trait

The unified interface that all payment gateways implement:

```rust
#[async_trait]
pub trait AgnosticPaymentGateway: Send + Sync {
    /// Get gateway configuration and supported methods
    async fn get_config(&self) -> Result<GatewayConfig>;
    
    /// Initiate a payment (server-side)
    async fn initiate_payment(
        &self,
        request: InitiatePaymentRequest,
    ) -> Result<InitiatePaymentResponse>;
    
    /// Complete a payment action (3DS, redirect return, etc.)
    async fn complete_payment_action(
        &self,
        request: CompletePaymentActionRequest,
    ) -> Result<CompletePaymentActionResponse>;
    
    /// Get current payment status
    async fn get_payment_status(&self, payment_id: &str) -> Result<PaymentStatus>;
    
    /// Refund a payment
    async fn refund_payment(&self, request: RefundRequest) -> Result<RefundResponse>;
    
    /// Handle incoming webhook from provider
    async fn handle_webhook(
        &self,
        payload: &[u8],
        headers: &[(String, String)],
    ) -> Result<WebhookEvent>;
}
```

### 2. Standardized Request/Response Types

```rust
/// Request to initiate a payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiatePaymentRequest {
    pub gateway_id: String,
    pub amount: Decimal,
    pub currency: String,
    pub payment_method: PaymentMethodData,
    pub order_id: String,
    pub customer_email: Option<String>,
    pub customer_id: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub idempotency_key: Option<String>,
    pub return_url: Option<String>,
    pub save_payment_method: bool,
}

/// Response from initiating a payment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InitiatePaymentResponse {
    /// Payment requires additional action (3DS, redirect)
    RequiresAction {
        payment_id: String,
        action_type: PaymentActionType,
        action_data: Value,
        expires_at: DateTime<Utc>,
    },
    /// Payment completed successfully
    Success {
        payment_id: String,
        transaction_id: String,
        payment_status: PaymentStatus,
        payment_method: PaymentMethodInfo,
        receipt_url: Option<String>,
    },
    /// Payment failed
    Failed {
        payment_id: String,
        error_code: String,
        error_message: String,
        retry_allowed: bool,
    },
}
```

### 3. PaymentMethodData - Unified Payment Input

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaymentMethodData {
    Card {
        number: String,
        exp_month: i32,
        exp_year: i32,
        cvc: String,
        name: Option<String>,
    },
    SavedCard {
        token: String,
    },
    BankTransfer {
        account_number: String,
        routing_number: String,
        account_holder_name: String,
        bank_name: String,
    },
    DigitalWallet {
        wallet_type: String, // "apple_pay", "google_pay", "paypal"
        token: String,
    },
}
```

### 4. Stripe Agnostic Gateway Implementation

```rust
pub struct StripeAgnosticGateway {
    api_key: String,
    webhook_secret: String,
    client: reqwest::Client,
}

#[async_trait]
impl AgnosticPaymentGateway for StripeAgnosticGateway {
    async fn initiate_payment(
        &self,
        request: InitiatePaymentRequest,
    ) -> Result<InitiatePaymentResponse> {
        // Convert amount to cents
        let amount_in_cents = ((request.amount * dec!(100))
            .to_string()
            .parse::<i64>()
            .unwrap_or(0)) as i64;
        
        // Build Stripe payment intent params
        let mut params = HashMap::new();
        params.insert("amount", amount_in_cents.to_string());
        params.insert("currency", request.currency.to_lowercase());
        params.insert("automatic_payment_methods[enabled]", "true".to_string());
        
        // Handle card data
        match &request.payment_method {
            PaymentMethodData::Card { number, exp_month, exp_year, cvc, name } => {
                // Create payment method from card details
                let pm_params = json!({
                    "type": "card",
                    "card": {
                        "number": number,
                        "exp_month": exp_month,
                        "exp_year": exp_year,
                        "cvc": cvc,
                    },
                    "billing_details": {
                        "name": name.as_deref().unwrap_or(""),
                    },
                });
                
                let pm_response = self.client
                    .post("https://api.stripe.com/v1/payment_methods")
                    .basic_auth(&self.api_key, Some(""))
                    .form(&pm_params)
                    .send()
                    .await?;
                
                let payment_method: StripePaymentMethod = pm_response.json().await?;
                params.insert("payment_method", payment_method.id);
            }
            PaymentMethodData::SavedCard { token } => {
                params.insert("payment_method", token.clone());
            }
            _ => return Err(Error::UnsupportedPaymentMethod),
        }
        
        // Create and confirm payment intent
        params.insert("confirm", "true".to_string());
        params.insert("off_session", "false".to_string());
        
        if let Some(return_url) = &request.return_url {
            params.insert("return_url", return_url.clone());
        }
        
        let response = self.client
            .post("https://api.stripe.com/v1/payment_intents")
            .basic_auth(&self.api_key, Some(""))
            .form(&params)
            .send()
            .await?;
        
        let intent: StripePaymentIntent = response.json().await?;
        
        // Convert Stripe response to agnostic format
        match intent.status.as_str() {
            "succeeded" => Ok(InitiatePaymentResponse::Success { ... }),
            "requires_action" => Ok(InitiatePaymentResponse::RequiresAction { ... }),
            _ => Ok(InitiatePaymentResponse::Failed { ... }),
        }
    }
}
```

## API Endpoints (v2 - Agnostic)

### Get Available Payment Methods

```http
POST /api/v2/payments/methods
Content-Type: application/json
Authorization: Bearer <token>

{
  "currency": "USD",
  "amount": "99.99"
}
```

**Response:**

```json
{
  "gateway_id": "stripe",
  "gateway_name": "Stripe",
  "payment_methods": [
    {
      "method_type": "card",
      "display_name": "Credit/Debit Card",
      "requires_redirect": false,
      "supports_3ds": true,
      "required_fields": [
        {
          "name": "number",
          "label": "Card Number",
          "field_type": "card_number",
          "required": true,
          "pattern": "^[\\d\\s]{13,19}$"
        },
        {
          "name": "exp_month",
          "label": "Expiry Month",
          "field_type": "expiry_date",
          "required": true,
          "pattern": "^(0[1-9]|1[0-2])$"
        }
      ]
    }
  ]
}
```

### Initiate Payment

```http
POST /api/v2/payments
Content-Type: application/json
Authorization: Bearer <token>

{
  "gateway_id": "stripe",
  "amount": "99.99",
  "currency": "USD",
  "payment_method": {
    "type": "card",
    "card": {
      "number": "4242424242424242",
      "exp_month": 12,
      "exp_year": 2025,
      "cvc": "123",
      "name": "John Doe"
    }
  },
  "order_id": "ord_123",
  "customer_email": "john@example.com",
  "description": "Order #123",
  "return_url": "https://yoursite.com/checkout/complete"
}
```

**Success Response:**

```json
{
  "type": "success",
  "payment_id": "pay_550e8400-e29b-41d4-a716-446655440000",
  "transaction_id": "pi_3O...",
  "payment_status": "succeeded",
  "payment_method": {
    "method_type": "card",
    "last_four": "4242",
    "card_brand": "visa",
    "exp_month": "12",
    "exp_year": "2025"
  },
  "receipt_url": "https://pay.stripe.com/receipts/..."
}
```

**Requires Action Response (3D Secure):**

```json
{
  "type": "requires_action",
  "payment_id": "pay_550e8400-e29b-41d4-a716-446655440000",
  "action_type": "three_d_secure",
  "action_data": {
    "redirect_url": "https://hooks.stripe.com/3d_secure/...",
    "type": "use_stripe_sdk"
  },
  "expires_at": "2026-01-28T11:00:00Z"
}
```

### Complete Payment Action

After 3D Secure or redirect completion:

```http
POST /api/v2/payments/{payment_id}/complete
Content-Type: application/json
Authorization: Bearer <token>

{
  "action_type": "three_d_secure",
  "action_data": {
    "payment_intent_client_secret": "pi_3O..._secret_..."
  }
}
```

### Refund Payment

```http
POST /api/v2/payments/{payment_id}/refund
Content-Type: application/json
Authorization: Bearer <token>

{
  "amount": "99.99",
  "reason": "requested_by_customer"
}
```

## Frontend Integration Example

```javascript
// checkout.js - New agnostic approach

async function processPayment() {
  // 1. Collect card data from form (NOT Stripe Elements)
  const cardData = {
    number: document.getElementById('cardNumber').value,
    exp_month: parseInt(document.getElementById('expMonth').value),
    exp_year: parseInt(document.getElementById('expYear').value),
    cvc: document.getElementById('cvc').value,
    name: document.getElementById('cardName').value
  };
  
  // 2. Send to R Commerce API
  const response = await fetch(`${API_BASE_URL}/v2/payments`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${API_KEY}`
    },
    body: JSON.stringify({
      gateway_id: 'stripe', // or 'airwallex', 'wechatpay', etc.
      amount: total,
      currency: 'usd',
      payment_method: {
        type: 'card',
        card: cardData
      },
      order_id: orderId,
      customer_email: customerEmail,
      return_url: window.location.origin + '/checkout/complete'
    })
  });
  
  const result = await response.json();
  
  // 3. Handle response
  switch (result.type) {
    case 'success':
      // Payment complete - create order
      await createOrder(result.payment_id);
      window.location.href = '/checkout/success';
      break;
      
    case 'requires_action':
      // Handle 3D Secure or redirect
      if (result.action_type === 'redirect') {
        // Redirect to bank/3DS page
        window.location.href = result.action_data.redirect_url;
      } else if (result.action_type === 'three_d_secure') {
        // Handle 3DS in iframe or redirect
        await handleThreeDSecure(result);
      }
      break;
      
    case 'failed':
      // Show error to customer
      showError(result.error_message);
      if (result.retry_allowed) {
        enableRetryButton();
      }
      break;
  }
}

// Handle 3D Secure completion
async function handleThreeDSecure(paymentResult) {
  // Option 1: Redirect approach
  window.location.href = paymentResult.action_data.redirect_url;
  
  // Option 2: Iframe approach (more complex)
  // ... display iframe, wait for completion, then call complete endpoint
}

// Called when customer returns from 3DS/redirect
async function completePayment(paymentId) {
  const response = await fetch(`${API_BASE_URL}/v2/payments/${paymentId}/complete`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${API_KEY}`
    },
    body: JSON.stringify({
      action_type: 'three_d_secure',
      action_data: {
        // Data from URL params or iframe
      }
    })
  });
  
  const result = await response.json();
  
  if (result.type === 'success') {
    await createOrder(result.payment_id);
    window.location.href = '/checkout/success';
  } else {
    showError(result.error_message);
  }
}
```

## Configuration

### TOML Configuration

```toml
[payment]
# Default gateway for new payments
default_gateway = "stripe"

# Required 3D Secure threshold (in currency units)
tds_threshold = 3000  # $30.00

# Auto-capture settings
auto_capture = true
capture_delay_seconds = 0

# Supported currencies
supported_currencies = ["USD", "EUR", "GBP", "JPY", "CNY"]

# Fraud detection
enable_fraud_check = true
risk_threshold = 75

[payment.stripe]
enabled = true
api_key = "${STRIPE_SECRET_KEY}"
webhook_secret = "${STRIPE_WEBHOOK_SECRET}"
statement_descriptor = "RCOMMERCE"

[payment.airwallex]
enabled = true
client_id = "${AIRWALLEX_CLIENT_ID}"
api_key = "${AIRWALLEX_API_KEY}"
webhook_secret = "${AIRWALLEX_WEBHOOK_SECRET}"

[payment.wechatpay]
enabled = true
mch_id = "${WECHATPAY_MCH_ID}"
app_id = "${WECHATPAY_APP_ID}"
api_key = "${WECHATPAY_API_KEY}"
serial_no = "${WECHATPAY_SERIAL_NO}"
private_key = """
-----BEGIN PRIVATE KEY-----
...
-----END PRIVATE KEY-----
"""

[payment.alipay]
enabled = true
app_id = "${ALIPAY_APP_ID}"
private_key = "${ALIPAY_PRIVATE_KEY}"
alipay_public_key = "${ALIPAY_PUBLIC_KEY}"
```

## Security Considerations

### 1. Server-Side Processing Benefits

- **No API keys in frontend**: All provider communication happens server-side
- **Unified security model**: Same security controls for all gateways
- **Audit logging**: All payment requests logged in one place
- **Rate limiting**: Centralized rate limiting on payment endpoints

### 2. PCI Compliance

```rust
// Card data is only held in memory during the request
// Never stored in database or logs
async fn initiate_payment(request: InitiatePaymentRequest) -> Result<...> {
    // Card data is used to create payment method with provider
    // Then immediately discarded
    let payment_method = gateway.create_payment_method(&request.card_data).await?;
    
    // Only store token/reference, never raw card data
    store_payment_method_reference(payment_method.id).await?;
    
    Ok(...)
}
```

### 3. Webhook Security

```rust
async fn handle_webhook(
    Path(gateway_id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse> {
    let gateway = gateway_factory.get(&gateway_id)?;
    
    // Verify webhook signature
    let signature = headers.get("stripe-signature")
        .or_else(|| headers.get("airwallex-signature"))
        .ok_or(Error::MissingSignature)?;
    
    let event = gateway.handle_webhook(&body, &[("signature".to_string(), signature.to_str()?.to_string())]).await?;
    
    // Process event
    match event {
        WebhookEvent::PaymentSucceeded { ... } => { ... }
        WebhookEvent::PaymentFailed { ... } => { ... }
        ...
    }
    
    Ok(StatusCode::OK)
}
```

### 4. Idempotency

```rust
pub async fn initiate_payment(
    &self,
    idempotency_key: &str,
    request: InitiatePaymentRequest,
) -> Result<InitiatePaymentResponse> {
    // Check if we've already processed this request
    if let Some(existing) = self.idempotency_store.get(idempotency_key).await? {
        return Ok(existing);
    }
    
    let result = self.initiate_payment_internal(request).await?;
    self.idempotency_store.store(idempotency_key, &result).await?;
    
    Ok(result)
}
```

## Testing Strategy

### Mock Gateway for Testing

```rust
pub struct MockAgnosticGateway {
    payments: Arc<Mutex<Vec<PaymentRecord>>>,
}

#[async_trait]
impl AgnosticPaymentGateway for MockAgnosticGateway {
    async fn initiate_payment(
        &self,
        request: InitiatePaymentRequest,
    ) -> Result<InitiatePaymentResponse> {
        // Simulate processing
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let payment_id = format!("mock_pay_{}", Uuid::new_v4());
        
        // Simulate 3D Secure for amounts > $50
        if request.amount > Decimal::from(50) {
            return Ok(InitiatePaymentResponse::RequiresAction {
                payment_id: payment_id.clone(),
                action_type: PaymentActionType::ThreeDSecure,
                action_data: json!({
                    "test_3ds_url": "/mock/3ds/complete"
                }),
                expires_at: Utc::now() + Duration::minutes(10),
            });
        }
        
        Ok(InitiatePaymentResponse::Success {
            payment_id,
            transaction_id: format!("mock_txn_{}", Uuid::new_v4()),
            payment_status: PaymentStatus::Succeeded,
            payment_method: PaymentMethodInfo {
                method_type: PaymentMethodType::Card,
                last_four: Some("4242".to_string()),
                card_brand: Some("visa".to_string()),
                exp_month: Some("12".to_string()),
                exp_year: Some("2025".to_string()),
                cardholder_name: Some("Test User".to_string()),
                token: None,
            },
            receipt_url: Some("https://mock.receipt/123".to_string()),
        })
    }
}
```

### Integration Testing

```rust
#[tokio::test]
async fn test_stripe_payment_flow() {
    let gateway = StripeAgnosticGateway::new(
        "sk_test_xxx".to_string(),
        "whsec_test_xxx".to_string(),
    );
    
    let request = InitiatePaymentRequest {
        gateway_id: "stripe".to_string(),
        amount: dec!(49.99),
        currency: "USD".to_string(),
        payment_method: PaymentMethodData::Card {
            number: "4242424242424242".to_string(),
            exp_month: 12,
            exp_year: 2025,
            cvc: "123".to_string(),
            name: Some("Test User".to_string()),
        },
        order_id: "order_123".to_string(),
        customer_email: Some("test@example.com".to_string()),
        description: Some("Test payment".to_string()),
        metadata: None,
        idempotency_key: Some("test-key-123".to_string()),
        return_url: Some("https://example.com/return".to_string()),
        save_payment_method: false,
    };
    
    let result = gateway.initiate_payment(request).await.unwrap();
    
    match result {
        InitiatePaymentResponse::Success { payment_status, .. } => {
            assert_eq!(payment_status, PaymentStatus::Succeeded);
        }
        _ => panic!("Expected success response"),
    }
}
```

## Migration from Old API

### Old Approach (v1)

```javascript
// Frontend used Stripe.js directly
const stripe = await loadStripe('pk_live_...');
const { client_secret } = await fetch('/api/v1/payments').then(r => r.json());
const result = await stripe.confirmCardPayment(client_secret, { ... });
```

### New Approach (v2)

```javascript
// Frontend sends card data to R Commerce API
const result = await fetch('/api/v2/payments', {
  method: 'POST',
  body: JSON.stringify({
    gateway_id: 'stripe',
    payment_method: { type: 'card', card: { number, exp_month, ... } }
  })
});
// Handle RequiresAction or Success response
```

### Benefits of New Approach

1. **Security**: No publishable keys in frontend
2. **Flexibility**: Same code works for all gateways
3. **Control**: Server controls the entire flow
4. **Consistency**: Unified error handling and logging

## Additional Resources

- [Stripe Testing Documentation](https://stripe.com/docs/testing)
- [Airwallex API Documentation](https://www.airwallex.com/docs/api)
- [WeChat Pay Documentation](https://pay.weixin.qq.com/wiki/doc/apiv3/index.shtml)
- [Alipay Documentation](https://opendocs.alipay.com/)
