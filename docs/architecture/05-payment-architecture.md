# Payment Integration Architecture

## Overview

The payment system is designed to be **payment-provider agnostic** with a unified interface for processing payments across multiple gateways. This enables merchants to accept payments through various methods (credit cards, bank transfers, digital wallets, etc.) while maintaining a consistent integration experience.

**Supported Gateways (Initially):**
- Stripe (Primary)
- Airwallex
- Braintree
- PayPal
- Manual/Bank Transfer

**Design Principle:** Every payment flows through the same core workflow, with gateway-specific implementations handling only the payment method differences.

## Payment Flow Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     API Layer (PaymentController)            │
│  - Validate payment request                                │
│  - Check order validity                                    │
│  - Rate limiting                                           │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                Payment Orchestrator                          │
│  - Gateway selection logic                                 │
│  - Payment flow coordination                               │
│  - Transaction management                                  │
│  - Fraud checks                                            │
└──────────────────────────┬──────────────────────────────────┘
                           │
                ┌──────────┴──────────┐
                │                     │
┌───────────────▼──────────┐  ┌────────▼──────────┐
│      Payment Service       │  │  Order Service    │
│  - Create payment record  │  │  - Reserve stock  │
│  - Payment lifecycle      │  │  - Update status  │
└───────────────┬──────────┘  └───────────────────┘
                │
┌───────────────▼────────────────────────────────────────────┐
│                 Gateway Factory                             │
│  - Provider selection                                       │
│  - Dynamic gateway loading                                  │
│  - Plugin system for extensibility                          │
└───────────────┬──────────────────────────────┬────────────┘
                │                              │
    ┌───────────▼────────────┐     ┌───────────▼─────────┐
    │   Payment Gateway      │     │   Payment Gateway   │
    │     (Stripe)           │     │    (Airwallex)      │
    │                        │     │                     │
    │ - Process payment      │     │ - Process payment   │
    │ - Handle 3D Secure     │     │ - Handle 3D Secure  │
    │ - Token management     │     │ - Token management  │
    │ - Webhook handling     │     │ - Webhook handling  │
    └───────────┬────────────┘     └───────────┬─────────┘
                │                              │
                └──────────────┬───────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│                   Payment Provider API                      │
