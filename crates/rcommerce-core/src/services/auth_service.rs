use std::sync::Arc;
use uuid::Uuid;
use sha2::{Sha256, Digest};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

use crate::{Result, Error, Config};

#[derive(Clone)]
pub struct AuthService {
    config: Arc<Config>,
}

impl AuthService {
    pub fn new(config: Config) -> Self {
        Self { config: Arc::new(config) }
    }
    
    /// Hash a password using bcrypt
    pub fn hash_password(&self, password: &str) -> Result<String> {
        hash(password, DEFAULT_COST)
            .map_err(|e| Error::internal(format!("Failed to hash password: {}", e)))
    }
    
    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        verify(password, hash)
            .map_err(|e| Error::internal(format!("Failed to verify password: {}", e)))
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
    
    /// Generate JWT access token for a customer
    pub fn generate_access_token(&self, customer_id: Uuid, email: &str) -> Result<String> {
        let expiry_hours = self.config.security.jwt.expiry_hours as i64;
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(expiry_hours))
            .expect("valid timestamp")
            .timestamp();
        
        let claims = JwtClaims {
            sub: customer_id,
            email: email.to_string(),
            token_type: TokenType::Access,
            permissions: vec!["read".to_string(), "write".to_string()],
            exp: expiration,
            iat: Utc::now().timestamp(),
            iss: "rcommerce".to_string(),
            aud: "rcommerce-api".to_string(),
        };
        
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(self.config.security.jwt.secret.as_bytes());
        
        encode(&header, &claims, &encoding_key)
            .map_err(|e| Error::internal(format!("Failed to generate JWT: {}", e)))
    }
    
    /// Generate JWT refresh token
    pub fn generate_refresh_token(&self, customer_id: Uuid) -> Result<String> {
        // Refresh tokens last 7 days
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24 * 7))
            .expect("valid timestamp")
            .timestamp();
        
        let claims = JwtClaims {
            sub: customer_id,
            email: String::new(), // Refresh tokens don't need email
            token_type: TokenType::Refresh,
            permissions: vec!["refresh".to_string()],
            exp: expiration,
            iat: Utc::now().timestamp(),
            iss: "rcommerce".to_string(),
            aud: "rcommerce-api".to_string(),
        };
        
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(self.config.security.jwt.secret.as_bytes());
        
        encode(&header, &claims, &encoding_key)
            .map_err(|e| Error::internal(format!("Failed to generate refresh token: {}", e)))
    }
    
    /// Verify and decode a JWT token
    pub fn verify_token(&self, token: &str) -> Result<JwtClaims> {
        let decoding_key = DecodingKey::from_secret(self.config.security.jwt.secret.as_bytes());
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        // Disable audience validation since we're using simple validation
        validation.validate_aud = false;
        
        decode::<JwtClaims>(token, &decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    Error::unauthorized("Token has expired")
                }
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    Error::unauthorized("Invalid token")
                }
                jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                    Error::unauthorized("Invalid token signature")
                }
                _ => Error::unauthorized(format!("Token validation failed: {}", e))
            })
    }
    
    /// Extract bearer token from Authorization header
    pub fn extract_bearer_token(auth_header: &str) -> Option<&str> {
        auth_header.strip_prefix("Bearer ")
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid, // User ID
    pub email: String,
    pub token_type: TokenType,
    pub permissions: Vec<String>,
    pub exp: i64,  // Expiration time
    pub iat: i64,  // Issued at
    pub iss: String, // Issuer
    pub aud: String, // Audience
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Access,
    Refresh,
}

/// Authenticated user extracted from JWT
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub customer_id: Uuid,
    pub email: String,
    pub permissions: Vec<String>,
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
    
    #[test]
    fn test_password_hashing() {
        let config = Config::default();
        let auth = AuthService::new(config);
        
        let password = "my_secure_password123";
        let hash = auth.hash_password(password).unwrap();
        
        // Verify correct password
        assert!(auth.verify_password(password, &hash).unwrap());
        
        // Verify wrong password fails
        assert!(!auth.verify_password("wrong_password", &hash).unwrap());
    }
    
    #[test]
    fn test_jwt_generation_and_verification() {
        let config = Config::default();
        let auth = AuthService::new(config);
        
        let customer_id = Uuid::new_v4();
        let email = "test@example.com";
        
        // Generate token
        let token = auth.generate_access_token(customer_id, email).unwrap();
        assert!(!token.is_empty());
        
        // Verify token
        let claims = auth.verify_token(&token).unwrap();
        assert_eq!(claims.sub, customer_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.token_type, TokenType::Access);
    }
    
    #[test]
    fn test_extract_bearer_token() {
        assert_eq!(
            AuthService::extract_bearer_token("Bearer my_token_123"),
            Some("my_token_123")
        );
        assert_eq!(
            AuthService::extract_bearer_token("Basic dXNlcjpwYXNz"),
            None
        );
        assert_eq!(
            AuthService::extract_bearer_token("my_token_123"),
            None
        );
    }
}
