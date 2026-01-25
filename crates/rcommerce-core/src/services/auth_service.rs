use std::sync::Arc;
use uuid::Uuid;
use sha2::{Sha256, Digest};

use crate::{Result, Error, Config};

#[derive(Clone)]
pub struct AuthService {
    config: Arc<Config>,
}

impl AuthService {
    pub fn new(config: Config) -> Self {
        Self { config: Arc::new(config) }
    }
    
    /// Generate API key
    pub fn generate_api_key(&self) -> ApiKey {
        let prefix = self.generate_prefix();
        let secret = self.generate_secret();
        let full_key = format!("{}.{}", prefix, secret);
        
        let mut hasher = Sha256::new();
        hasher.update(full_key.as_bytes());
        let hash = hasher.finalize();
        let hash_hex = hex::encode(hash);
        
        ApiKey {
            prefix,
            full_key: Some(full_key),
            hash: hash_hex,
        }
    }
    
    /// Verify API key
    pub fn verify_api_key(&self, provided_key: &str, stored_hash: &str) -> Result<bool> {
        let parts: Vec<&str> = provided_key.splitn(2, '.').collect();
        if parts.len() != 2 {
            return Ok(false);
        }
        
        let mut hasher = Sha256::new();
        hasher.update(provided_key.as_bytes());
        let hash = hasher.finalize();
        let hash_hex = hex::encode(hash);
        
        Ok(hash_hex == stored_hash)
    }
    
    /// Generate JWT token (placeholder)
    pub fn generate_jwt(&self, _claims: JwtClaims) -> Result<String> {
        // TODO: Implement JWT generation
        Err(Error::not_implemented("JWT generation not yet implemented"))
    }
    
    /// Verify JWT token (placeholder)
    pub fn verify_jwt(&self, _token: &str) -> Result<JwtClaims> {
        // TODO: Implement JWT verification
        Err(Error::not_implemented("JWT verification not yet implemented"))
    }
    
    fn generate_prefix(&self) -> String {
        use rand::Rng;
        let prefix_length = self.config.security.api_key_prefix_length;
        
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(prefix_length)
            .map(char::from)
            .collect()
    }
    
    fn generate_secret(&self) -> String {
        use rand::Rng;
        let secret_length = self.config.security.api_key_secret_length;
        
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(secret_length)
            .map(char::from)
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: String,
    pub full_key: Option<String>, // Only available on generation, never stored
    pub hash: String,
}

#[derive(Debug, Clone)]
pub struct JwtClaims {
    pub sub: Uuid, // User ID
    pub email: String,
    pub permissions: Vec<String>,
    pub exp: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_and_verify_api_key() {
        let config = Config::default();
        let auth = AuthService::new(config);
        
        let api_key = auth.generate_api_key();
        assert!(api_key.full_key.is_some());
        
        let full_key = api_key.full_key.unwrap();
        let is_valid = auth.verify_api_key(&full_key, &api_key.hash).unwrap();
        assert!(is_valid);
        
        // Test invalid key
        let is_valid = auth.verify_api_key("invalid.key", &api_key.hash).unwrap();
        assert!(!is_valid);
    }
}