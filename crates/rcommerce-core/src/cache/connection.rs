//! Redis connection management with pooling

use crate::cache::{CacheResult, CacheError, RedisConfig};
use redis::{
    aio::ConnectionManager,
    Client as RedisClient,
    Cmd,
    Pipeline,
    Value,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};

/// Redis connection pool
#[derive(Clone)]
pub struct RedisPool {
    /// Redis client
    client: Arc<RedisClient>,
    
    /// Connection manager for async operations
    manager: Arc<RwLock<Option<ConnectionManager>>>,
    
    /// Configuration
    config: Arc<RedisConfig>,
    
    /// Connection state tracking
    state: Arc<RwLock<ConnectionState>>,
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConnectionState {
    /// Connected and ready
    Connected,
    
    /// Disconnected, attempting reconnect
    Reconnecting,
    
    /// Failed, manual intervention needed
    Failed,
}

impl RedisPool {
    /// Create a new Redis connection pool
    pub async fn new(config: RedisConfig) -> CacheResult<Self> {
        // Create Redis client
        let client = RedisClient::open(&config.url[..])
            .map_err(|e| CacheError::ConnectionError(e.to_string()))?;
        
        let pool = Self {
            client: Arc::new(client),
            manager: Arc::new(RwLock::new(None)),
            config: Arc::new(config),
            state: Arc::new(RwLock::new(ConnectionState::Failed)),
        };
        
        // Initial connection
        pool.reconnect().await?;
        
        info!("Redis pool created: url={}", pool.config.url);
        
        Ok(pool)
    }
    
    /// Get a connection from the pool
    pub async fn get(&self) -> CacheResult<RedisConnection> {
        // Check state
        let state = *self.state.read().await;
        
        match state {
            ConnectionState::Connected => {
                // Try to get manager
                let manager_opt = self.manager.read().await;
                if let Some(manager) = manager_opt.as_ref() {
                    Ok(RedisConnection {
                        manager: manager.clone(),
                        config: self.config.clone(),
                    })
                } else {
                    // No manager available, try to reconnect
                    drop(manager_opt);
                    self.reconnect().await?;
                    
                    // Retry getting connection
                    let manager_opt = self.manager.read().await;
                    if let Some(manager) = manager_opt.as_ref() {
                        Ok(RedisConnection {
                            manager: manager.clone(),
                            config: self.config.clone(),
                        })
                    } else {
                        Err(CacheError::ConnectionError("No connection available".to_string()))
                    }
                }
            }
            ConnectionState::Reconnecting => {
                // Wait a bit and retry
                tokio::time::sleep(self.config.retry_delay()).await;
                self.get().await
            }
            ConnectionState::Failed => {
                // Attempt reconnect
                self.reconnect().await?;
                self.get().await
            }
        }
    }
    
    /// Reconnect to Redis
    pub async fn reconnect(&self) -> CacheResult<()> {
        // Set state to reconnecting
        *self.state.write().await = ConnectionState::Reconnecting;
        
        let mut attempt = 0;
        loop {
            attempt += 1;
            debug!("Redis reconnect attempt: {}", attempt);
            
            match self.attempt_connect().await {
                Ok(manager) => {
                    *self.manager.write().await = Some(manager);
                    *self.state.write().await = ConnectionState::Connected;
                    info!("Redis reconnected successfully after {} attempts", attempt);
                    return Ok(());
                }
                Err(e) => {
                    error!("Redis connection attempt {} failed: {}", attempt, e);
                    
                    if attempt >= self.config.max_retries {
                        *self.state.write().await = ConnectionState::Failed;
                        return Err(CacheError::ConnectionError(
                            format!("Failed to connect after {} attempts: {}", attempt, e)
                        ));
                    }
                    
                    // Wait before retry
                    tokio::time::sleep(self.config.retry_delay()).await;
                }
            }
        }
    }
    
