#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::payment::gateways::StripeGateway;
    use rust_decimal_macros::dec;
    
    #[tokio::test]
    async fn test_stripe_payment_gateway_creation() {
        let gateway = StripeGateway::new(
            "sk_test_1234567890".to_string(),
            "whsec_test123456".to_string()
        );
        
        assert_eq!(gateway.id(), "stripe");
        assert_eq!(gateway.name(), "Stripe");
    }
    
    #[test]
    fn test_create_payment_request_validation() {
        let request = CreatePaymentRequest {
            amount: dec!(99.99),
            currency: "USD".to_string(),
            order_id: uuid::Uuid::new_v4(),
            customer_id: Some(uuid::Uuid::new_v4()),
            customer_email: "customer@example.com".to_string(),
            payment_method: PaymentMethod::Card(CardDetails {
                number: "4242424242424242".to_string(),
                exp_month: 12,
                exp_year: 2025,
                cvc: "123".to_string(),
                name: "John Doe".to_string(),
            }),
            billing_address: Some(Address {
                line1: "123 Main St".to_string(),
                line2: None,
                city: "San Francisco".to_string(),
                state: Some("CA".to_string()),
                postal_code: "94105".to_string(),
                country: "US".to_string(),
            }),
            metadata: serde_json::json!({"test": true}),
        };
        
        assert_eq!(request.amount, dec!(99.99));
        assert_eq!(request.currency, "USD");
        assert!(request.customer_id.is_some());
    }
    
    #[test]
    fn test_payment_status_transitions() {
        use PaymentStatus::*;
        
        // Test valid transitions
        assert!(Pending.can_transition_to(Processing));
        assert!(Processing.can_transition_to(Succeeded));
        assert!(Processing.can_transition_to(Failed));
        assert!(Succeeded.can_transition_to(Refunded));
        
        // Test invalid transitions
        assert!(!Succeeded.can_transition_to(Processing));
        assert!(!Failed.can_transition_to(Succeeded));
    }
    
    #[test]
    fn test_certificate_info_validation() {
        let info = CertificateInfo {
            domain: "example.com".to_string(),
            certificate_path: std::path::PathBuf::from("/tmp/cert.pem"),
            private_key_path: std::path::PathBuf::from("/tmp/key.pem"),
            expires_at: chrono::Utc::now() + chrono::Duration::days(90),
            issued_at: chrono::Utc::now(),
            serial_number: "1234567890".to_string(),
        };
        
        assert_eq!(info.domain, "example.com");
        assert!(info.expires_at > chrono::Utc::now());
    }
}

impl PaymentStatus {
    pub fn can_transition_to(&self, new_status: PaymentStatus) -> bool {
        use PaymentStatus::*;
        
        match (self, new_status) {
            (Pending, Processing) => true,
            (Processing, Succeeded) => true,
            (Processing, Failed) => true,
            (Succeeded, Refunded) => true,
            (Failed, Processing) => true, // Retry
            _ => false,
        }
    }
}