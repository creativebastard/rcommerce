//! WebSocket connection management
//!
//! This module provides connection lifecycle management, tracking, and
//! efficient per-connection state handling.

use crate::websocket::{
    WebSocketMessage, MessageType, WebSocketConfig, MessageRateLimiter, AuthToken,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::Arc,
    time::Instant,
};
use tokio::{
    net::TcpStream,
    sync::mpsc::{self, UnboundedSender, UnboundedReceiver},
    time::{self, Duration},
};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{
        protocol::{CloseFrame, frame::coding::CloseCode},
        Message,
    },
    WebSocketStream,
};
use tracing::{info, warn, error, debug};
use uuid::Uuid;

/// Unique connection identifier
pub type ConnectionId = Uuid;

/// User identifier (extracted from auth)
pub type UserId = Uuid;

/// Connection state tracking
#[derive(Debug, Clone)]
pub struct WebSocketConnection {
    /// Unique connection ID
    pub id: ConnectionId,
    
    /// User ID (if authenticated)
    pub user_id: Option<UserId>,
    
    /// Client IP address
    pub client_ip: String,
    
    /// Connection start time
    pub connected_at: Instant,
    
    /// Last activity timestamp
    pub last_activity: Arc<std::sync::RwLock<Instant>>,
    
    /// Subscribed topics
    pub subscriptions: Arc<std::sync::RwLock<HashSet<String>>>,
    
    /// Authenticated status
    pub is_authenticated: Arc<std::sync::AtomicBool>,
    
    /// Connection closed flag
    pub is_closed: Arc<std::sync::AtomicBool>,
    
    /// Total messages sent
    pub messages_sent: Arc<std::sync::AtomicU64>,
    
    /// Total messages received
    pub messages_received: Arc<std::sync::AtomicU64>,
    
    /// Last ping timestamp
    pub last_ping: Arc<std::sync::RwLock<Option<Instant>>>,
    
    /// Rate limiter for messages
    pub message_rate_limiter: Arc<MessageRateLimiter>,
    
    /// Outgoing message sender
    pub tx: UnboundedSender<WebSocketMessage>,
}

impl WebSocketConnection {
    /// Create a new WebSocket connection
    pub fn new(
        client_ip: String,
        config: &WebSocketConfig,
    ) -> Self {
        let (tx, _rx) = mpsc::unbounded_channel();
        let now = Instant::now();
        
        Self {
            id: ConnectionId::new_v4(),
            user_id: None,
            client_ip,
            connected_at: now,
            last_activity: Arc::new(std::sync::RwLock::new(now)),
            subscriptions: Arc::new(std::sync::RwLock::new(HashSet::new())),
            is_authenticated: Arc::new(std::sync::AtomicBool::new(false)),
            is_closed: Arc::new(std::sync::AtomicBool::new(false)),
            messages_sent: Arc::new(std::sync::AtomicU64::new(0)),
            messages_received: Arc::new(std::sync::AtomicU64::new(0)),
            last_ping: Arc::new(std::sync::RwLock::new(None)),
            message_rate_limiter: Arc::new(MessageRateLimiter::new(
                config.max_messages_per_minute,
                config.max_message_size,
            )),
            tx,
        }
    }
    
    /// Update last activity timestamp
    pub fn update_activity(&self) {
        if let Ok(mut last) = self.last_activity.write() {
            *last = Instant::now();
        }
    }
    
    /// Get duration since last activity
    pub fn time_since_activity(&self) -> Duration {
        self.last_activity.read()
            .map(|last| last.elapsed())
            .unwrap_or(Duration::from_secs(0))
    }
    
    /// Mark connection as authenticated
    pub fn set_authenticated(&self, user_id: UserId) {
        self.is_authenticated.store(true, std::sync::atomic::Ordering::SeqCst);
        self.user_id = Some(user_id);
        self.update_activity();
        info!("Connection authenticated: id={}, user_id={}", self.id, user_id);
    }
    
