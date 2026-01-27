//! Payment gateway implementations

pub mod stripe;
pub mod airwallex;
pub mod wechatpay;
pub mod alipay;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::Result;
use crate::payment::{
    PaymentGateway, CreatePaymentRequest, PaymentSession, Payment, PaymentStatus, 
    PaymentSessionStatus, Refund, RefundStatus, WebhookEvent, WebhookEventType
};
use rust_decimal::Decimal;


/// Legacy payment response structure (kept for backward compatibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub success: bool,
    pub transaction_id: Option<String>,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub error_message: Option<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Mock payment gateway for local development and testing
pub struct MockPaymentGateway;

impl MockPaymentGateway {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MockPaymentGateway {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PaymentGateway for MockPaymentGateway {
    fn id(&self) -> &'static str {
        "mock"
    }
    
    fn name(&self) -> &'static str {
        "Mock Payment Gateway"
    }
    
    async fn create_payment(&self, request: CreatePaymentRequest) -> Result<PaymentSession> {
        Ok(PaymentSession {
            id: format!("mock_pay_{}", uuid::Uuid::new_v4()),
            client_secret: format!("mock_secret_{}", uuid::Uuid::new_v4()),
            status: PaymentSessionStatus::Open,
            amount: request.amount,
            currency: request.currency,
            customer_id: request.customer_id,
        })
    }
    
    async fn confirm_payment(&self, payment_id: &str) -> Result<Payment> {
        Ok(Payment {
            id: payment_id.to_string(),
            gateway: self.id().to_string(),
            amount: Decimal::new(5000, 2),
            currency: "USD".to_string(),
            status: PaymentStatus::Succeeded,
            order_id: uuid::Uuid::new_v4(),
            customer_id: None,
            payment_method: "card".to_string(),
            transaction_id: payment_id.to_string(),
            captured_at: Some(chrono::Utc::now()),
            created_at: chrono::Utc::now(),
        })
    }
    
    async fn capture_payment(&self, payment_id: &str, amount: Option<Decimal>) -> Result<Payment> {
        Ok(Payment {
            id: payment_id.to_string(),
            gateway: self.id().to_string(),
            amount: amount.unwrap_or(Decimal::new(5000, 2)),
            currency: "USD".to_string(),
            status: PaymentStatus::Succeeded,
            order_id: uuid::Uuid::new_v4(),
            customer_id: None,
            payment_method: "card".to_string(),
            transaction_id: payment_id.to_string(),
            captured_at: Some(chrono::Utc::now()),
            created_at: chrono::Utc::now(),
        })
    }
    
    async fn refund_payment(&self, payment_id: &str, amount: Option<Decimal>, reason: &str) -> Result<Refund> {
        Ok(Refund {
            id: format!("ref_{}", uuid::Uuid::new_v4()),
            payment_id: payment_id.to_string(),
            amount: amount.unwrap_or(Decimal::new(5000, 2)),
            currency: "USD".to_string(),
            status: RefundStatus::Succeeded,
            reason: reason.to_string(),
            created_at: chrono::Utc::now(),
        })
    }
    
    async fn get_payment(&self, payment_id: &str) -> Result<Payment> {
        Ok(Payment {
            id: payment_id.to_string(),
            gateway: self.id().to_string(),
            amount: Decimal::new(5000, 2),
            currency: "USD".to_string(),
            status: PaymentStatus::Succeeded,
            order_id: uuid::Uuid::new_v4(),
            customer_id: None,
            payment_method: "card".to_string(),
            transaction_id: payment_id.to_string(),
            captured_at: Some(chrono::Utc::now()),
            created_at: chrono::Utc::now(),
        })
    }
    
    async fn handle_webhook(&self, _payload: &[u8], _signature: &str) -> Result<WebhookEvent> {
        Ok(WebhookEvent {
            event_type: WebhookEventType::PaymentSucceeded,
            payment_id: format!("mock_pay_{}", uuid::Uuid::new_v4()),
            data: serde_json::json!({}),
        })
    }
}

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_mock_gateway_create_payment() {
        let gateway = MockPaymentGateway::new();
        let request = CreatePaymentRequest {
            amount: Decimal::new(5000, 2),
            currency: "USD".to_string(),
            order_id: Uuid::new_v4(),
            customer_id: None,
            customer_email: "test@example.com".to_string(),
            payment_method: crate::payment::PaymentMethod::Card(crate::payment::CardDetails {
                number: "4242424242424242".to_string(),
                exp_month: 12,
                exp_year: 2025,
                cvc: "123".to_string(),
                name: "Test User".to_string(),
            }),
            billing_address: None,
            metadata: serde_json::json!({}),
        };
        
        let result = gateway.create_payment(request).await.unwrap();
        assert_eq!(result.amount, Decimal::new(5000, 2));
        assert_eq!(result.currency, "USD");
        assert_eq!(result.status, PaymentSessionStatus::Open);
    }

    #[tokio::test]
    async fn test_mock_gateway_confirm_payment() {
        let gateway = MockPaymentGateway::new();
        let result = gateway.confirm_payment("mock_pay_123").await.unwrap();
        
        assert_eq!(result.status, PaymentStatus::Succeeded);
        assert_eq!(result.gateway, "mock");
    }
    
    #[tokio::test]
    async fn test_mock_gateway_refund() {
        let gateway = MockPaymentGateway::new();
        let result = gateway.refund_payment("mock_pay_123", Some(Decimal::new(2500, 2)), "customer_request").await.unwrap();
        
        assert_eq!(result.status, RefundStatus::Succeeded);
        assert_eq!(result.amount, Decimal::new(2500, 2));
    }
}
