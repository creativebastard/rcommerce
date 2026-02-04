use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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
    
    #[serde(default)]
    pub import: ImportConfig,
    
    #[serde(default)]
    pub tls: TlsConfig,
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
            import: ImportConfig::default(),
            tls: TlsConfig::default(),
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
        if self.server.port == 0 {
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

impl DatabaseConfig {
    /// Build database connection URL
    pub fn url(&self) -> String {
        match self.db_type {
            DatabaseType::Postgres => {
                format!(
                    "postgres://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, self.port, self.database
                )
            }
            DatabaseType::Mysql => {
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, self.port, self.database
                )
            }
            DatabaseType::Sqlite => {
                format!("sqlite://{}", self.sqlite_path)
            }
        }
    }
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

/// Import configuration for platform API keys and settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportConfig {
    /// Shopify API configuration
    #[serde(default)]
    pub shopify: Option<PlatformImportConfig>,
    
    /// WooCommerce API configuration
    #[serde(default)]
    pub woocommerce: Option<PlatformImportConfig>,
    
    /// Magento API configuration
    #[serde(default)]
    pub magento: Option<PlatformImportConfig>,
    
    /// Medusa API configuration
    #[serde(default)]
    pub medusa: Option<PlatformImportConfig>,
    
    /// Default import options
    #[serde(default)]
    pub default_options: DefaultImportOptions,
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self {
            shopify: None,
            woocommerce: None,
            magento: None,
            medusa: None,
            default_options: DefaultImportOptions::default(),
        }
    }
}

/// Platform-specific import configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformImportConfig {
    /// API base URL
    pub api_url: String,
    
    /// API key or access token
    pub api_key: String,
    
    /// API secret (for WooCommerce)
    #[serde(default)]
    pub api_secret: Option<String>,
    
    /// Additional headers
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
    
    /// Default entity types to import
    #[serde(default = "default_import_entities")]
    pub entities: Vec<String>,
}

/// Default import options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultImportOptions {
    /// Default batch size
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    
    /// Skip existing records by default
    #[serde(default = "default_true")]
    pub skip_existing: bool,
    
    /// Continue on error by default
    #[serde(default = "default_true")]
    pub continue_on_error: bool,
}

impl Default for DefaultImportOptions {
    fn default() -> Self {
        Self {
            batch_size: 100,
            skip_existing: true,
            continue_on_error: true,
        }
    }
}

fn default_import_entities() -> Vec<String> {
    vec!["all".to_string()]
}

fn default_batch_size() -> usize {
    100
}

/// TLS configuration for secure HTTPS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Enable TLS/HTTPS
    #[serde(default = "default_false")]
    pub enabled: bool,

    /// Minimum TLS version (1.3 strongly recommended)
    #[serde(default = "default_min_tls_version")]
    pub min_tls_version: TlsVersion,

    /// Maximum TLS version
    #[serde(default = "default_max_tls_version")]
    pub max_tls_version: TlsVersion,

    /// Certificate file path (for manual certs)
    pub cert_file: Option<PathBuf>,

    /// Private key file path (for manual certs)
    pub key_file: Option<PathBuf>,

    /// Let's Encrypt configuration
    #[serde(default)]
    pub lets_encrypt: Option<LetsEncryptConfig>,

    /// HSTS (HTTP Strict Transport Security) configuration
    #[serde(default)]
    pub hsts: Option<HstsConfig>,

    /// Cipher suites (defaults to modern, secure ciphers)
    #[serde(default)]
    pub cipher_suites: Vec<String>,

    /// Enable OCSP stapling
    #[serde(default = "default_true")]
    pub ocsp_stapling: bool,
    
    /// HTTP port for ACME challenges and redirects (default: 80)
    #[serde(default = "default_http_port")]
    pub http_port: u16,
    
    /// HTTPS port (default: 443)
    #[serde(default = "default_https_port")]
    pub https_port: u16,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            min_tls_version: default_min_tls_version(),
            max_tls_version: default_max_tls_version(),
            cert_file: None,
            key_file: None,
            lets_encrypt: None,
            hsts: None,
            cipher_suites: vec![],
            ocsp_stapling: true,
            http_port: 80,
            https_port: 443,
        }
    }
}