    /// Check if connection is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.is_authenticated.load(std::sync::atomic::Ordering::Relaxed)
    }
    
    /// Subscribe to a topic
    pub fn subscribe(&self, topic: String) -> Result<usize, SubscribeError> {
        // Check subscription limit
        let current = self.subscriptions.read()
            .map(|subs| subs.len())
            .unwrap_or(0);
        
        if current >= 50 { // Configurable limit
            return Err(SubscribeError::TooManySubscriptions);
        }
        
        // Add subscription
        let mut subscriptions = self.subscriptions.write().unwrap();
        subscriptions.insert(topic.clone());
        let count = subscriptions.len();
        
        self.update_activity();
        debug!("Subscribed: connection_id={}, topic={}, count={}", self.id, topic, count);
        
        Ok(count)
    }
    
    /// Unsubscribe from a topic
    pub fn unsubscribe(&self, topic: &str) -> Result<usize, UnsubscribeError> {
        let mut subscriptions = self.subscriptions.write().unwrap();
        subscriptions.remove(topic);
        let count = subscriptions.len();
        
        self.update_activity();
        debug!("Unsubscribed: connection_id={}, topic={}, count={}", self.id, topic, count);
        
        Ok(count)
    }
    
    /// Check if subscribed to a topic
    pub fn is_subscribed(&self, topic: &str) -> bool {
        self.subscriptions.read()
            .map(|subs| subs.contains(topic))
            .unwrap_or(false)
    }
    
    /// Get all subscriptions
    pub fn get_subscriptions(&self) -> Vec<String> {
        self.subscriptions.read()
            .map(|subs| subs.iter().cloned().collect())
            .unwrap_or_default()
    }
    
    /// Queue a message for sending
    pub fn send(&self, message: WebSocketMessage) -> Result<(), SendError> {
        // Check if connection is closed
        if self.is_closed.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(SendError::ConnectionClosed);
        }
        
        // Validate message size
        let size = message.estimated_size();
        if size > 1024 * 1024 { // 1MB limit
            return Err(SendError::MessageTooLarge(size));
        }
        
        // Rate limit check
        if message.message_type.is_rate_limited() {
            self.message_rate_limiter.check_allowed()?;
        }
        
        // Send message
        match self.tx.send(message) {
            Ok(_) => {
                self.messages_sent.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                self.update_activity();
                Ok(())
            }
            Err(_) => Err(SendError::ChannelClosed),
        }
    }
    
    /// Queue multiple messages
    pub fn send_batch(&self, messages: Vec<WebSocketMessage>) -> Result<(), SendError> {
        for message in messages {
            self.send(message)?;
        }
        Ok(())
    }
    
    /// Mark connection as closed
    pub fn close(&self) {
        self.is_closed.store(true, std::sync::atomic::Ordering::SeqCst);
        info!("Connection closed: id={}, user_id={:?}", self.id, self.user_id);
    }
    
    /// Check if connection is closed
    pub fn is_closed(&self) -> bool {
        self.is_closed.load(std::sync::atomic::Ordering::Relaxed)
    }
    
    /// Get connection statistics
    pub fn stats(&self) -> ConnectionStats {
        ConnectionStats {
            connection_id: self.id,
            user_id: self.user_id,
            client_ip: self.client_ip.clone(),
            connected_at: self.connected_at,
            duration: self.connected_at.elapsed(),
            last_activity: *self.last_activity.read().unwrap(),
            is_authenticated: self.is_authenticated(),
            is_closed: self.is_closed(),
            messages_sent: self.messages_sent.load(std::sync::atomic::Ordering::Relaxed),
            messages_received: self.messages_received.load(std::sync::atomic::Ordering::Relaxed),
            subscriptions: self.get_subscriptions(),
        }
    }
    
    /// Record received message
    pub fn record_received_message(&self) {
        self.messages_received.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.update_activity();
    }
}

/// Statistics for a WebSocket connection
#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub connection_id: ConnectionId,
    pub user_id: Option<UserId>,
    pub client_ip: String,
    pub connected_at: Instant,
    pub duration: Duration,
    pub last_activity: Instant,
    pub is_authenticated: bool,
    pub is_closed: bool,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub subscriptions: Vec<String>,
}

impl ConnectionStats {
    /// Format as human-readable string
    pub fn format(&self) -> String {
        format!(
            "Connection[id={}, user={:?}, ip={}, duration={:.1}s, sent={}, recv={}, subs={}]",
            self.connection_id,
            self.user_id,
            self.client_ip,
            self.duration.as_secs_f32(),
            self.messages_sent,
            self.messages_received,
            self.subscriptions.len()
        )
    }
}

