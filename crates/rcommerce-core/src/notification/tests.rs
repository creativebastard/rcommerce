/// Integration tests for notification module
#[cfg(test)]
mod notification_integration_tests {
    use super::*;
    use crate::notification::{
        NotificationTemplate, TemplateVariables, NotificationChannel,
        Notification, Recipient, NotificationPriority
    };
    use uuid::Uuid;
    
    #[test]
    fn test_template_loading_and_rendering() {
        // Load HTML template
        let template_result = NotificationTemplate::load("order_confirmation_html");
        assert!(template_result.is_ok(), "Failed to load template: {:?}", template_result.err());
        
        let template = template_result.unwrap();
        assert_eq!(template.id, "order_confirmation_html");
        assert_eq!(template.name, "Order Confirmation HTML");
        assert!(template.html_body.is_some(), "HTML body should be present");
        assert_eq!(template.channel, NotificationChannel::Email);
        
        // Verify template has expected content
        let html_content = template.html_body.unwrap();
        assert!(html_content.contains("R COMMERCE"), "Template should contain branding");
        assert!(html_content.contains("{{ order_number }}"), "Template should have order_number placeholder");
        assert!(html_content.contains("{{ customer_name }}"), "Template should have customer_name placeholder");
        assert!(html_content.contains("{{ order_total }}"), "Template should have order_total placeholder");
    }
    
    #[test]
    fn test_template_variable_population() {
        let mut variables = TemplateVariables::new();
        
        // Add test data
        variables.add("order_number".to_string(), "ORD-12345".to_string());
        variables.add("customer_name".to_string(), "John Doe".to_string());
        variables.add("order_total".to_string(), "99.99".to_string());
        variables.add("order_date".to_string(), "Jan 25, 2026".to_string());
        variables.add("company_name".to_string(), "R Commerce".to_string());
        variables.add("support_email".to_string(), "support@rcommerce.com".to_string());
        
        // Test plain text rendering
        let template = NotificationTemplate::load("order_confirmation").unwrap();
        let plain_text = template.render(&variables).unwrap();
        
        assert!(plain_text.contains("John Doe"));
        assert!(plain_text.contains("ORD-12345"));
        assert!(plain_text.contains("$99.99"));
        
        // Test HTML rendering
        let html_template = NotificationTemplate::load("order_confirmation_html").unwrap();
        let html_result = html_template.render_html(&variables);
        assert!(html_result.is_ok());
        
        let html_content = html_result.unwrap();
        assert!(html_content.is_some(), "HTML content should be generated");
        
        let html = html_content.unwrap();
        assert!(html.contains("ORD-12345"), "HTML should contain order number");
        assert!(html.contains("John Doe"), "HTML should contain customer name");
        assert!(html.contains("$99.99"), "HTML should contain order total");
        assert!(html.contains("R Commerce"), "HTML should contain company name");
    }
    
