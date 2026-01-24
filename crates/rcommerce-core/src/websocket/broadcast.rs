//! Message broadcasting system
//!
//! This module provides efficient message broadcasting to multiple
//! WebSocket connections using pub/sub pattern.

use crate::websocket::{WebSocketMessage, ConnectionId, WebSocketConnection};
use dashmap::DashMap;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, error, info};

/// Subscription topic
pub type Topic = String;

/// Subscription ID
pub type SubscriptionId = uuid::Uuid;

/// Broadcast manager for pub/sub messaging
pub struct BroadcastManager {
    /// Topic -> (ConnectionId -> Sender) mapping
    subscriptions: Arc<DashMap<Topic, HashMap<ConnectionId, UnboundedSender<WebSocketMessage>>>>,
    
    /// ConnectionId -> Topic -> SubscriptionId mapping (for cleanup)
    connection_topics: Arc<DashMap<ConnectionId, HashMap<Topic, SubscriptionId>>>,
}

impl BroadcastManager {
    /// Create a new broadcast manager
    pub fn new() -> Self {
        Self {
            subscriptions: Arc::new(DashMap::new()),
            connection_topics: Arc::new(DashMap::new()),
        }
    }
    
    /// Subscribe a connection to a topic
    pub fn subscribe(
        &self,
        topic: Topic,
        connection_id: ConnectionId,
        sender: UnboundedSender<WebSocketMessage>,
    ) -> Result<SubscriptionId, SubscribeError> {
        // Generate subscription ID
        let subscription_id = SubscriptionId::new_v4();
        
        // Add to subscriptions map
        let mut topic_subs = self.subscriptions
            .entry(topic.clone())
            .or_insert_with(HashMap::new);
        topic_subs.insert(connection_id, sender);
        drop(topic_subs);
        
        // Add to connection_topics map
        let mut conn_topics = self.connection_topics
            .entry(connection_id)
            .or_insert_with(HashMap::new);
        conn_topics.insert(topic, subscription_id);
        drop(conn_topics);
        
        debug!(
            "Subscribed: connection_id={}, topic={}, subscription_id={}",
            connection_id, topic, subscription_id
        );
        
        Ok(subscription_id)
    }
    
    /// Unsubscribe a connection from a topic
    pub fn unsubscribe(
        &self,
        topic: &Topic,
        connection_id: ConnectionId,
    ) -> Result<(), UnsubscribeError> {
        // Remove from subscriptions map
        if let Some(mut topic_subs) = self.subscriptions.get_mut(topic) {
            topic_subs.remove(&connection_id);
            if topic_subs.is_empty() {
                drop(topic_subs);
                self.subscriptions.remove(topic);
            }
        }
        
        // Remove from connection_topics map
        if let Some(mut conn_topics) = self.connection_topics.get_mut(&connection_id) {
            conn_topics.remove(topic);
            if conn_topics.is_empty() {
                drop(conn_topics);
                self.connection_topics.remove(&connection_id);
            }
        }
        
        debug!(
            "Unsubscribed: connection_id={}, topic={}",
            connection_id, topic
        );
        
        Ok(())
    }
    
    /// Unsubscribe from all topics for a connection
    pub fn unsubscribe_all(&self, connection_id: ConnectionId) {
        if let Some(conn_topics) = self.connection_topics.get(&connection_id) {
            let topics: Vec<Topic> = conn_topics.keys().cloned().collect();
            drop(conn_topics);
            
            for topic in topics {
                let _ = self.unsubscribe(&topic, connection_id);
            }
        }
        
        self.connection_topics.remove(&connection_id);
    }
    
    /// Broadcast message to all subscribers of a topic
    pub fn broadcast_to_topic(&self, topic: &Topic, message: WebSocketMessage) -> usize {
        let mut sent = 0;
        
        if let Some(topic_subs) = self.subscriptions.get(topic) {
            let senders: Vec<_> = topic_subs
                .iter()
                .map(|(_, sender)| sender.clone())
                .collect();
            drop(topic_subs);
            
            for sender in senders {
                match sender.send(message.clone()) {
                    Ok(_) => sent += 1,
                    Err(_) => {
                        // Receiver dropped, connection probably closed
                        debug!("Failed to send to subscriber: topic={}", topic);
                    }
                }
            }
        }
        
        if sent > 0 {
            debug!("Broadcast: topic={}, recipients={}", topic, sent);
        }
        
        sent
    }
    
    /// Broadcast message to multiple topics
    pub fn broadcast_to_topics(&self, topics: &[Topic], message: WebSocketMessage) -> usize {
        let mut total_sent = 0;
        let mut sent_to = HashMap::new(); // Track to avoid duplicates
        
        for topic in topics {
            if let Some(topic_subs) = self.subscriptions.get(topic) {
                for (conn_id, sender) in topic_subs.iter() {
                    if !sent_to.contains_key(conn_id) {
                        match sender.send(message.clone()) {
                            Ok(_) => {
                                total_sent += 1;
                                sent_to.insert(*conn_id, ());
                            }
                            Err(_) => {
                                debug!("Failed to send to subscriber in topic: {}", topic);
                            }
                        }
                    }
                }
                drop(topic_subs);
            }
        }
        
        if total_sent > 0 {
            debug!("Broadcast to topics: count={}, recipients={}", topics.len(), total_sent);
        }
        
        total_sent
    }
    
