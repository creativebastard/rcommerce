use serde::{Deserialize, Serialize};
use std::path::Path;

/// Main configuration structure for R commerce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    
    #[serde(default)]
    pub database: DatabaseConfig,
    
    #[serde(default)]
    pub logging: LoggingConfig,
    
    #[serde(default)]
    pub cache: CacheConfig,
    
    #[serde(default)]
    pub security: SecurityConfig,
    
    #[serde(default)]
    pub media: MediaConfig,
    
    #[serde(default)]
    pub notifications: NotificationConfig,
    
    #[serde(default)]
    pub rate_limiting: RateLimitConfig,
    
    #[serde(default)]
    pub features: FeatureFlags,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            logging: LoggingConfig::default(),
            cache: CacheConfig::default(),
            security: SecurityConfig::default(),
            media: MediaConfig::default(),
            notifications: NotificationConfig::default(),
            rate_limiting: RateLimitConfig::default(),
            features: FeatureFlags::default(),
        }
    }
}

impl Config {
    /// Load configuration from a TOML file
    pub fn load(path: &str) -> Result<Self, crate::Error> {
        use crate::Error;
        
        let contents = std::fs::read_to_string(path)
            .map_err(|e| Error::Config(format!("Failed to read config file: {}", e)))?;
        
        let config: Config = toml::from_str(&contents)
            .map_err(|e| Error::Config(format!("Failed to parse config: {}", e)))?;
        
        config.validate()?;
        
        Ok(config)
    }
    
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, crate::Error> {
        // Try to load from RCOMMERCE_CONFIG env var first
        if let Ok(config_path) = std::env::var("RCOMMERCE_CONFIG") {
            return Self::load(&config_path);
        }
        
        // Try default locations
        let default_paths = [
            "./config/default.toml",
            "./config/production.toml",
            "/etc/rcommerce/config.toml",
        ];
        
        for path in &default_paths {
            if Path::new(path).exists() {
                return Self::load(path);
            }
        }
        
        // Return default config if no file found
        Ok(Self::default())
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), crate::Error> {
        use crate::Error;
        
        // Validate server config
        if self.server.port == 0 || self.server.port > 65535 {
            return Err(Error::Config("Invalid server port".to_string()));
        }
        
        // Validate database config
        if self.database.pool_size == 0 {
            return Err(Error::Config("Database pool size must be > 0".to_string()));
        }
        
        // Validate cache config
        if self.cache.max_size_mb > Some(10000) {
            return Err(Error::Config("Cache size too large (max 10GB)".to_string()));
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    
    #[serde(default = "default_port")]
    pub port: u16,
    
    #[serde(default = "default_workers")]
    pub worker_threads: usize,
    
    #[serde(default = "default_graceful_shutdown")]
    pub graceful_shutdown_timeout_secs: u64,
    
    #[serde(default)]
    pub cors: CorsConfig,
    
    #[serde(default)]
    pub limits: LimitsConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            worker_threads: default_workers(),
            graceful_shutdown_timeout_secs: default_graceful_shutdown(),
            cors: CorsConfig::default(),
            limits: LimitsConfig::default(),
        }
    }
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_workers() -> usize {
    0 // 0 means use number of CPU cores
}

fn default_graceful_shutdown() -> u64 {
    30
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    #[serde(default)]
    pub allowed_origins: Vec<String>,
    
    #[serde(default)]
    pub allowed_methods: Vec<String>,
    
    #[serde(default)]
    pub allowed_headers: Vec<String>,
    
    #[serde(default = "default_true")]
    pub allow_credentials: bool,
    
    #[serde(default)]
    pub max_age: Option<u64>,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET", "POST", "PUT", "PATCH", "DELETE"]
                .into_iter()
                .map(String::from)
                .collect(),
            allowed_headers: vec!["Content-Type", "Authorization"]
                .into_iter()
                .map(String::from)
                .collect(),
            allow_credentials: true,
            max_age: Some(3600),
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitsConfig {
    #[serde(default = "default_max_request_size")]
    pub max_request_size_mb: u64,
    
    #[serde(default = "default_rate_limit")]
    pub rate_limit_per_minute: u64,
    
    #[serde(default = "default_burst")]
    pub rate_limit_burst: u64,
}

impl Default for LimitsConfig {
    fn default() -> Self {
        Self {
            max_request_size_mb: default_max_request_size(),
            rate_limit_per_minute: default_rate_limit(),
            rate_limit_burst: default_burst(),
        }
    }
}

fn default_max_request_size() -> u64 {
    10 // 10MB
}

fn default_rate_limit() -> u64 {
    1000 // requests per minute
}

fn default_burst() -> u64 {
    200
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    #[serde(default = "default_db_type")]
    pub db_type: DatabaseType,
    
    pub host: String,
    
    #[serde(default = "default_db_port")]
    pub port: u16,
    
    pub database: String,
    pub username: String,
    pub password: String,
    
    #[serde(default = "default_pool_size")]
    pub pool_size: u32,
    
    #[serde(default)]
    pub ssl_mode: SslMode,
    
    #[serde(default = "default_sqlite_path")]
    pub sqlite_path: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            db_type: default_db_type(),
            host: "localhost".to_string(),
            port: default_db_port(),
            database: "rcommerce".to_string(),
            username: "rcommerce".to_string(),
            password: "password".to_string(),
            pool_size: default_pool_size(),
            ssl_mode: SslMode::default(),
            sqlite_path: default_sqlite_path(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DatabaseType {
    Postgres,
    Mysql,
    Sqlite,
}

fn default_db_type() -> DatabaseType {
    DatabaseType::Postgres
}

fn default_db_port() -> u16 {
    5432 // PostgreSQL default
}

fn default_pool_size() -> u32 {
    20
}

fn default_sqlite_path() -> String {
    "./rcommerce.db".to_string()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SslMode {
    Disable,
    Prefer,
    Require,
}

impl Default for SslMode {
    fn default() -> Self {
        SslMode::Prefer
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    
    #[serde(default = "default_log_format")]
    pub format: LogFormat,
    
    #[serde(default)]
    pub file: Option<FileLogConfig>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            format: default_log_format(),
            file: None,
        }
    }
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_format() -> LogFormat {
    LogFormat::Json
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogFormat {
    Json,
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLogConfig {
    pub path: String,
    pub rotation: LogRotation,
    #[serde(default)]
    pub max_size_mb: Option<u64>,
    #[serde(default)]
    pub max_files: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogRotation {
    Daily,
    Hourly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(default = "default_cache_type")]
    pub cache_type: CacheType,
    
    #[serde(default = "default_cache_max_size")]
    pub max_size_mb: Option<u64>,
    
    pub redis_url: Option<String>,
    
    #[serde(default = "default_redis_pool_size")]
    pub redis_pool_size: u32,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            cache_type: default_cache_type(),
            max_size_mb: default_cache_max_size(),
            redis_url: None,
            redis_pool_size: default_redis_pool_size(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CacheType {
    Memory,
    Redis,
}

fn default_cache_type() -> CacheType {
    CacheType::Memory
}

fn default_cache_max_size() -> Option<u64> {
    Some(100) // 100MB
}

fn default_redis_pool_size() -> u32 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    #[serde(default = "default_api_key_prefix_length")]
    pub api_key_prefix_length: usize,
    
    #[serde(default = "default_api_secret_length")]
    pub api_key_secret_length: usize,
    
    #[serde(default)]
    pub jwt: JwtConfig,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            api_key_prefix_length: default_api_key_prefix_length(),
            api_key_secret_length: default_api_secret_length(),
            jwt: JwtConfig::default(),
        }
    }
}

fn default_api_key_prefix_length() -> usize {
    8
}

fn default_api_secret_length() -> usize {
    32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    #[serde(default = "default_jwt_secret")]
    pub secret: String,
    
    #[serde(default = "default_jwt_expiry")]
    pub expiry_hours: u64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: default_jwt_secret(),
            expiry_hours: default_jwt_expiry(),
        }
    }
}

fn default_jwt_secret() -> String {
    // WARNING: This is a default! Replace with secure key in production
    "change_this_in_production_to_a_secure_random_key".to_string()
}

fn default_jwt_expiry() -> u64 {
    24 // hours
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaConfig {
    #[serde(default = "default_storage_type")]
    pub storage_type: StorageType,
    
    pub local_path: Option<String>,
    pub local_base_url: Option<String>,
    
    pub s3_region: Option<String>,
    pub s3_bucket: Option<String>,
    pub s3_access_key: Option<String>,
    pub s3_secret_key: Option<String>,
    pub s3_endpoint: Option<String>,
    
    #[serde(default)]
    pub image_processing: ImageProcessingConfig,
}

impl Default for MediaConfig {
    fn default() -> Self {
        Self {
            storage_type: default_storage_type(),
            local_path: Some("./uploads".to_string()),
            local_base_url: Some("http://localhost:8080/uploads".to_string()),
            s3_region: None,
            s3_bucket: None,
            s3_access_key: None,
            s3_secret_key: None,
            s3_endpoint: None,
            image_processing: ImageProcessingConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StorageType {
    Local,
    S3,
    Gcs,
    Azure,
}

fn default_storage_type() -> StorageType {
    StorageType::Local
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageProcessingConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    #[serde(default = "default_image_quality")]
    pub default_quality: u8,
    
    #[serde(default = "default_image_sizes")]
    pub sizes: Vec<ImageSize>,
}

impl Default for ImageProcessingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_quality: default_image_quality(),
            sizes: default_image_sizes(),
        }
    }
}

fn default_image_quality() -> u8 {
    85
}

fn default_image_sizes() -> Vec<ImageSize> {
    vec![
        ImageSize { name: "thumbnail".to_string(), width: 150, height: 150, crop: true },
        ImageSize { name: "small".to_string(), width: 300, height: 300, crop: false },
        ImageSize { name: "medium".to_string(), width: 600, height: 600, crop: false },
        ImageSize { name: "large".to_string(), width: 1200, height: 1200, crop: false },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSize {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub crop: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    #[serde(default)]
    pub email: EmailConfig,
    
    #[serde(default)]
    pub sms: SmsConfig,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            email: EmailConfig::default(),
            sms: SmsConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    #[serde(default)]
    pub smtp_host: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_user: Option<String>,
    pub smtp_pass: Option<String>,
    pub from_name: Option<String>,
    pub from_email: Option<String>,
    #[serde(default = "default_true")]
    pub smtp_tls: bool,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            smtp_host: None,
            smtp_port: None,
            smtp_user: None,
            smtp_pass: None,
            from_name: None,
            from_email: None,
            smtp_tls: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsConfig {
    #[serde(default)]
    pub provider: Option<String>,
    pub twilio_account_sid: Option<String>,
    pub twilio_auth_token: Option<String>,
    pub twilio_from_number: Option<String>,
}

impl Default for SmsConfig {
    fn default() -> Self {
        Self {
            provider: None,
            twilio_account_sid: None,
            twilio_auth_token: None,
            twilio_from_number: None,
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Maximum requests per minute from a single IP
    #[serde(default = "default_rate_limit_minute")]
    pub requests_per_minute: u32,
    
    /// Maximum requests per hour from a single IP
    #[serde(default = "default_rate_limit_hour")]
    pub requests_per_hour: u32,
    
    /// Maximum requests per day from a single IP
    #[serde(default = "default_rate_limit_day")]
    pub requests_per_day: u32,
    
    /// Maximum concurrent connections per IP
    #[serde(default = "default_max_concurrent")]
    pub max_concurrent_per_ip: u32,
    
    /// Enable API key rate limiting (more permissive)
    #[serde(default = "default_true")]
    pub api_key_limiting: bool,
    
    /// Maximum requests per minute with valid API key
    #[serde(default = "default_rate_limit_api_key")]
    pub api_key_requests_per_minute: u32,
    
    /// Blocklist of IP addresses
    #[serde(default)]
    pub blocklist: Vec<String>,
    
    /// Allowlist of trusted IP addresses
    #[serde(default)]
    pub allowlist: Vec<String>,
    
    /// Enable DDoS protection mode (stricter limits under attack)
    #[serde(default = "default_true")]
    pub ddos_protection: bool,
    
    /// Response headers to include rate limit info
    #[serde(default = "default_true")]
    pub expose_headers: bool,
    
    /// Store rate limit data in Redis (true) or memory (false)
    #[serde(default = "default_false")]
    pub use_redis: bool,
    
    /// Redis connection string
    #[serde(default)]
    pub redis_url: Option<String>,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_minute: 60,
            requests_per_hour: 1000,
            requests_per_day: 10000,
            max_concurrent_per_ip: 10,
            api_key_limiting: true,
            api_key_requests_per_minute: 1000,
            blocklist: vec![],
            allowlist: vec![],
            ddos_protection: true,
            expose_headers: true,
            use_redis: false,
            redis_url: None,
        }
    }
}

// Default value helper functions
fn default_true() -> bool { true }
fn default_false() -> bool { false }
fn default_rate_limit_minute() -> u32 { 60 }
fn default_rate_limit_hour() -> u32 { 1000 }
fn default_rate_limit_day() -> u32 { 10000 }
fn default_max_concurrent() -> u32 { 10 }
fn default_rate_limit_api_key() -> u32 { 1000 }

/// Feature flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    #[serde(default = "default_true")]
    pub debug_api: bool,
    
    #[serde(default = "default_true")]
    pub metrics: bool,
    
    #[serde(default = "default_true")]
    pub health_check: bool,
    
    #[serde(default)]
    pub experimental: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            debug_api: true,
            metrics: true,
            health_check: true,
            experimental: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.database.pool_size, 20);
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        config.server.port = 99999;
        assert!(config.validate().is_err());
        
        config.server.port = 8080;
        assert!(config.validate().is_ok());
    }
}