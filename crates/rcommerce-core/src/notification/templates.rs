use std::collections::HashMap;

use crate::{Result, Error};

/// Notification template with placeholders
#[derive(Debug, Clone)]
pub struct NotificationTemplate {
    pub id: String,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub html_body: Option<String>,
    pub channel: crate::notification::NotificationChannel,
    pub variables: Vec<String>,
}

impl NotificationTemplate {
    /// Load from file or database
    pub fn load(id: &str) -> Result<Self> {
        match id {
            "order_confirmation" => Ok(Self::order_confirmation()),
            "order_confirmation_html" => Ok(Self::order_confirmation_html()),
            "order_shipped" => Ok(Self::order_shipped()),
            "low_stock_alert" => Ok(Self::low_stock_alert()),
            _ => Err(Error::not_found("Template not found")),
        }
    }
    
    /// Render template with variables
    pub fn render(&self, variables: &TemplateVariables) -> Result<String> {
        let mut rendered = self.body.clone();
        
        for (key, value) in variables.inner.iter() {
            let placeholder = format!("{{ {{ {} }} }}", key);
            rendered = rendered.replace(&placeholder, value);
        }
        
        Ok(rendered)
    }
    
    /// Render HTML template with variables
    pub fn render_html(&self, variables: &TemplateVariables) -> Result<Option<String>> {
        if let Some(ref html_template) = self.html_body {
            let mut rendered = html_template.clone();
            
            for (key, value) in variables.inner.iter() {
                let placeholder = format!("{{ {{ {} }} }}", key);
                rendered = rendered.replace(&placeholder, value);
            }
            
            Ok(Some(rendered))
        } else {
            Ok(None)
        }
    }
    
    /// Load HTML template from embedded file
    fn load_html_template(path: &str) -> Result<String> {
        // In a real implementation, this would load from filesystem
        // For now, we'll include the template as a string literal
        match path {
            "invoice.html" => Ok(include_str!("templates/invoice.html").to_string()),
            _ => Err(Error::not_found("HTML template not found")),
        }
    }
    
    /// Order confirmation template (plain text)
    fn order_confirmation() -> Self {
        Self {
            id: "order_confirmation".to_string(),
            name: "Order Confirmation".to_string(),
            subject: "Order Confirmed: {{ order_number }}".to_string(),
            body: r#"
Hello {{ customer_name }},

Thank you for your order! Your order #{{ order_number }} has been confirmed.

Order Details:
----------------
Total: ${{ order_total }}
Items: {{ item_count }}

We'll send you another email when your order ships.

Best regards,
The R Commerce Team
        "#.to_string(),
            html_body: None,
            channel: crate::notification::NotificationChannel::Email,
            variables: vec![
                "customer_name".to_string(),
                "order_number".to_string(),
                "order_total".to_string(),
                "item_count".to_string(),
            ],
        }
    }
    
    /// Order confirmation HTML template with invoice layout
    fn order_confirmation_html() -> Self {
        Self {
            id: "order_confirmation_html".to_string(),
            name: "Order Confirmation HTML".to_string(),
            subject: "Order Confirmed: {{ order_number }}".to_string(),
            body: r#"
Hello {{ customer_name }},

Thank you for your order! Your order #{{ order_number }} has been confirmed.

Order Details:
----------------
Total: ${{ order_total }}
Items: {{ item_count }}

We'll send you another email when your order ships.

Best regards,
The R Commerce Team
        "#.to_string(),
            html_body: Some(Self::load_html_template("invoice.html").unwrap_or_else(|_| Self::get_default_html_template())),
            channel: crate::notification::NotificationChannel::Email,
            variables: vec![
                "order_number".to_string(),
                "order_date".to_string(),
                "order_total".to_string(),
                "customer_name".to_string(),
                "customer_email".to_string(),
                "shipping_address".to_string(),
                "billing_address".to_string(),
                "subtotal".to_string(),
                "shipping_cost".to_string(),
                "tax".to_string(),
                "items".to_string(),
                "company_name".to_string(),
                "support_email".to_string(),
            ],
        }
    }
    