    /// Send message to single connection
    pub fn send_to_connection(
        &self,
        connection_id: ConnectionId,
        message: WebSocketMessage,
    ) -> bool {
        // Try to find connection in any topic
        for entry in self.subscriptions.iter() {
            if let Some(sender) = entry.get(&connection_id) {
                match sender.send(message) {
                    Ok(_) => {
                        debug!("Sent to connection: id={}", connection_id);
                        return true;
                    }
                    Err(_) => {
                        debug!("Failed to send to connection: id={}", connection_id);
                        return false;
                    }
                }
            }
        }
        
        false
    }
    
    /// Get subscriber count for a topic
    pub fn subscriber_count(&self, topic: &Topic) -> usize {
        self.subscriptions
            .get(topic)
            .map(|subs| subs.len())
            .unwrap_or(0)
    }
    
    /// Get all topics with their subscriber counts
    pub fn topic_stats(&self) -> HashMap<Topic, usize> {
        let mut stats = HashMap::new();
        
        for entry in self.subscriptions.iter() {
            let (topic, subscribers) = entry.pair();
            stats.insert(topic.clone(), subscribers.len());
        }
        
        stats
    }
    
    /// Get topics for a connection
    pub fn get_connection_topics(&self, connection_id: ConnectionId) -> Vec<Topic> {
        self.connection_topics
            .get(&connection_id)
            .map(|topics| topics.keys().cloned().collect())
            .unwrap_or_default()
    }
}

impl Default for BroadcastManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for BroadcastManager {
    fn drop(&mut self) {
        info!("BroadcastManager dropped, {} topics, {} connections",
            self.subscriptions.len(),
            self.connection_topics.len()
        );
    }
}

/// Errors that can occur when subscribing
#[derive(Debug, thiserror::Error)]
pub enum SubscribeError {
    #[error("Topic name too long")]
    TopicTooLong,
    
    #[error("Invalid topic format")]
    InvalidTopic,
    
    #[error("Channel disconnected")]
    ChannelClosed,
}

/// Errors that can occur when unsubscribing
#[derive(Debug, thiserror::Error)]
pub enum UnsubscribeError {
    #[error("Not subscribed to topic")]
    NotSubscribed,
    
    #[error("Topic not found")]
    TopicNotFound,
}

/// Subscription handle for managing subscriptions
pub struct Subscription {
    /// Subscription ID
    pub id: SubscriptionId,
    
    /// Topic name
    pub topic: Topic,
    
    /// Connection ID
    pub connection_id: ConnectionId,
    
    /// Broadcast manager reference
    broadcast_manager: Arc<BroadcastManager>,
}

impl Subscription {
    /// Create a new subscription handle
    pub fn new(
        id: SubscriptionId,
        topic: Topic,
        connection_id: ConnectionId,
        broadcast_manager: Arc<BroadcastManager>,
    ) -> Self {
        Self {
            id,
            topic,
            connection_id,
            broadcast_manager,
        }
    }
    
    /// Unsubscribe from the topic
    pub fn unsubscribe(&self) -> Result<(), UnsubscribeError> {
        self.broadcast_manager.unsubscribe(&self.topic, self.connection_id)
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        // Auto-unsubscribe when dropped
        let _ = self.unsubscribe();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::websocket::WebSocketMessage;
    use tokio::sync::mpsc::unbounded_channel;
    
    #[test]
    fn test_subscribe_unsubscribe() {
        let manager = BroadcastManager::new();
        let conn_id = ConnectionId::new_v4();
        let (tx, _rx) = unbounded_channel();
        
        // Subscribe
        let sub_id = manager.subscribe("orders".to_string(), conn_id, tx).unwrap();
        assert!(!sub_id.to_string().is_empty());
        
        // Check subscriber count
        assert_eq!(manager.subscriber_count("orders"), 1);
        
        // Unsubscribe
        manager.unsubscribe("orders", conn_id).unwrap();
        assert_eq!(manager.subscriber_count("orders"), 0);
    }
    
    #[test]
    fn test_broadcast_to_topic() {
        let manager = BroadcastManager::new();
        let conn_id = ConnectionId::new_v4();
        let (tx, mut rx) = unbounded_channel();
        
        // Subscribe
        manager.subscribe("orders".to_string(), conn_id, tx).unwrap();
        
        // Broadcast message
        let msg = WebSocketMessage::ping();
        let sent = manager.broadcast_to_topic("orders", msg.clone());
        assert_eq!(sent, 1);
        
        // Verify received
        let received = rx.try_recv();
        assert!(received.is_ok());
    }
    
    #[test]
    fn test_unsubscribe_all() {
        let manager = BroadcastManager::new();
        let conn_id = ConnectionId::new_v4();
        let (tx, _rx) = unbounded_channel();
        
        // Subscribe to multiple topics
        manager.subscribe("orders".to_string(), conn_id, tx.clone()).unwrap();
        manager.subscribe("inventory".to_string(), conn_id, tx.clone()).unwrap();
        manager.subscribe("payments".to_string(), conn_id, tx).unwrap();
        
        assert_eq!(manager.subscriber_count("orders"), 1);
        assert_eq!(manager.subscriber_count("inventory"), 1);
        assert_eq!(manager.subscriber_count("payments"), 1);
        
        // Unsubscribe from all
        manager.unsubscribe_all(conn_id);
        
        assert_eq!(manager.subscriber_count("orders"), 0);
        assert_eq!(manager.subscriber_count("inventory"), 0);
        assert_eq!(manager.subscriber_count("payments"), 0);
    }
    
    #[test]
    fn test_topic_stats() {
        let manager = BroadcastManager::new();
        let conn_id = ConnectionId::new_v4();
        let (tx, _rx) = unbounded_channel();
        
        manager.subscribe("orders".to_string(), conn_id, tx).unwrap();
        
        let stats = manager.topic_stats();
        assert_eq!(stats.get("orders"), Some(&1));
    }
}