│              (Stripe API / Airwallex API)                   │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Payment Entity

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub gateway: String,  // "stripe", "airwallex", "braintree"
    pub amount: Decimal,
    pub currency: String,
    pub method: PaymentMethod,  // Card, BankTransfer, Wallet
    pub status: PaymentStatus,
    pub provider_payment_id: Option<String>,  // Gateway's transaction ID
    pub provider_response: Option<serde_json::Value>,
    pub provider_metadata: Option<serde_json::Value>,
    pub fraud_check_result: Option<FraudCheckResult>,
    pub refunded_amount: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "payment_status", rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,        // Payment initiated, awaiting confirmation
    Authorizing,    // Awaiting 3D Secure or bank authorization
    Authorized,     // Authorized but not captured
    Paid,           // Successfully paid/captured
    Failed,         // Payment failed
    Cancelled,      // Payment cancelled
    Refunded,       // Fully refunded
    PartiallyRefunded, // Partially refunded
    Disputed,       // Chargeback/dispute opened
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    Card(CardDetails),
    BankTransfer(BankTransferDetails),
    DigitalWallet(WalletDetails),
    BuyNowPayLater(BnplDetails),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDetails {
    pub last4: String,
    pub brand: String,  // "visa", "mastercard", etc.
    pub exp_month: i32,
    pub exp_year: i32,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub three_d_secure: Option<ThreeDSecureStatus>,
}
```

### 2. Payment Gateway Trait

```rust
#[async_trait]
pub trait PaymentGateway: Send + Sync + 'static {
    /// Gateway identifier (e.g., "stripe", "airwallex")
    fn id(&self) -> &'static str;
    
    /// Gateway display name
    fn name(&self) -> &'static str;
    
    /// Supported payment methods
    fn supported_methods(&self) -> Vec<PaymentMethodType>;
    
    /// Supported currencies
    fn supported_currencies(&self) -> Vec<&str>;
    
    /// Process a payment
    async fn process_payment(
        &self,
        payment: &Payment,
        payment_method: &PaymentMethodData,
    ) -> Result<PaymentResult>;
    
    /// Authorize only (for delayed capture)
    async fn authorize_payment(
        &self,
        payment: &Payment,
        payment_method: &PaymentMethodData,
    ) -> Result<PaymentResult>;
    
    /// Capture authorized payment
    async fn capture_payment(
        &self,
        payment: &Payment,
        amount: Option<Decimal>,
    ) -> Result<CaptureResult>;
    
    /// Refund payment
    async fn refund_payment(
        &self,
        payment: &Payment,
        amount: Decimal,
        reason: Option<String>,
    ) -> Result<RefundResult>;
    
    /// Cancel/void payment
    async fn cancel_payment(&self, payment: &Payment) -> Result<CancelResult>;
    
    /// Create customer in gateway (for saved payment methods)
    async fn create_customer(
        &self,
        customer: &Customer,
    ) -> Result<GatewayCustomer>;
    
    /// Save payment method for customer
    async fn attach_payment_method(
        &self,
        customer_id: &str,
        payment_method: &PaymentMethodData,
    ) -> Result<PaymentMethodId>;
    
    /// Get saved payment methods
    async fn list_payment_methods(
        &self,
        customer_id: &str,
    ) -> Result<Vec<SavedPaymentMethod>>;
    
    /// Handle incoming webhook
    async fn handle_webhook(
        &self,
        payload: &[u8],
        signature: Option<&str>,
        event_type: Option<&str>,
    ) -> Result<WebhookEvent>;
    
    /// Verify webhook signature
    fn verify_webhook_signature(
        &self,
        payload: &[u8],
        signature: &str,
        secret: &str,
    ) -> Result<bool>;
    
    /// Get minimum/maximum amounts for currency
    fn get_payment_limits(&self, currency: &str) -> Option<(Decimal, Decimal)>;
}
```

### 3. Stripe Implementation

```rust
pub struct StripeGateway {
    client: stripe::Client,
    webhook_secret: String,
}

#[async_trait]
impl PaymentGateway for StripeGateway {
    fn id(&self) -> &'static str { "stripe" }
    
    fn name(&self) -> &'static str { "Stripe" }
    
    async fn process_payment(
        &self,
        payment: &Payment,
        payment_method: &PaymentMethodData,
    ) -> Result<PaymentResult> {
        match payment_method {
            PaymentMethodData::CardToken(token) => {
                self.process_with_card_token(payment, token).await
            }
            PaymentMethodData::PaymentMethodId(pm_id) => {
                self.process_with_payment_method(payment, pm_id).await
            }
            PaymentMethodData::PaymentIntent(pi_id) => {
                self.confirm_payment_intent(payment, pi_id).await
            }
        }
    }
    
    async fn process_with_card_token(
        &self,
        payment: &Payment,
        token: &str,
    ) -> Result<PaymentResult> {
        // Convert amount to smallest currency unit (e.g., cents)
        let amount = (payment.amount * Decimal::from(100))
            .to_u64()
            .ok_or_else(|| Error::InvalidAmount)?;
            
        let mut params = stripe::CreatePaymentIntent::new(amount, &payment.currency.to_lowercase());
        params.payment_method_data = Some(stripe::PaymentMethodData::Card(stripe::PaymentMethodDetailsCard {
            token: Some(token.to_string()),
            ..Default::default()
        }));
        
        params.confirm = Some(true);
        params.off_session = Some(false);
        params.return_url = Some("https://yourstore.com/checkout/complete");
        params.metadata = Some(stripe::Metadata {
            order_id: payment.order_id.to_string(),
            gateway_payment_id: payment.id.to_string(),
        });
        
        let payment_intent = stripe::PaymentIntent::create(&self.client, params).await?;
        
        self.parse_payment_intent(payment_intent)
    }
    
