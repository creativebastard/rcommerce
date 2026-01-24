pub mod channels;
pub mod templates;
pub mod service;

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Result, Error};

pub use service::NotificationService;
pub use templates::{NotificationTemplate, TemplateVariables};

/// Notification channels
#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(type_name = "notification_channel", rename_all = "snake_case")]
pub enum NotificationChannel {
    Email,
    Sms,
    Push,
    Webhook,
    InApp,
}

/// Notification priority
#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(type_name = "notification_priority", rename_all = "snake_case")]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Urgent,
}

impl Default for NotificationPriority {
    fn default() -> Self {
        NotificationPriority::Normal
    }
}

/// Main notification struct
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: Uuid,
    pub channel: NotificationChannel,
    pub recipient: Recipient,
    pub subject: String,
    pub body: String,
    pub priority: NotificationPriority,
    pub metadata: serde_json::Value,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Notification recipient
#[derive(Debug, Clone)]
pub struct Recipient {
    pub id: Option<Uuid>,
    pub channel: NotificationChannel,
    pub address: String,
    pub name: Option<String>,
}

impl Recipient {
    pub fn email(address: String, name: Option<String>) -> Self {
        Self {
            id: None,
            channel: NotificationChannel::Email,
            address,
            name,
        }
    }
    
    pub fn sms(address: String) -> Self {
        Self {
            id: None,
            channel: NotificationChannel::Sms,
            address,
            name: None,
        }
    }
    
    pub fn webhook(url: String) -> Self {
        Self {
            id: None,
            channel: NotificationChannel::Webhook,
            address: url,
            name: None,
        }
    }
}

/// Delivery status tracking
#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(type_name = "delivery_status", rename_all = "snake_case")]
pub enum DeliveryStatus {
    Pending,
    Queued,
    Sent,
    Delivered,
    Failed,
    Bounced,
    Opened,
    Clicked,
}

impl Default for DeliveryStatus {
    fn default() -> Self {
        DeliveryStatus::Pending
    }
}

/// Delivery attempt record
#[derive(Debug, Clone)]
pub struct DeliveryAttempt {
    pub notification_id: Uuid,
    pub attempt_number: i32,
    pub status: DeliveryStatus,
    pub provider: String,
    pub provider_response: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

impl DeliveryAttempt {
    pub fn new(notification_id: Uuid, provider: String) -> Self {
        Self {
            notification_id,
            attempt_number: 1,
            status: DeliveryStatus::Pending,
            provider,
            provider_response: None,
            sent_at: None,
            delivered_at: None,
            error_message: None,
        }
    }
    
    pub fn mark_sent(&mut self) {
        self.status = DeliveryStatus::Sent;
        self.sent_at = Some(Utc::now());
    }
    
    pub fn mark_delivered(&mut self) {
        self.status = DeliveryStatus::Delivered;
        self.delivered_at = Some(Utc::now());
    }
    
    pub fn mark_failed(&mut self, error: String) {
        self.status = DeliveryStatus::Failed;
        self.error_message = Some(error);
    }
    
    pub fn retry(&self) -> Self {
        let mut retry = self.clone();
        retry.attempt_number += 1;
        retry.status = DeliveryStatus::Pending;
        retry.error_message = None;
        retry
    }
}

/// Notification preferences
#[derive(Debug, Clone)]
pub struct NotificationPreferences {
    pub customer_id: Uuid,
    pub order_notifications: bool,
    pub shipping_notifications: bool,
    pub promotional_emails: bool,
    pub low_stock_alerts: bool,
    pub wishlist_notifications: bool,
    pub timezone: String,
}

impl Default for NotificationPreferences {
    fn default() -> Self {
        Self {
            customer_id: Uuid::nil(),
            order_notifications: true,
            shipping_notifications: true,
            promotional_emails: false,
            low_stock_alerts: false,
            wishlist_notifications: false,
            timezone: "UTC".to_string(),
        }
    }
}

/// Rate limiting for notifications
#[derive(Debug, Clone)]
pub struct RateLimit {
    pub max_per_minute: u32,
    pub max_per_hour: u32,
    pub max_per_day: u32,
    pub current_count: u32,
    pub window_start: DateTime<Utc>,
}

impl RateLimit {
    pub fn new(max_per_minute: u32, max_per_hour: u32, max_per_day: u32) -> Self {
        Self {
            max_per_minute,
            max_per_hour,
            max_per_day,
            current_count: 0,
            window_start: Utc::now(),
        }
    }
    
    pub fn can_send(&self) -> bool {
        self.current_count < self.max_per_day
    }
    
    pub fn increment(&mut self) {
        self.current_count += 1;
    }
    
    pub fn reset(&mut self) {
        self.current_count = 0;
        self.window_start = Utc::now();
    }
}

impl Error {
    pub fn notification_error<T: Into<String>>(msg: T) -> Self {
        Error::Notification(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_recipient_creation() {
        let email = Recipient::email("test@example.com".to_string(), Some("Test User".to_string()));
        assert_eq!(email.channel, NotificationChannel::Email);
        assert_eq!(email.address, "test@example.com");
        assert_eq!(email.name, Some("Test User".to_string()));
        
        let sms = Recipient::sms("+1234567890".to_string());
        assert_eq!(sms.channel, NotificationChannel::Sms);
        assert_eq!(sms.address, "+1234567890");
    }
    
    #[test]
    fn test_delivery_attempt() {
        let attempt = DeliveryAttempt::new(Uuid::new_v4(), "test_provider".to_string());
        assert_eq!(attempt.status, DeliveryStatus::Pending);
        assert_eq!(attempt.attempt_number, 1);
        
        let mut attempt = attempt;
        attempt.mark_sent();
        assert_eq!(attempt.status, DeliveryStatus::Sent);
        assert!(attempt.sent_at.is_some());
    }
    
    #[test]
    fn test_rate_limit() {
        let mut limit = RateLimit::new(10, 100, 1000);
        assert!(limit.can_send());
        
        limit.increment();
        assert_eq!(limit.current_count, 1);
        assert!(limit.can_send());
        
        limit.current_count = 1000;
        assert!(!limit.can_send());
    }
}