use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// TLS configuration for secure HTTPS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Enable TLS/HTTPS
    #[serde(default = "default_true")]
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
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_tls_version: default_min_tls_version(),
            max_tls_version: default_max_tls_version(),
            cert_file: None,
            key_file: None,
            lets_encrypt: Some(LetsEncryptConfig::default()),
            hsts: Some(HstsConfig::default()),
            cipher_suites: default_cipher_suites(),
            ocsp_stapling: true,
        }
    }
}

impl TlsConfig {
    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.enabled {
            // Check if either manual certs or Let's Encrypt is configured
            let has_manual_certs = self.cert_file.is_some() && self.key_file.is_some();
            let has_lets_encrypt = self.lets_encrypt.is_some();

            if !has_manual_certs && !has_lets_encrypt {
                return Err(
                    "Either certificate files or Let's Encrypt must be configured".to_string(),
                );
            }

            // Verify TLS version is at least 1.3
            if self.min_tls_version < TlsVersion::Tls1_3 {
                return Err("Minimum TLS version must be 1.3 or higher for security".to_string());
            }
        }

        Ok(())
    }

    /// Check if this is a production configuration
    pub fn is_production(&self) -> bool {
        self.enabled && self.min_tls_version >= TlsVersion::Tls1_3 && self.hsts.is_some()
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

// Default values
fn default_true() -> bool {
    true
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

fn default_cipher_suites() -> Vec<String> {
    vec![
        "TLS_AES_128_GCM_SHA256".to_string(),
        "TLS_AES_256_GCM_SHA384".to_string(),
        "TLS_CHACHA20_POLY1305_SHA256".to_string(),
        // These are TLS 1.2 ciphers for backward compatibility if needed
        // But we strongly recommend TLS 1.3 only
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_config_defaults() {
        let config = TlsConfig::default();
        assert!(config.enabled);
        assert_eq!(config.min_tls_version, TlsVersion::Tls1_3);
        assert_eq!(config.max_tls_version, TlsVersion::Tls1_3);
        assert!(config.hsts.is_some());
        assert!(config.ocsp_stapling);
    }

    #[test]
    fn test_tls_config_validation() {
        let mut config = TlsConfig::default();

        // Should pass with Let's Encrypt
        assert!(config.validate().is_ok());

        // Should fail with no cert configuration
        config.lets_encrypt = None;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_lets_encrypt_validation() {
        let mut le = LetsEncryptConfig::default();
        le.email = "admin@example.com".to_string();
        le.domains = vec!["example.com".to_string()];

        assert!(le.validate().is_ok());

        // Should fail with invalid email
        le.email = String::new();
        assert!(le.validate().is_err());

        // Should fail with no domains
        le.email = "admin@example.com".to_string();
        le.domains = vec![];
        assert!(le.validate().is_err());
    }

    #[test]
    fn test_hsts_header_generation() {
        let hsts = HstsConfig::default();
        let header = hsts.header_value();

        assert!(header.contains("max-age=31536000"));
        assert!(header.contains("includeSubDomains"));
        assert!(!header.contains("preload"));

        let hsts_with_preload = HstsConfig {
            preload: true,
            ..Default::default()
        };
        let header = hsts_with_preload.header_value();
        assert!(header.contains("preload"));
    }
}