    fn parse_payment_intent(&self, pi: stripe::PaymentIntent) -> Result<PaymentResult> {
        let status = match pi.status {
            stripe::PaymentIntentStatus::Succeeded => PaymentStatus::Paid,
            stripe::PaymentIntentStatus::RequiresAction => PaymentStatus::Authorizing,
            stripe::PaymentIntentStatus::Processing => PaymentStatus::Authorizing,
            stripe::PaymentIntentStatus::RequiresPaymentMethod => PaymentStatus::Failed,
            stripe::PaymentIntentStatus::Canceled => PaymentStatus::Cancelled,
            _ => PaymentStatus::Pending,
        };
        
        let mut provider_metadata = serde_json::Map::new();
        provider_metadata.insert("payment_intent_id".to_string(), json!(pi.id));
        provider_metadata.insert("charges".to_string(), json!(pi.charges));
        
        Ok(PaymentResult {
            status,
            provider_transaction_id: Some(pi.id),
            provider_response: Some(json!(pi)),
            provider_metadata: Some(json!(provider_metadata)),
            requires_action: pi.status == stripe::PaymentIntentStatus::RequiresAction,
            next_action: pi.next_action.map(|na| self.parse_next_action(na)),
        })
    }
    
    async fn handle_webhook(
        &self,
        payload: &[u8],
        signature: Option<&str>,
        _event_type: Option<&str>,
    ) -> Result<WebhookEvent> {
        let signature = signature.ok_or_else(|| Error::MissingWebhookSignature)?;
        let event = stripe::Webhook::construct_event(
            payload,
            signature,
            &self.webhook_secret,
        )?;
        
        match event.type_ {
            stripe::EventType::PaymentIntentPaymentFailed => {
                let payment_intent = event.data.object.as_payment_intent()
                    .ok_or_else(|| Error::InvalidWebhookPayload)?;
                    
                Ok(WebhookEvent::PaymentFailed {
                    gateway_transaction_id: payment_intent.id.to_string(),
                    reason: self.extract_failure_reason(payment_intent),
                    metadata: self.extract_metadata(payment_intent),
                })
            }
            stripe::EventType::PaymentIntentSucceeded => {
                let payment_intent = event.data.object.as_payment_intent()
                    .ok_or_else(|| Error::InvalidWebhookPayload)?;
                    
                Ok(WebhookEvent::PaymentSucceeded {
                    gateway_transaction_id: payment_intent.id.to_string(),
                    amount_captured: self.extract_captured_amount(payment_intent),
                    metadata: self.extract_metadata(payment_intent),
                })
            }
            // ... handle other event types
            _ => Err(Error::UnhandledWebhookEvent(event.type_)),
        }
    }
}
```

### 4. Gateway Factory & Registry

```rust
pub struct GatewayFactory {
    gateways: HashMap<String, Arc<dyn PaymentGateway>>,
}

