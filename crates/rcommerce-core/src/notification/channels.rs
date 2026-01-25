//! Notification channel implementations

use async_trait::async_trait;
use crate::notification::types::{NotificationMessage, NotificationResult};
use crate::Result;

/// Email notification channel
pub struct EmailChannel;

#[async_trait]
pub trait NotificationChannel: Send + Sync {
    async fn send(&self, message: &NotificationMessage) -> Result<NotificationResult>;
    fn channel_name(&self) -> &'static str;
}

#[async_trait]
impl NotificationChannel for EmailChannel {
    async fn send(&self, message: &NotificationMessage) -> Result<NotificationResult> {
        // In production, integrate with email service (SendGrid, AWS SES)
        tracing::info!(
            "Sending email to: {}, subject: {}",
            message.recipient,
            message.subject.as_deref().unwrap_or("(no subject)")
        );
        
        Ok(NotificationResult {
            success: true,
            channel: "email".to_string(),
            message_id: Some(format!("msg_{}", uuid::Uuid::new_v4())),
            error: None,
        })
    }
    
    fn channel_name(&self) -> &'static str {
        "email"
    }
}

/// SMS notification channel
pub struct SmsChannel;

#[async_trait]
impl NotificationChannel for SmsChannel {
    async fn send(&self, message: &NotificationMessage) -> Result<NotificationResult> {
        tracing::info!(
            "Sending SMS to: {}, body length: {}",
            message.recipient,
            message.body.len()
        );
        
        Ok(NotificationResult {
            success: true,
            channel: "sms".to_string(),
            message_id: Some(format!("sms_{}", uuid::Uuid::new_v4())),
            error: None,
        })
    }
    
    fn channel_name(&self) -> &'static str {
        "sms"
    }
}

/// Push notification channel
pub struct PushChannel;

#[async_trait]
impl NotificationChannel for PushChannel {
    async fn send(&self, message: &NotificationMessage) -> Result<NotificationResult> {
        tracing::info!(
            "Sending push notification to: {}",
            message.recipient
        );
        
        Ok(NotificationResult {
            success: true,
            channel: "push".to_string(),
            message_id: Some(format!("push_{}", uuid::Uuid::new_v4())),
            error: None,
        })
    }
    
    fn channel_name(&self) -> &'static str {
        "push"
    }
}

/// Webhook notification channel
pub struct WebhookChannel;

#[async_trait]
impl NotificationChannel for WebhookChannel {
    async fn send(&self, message: &NotificationMessage) -> Result<NotificationResult> {
        tracing::info!(
            "Sending webhook to: {}",
            message.recipient
        );
        
        Ok(NotificationResult {
            success: true,
            channel: "webhook".to_string(),
            message_id: Some(format!("webhook_{}", uuid::Uuid::new_v4())),
            error: None,
        })
    }
    
    fn channel_name(&self) -> &'static str {
        "webhook"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notification::types::TemplateVariables;

    #[tokio::test]
    async fn test_email_channel() {
        let channel = EmailChannel;
        let message = NotificationMessage {
            template_id: "order_confirmation".to_string(),
            recipient: "customer@example.com".to_string(),
            subject: Some("Order Confirmed".to_string()),
            body: "Thank you for your order!".to_string(),
            variables: TemplateVariables::new(),
        };
        
        let result = channel.send(&message).await.unwrap();
        assert!(result.success);
        assert_eq!(result.channel, "email");
        assert!(result.message_id.is_some());
    }

    #[tokio::test]
    async fn test_sms_channel() {
        let channel = SmsChannel;
        let message = NotificationMessage {
            template_id: "shipped".to_string(),
            recipient: "+1234567890".to_string(),
            subject: None,
            body: "Your order has shipped!".to_string(),
            variables: TemplateVariables::new(),
        };
        
        let result = channel.send(&message).await.unwrap();
        assert!(result.success);
        assert_eq!(result.channel, "sms");
    }

    #[tokio::test]
    async fn test_push_channel() {
        let channel = PushChannel;
        let message = NotificationMessage {
            template_id: "in_stock".to_string(),
            recipient: "device_token_123".to_string(),
            subject: None,
            body: "Item back in stock".to_string(),
            variables: TemplateVariables::new(),
        };
        
        let result = channel.send(&message).await.unwrap();
        assert!(result.success);
        assert_eq!(result.channel, "push");
    }
}
