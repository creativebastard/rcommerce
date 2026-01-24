//! WebSocket support for real-time updates and bi-directional communication
//!
//! This module provides a secure, memory-efficient WebSocket implementation
//! with the following features:
//!
//! ✅ Security:
//! - Origin validation
//! - Authentication required
//! - CSRF protection
//! - Rate limiting (connections & messages)
//! - Message size limits
//! - Input validation
//!
//! ✅ Type Safety:
//! - Typed messages using enums
//! - Strong typing for all events
//! - Serde serialization
//! - Generic connection handling
//!
//! ✅ Memory Efficiency:
//! - Connection pooling with DashMap
//! - Efficient broadcasting (O(n) not O(n^2))
//! - Message size limits
//! - Dead connection cleanup
//! - Backpressure handling
//!
//! ✅ Clean Code:
//! - Clear separation of concerns
//! - Modular architecture
//! - Comprehensive error handling
//! - Logging & metrics
//!
//! Typical usage:
//!
//! ```ignore
//! use rcommerce_core::websocket::{WebSocketManager, WebSocketConfig};
//!
//! // Create WebSocket manager
//! let ws_manager = WebSocketManager::new(WebSocketConfig::default());
//!
//! // Handle WebSocket upgrade
//! let (ws_stream, user_id) = ws_manager.authenticate_and_upgrade(request).await?;
//!
//! // Handle connection in background task
//! tokio::spawn(async move {
//!     ws_manager.handle_connection(ws_stream, user_id).await;
//! });
//!
//! // Broadcast message to user
//! ws_manager.send_to_user(user_id, WebSocketMessage::OrderUpdate {
//!     order_id,
//!     status: "shipped".to_string(),
//! }).await?;
//! ```

pub mod config;
pub mod connection;
pub mod message;
pub mod broadcast;
pub mod auth;
pub mod rate_limit;
pub mod manager;

// Re-export main types
pub use config::WebSocketConfig;
pub use connection::{WebSocketConnection, ConnectionId, UserId};
pub use message::{WebSocketMessage, MessageType};
pub use broadcast::{BroadcastManager, Subscription};
pub use auth::{AuthToken, OriginValidator};
pub use rate_limit::{ConnectionRateLimiter, MessageRateLimiter};
pub use manager::WebSocketManager;

// Prelude for convenient imports
pub mod prelude {
    pub use super::{
        WebSocketConfig, WebSocketManager, WebSocketMessage, MessageType,
        WebSocketConnection, ConnectionId, UserId, AuthToken,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_websocket_module_compiles() {
        let config = WebSocketConfig::default();
        assert_eq!(config.enabled, true);
        assert_eq!(config.max_connections, 10000);
    }
}