impl GatewayFactory {
    pub fn new() -> Self {
        Self {
            gateways: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, gateway: Arc<dyn PaymentGateway>) {
        self.gateways.insert(gateway.id().to_string(), gateway);
    }
    
    pub fn get(&self, gateway_id: &str) -> Result<Arc<dyn PaymentGateway>> {
        self.gateways.get(gateway_id)
            .cloned()
            .ok_or_else(|| Error::GatewayNotFound(gateway_id.to_string()))
    }
    
    pub fn list(&self) -> Vec<Arc<dyn PaymentGateway>> {
        self.gateways.values().cloned().collect()
    }
    
    /// Initialize from configuration
    pub fn from_config(config: &PaymentConfig) -> Result<Self> {
        let mut factory = Self::new();
        
        // Register Stripe
        if let Some(stripe_config) = &config.stripe {
            let gateway = Arc::new(StripeGateway::new(
                stripe_config.secret_key.clone(),
                stripe_config.webhook_secret.clone(),
            )?);
            factory.register(gateway);
        }
        
        // Register Airwallex
        if let Some(awx_config) = &config.airwallex {
            let gateway = Arc::new(AirwallexGateway::new(
                awx_config.client_id.clone(),
                awx_config.api_key.clone(),
                awx_config.webhook_secret.clone(),
            )?);
            factory.register(gateway);
        }
        
        // Register other gateways...
        
        Ok(factory)
    }
}

impl Default for GatewayFactory {
    fn default() -> Self {
        let mut factory = Self::new();
        
        // Register built-in gateways with placeholder configs
        // Real initialization happens in from_config
        factory.register(Arc::new(StripeGateway::default()));
        factory.register(Arc::new(AirwallexGateway::default()));
        factory.register(Arc::new(BraintreeGateway::default()));
        factory.register(Arc::new(PayPalGateway::default()));
        factory.register(Arc::new(ManualGateway));
        
        factory
    }
}
```

### 5. Payment Orchestrator Service

```rust
pub struct PaymentOrchestrator {
    gateway_factory: GatewayFactory,
    payment_repo: Arc<dyn PaymentRepository>,
    order_service: Arc<dyn OrderService>,
    fraud_service: Arc<dyn FraudDetectionService>,
    event_dispatcher: Arc<dyn EventDispatcher>,
}

impl PaymentOrchestrator {
    pub async fn process_payment(
        &self,
        order_id: Uuid,
        gateway_id: &str,
        payment_method: PaymentMethodData,
    ) -> Result<PaymentResult> {
        // 1. Load order
        let order = self.order_service.get_order(order_id).await?;
        
        // 2. Validate order can be paid
        self.validate_order_for_payment(&order)?;
        
        // 3. Create payment record
        let mut payment = Payment {
            id: Uuid::new_v4(),
            order_id,
            gateway: gateway_id.to_string(),
            amount: order.total,
            currency: order.currency.clone(),
            method: payment_method.to_payment_method(),
            status: PaymentStatus::Pending,
            provider_payment_id: None,
            provider_response: None,
            provider_metadata: None,
            fraud_check_result: None,
            refunded_amount: Decimal::ZERO,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
        };
        
        payment = self.payment_repo.create(payment).await?;
        
        // 4. Pre-fraud check (if enabled)
        if let Some(fraud_check) = self.fraud_service.check_payment(&payment).await? {
            payment.fraud_check_result = Some(fraud_check.clone());
            payment = self.payment_repo.update(payment).await?;
            
            if fraud_check.recommendation == FraudRecommendation::Block {
                payment.status = PaymentStatus::Failed;
                payment = self.payment_repo.update(payment).await?;
                
                return Err(Error::PaymentBlockedByFraud(fraud_check.reason));
            }
        }
        
        // 5. Get gateway and process payment
        let gateway = self.gateway_factory.get(gateway_id)?;
        let result = gateway.process_payment(&payment, &payment_method).await?;
        
        // 6. Update payment record
        payment.status = result.status;
        payment.provider_payment_id = result.provider_transaction_id;
        payment.provider_response = result.provider_response;
        payment.provider_metadata = result.provider_metadata;
        payment.updated_at = Utc::now();
        
        if result.status == PaymentStatus::Paid {
            payment.completed_at = Some(Utc::now());
        }
        
        payment = self.payment_repo.update(payment).await?;
        
        // 7. Update order status if payment succeeded
        match result.status {
            PaymentStatus::Paid => {
                self.order_service.update_order_status(
                    order_id,
                    OrderStatus::Confirmed
                ).await?;
                
                self.event_dispatcher.dispatch(
                    Event::OrderPaid {
                        order_id,
                        payment_id: payment.id,
                        amount: payment.amount,
                    }
                ).await?;
            }
            PaymentStatus::Failed => {
                self.event_dispatcher.dispatch(
                    Event::PaymentFailed {
                        order_id,
                        payment_id: payment.id,
                        reason: self.extract_failure_reason(&result),
                    }
                ).await?;
            }
            _ => {}
        }
        
        // 8. Return result to caller
        Ok(PaymentResult {
            payment_id: payment.id,
            status: result.status,
            requires_action: result.requires_action,
            next_action: result.next_action,
            ..result
        })
    }
    
    pub async fn handle_webhook(
        &self,
        gateway_id: &str,
        payload: &[u8],
        signature: Option<&str>,
        event_type: Option<&str>,
    ) -> Result<()> {
        let gateway = self.gateway_factory.get(gateway_id)?;
        let event = gateway.handle_webhook(payload, signature, event_type).await?;
        
        match event {
            WebhookEvent::PaymentSucceeded {
                gateway_transaction_id,
                amount_captured,
                metadata,
            } => {
                if let Some(payment_id) = metadata.get("gateway_payment_id") {
                    let payment_id = Uuid::parse_str(payment_id)?;
                    self.complete_payment(&payment_id, amount_captured).await?;
                }
            }
            WebhookEvent::PaymentFailed { gateway_transaction_id, reason, metadata } => {
                self.handle_failed_payment(gateway_transaction_id, reason, metadata).await?;
            }
            WebhookEvent::PaymentDisputed { gateway_transaction_id, dispute_reason } => {
                self.handle_dispute(gateway_transaction_id, dispute_reason).await?;
            }
            WebhookEvent::RefundProcessed { gateway_transaction_id, amount } => {
                self.handle_refund(gateway_transaction_id, amount).await?;
            }
        }
        
        Ok(())
    }
    