    fn get_default_html_template() -> String {
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Order Confirmation - R Commerce</title>
    <style>
        body { font-family: Arial, sans-serif; background-color: #f9fafb; margin: 0; padding: 20px; }
        .container { max-width: 600px; margin: 0 auto; background-color: #ffffff; border: 1px solid #e5e7eb; }
        .header { padding: 40px; text-align: center; border-bottom: 1px solid #f3f4f6; }
        .content { padding: 40px; }
        .footer { background-color: #0F0F0F; color: #9ca3af; padding: 40px; text-align: center; font-size: 12px; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1 style="color: #EB4F27;">R COMMERCE</h1>
        </div>
        <div class="content">
            <h1>Order Confirmed: {{ order_number }}</h1>
            <p>Thank you for your order, {{ customer_name }}!</p>
            <p><strong>Total:</strong> ${{ order_total }}</p>
        </div>
        <div class="footer">
            <p>Questions? Contact us at <a href="mailto:{{ support_email }}">{{ support_email }}</a></p>
            <p>© 2026 {{ company_name }}</p>
        </div>
    </div>
</body>
</html>"#.to_string()
    }
    
    /// Order shipped template
    fn order_shipped() -> Self {
        Self {
            id: "order_shipped".to_string(),
            name: "Order Shipped".to_string(),
            subject: "Your Order Has Shipped!".to_string(),
            body: r#"
Great news! Your order #{{ order_number }} has been shipped.

Tracking Information:
{{#if tracking_number}}
Tracking Number: {{ tracking_number }}
Carrier: {{ carrier }}
{{/if}}

Estimated Delivery: {{ estimated_delivery }}

Thank you for your business!
        "#.to_string(),
            html_body: None,
            channel: crate::notification::NotificationChannel::Email,
            variables: vec![
                "order_number".to_string(),
                "tracking_number".to_string(),
                "carrier".to_string(),
                "estimated_delivery".to_string(),
            ],
        }
    }
    
    /// Low stock alert template
    fn low_stock_alert() -> Self {
        Self {
            id: "low_stock_alert".to_string(),
            name: "Low Stock Alert".to_string(),
            subject: "Low Stock Alert: {{ product_name }}".to_string(),
            body: r#"
⚠️ LOW STOCK ALERT

Product: {{ product_name }}
Current Stock: {{ current_stock }}
Threshold: {{ threshold }}

Recommended Reorder Quantity: {{ reorder_quantity }}

{{#if is_critical}}
🚨 CRITICAL: This product is at critically low stock levels!
{{/if}}
        "#.to_string(),
            html_body: None,
            channel: crate::notification::NotificationChannel::Email,
            variables: vec![
                "product_name".to_string(),
                "current_stock".to_string(),
                "threshold".to_string(),
                "reorder_quantity".to_string(),
                "is_critical".to_string(),
            ],
        }
    }
}

/// Template variables for rendering
#[derive(Debug, Default)]
pub struct TemplateVariables {
    inner: HashMap<String, String>,
}

impl TemplateVariables {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    
    pub fn add(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }
    
    pub fn add_order(&mut self, order: &crate::order::Order) {
        self.add("order_id".to_string(), order.id.to_string());
        self.add("order_number".to_string(), order.order_number.clone());
        self.add("order_total".to_string(), order.total.to_string());
        self.add("order_currency".to_string(), order.currency.clone());
        // Add formatted date
        use chrono::Timelike;
        let date_str = order.created_at.format("%b %d, %Y").to_string();
        self.add("order_date".to_string(), date_str);
    }
    
    pub fn add_customer(&mut self, customer: &crate::models::customer::Customer) {
        self.add("customer_id".to_string(), customer.id.to_string());
        self.add("customer_name".to_string(), format!("{} {}", customer.first_name, customer.last_name));
        self.add("customer_email".to_string(), customer.email.clone());
    }
    
    pub fn add_product(&mut self, product: &crate::models::product::Product) {
        self.add("product_id".to_string(), product.id.to_string());
        self.add("product_name".to_string(), product.title.clone());
        self.add("product_price".to_string(), product.price.to_string());
    }
    
    pub fn add_inventory_alert(&mut self, alert: &crate::inventory::LowStockAlert) {
        self.add("product_id".to_string(), alert.product_id.to_string());
        self.add("product_name".to_string(), alert.product_name.clone());
        self.add("current_stock".to_string(), alert.current_stock.to_string());
        self.add("threshold".to_string(), alert.threshold.to_string());
        self.add("reorder_quantity".to_string(), alert.recommended_reorder_quantity.to_string());
        self.add("is_critical".to_string(), alert.is_critical().to_string());
    }
    
    pub fn add_order_items(&mut self, items: &[crate::order::OrderItem]) {
        // Create a simple items list for plain text emails
        let items_text = items
            .iter()
            .map(|item| format!("{} x {} - ${}", item.quantity, item.product_title, item.price))
            .collect::<Vec<_>>()
            .join("\n");
        self.add("items".to_string(), items_text);
        
        // Calculate subtotal
        let subtotal: f64 = items.iter().map(|item| item.price * item.quantity as f64).sum();
        self.add("subtotal".to_string(), format!("{:.2}", subtotal));
    }
    
    pub fn add_addresses(&mut self, shipping: &crate::models::address::Address, billing: &crate::models::address::Address) {
        let shipping_str = format!(
            "{}<br>{}<br>{}, {} {}<br>{}",
            shipping.recipient_name,
            shipping.street_address,
            shipping.city,
            shipping.state,
            shipping.zip_code,
            shipping.country
        );
        self.add("shipping_address".to_string(), shipping_str);
        
        let billing_str = format!(
            "{}<br>{}<br>{}, {} {}<br>{}",
            billing.recipient_name,
            billing.street_address,
            billing.city,
            billing.state,
            billing.zip_code,
            billing.country
        );
        self.add("billing_address".to_string(), billing_str);
        
        // Add individual address fields for HTML template
        self.add("customer_name".to_string(), shipping.recipient_name.clone());
        self.add("shipping_street".to_string(), shipping.street_address.clone());
        self.add("shipping_city_state_zip".to_string(), format!("{}, {} {}", shipping.city, shipping.state, shipping.zip_code));
        self.add("shipping_country".to_string(), shipping.country.clone());
        
        self.add("billing_company".to_string(), billing.recipient_name.clone());
        self.add("billing_street".to_string(), billing.street_address.clone());
        self.add("billing_city".to_string(), billing.city.clone());
        self.add("billing_country".to_string(), billing.country.clone());
    }
    
    pub fn add_company_info(&mut self, company_name: &str, support_email: &str) {
        self.add("company_name".to_string(), company_name.to_string());
        self.add("support_email".to_string(), support_email.to_string());
    }
    
    pub fn add_shipping(&mut self, cost: f64, method: &str) {
        self.add("shipping_cost".to_string(), format!("{:.2}", cost));
        self.add("shipping_method".to_string(), method.to_string());
    }
    
    pub fn add_tax(&mut self, tax_amount: f64) {
        self.add("tax".to_string(), format!("{:.2}", tax_amount));
        self.add("tax_percent".to_string(), "0".to_string()); // For displaying "Tax (0%)"
    }
    
    pub fn add_totals(&mut self, order: &crate::order::Order) {
        self.add("subtotal".to_string(), order.subtotal.to_string());
        self.add("tax".to_string(), order.tax_total.to_string());
        self.add("shipping_cost".to_string(), order.shipping_total.to_string());
        self.add("order_total".to_string(), order.total.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_template_rendering() {
        let template = NotificationTemplate::load("order_confirmation").unwrap();
        let mut vars = TemplateVariables::new();
        vars.add("customer_name".to_string(), "John Doe".to_string());
        vars.add("order_number".to_string(), "ORD-12345".to_string());
        vars.add("order_total".to_string(), "99.99".to_string());
        vars.add("item_count".to_string(), "3".to_string());
        
        let rendered = template.render(&vars).unwrap();
        assert!(rendered.contains("John Doe"));
        assert!(rendered.contains("ORD-12345"));
        assert!(rendered.contains("$99.99"));
    }
    
    #[test]
    fn test_html_template_loading() {
        let template = NotificationTemplate::load("order_confirmation_html").unwrap();
        assert!(template.html_body.is_some());
        
        let html = template.html_body.unwrap();
        assert!(html.contains("R COMMERCE"));
        assert!(html.contains("{{ order_number }}"));
    }
}