impl TlsConfig {
    /// Validate TLS configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.enabled {
            // Check if either manual certs or Let's Encrypt is configured
            let has_manual_certs = self.cert_file.is_some() && self.key_file.is_some();
            let has_lets_encrypt = self.lets_encrypt.as_ref().map(|le| le.enabled).unwrap_or(false);

            if !has_manual_certs && !has_lets_encrypt {
                return Err(
                    "Either certificate files or Let's Encrypt must be configured when TLS is enabled".to_string(),
                );
            }

            // Verify TLS version is at least 1.2 (1.3 recommended)
            if self.min_tls_version < TlsVersion::Tls1_2 {
                return Err("Minimum TLS version must be 1.2 or higher".to_string());
            }
        }

        Ok(())
    }

    /// Check if this is using Let's Encrypt
    pub fn uses_lets_encrypt(&self) -> bool {
        self.enabled && self.lets_encrypt.as_ref().map(|le| le.enabled).unwrap_or(false)
    }

    /// Check if this is using manual certificates
    pub fn uses_manual_certs(&self) -> bool {
        self.enabled && self.cert_file.is_some() && self.key_file.is_some()
    }
}

/// TLS version enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TlsVersion {
    #[serde(rename = "1.2")]
    Tls1_2,
    #[serde(rename = "1.3")]
    Tls1_3,
}

impl Default for TlsVersion {
    fn default() -> Self {
        TlsVersion::Tls1_3
    }
}

fn default_min_tls_version() -> TlsVersion {
    TlsVersion::Tls1_3
}

fn default_max_tls_version() -> TlsVersion {
    TlsVersion::Tls1_3
}

fn default_http_port() -> u16 {
    80
}

fn default_https_port() -> u16 {
    443
}

/// Let's Encrypt configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetsEncryptConfig {
    /// Enable automatic certificate provisioning
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Contact email for Let's Encrypt account
    pub email: String,

    /// Domain(s) to get certificates for
    pub domains: Vec<String>,

    /// ACME directory URL (production or staging)
    #[serde(default = "default_acme_directory")]
    pub acme_directory: String,

    /// Use staging server for testing (default: false)
    #[serde(default)]
    pub use_staging: bool,

    /// Certificate renewal threshold (days before expiry)
    #[serde(default = "default_renewal_days")]
    pub renewal_threshold_days: i32,

    /// Auto-renew certificates
    #[serde(default = "default_true")]
    pub auto_renew: bool,

    /// Certificate cache directory
    #[serde(default = "default_cert_cache_dir")]
    pub cache_dir: PathBuf,
}

impl Default for LetsEncryptConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            email: String::new(),
            domains: vec![],
            acme_directory: default_acme_directory(),
            use_staging: false,
            renewal_threshold_days: 30,
            auto_renew: true,
            cache_dir: default_cert_cache_dir(),
        }
    }
}

impl LetsEncryptConfig {
    /// Validate Let's Encrypt configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.enabled {
            if self.email.is_empty() {
                return Err("Let's Encrypt email is required".to_string());
            }

            if self.domains.is_empty() {
                return Err("At least one domain is required for Let's Encrypt".to_string());
            }

            // Validate domains
            for domain in &self.domains {
                if !domain.contains('.') {
                    return Err(format!("Invalid domain: {}", domain));
                }
            }
        }

        Ok(())
    }
}

/// HSTS (HTTP Strict Transport Security) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HstsConfig {
    /// Enable HSTS
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Max age in seconds (default: 1 year)
    #[serde(default = "default_hsts_max_age")]
    pub max_age: u64,

    /// Include subdomains
    #[serde(default = "default_true")]
    pub include_subdomains: bool,

    /// Preload in browsers
    #[serde(default)]
    pub preload: bool,
}

