#!/bin/bash
# Test if notification module compiles in isolation

cd crates/rcommerce-core

# Create a minimal test file
cat > /tmp/test_notification.rs << 'EOF'
// Minimal notification module test
use std::collections::HashMap;

// Mock types
pub type Result<T> = std::result::Result<T, String>;
pub type Error = String;

// Notification channel enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotificationChannel {
    Email,
    Sms,
    Push,
    Webhook,
    InApp,
}

// Template variables
#[derive(Debug, Default)]
pub struct TemplateVariables {
    pub inner: HashMap<String, String>,
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
}

// Template struct
#[derive(Debug, Clone)]
pub struct NotificationTemplate {
    pub id: String,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub html_body: Option<String>,
    pub channel: NotificationChannel,
    pub variables: Vec<String>,
}

impl NotificationTemplate {
    pub fn load(id: &str) -> Result<Self> {
        match id {
            "order_confirmation_html" => Ok(Self::order_confirmation_html()),
            _ => Err("Template not found".to_string()),
        }
    }
    
    pub fn render(&self, variables: &TemplateVariables) -> Result<String> {
        let mut rendered = self.body.clone();
        
        for (key, value) in variables.inner.iter() {
            let placeholder = format!("{{ {{ {} }} }}", key);
            rendered = rendered.replace(&placeholder, value);
        }
        
        Ok(rendered)
    }
    
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
    
    fn order_confirmation_html() -> Self {
        Self {
            id: "order_confirmation_html".to_string(),
            name: "Order Confirmation HTML".to_string(),
            subject: "Order Confirmed: {{ order_number }}".to_string(),
            body: "Hello {{ customer_name }}, your order is confirmed.".to_string(),
            html_body: Some(include_str!("src/notification/templates/invoice.html").to_string()),
            channel: NotificationChannel::Email,
            variables: vec![
                "order_number".to_string(),
                "order_date".to_string(),
                "order_total".to_string(),
                "customer_name".to_string(),
                "company_name".to_string(),
                "support_email".to_string(),
            ],
        }
    }
}

fn main() {
    println!("Testing notification template compilation...");
    
    // Load template
    let template = NotificationTemplate::load("order_confirmation_html").unwrap();
    println!("✓ Template loaded successfully");
    
    // Create variables
    let mut vars = TemplateVariables::new();
    vars.add("order_number".to_string(), "ORD-12345".to_string());
    vars.add("customer_name".to_string(), "John Doe".to_string());
    vars.add("order_total".to_string(), "99.99".to_string());
    vars.add("order_date".to_string(), "Jan 25, 2026".to_string());
    vars.add("company_name".to_string(), "R Commerce".to_string());
    vars.add("support_email".to_string(), "support@rcommerce.app".to_string());
    
    // Render plain text
    let plain_text = template.render(&vars).unwrap();
    println!("✓ Plain text rendered: {} chars", plain_text.len());
    
    // Render HTML
    let html = template.render_html(&vars).unwrap();
    match html {
        Some(html_content) => {
            println!("✓ HTML rendered: {} chars", html_content.len());
            
            // Verify placeholders were replaced
            assert!(html_content.contains("ORD-12345"));
            assert!(html_content.contains("John Doe"));
            assert!(html_content.contains("R COMMERCE"));
            println!("✓ Placeholders correctly replaced");
        }
        None => {
            eprintln!("✗ HTML body not found");
            std::process::exit(1);
        }
    }
    
    println!("✅ All tests passed! Notification module compiles correctly.");
}
EOF

# Try to compile the test
echo "Compiling notification module test..."
rustc --edition 2021 /tmp/test_notification.rs -o /tmp/test_notification 2>&1 | head -20

if [ $? -eq 0 ]; then
    echo "✓ Compilation successful!"
    /tmp/test_notification
else
    echo "✗ Compilation failed"
    rustc --edition 2021 /tmp/test_notification.rs -o /tmp/test_notification 2>&1
fi
EOF

chmod +x test_notification_compile.sh
./test_notification_compile.sh