/// Errors that can occur when subscribing
#[derive(Debug, thiserror::Error)]
pub enum SubscribeError {
    #[error("Too many subscriptions (max 50)")]
    TooManySubscriptions,
    
    #[error("Connection is closed")]
    ConnectionClosed,
}

/// Errors that can occur when unsubscribing
#[derive(Debug, thiserror::Error)]
pub enum UnsubscribeError {
    #[error("Topic not found in subscriptions")]
    TopicNotFound,
    
    #[error("Connection is closed")]
    ConnectionClosed,
}

/// Errors that can occur when sending messages
#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("Connection is closed")]
    ConnectionClosed,
    
    #[error("Message too large: {0} bytes (max 1MB)")]
    MessageTooLarge(usize),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimited(String),
    
    #[error("Send channel closed")]
    ChannelClosed,
}

impl From<crate::websocket::rate_limit::RateLimitError> for SendError {
    fn from(err: crate::websocket::rate_limit::RateLimitError) -> Self {
        SendError::RateLimited(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::websocket::WebSocketConfig;
    
    #[test]
    fn test_connection_creation() {
        let config = WebSocketConfig::default();
        let conn = WebSocketConnection::new("192.168.1.1".to_string(), &config);
        
        assert_eq!(conn.is_authenticated(), false);
        assert_eq!(conn.is_closed(), false);
        assert_eq!(conn.user_id, None);
        assert_eq!(conn.client_ip, "192.168.1.1");
    }
    
    #[test]
    fn test_connection_authentication() {
        let config = WebSocketConfig::default();
        let conn = WebSocketConnection::new("192.168.1.1".to_string(), &config);
        let user_id = UserId::new_v4();
        
        conn.set_authenticated(user_id);
        assert_eq!(conn.is_authenticated(), true);
        assert_eq!(conn.user_id, Some(user_id));
    }
    
    #[test]
    fn test_connection_subscriptions() {
        let config = WebSocketConfig::default();
        let conn = WebSocketConnection::new("192.168.1.1".to_string(), &config);
        
        // Subscribe to topics
        assert!(conn.subscribe("orders".to_string()).is_ok());
        assert!(conn.subscribe("inventory".to_string()).is_ok());
        
        // Check subscriptions
        assert_eq!(conn.get_subscriptions().len(), 2);
        assert!(conn.is_subscribed("orders"));
        assert!(conn.is_subscribed("inventory"));
        assert!(!conn.is_subscribed("payments"));
        
        // Unsubscribe
        assert!(conn.unsubscribe("orders").is_ok());
        assert_eq!(conn.get_subscriptions().len(), 1);
        assert!(!conn.is_subscribed("orders"));
    }
    
    #[test]
    fn test_connection_subscribe_limit() {
        let config = WebSocketConfig::default();
        let conn = WebSocketConnection::new("192.168.1.1".to_string(), &config);
        
        // Try to subscribe to too many topics
        for i in 0..55 {
            let result = conn.subscribe(format!("topic-{}", i));
            if i >= 50 {
                assert!(matches!(result, Err(SubscribeError::TooManySubscriptions)));
            } else {
                assert!(result.is_ok());
            }
        }
    }
    
    #[test]
    fn test_connection_stats() {
        let config = WebSocketConfig::default();
        let conn = WebSocketConnection::new("192.168.1.1".to_string(), &config);
        
        let stats = conn.stats();
        assert_eq!(stats.connection_id, conn.id);
        assert_eq!(stats.messages_sent, 0);
        assert_eq!(stats.messages_received, 0);
        assert_eq!(stats.subscriptions.len(), 0);
        assert!(stats.duration.as_millis() < 100); // Should be very quick
    }
    
    #[test]
    fn test_connection_activity() {
        let config = WebSocketConfig::default();
        let conn = WebSocketConnection::new("192.168.1.1".to_string(), &config);
        
        let before = conn.time_since_activity();
        std::thread::sleep(Duration::from_millis(10));
        conn.update_activity();
        let after = conn.time_since_activity();
        
        assert!(before < after);
    }
}