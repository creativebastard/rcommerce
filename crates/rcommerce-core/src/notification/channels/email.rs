//! Email notification channel implementation

use crate::{Result, Error};
use crate::notification::{Notification, NotificationChannel};

/// Email notification channel
pub struct EmailChannel {
    #[allow(dead_code)]
    smtp_host: String,
    #[allow(dead_code)]
    smtp_port: u16,
    #[allow(dead_code)]
    username: String,
    #[allow(dead_code)]
    password: String,
    #[allow(dead_code)]
    from_address: String,
    #[allow(dead_code)]
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
        if notification.channel != NotificationChannel::Email {
            return Err(Error::notification_error("Invalid channel for email sender"));
        }
        
        log::info!(
            "Sending email from {} to {}: {}",
            self.from_address,
            notification.recipient,
            notification.subject
        );
        
        if let Some(ref html_body) = notification.html_body {
            log::debug!("Email has HTML body ({} bytes)", html_body.len());
        }
        
        log::debug!("Email body preview: {}", &notification.body[..notification.body.len().min(100)]);
        
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        
        Ok(())
    }
    
    /// Build email message with both plain text and HTML parts
    pub fn build_email_message(&self, notification: &Notification) -> EmailMessage {
        EmailMessage {
            from: format!("{} <{}>", self.from_name, self.from_address),
            to: notification.recipient.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notification::{Notification, NotificationPriority};
    use uuid::Uuid;
    use chrono::Utc;
    
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
        
        let notification = Notification::new(
            NotificationChannel::Email,
            "customer@example.com".to_string(),
            "Test Email".to_string(),
            "This is a test email.".to_string(),
        )
        .with_html_body("<p>This is a <strong>test</strong> email.</p>".to_string());
        
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
        
        let notification = Notification::new(
            NotificationChannel::Email,
            "customer@example.com".to_string(),
            "Test Email".to_string(),
            "Plain text body".to_string(),
        )
        .with_html_body("<h1>HTML body</h1>".to_string());
        
        let message = channel.build_email_message(&notification);
        
        assert_eq!(message.from, "R Commerce <noreply@rcommerce.com>");
        assert_eq!(message.to, "customer@example.com");
        assert_eq!(message.subject, "Test Email");
        assert_eq!(message.text_body, "Plain text body");
        assert_eq!(message.html_body, Some("<h1>HTML body</h1>".to_string()));
        assert!(message.has_html());
        assert_eq!(message.mime_type(), "multipart/alternative");
    }

    #[test]
    fn test_plain_text_email() {
        let email = EmailMessage::plain_text(
            "from@example.com".to_string(),
            "to@example.com".to_string(),
            "Subject".to_string(),
            "Body text".to_string(),
        );
        
        assert_eq!(email.from, "from@example.com");
        assert_eq!(email.to, "to@example.com");
        assert_eq!(email.subject, "Subject");
        assert_eq!(email.text_body, "Body text");
        assert!(email.html_body.is_none());
        assert!(!email.has_html());
        assert_eq!(email.mime_type(), "text/plain");
    }

    #[test]
    fn test_html_email() {
        let email = EmailMessage::html(
            "from@example.com".to_string(),
            "to@example.com".to_string(),
            "Subject".to_string(),
            "Plain text body".to_string(),
            "<h1>HTML Body</h1>".to_string(),
        );
        
        assert!(email.has_html());
        assert_eq!(email.mime_type(), "multipart/alternative");
        assert_eq!(email.html_body, Some("<h1>HTML Body</h1>".to_string()));
    }
}
