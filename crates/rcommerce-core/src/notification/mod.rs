pub mod channels;
pub mod templates;
pub mod service;
pub mod types;

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

pub use service::NotificationService;
pub use templates::{NotificationTemplate, TemplateVariables};
pub use types::{NotificationMessage, NotificationResult, DeliveryStatus, DeliveryAttempt, NotificationPriority, Notification, Recipient, NotificationPreferences};

/// Notification channels
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "notification_channel", rename_all = "snake_case")]
pub enum NotificationChannel {
    Email,
    Sms,
    Push,
    Webhook,
    InApp,
}

#[cfg(test)]
mod notification_tests {
    use super::*;
    
    #[test]
    fn test_notification_channel() {
        let channel = NotificationChannel::Email;
        assert_eq!(channel, NotificationChannel::Email);
    }
}