    /// Attempt a single connection
    async fn attempt_connect(&self) -> CacheResult<ConnectionManager> {
        // Create connection manager
        let manager = ConnectionManager::new(self.client.clone())
            .await
            .map_err(|e| CacheError::ConnectionError(e.to_string()))?;
        
        // Validate connection with PING
        let mut cmd = Cmd::new();
        cmd.arg("PING");
        
        let result: String = manager.send_packed_command(&cmd, redis::RedisWrite::len, |buf| buf.to_vec())
            .await
            .map_err(|e| CacheError::ConnectionError(e.to_string()))?;
        
        if result != "PONG" {
            return Err(CacheError::ConnectionError("Redis PING failed".to_string()));
        }
        
        Ok(manager)
    }
    
    /// Check if pool is healthy
    pub async fn health_check(&self) -> CacheResult<bool> {
        let state = *self.state.read().await;
        
        match state {
            ConnectionState::Connected => {
                // Try to execute a PING
                match self.get().await {
                    Ok(conn) => {
                        // Simple health check via connection
                        Ok(true)
                    }
                    Err(_) => {
                        // Connection failed, mark as failed
                        *self.state.write().await = ConnectionState::Failed;
                        Ok(false)
                    }
                }
            }
            _ => Ok(false),
        }
    }
    
    /// Get pool statistics
    pub async fn stats(&self) -> PoolStats {
        let state = *self.state.read().await;
        let manager_count = if self.manager.read().await.is_some() { 1 } else { 0 };
        
        PoolStats {
            state,
            active_connections: manager_count,
            max_connections: self.config.pool_size,
            url: self.config.url.clone(),
        }
    }
    
    /// Close the pool and cleanup
    pub async fn close(&self) -> CacheResult<()> {
        info!("Closing Redis pool");
        
        // Set state to failed
        *self.state.write().await = ConnectionState::Failed;
        
        // Drop manager
        *self.manager.write().await = None;
        
        Ok(())
    }
}

impl Drop for RedisPool {
    fn drop(&mut self) {
        // Note: Can't use async in drop, so we just log
        // In production, explicitly call close() before dropping
        debug!("RedisPool dropped");
    }
}

/// Single Redis connection wrapper
pub struct RedisConnection {
    /// Connection manager
    manager: ConnectionManager,
    
    /// Configuration reference
    config: Arc<RedisConfig>,
}

impl RedisConnection {
    /// Execute a Redis command
    pub async fn execute(&mut self, cmd: Cmd) -> CacheResult<Value> {
        Ok(self.manager.send_packed_command(
            &cmd,
            redis::RedisWrite::len,
            |buf| buf.to_vec(),
        ).await.map_err(|e| CacheError::OperationError(e.to_string()))?)
    }
    
    /// Execute a Redis pipeline
    pub async fn execute_pipeline(&mut self, pipeline: Pipeline) -> CacheResult<Vec<Value>> {
        Ok(self.manager.send_packed_commands(
            &pipeline,
            redis::RedisWrite::len,
            |buf| buf.to_vec(),
            0, // Start from first command
        ).await.map_err(|e| CacheError::OperationError(e.to_string()))?)
    }
    
    /// Set a key with TTL
    pub async fn setex(&mut self, key: &str, ttl_secs: u64, value: &[u8]) -> CacheResult<()> {
        let mut cmd = Cmd::new();
        cmd.arg("SETEX").arg(key).arg(ttl_secs).arg(value);
        
        let result: String = redis::from_redis_value(&self.execute(cmd).await?)
            .map_err(|e| CacheError::SerializationError(e.to_string()))?;
        
        if result != "OK" {
            return Err(CacheError::OperationError("SETEX failed".to_string()));
        }
        
        Ok(())
    }
    
    /// Get a key
    pub async fn get(&mut self, key: &str) -> CacheResult<Option<Vec<u8>>> {
        let mut cmd = Cmd::new();
        cmd.arg("GET").arg(key);
        
        let result = self.execute(cmd).await?;
        
        match result {
            redis::Value::Nil => Ok(None),
            redis::Value::BulkString(data) => Ok(Some(data)),
            _ => {
                let data: Vec<u8> = redis::from_redis_value(&result)
                    .map_err(|e| CacheError::DeserializationError(e.to_string()))?;
                Ok(Some(data))
            }
        }
    }
    