    #[test]
    fn test_notification_creation_with_html() {
        use crate::models::customer::Customer;
        use crate::models::address::Address;
        use crate::order::{Order, OrderItem};
        use rust_decimal::Decimal;
        use chrono::Utc;
        
        // Create mock order
        let order = Order {
            id: Uuid::new_v4(),
            order_number: "ORD-TEST-001".to_string(),
            customer_id: Some(Uuid::new_v4()),
            customer_email: "test@example.com".to_string(),
            billing_address_id: Some(Uuid::new_v4()),
            shipping_address_id: Some(Uuid::new_v4()),
            status: crate::order::lifecycle::OrderStatus::Confirmed,
            fulfillment_status: crate::order::fulfillment::FulfillmentStatus::Unfulfilled,
            payment_status: crate::payment::PaymentStatus::Paid,
            currency: "USD".to_string(),
            subtotal: Decimal::new(9999, 2),
            tax_total: Decimal::new(0, 2),
            shipping_total: Decimal::new(999, 2),
            discount_total: Decimal::new(0, 2),
            total: Decimal::new(10998, 2),
            notes: None,
            tags: vec![],
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
        };
        
        // Create mock customer
        let customer = Customer {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            phone: Some("+1234567890".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            email_verified: true,
            metadata: serde_json::json!({}),
        };
        
        // Create mock addresses
        let shipping = Address {
            id: Uuid::new_v4(),
            customer_id: Some(customer.id),
            recipient_name: "Jane Smith".to_string(),
            company: None,
            street_address: "123 Main St".to_string(),
            city: "San Francisco".to_string(),
            state: "CA".to_string(),
            zip_code: "94102".to_string(),
            country: "United States".to_string(),
            phone: None,
            is_default: true,
            address_type: crate::models::address::AddressType::Shipping,
        };
        
        let billing = Address {
            id: Uuid::new_v4(),
            customer_id: Some(customer.id),
            recipient_name: "Jane Smith".to_string(),
            company: Some("Acme Corp".to_string()),
            street_address: "456 Business Ave".to_string(),
            city: "New York".to_string(),
            state: "NY".to_string(),
            zip_code: "10001".to_string(),
            country: "United States".to_string(),
            phone: None,
            is_default: false,
            address_type: crate::models::address::AddressType::Billing,
        };
        
        // Create recipient
        let recipient = Recipient::email(
            "test@example.com".to_string(),
            Some("Jane Smith".to_string())
        );
        
        // Test notification creation
        let notification = Notification {
            id: Uuid::new_v4(),
            channel: NotificationChannel::Email,
            recipient,
            subject: format!("Order Confirmed: {}", order.order_number),
            body: "Your order has been confirmed".to_string(),
            html_body: Some("<h1>Order Confirmed</h1><p>Your order has been confirmed</p>".to_string()),
            priority: NotificationPriority::High,
            metadata: serde_json::json!({
                "order_id": order.id,
                "customer_id": customer.id,
            }),
            scheduled_at: None,
            created_at: Utc::now(),
        };
        
        assert_eq!(notification.channel, NotificationChannel::Email);
        assert!(notification.html_body.is_some());
        assert_eq!(notification.priority, NotificationPriority::High);
    }
    
    #[test]
    fn test_email_message_structure() {
        use crate::notification::channels::email::EmailMessage;
        
        // Test plain text email
        let plain_email = EmailMessage::plain_text(
            "from@example.com".to_string(),
            "to@example.com".to_string(),
            "Subject".to_string(),
            "Body text".to_string(),
        );
        
        assert_eq!(plain_email.from, "from@example.com");
        assert_eq!(plain_email.to, "to@example.com");
        assert_eq!(plain_email.subject, "Subject");
        assert_eq!(plain_email.text_body, "Body text");
        assert!(plain_email.html_body.is_none());
        assert!(!plain_email.has_html());
        assert_eq!(plain_email.mime_type(), "text/plain");
        
        // Test HTML email
        let html_email = EmailMessage::html(
            "from@example.com".to_string(),
            "to@example.com".to_string(),
            "Subject".to_string(),
            "Plain text body".to_string(),
            "<h1>HTML Body</h1>".to_string(),
        );
        
        assert!(html_email.has_html());
        assert_eq!(html_email.mime_type(), "multipart/alternative");
        assert_eq!(html_email.html_body, Some("<h1>HTML Body</h1>".to_string()));
    }
    
    #[test]
    fn test_all_placeholders_in_template() {
        let template = NotificationTemplate::load("order_confirmation_html").unwrap();
        let html = template.html_body.unwrap();
        
        // List of all expected placeholders
        let expected_placeholders = vec![
            "order_number",
            "order_date",
            "order_total",
            "customer_name",
            "shipping_street",
            "shipping_city_state_zip",
            "shipping_country",
            "billing_company",
            "billing_street",
            "billing_city",
            "billing_country",
            "company_name",
            "support_email",
        ];
        
        for placeholder in expected_placeholders {
            let placeholder_str = format!("{{ {{ {} }} }}", placeholder);
            assert!(
                html.contains(&placeholder_str),
                "Template missing placeholder: {}", placeholder
            );
        }
    }
}