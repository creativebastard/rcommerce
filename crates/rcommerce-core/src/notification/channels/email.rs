use async_trait::async_trait;

use crate::{Result, Error};
use crate::notification::Notification;

/// Email notification channel
pub struct EmailChannel {
    smtp_host: String,
    smtp_port: u16,
    username: String,
    password: String,
    from_address: String,
    from_name: String,
}

impl EmailChannel {
    pub fn new(
        smtp_host: String,
        smtp_port: u16,
        username: String,
        password: String,
        from_address: String,
        from_name: String,
    ) -> Self {
        Self {
            smtp_host,
            smtp_port,
            username,
            password,
            from_address,
            from_name,
        }
    }
    
    /// Send an email notification
    pub async fn send(&self, notification: &Notification) -> Result<()> {
        if notification.channel != crate::notification::NotificationChannel::Email {
            return Err(Error::notification_error("Invalid channel for email sender"));
        }
        
        // TODO: Implement actual SMTP sending using lettre or similar crate
        // For now, we'll simulate the email structure
        
        log::info!(
            "Sending email from {} to {}: {}",
            self.from_address,
            notification.recipient.address,
            notification.subject
        );
        
        // Log email details
        if let Some(ref html_body) = notification.html_body {
            log::debug!("Email has HTML body ({} bytes)", html_body.len());
        } else {
            log::debug!("Email has plain text body only");
        }
        
        log::debug!("Email body preview: {}", &notification.body[..notification.body.len().min(100)]);
        
        // Simulate sending
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        
        Ok(())
    }
    
    /// Build email message with both plain text and HTML parts
    pub fn build_email_message(&self, notification: &Notification) -> EmailMessage {
        EmailMessage {
            from: format!("{} <{}>", self.from_name, self.from_address),
            to: format!("{} <{}>", 
                notification.recipient.name.as_deref().unwrap_or(""), 
                notification.recipient.address
            ),
            subject: notification.subject.clone(),
            text_body: notification.body.clone(),
            html_body: notification.html_body.clone(),
        }
    }
}

/// Represents an email message with both plain text and HTML parts
#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub text_body: String,
    pub html_body: Option<String>,
}

impl EmailMessage {
    /// Create a plain text email
    pub fn plain_text(from: String, to: String, subject: String, body: String) -> Self {
        Self {
            from,
            to,
            subject,
            text_body: body,
            html_body: None,
        }
    }
    
    /// Create an HTML email with plain text fallback
    pub fn html(from: String, to: String, subject: String, text_body: String, html_body: String) -> Self {
        Self {
            from,
            to,
            subject,
            text_body,
            html_body: Some(html_body),
        }
    }
    
    /// Check if this email has HTML content
    pub fn has_html(&self) -> bool {
        self.html_body.is_some()
    }
    
    /// Get the MIME type for the email
    pub fn mime_type(&self) -> &'static str {
        if self.has_html() {
            "multipart/alternative"
        } else {
            "text/plain"
        }
    }
}

#[async_trait::async_trait]
impl super::NotificationChannelProvider for EmailChannel {
    fn channel(&self) -> crate::notification::NotificationChannel {
        crate::notification::NotificationChannel::Email
    }
    
    async fn send_notification(&self, notification: &Notification) -> Result<()> {
        self.send(notification).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notification::{Notification, Recipient, NotificationChannel, NotificationPriority};
    use uuid::Uuid;
    
    #[tokio::test]
    async fn test_email_channel() {
        let channel = EmailChannel::new(
            "smtp.example.com".to_string(),
            587,
            "user@example.com".to_string(),
            "password".to_string(),
            "noreply@rcommerce.com".to_string(),
            "R Commerce".to_string(),
        );
        
        let notification = Notification {
            id: Uuid::new_v4(),
            channel: NotificationChannel::Email,
            recipient: Recipient::email("customer@example.com".to_string(), Some("John Doe".to_string())),
            subject: "Test Email".to_string(),
            body: "This is a test email.".to_string(),
            html_body: Some("<p>This is a <strong>test</strong> email.</p>".to_string()),
            priority: NotificationPriority::Normal,
            metadata: serde_json::json!({}),
            scheduled_at: None,
            created_at: Utc::now(),
        };
        
        let result = channel.send(&notification).await;
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_email_message_builder() {
        let channel = EmailChannel::new(
            "smtp.example.com".to_string(),
            587,
            "user@example.com".to_string(),
            "password".to_string(),
            "noreply@rcommerce.com".to_string(),
            "R Commerce".to_string(),
        );
        
        let notification = Notification {
            id: Uuid::new_v4(),
            channel: NotificationChannel::Email,
            recipient: Recipient::email("customer@example.com".to_string(), Some("John Doe".to_string())),
            subject: "Test Email".to_string(),
            body: "Plain text body".to_string(),
            html_body: Some("<h1>HTML body</h1>".to_string()),
            priority: NotificationPriority::Normal,
            metadata: serde_json::json!({}),
            scheduled_at: None,
            created_at: Utc::now(),
        };
        
        let message = channel.build_email_message(&notification);
        
        assert_eq!(message.from, "R Commerce <noreply@rcommerce.com>");
        assert_eq!(message.to, "John Doe <customer@example.com>");
        assert_eq!(message.subject, "Test Email");
        assert_eq!(message.text_body, "Plain text body");
        assert_eq!(message.html_body, Some("<h1>HTML body</h1>".to_string()));
        assert!(message.has_html());
        assert_eq!(message.mime_type(), "multipart/alternative");
    }
}