impl Default for HstsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_age: 31_536_000, // 1 year in seconds
            include_subdomains: true,
            preload: false,
        }
    }
}

impl HstsConfig {
    /// Generate HSTS header value
    pub fn header_value(&self) -> String {
        let mut parts = vec![format!("max-age={}", self.max_age)];

        if self.include_subdomains {
            parts.push("includeSubDomains".to_string());
        }

        if self.preload {
            parts.push("preload".to_string());
        }

        parts.join("; ")
    }
}

fn default_acme_directory() -> String {
    "https://acme-v02.api.letsencrypt.org/directory".to_string()
}

fn default_renewal_days() -> i32 {
    30
}

fn default_cert_cache_dir() -> PathBuf {
    PathBuf::from("/var/lib/rcommerce/certs")
}

fn default_hsts_max_age() -> u64 {
    31_536_000 // 1 year
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
        config.server.port = 0;  // 0 is an invalid port for binding
        assert!(config.validate().is_err());
        
        config.server.port = 8080;
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_tls_config_defaults() {
        let tls_config = TlsConfig::default();
        assert!(!tls_config.enabled);  // TLS disabled by default
        assert_eq!(tls_config.min_tls_version, TlsVersion::Tls1_3);
        assert_eq!(tls_config.max_tls_version, TlsVersion::Tls1_3);
        assert!(tls_config.ocsp_stapling);
        assert!(tls_config.hsts.is_none());  // No HSTS by default when disabled
        assert_eq!(tls_config.http_port, 80);
        assert_eq!(tls_config.https_port, 443);
    }
    
    #[test]
    fn test_tls_config_validation() {
        // When TLS is disabled, validation should pass regardless of cert config
        let tls_config = TlsConfig::default();
        assert!(!tls_config.enabled);
        assert!(tls_config.validate().is_ok());
        
        // When TLS is enabled, need certificate source
        let mut tls_config = TlsConfig::default();
        tls_config.enabled = true;
        
        // Invalid: no cert source
        assert!(tls_config.validate().is_err());
        
        // Valid: Let's Encrypt
        tls_config.lets_encrypt = Some(LetsEncryptConfig::default());
        assert!(tls_config.validate().is_ok());
        
        // Valid: manual certificates
        tls_config.lets_encrypt = None;
        tls_config.cert_file = Some(PathBuf::from("/path/to/cert.pem"));
        tls_config.key_file = Some(PathBuf::from("/path/to/key.pem"));
        assert!(tls_config.validate().is_ok());
    }
    
    #[test]
    fn test_lets_encrypt_config_validation() {
        let mut le_config = LetsEncryptConfig::default();
        le_config.email = "admin@example.com".to_string();
        le_config.domains = vec!["example.com".to_string()];
        
        assert!(le_config.validate().is_ok());
        
        // Invalid: empty email
        le_config.email = String::new();
        assert!(le_config.validate().is_err());
        
        // Invalid: no domains
        le_config.email = "admin@example.com".to_string();
        le_config.domains = vec![];
        assert!(le_config.validate().is_err());
    }
    
    #[test]
    fn test_tls_version_ordering() {
        assert!(TlsVersion::Tls1_3 > TlsVersion::Tls1_2);
        assert_eq!(TlsVersion::Tls1_3, TlsVersion::Tls1_3);
    }
    
    #[test]
    fn test_hsts_header_generation() {
        let hsts = HstsConfig::default();
        let header = hsts.header_value();
        
        assert!(header.contains("max-age=31536000"));
        assert!(header.contains("includeSubDomains"));
        assert!(!header.contains("preload"));
        
        let hsts_preload = HstsConfig {
            preload: true,
            ..Default::default()
        };
        let header_preload = hsts_preload.header_value();
        assert!(header_preload.contains("preload"));
    }
}