    async fn complete_payment(&self, payment_id: &Uuid, amount_captured: Decimal) -> Result<()> {
        let mut payment = self.payment_repo.find_by_id(*payment_id).await?
            .ok_or_else(|| Error::PaymentNotFound(*payment_id))?;
        
        payment.status = PaymentStatus::Paid;
        payment.completed_at = Some(Utc::now());
        payment.updated_at = Utc::now();
        
        self.payment_repo.update(payment.clone()).await?;
        
        self.order_service.update_order_status(
            payment.order_id,
            OrderStatus::Confirmed,
        ).await?;
        
        self.event_dispatcher.dispatch(
            Event::PaymentCompleted {
                order_id: payment.order_id,
                payment_id: payment.id,
                amount: amount_captured,
            }
        ).await?;
        
        Ok(())
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
# Amounts >= this require 3DS
tds_threshold = 3000  # $30.00

# Auto-capture settings
auto_capture = true
# Delay capture by N seconds (0 = capture immediately)
capture_delay_seconds = 0

# Supported currencies
supported_currencies = ["USD", "EUR", "GBP", "JPY", "CNY"]

# Fraud detection (optional)
enable_fraud_check = true
risk_threshold = 75  # Score > 75 blocks payment

[payment.stripe]
enabled = true
secret_key = "sk_live_xxx"
publishable_key = "pk_live_xxx"
webhook_secret = "whsec_xxx"

# Stripe-specific settings
statement_descriptor = "RCOMMERCE"
statement_descriptor_suffix = "store"

# Connect settings (for marketplace features)
connect_enabled = false
connect_client_id = "ca_xxx"

[payment.airwallex]
enabled = true
client_id = "your_client_id"
api_key = "your_api_key"
webhook_secret = "your_webhook_secret"

# Airwallex-specific settings
account_id = "your_account_id"
request_currency = true  # Request payment in specific currency

[payment.braintree]
enabled = false
merchant_id = "your_merchant_id"
public_key = "your_public_key"
private_key = "your_private_key"

[payment.paypal]
enabled = false
client_id = "your_client_id"
client_secret = "your_client_secret"

# Third-party fraud detection (optional integration)
[fraud_detection]
provider = "sift"  # Options: "sift", "signifyd", "kount"
api_key = "your_fraud_api_key"
```

## Security Considerations

### 1. **No Sensitive Data Storage**
- Never store full card numbers, CVV, or PINs
- Store only last 4 digits and card fingerprint
- Use gateway tokens for saved payment methods

### 2. **PCI Compliance**
- All card data goes directly to payment gateway (Stripe Elements, etc.)
- Backend only receives tokens/payment method IDs
- Network segmentation for payment processing

### 3. **Webhook Security**
```rust
// Verify webhook signature
async fn verify_webhook(&self, gateway: &str, payload: &[u8], signature: &str) -> Result<bool> {
    let gateway = self.gateway_factory.get(gateway)?;
    let secret = self.get_webhook_secret(gateway.id())?;
    Ok(gateway.verify_webhook_signature(payload, signature, &secret)?)
}
```

### 4. **Idempotency**
```rust
// All payment requests must be idempotent
pub async fn process_payment(
    &self,
    idempotency_key: &str,
    request: ProcessPaymentRequest,
) -> Result<PaymentResult> {
    // Check if we've already processed this request
    if let Some(existing) = self.idempotency_store.get(idempotency_key).await? {
        return Ok(existing);
    }
    
    let result = self.process_payment_internal(request).await?;
    self.idempotency_store.store(idempotency_key, &result).await?;
    
    Ok(result)
}
```

### 5. **Rate Limiting**
- Rate limit payment attempts per order (e.g., 5 attempts/hour)
- Rate limit payment attempts per customer
- Rate limit payment attempts per IP address

## Testing Strategy

### 1. **Mock Gateway Implementation**

```rust
pub struct MockGateway {
    payments: Arc<Mutex<Vec<PaymentRecord>>>,
}

#[async_trait]
impl PaymentGateway for MockGateway {
    fn id(&self) -> &'static str { "mock" }
    
    async fn process_payment(
        &self,
        payment: &Payment,
        _payment_method: &PaymentMethodData,
    ) -> Result<PaymentResult> {
        // Simulate network delay
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Record payment
        let mut payments = self.payments.lock().await;
        let record = PaymentRecord {
            id: payment.id,
            status: PaymentStatus::Paid,
            amount: payment.amount,
        };
        payments.push(record);
        
        Ok(PaymentResult {
            status: PaymentStatus::Paid,
            provider_transaction_id: Some(format!("mock_txn_{}", Uuid::new_v4())),
            provider_response: Some(json!({ "mock": true })),
            requires_action: false,
            next_action: None,
            ..Default::default()
        })
    }
}
```

### 2. **Integration Testing**

```rust
#[tokio::test]
async fn test_stripe_payment_flow() {
    // Use Stripe test mode
    let gateway = StripeGateway::new(
        "sk_test_xxx".to_string(),
        "whsec_test_xxx".to_string(),
    ).unwrap();
    
    let payment = Payment {
        id: Uuid::new_v4(),
        order_id: Uuid::new_v4(),
        amount: dec!(49.99),
        currency: "USD".to_string(),
        ..Default::default()
    };
    
    let payment_method = PaymentMethodData::CardToken("tok_visa".to_string());
    let result = gateway.process_payment(&payment, &payment_method).await.unwrap();
    
    assert_eq!(result.status, PaymentStatus::Paid);
    assert!(result.provider_transaction_id.is_some());
}
```

### 3. **Simulated Failures**

```rust
#[tokio::test]
async fn test_payment_fails_with_insufficient_funds() {
    let gateway = MockGateway::new();
    gateway.set_next_result(PaymentResult {
        status: PaymentStatus::Failed,
        failure_code: Some("insufficient_funds".to_string()),
        failure_message: Some("The card has insufficient funds.".to_string()),
        ..Default::default()
    });
    
    let result = orchestrator.process_payment(
        order_id,
        "mock",
        PaymentMethodData::CardToken("tok_insufficient_funds".to_string())
    ).await;
    
    assert!(matches!(result, Err(Error::PaymentFailed { .. })));
}
```

## Monitoring & Observability

### Metrics
```rust
// Prometheus metrics
let payment_processing_duration = Histogram::with_opts(
    HistogramOpts::new(
        "rcommerce_payment_processing_duration_seconds",
        "Time taken to process payments",
    )
    .buckets(vec![0.01, 0.05, 0.1, 0.5, 1.0, 2.5, 5.0, 10.0])
)?;

let payment_success_total = Counter::with_opts(
    Opts::new("rcommerce_payment_success_total", "Total successful payments")
        .const_label("gateway", gateway_id)
)?;

let payment_failure_total = Counter::with_opts(
    Opts::new("rcommerce_payment_failure_total", "Total failed payments")
        .const_label("gateway", gateway_id)
        .const_label("reason", failure_reason)
)?;
```

### Tracing
```rust
async fn process_payment(&self, payment: &Payment) -> Result<PaymentResult> {
    let span = tracing::info_span!("process_payment",
        payment.id = %payment.id,
        order.id = %payment.order_id,
        gateway = %payment.gateway,
        amount = %payment.amount,
        currency = %payment.currency,
    );
    
    async move {
        let _gateway_span = tracing::info_span!("gateway_request", gateway = %payment.gateway).entered();
        let result = gateway.process_payment(payment, &payment_method).await?;
        drop(_gateway_span);
        
        if result.status == PaymentStatus::Paid {
            tracing::info!("Payment successful");
        } else {
            tracing::warn!("Payment failed", status = ?result.status);
        }
        
        Ok(result)
    }
    .instrument(span)
    .await
}
```

---

Next: [06-shipping-integration.md](06-shipping-integration.md) - Shipping provider architecture
