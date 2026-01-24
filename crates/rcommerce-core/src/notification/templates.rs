use std::collections::HashMap;

use crate::{{Result, Error}};

/// Notification template with placeholders
#[derive(Debug, Clone)]
pub struct NotificationTemplate {
    pub id: String,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub channel: crate::notification::NotificationChannel,
    pub variables: Vec<String>,
}

impl NotificationTemplate {
    /// Load from file or database
    pub fn load(id: &str) -> Result<Self> {
        // TODO: Load from database or file system
        match id {
            "order_confirmation" => Ok(Self::order_confirmation()),
            "order_shipped" => Ok(Self::order_shipped()),
            "low_stock_alert" => Ok(Self::low_stock_alert()),
            _ => Err(Error::not_found("Template not found")),
        }
    }
    
    /// Render template with variables
    pub fn render(&self, variables: &TemplateVariables) -> Result<String> {
        let mut rendered = self.body.clone();
        
        for (key, value) in variables.inner.iter() {
            let placeholder = format!("{{{{ {} }}}}", key);
            rendered = rendered.replace(&placeholder, value);
        }
        
        Ok(rendered)
    }
    
    /// Order confirmation template
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
            channel: crate::notification::NotificationChannel::Email,
            variables: vec![
                "customer_name".to_string(),
                "order_number".to_string(),
                "order_total".to_string(),
                "item_count".to_string(),
            ],
        }
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
}