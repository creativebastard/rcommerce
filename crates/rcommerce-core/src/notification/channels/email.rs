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
        
        log::info!(
            "Sending email from {} to {}: {}",
            self.from_address,
            notification.recipient.address,
            notification.subject
        );
        
        // Simulate sending
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        
        Ok(())
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