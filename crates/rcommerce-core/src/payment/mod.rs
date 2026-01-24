pub mod gateways;

#[cfg(test)]
mod tests;

use async_trait::async_trait;
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::{Result, Error};

/// Payment gateway trait
#[async_trait]
pub trait PaymentGateway: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    
    /// Create a payment intent/session
    async fn create_payment(&self, request: CreatePaymentRequest) -> Result<PaymentSession>;
    
    /// Confirm a payment
    async fn confirm_payment(&self, payment_id: &str) -> Result<Payment>;
    
    /// Capture a payment
    async fn capture_payment(&self, payment_id: &str, amount: Option<Decimal>) -> Result<Payment>;
    
    /// Refund a payment
    async fn refund_payment(&self, payment_id: &str, amount: Option<Decimal>, reason: &str) -> Result<Refund>;
    
    /// Get payment details
    async fn get_payment(&self, payment_id: &str) -> Result<Payment>;
    
    /// Handle webhook
    async fn handle_webhook(&self, payload: &[u8], signature: &str) -> Result<WebhookEvent>;
}

/// Create payment request
#[derive(Debug, Clone)]
pub struct CreatePaymentRequest {
    pub amount: Decimal,
    pub currency: String,
    pub order_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub customer_email: String,
    pub payment_method: PaymentMethod,
    pub billing_address: Option<Address>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum PaymentMethod {
    Card(CardDetails),
    GooglePay,
    ApplePay,
    BankTransfer,
    CashOnDelivery,
}

#[derive(Debug, Clone)]
pub struct CardDetails {
    pub number: String,
    pub exp_month: u32,
    pub exp_year: u32,
    pub cvc: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
}

/// Payment session (for client-side checkout)
#[derive(Debug, Clone)]
pub struct PaymentSession {
    pub id: String,
    pub client_secret: String,
    pub status: PaymentSessionStatus,
    pub amount: Decimal,
    pub currency: String,
    pub customer_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentSessionStatus {
    Open,
    Complete,
    Expired,
}

/// Payment record
#[derive(Debug, Clone)]
pub struct Payment {
    pub id: String,
    pub gateway: String,
    pub amount: Decimal,
    pub currency: String,
    pub status: PaymentStatus,
    pub order_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub payment_method: String,
    pub transaction_id: String,
    pub captured_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Succeeded,
    Failed,
    Canceled,
    Disputed,
    Refunded,
}

/// Refund record
#[derive(Debug, Clone)]
pub struct Refund {
    pub id: String,
    pub payment_id: String,
    pub amount: Decimal,
    pub currency: String,
    pub status: RefundStatus,
    pub reason: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RefundStatus {
    Pending,
    Succeeded,
    Failed,
    Canceled,
}

/// Webhook event
#[derive(Debug, Clone)]
pub struct WebhookEvent {
    pub event_type: WebhookEventType,
    pub payment_id: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WebhookEventType {
    PaymentSucceeded,
    PaymentFailed,
    PaymentCanceled,
    RefundSucceeded,
    DisputeCreated,
}

impl Error {
    pub fn payment_error<T: Into<String>>(msg: T) -> Self {
        Error::Payment(msg.into())
    }
}