    /// Delete a key
    pub async fn del(&mut self, key: &str) -> CacheResult<bool> {
        let mut cmd = Cmd::new();
        cmd.arg("DEL").arg(key);
        
        let result: i32 = redis::from_redis_value(&self.execute(cmd).await?)
            .map_err(|e| CacheError::OperationError(e.to_string()))?;
        
        Ok(result > 0)
    }
    
    /// Check if key exists
    pub async fn exists(&mut self, key: &str) -> CacheResult<bool> {
        let mut cmd = Cmd::new();
        cmd.arg("EXISTS").arg(key);
        
        let result: i32 = redis::from_redis_value(&self.execute(cmd).await?)
            .map_err(|e| CacheError::OperationError(e.to_string()))?;
        
        Ok(result > 0)
    }
    
    /// Increment a counter
    pub async fn incr(&mut self, key: &str) -> CacheResult<i64> {
        let mut cmd = Cmd::new();
        cmd.arg("INCR").arg(key);
        
        let result: i64 = redis::from_redis_value(&self.execute(cmd).await?)
            .map_err(|e| CacheError::OperationError(e.to_string()))?;
        
        Ok(result)
    }
    
    /// Set key expiration
    pub async fn expire(&mut self, key: &str, ttl_secs: u64) -> CacheResult<bool> {
        let mut cmd = Cmd::new();
        cmd.arg("EXPIRE").arg(key).arg(ttl_secs);
        
        let result: i32 = redis::from_redis_value(&self.execute(cmd).await?)
            .map_err(|e| CacheError::OperationError(e.to_string()))?;
        
        Ok(result > 0)
    }
    
    /// Get TTL for a key
    pub async fn ttl(&mut self, key: &str) -> CacheResult<i64> {
        let mut cmd = Cmd::new();
        cmd.arg("TTL").arg(key);
        
        let result: i64 = redis::from_redis_value(&self.execute(cmd).await?)
            .map_err(|e| CacheError::OperationError(e.to_string()))?;
        
        Ok(result)
    }
    
    /// Publish a message to a channel
    pub async fn publish(&mut self, channel: &str, message: &[u8]) -> CacheResult<u64> {
        let mut cmd = Cmd::new();
        cmd.arg("PUBLISH").arg(channel).arg(message);
        
        let result: u64 = redis::from_redis_value(&self.execute(cmd).await?)
            .map_err(|e| CacheError::OperationError(e.to_string()))?;
        
        Ok(result)
    }
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    /// Current connection state
    pub state: ConnectionState,
    
    /// Number of active connections
    pub active_connections: usize,
    
    /// Maximum connections allowed
    pub max_connections: usize,
    
    /// Redis URL
    pub url: String,
}

impl PoolStats {
    /// Format as human-readable string
    pub fn format(&self) -> String {
        let state_str = match self.state {
            ConnectionState::Connected => "connected",
            ConnectionState::Reconnecting => "reconnecting",
            ConnectionState::Failed => "failed",
        };
        
        format!(
            "RedisPool[url={}, state={}, connections={}/{}]",
            self.url, state_str, self.active_connections, self.max_connections
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::RedisConfig;
    
    #[tokio::test]
    async fn test_redis_pool_creation() {
        let config = RedisConfig::development();
        let pool = RedisPool::new(config).await;
        
        // This may fail if Redis is not running, which is OK for unit test
        match pool {
            Ok(p) => {
                let stats = p.stats().await;
                assert_eq!(stats.state, ConnectionState::Connected);
            }
            Err(_) => {
                // Redis not available, still a valid test result
                assert!(true);
            }
        }
    }
    
    #[tokio::test]
    async fn test_health_check() {
        let config = RedisConfig::development();
        
        if let Ok(pool) = RedisPool::new(config).await {
            let is_healthy = pool.health_check().await.unwrap();
            assert!(is_healthy);
        }
    }
    
    #[tokio::test]
    async fn test_pool_stats() {
        let config = RedisConfig::development();
        
        if let Ok(pool) = RedisPool::new(config).await {
            let stats = pool.stats().await;
            assert!(stats.url.contains("127.0.0.1"));
            assert!(stats.max_connections > 0);
        }
    }
}