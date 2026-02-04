use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

use acme_lib::create_rsa_key;
use acme_lib::persist::FilePersist;
use acme_lib::{Directory, DirectoryUrl};

use crate::{Error, Result};

/// Import from rcommerce_core
use rcommerce_core::config::LetsEncryptConfig;

/// Let's Encrypt certificate manager
pub struct LetsEncryptManager {
    config: LetsEncryptConfig,
    certificates: Arc<RwLock<HashMap<String, CertificateInfo>>>,
    /// Challenge tokens storage for HTTP-01 validation
    challenge_tokens: Arc<RwLock<HashMap<String, String>>>,
}

#[derive(Debug, Clone)]
pub struct CertificateInfo {
    pub domain: String,
    pub certificate_path: PathBuf,
    pub private_key_path: PathBuf,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub serial_number: String,
}

impl LetsEncryptManager {
    pub fn new(config: LetsEncryptConfig) -> Result<Self> {
        // Ensure cache directory exists
        std::fs::create_dir_all(&config.cache_dir)
            .map_err(|e| Error::Config(format!("Failed to create cert cache dir: {}", e)))?;

        // Ensure challenges directory exists
        let challenges_dir = config.cache_dir.join("challenges");
        std::fs::create_dir_all(&challenges_dir)
            .map_err(|e| Error::Config(format!("Failed to create challenges dir: {}", e)))?;

        Ok(Self {
            config,
            certificates: Arc::new(RwLock::new(HashMap::new())),
            challenge_tokens: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Initialize Let's Encrypt account
    /// 
    /// Creates or loads an existing ACME account with Let's Encrypt.
    /// This must be called before obtaining certificates.
    pub async fn init_account(&self) -> Result<()> {
        info!("Initializing Let's Encrypt account");

        // Create the persistence layer for storing account data
        let persist_dir = self.config.cache_dir.join("accounts");
        std::fs::create_dir_all(&persist_dir)
            .map_err(|e| Error::Config(format!("Failed to create account persist dir: {}", e)))?;

        let persist = FilePersist::new(&persist_dir);

        // Select ACME directory based on configuration
        let dir_url = if self.config.use_staging {
            info!("Using Let's Encrypt staging environment");
            DirectoryUrl::LetsEncryptStaging
        } else {
            info!("Using Let's Encrypt production environment");
            DirectoryUrl::LetsEncrypt
        };

        // Create the ACME directory
        let dir = Directory::from_url(persist, dir_url)
            .map_err(|e| Error::Network(format!("Failed to create ACME directory: {}", e)))?;

        // Create or load account
        let _account = dir
            .account(&self.config.email)
            .map_err(|e| Error::Network(format!("Failed to create/load ACME account: {}", e)))?;

        info!(
            "Let's Encrypt account initialized successfully for email: {}",
            self.config.email
        );

        Ok(())
    }

    /// Obtain or renew certificate for a domain using ACME HTTP-01 challenge
    /// 
    /// This method:
    /// 1. Creates a new certificate order
    /// 2. Handles HTTP-01 challenges for each domain
    /// 3. Validates domain ownership
    /// 4. Downloads and saves the certificate
    pub async fn obtain_certificate(&self, domain: &str) -> Result<CertificateInfo> {
        info!("Obtaining certificate for domain: {}", domain);

        // Check if we already have a valid certificate
        if let Ok(Some(existing)) = self.get_certificate_info(domain).await {
            if !self.should_renew(&existing) {
                info!("Certificate for {} is still valid, skipping", domain);
                return Ok(existing);
            }
            info!("Certificate for {} needs renewal", domain);
        }

        // Create the persistence layer
        let persist_dir = self.config.cache_dir.join("accounts");
        let persist = FilePersist::new(&persist_dir);

        // Select ACME directory
        let dir_url = if self.config.use_staging {
            DirectoryUrl::LetsEncryptStaging
        } else {
            DirectoryUrl::LetsEncrypt
        };

        // Create the ACME directory
        let dir = Directory::from_url(persist, dir_url)
            .map_err(|e| Error::Network(format!("Failed to create ACME directory: {}", e)))?;

        // Load the account
        let account = dir
            .account(&self.config.email)
            .map_err(|e| Error::Network(format!("Failed to load ACME account: {}", e)))?;

        // Create a new order for the domain
        let mut order = account
            .new_order(domain, &[])
            .map_err(|e| Error::Network(format!("Failed to create certificate order: {}", e)))?;

        // Get the authorizations (challenges)
        let auths = order
            .authorizations()
            .map_err(|e| Error::Network(format!("Failed to get authorizations: {}", e)))?;

        // Handle each authorization
        for auth in &auths {
            // Check if we actually need to do the challenge
            if !auth.need_challenge() {
                info!("Authorization for {} is already valid, skipping challenge", auth.domain_name());
                continue;
            }

            // Get the HTTP-01 challenge
            let challenge = auth.http_challenge();

            let token = challenge.http_token().to_string();
            let proof = challenge.http_proof();

            info!("Handling HTTP-01 challenge for domain: {}", auth.domain_name());
            debug!("Challenge token: {}", token);

            // Save the challenge token
            self.save_challenge_token(auth.domain_name(), &token, &proof).await?;

            // Validate the challenge (timeout in milliseconds)
            let validation_result = challenge.validate(5000);
            
            // Clean up the challenge token regardless of result
            self.remove_challenge_token(auth.domain_name(), &token).await?;

            validation_result.map_err(|e| {
                Error::Network(format!("Challenge validation failed: {}", e))
            })?;

            info!("Authorization successful for domain: {}", auth.domain_name());
        }

        // Refresh the order to get updated status
        order.refresh().map_err(|e| {
            Error::Network(format!("Failed to refresh order: {}", e))
        })?;

        // Check if order is validated
        if !order.is_validated() {
            return Err(Error::Network(
                "Order is not validated after challenges".to_string(),
            ));
        }

        // Get the CSR order
        let csr_order = order.confirm_validations().ok_or_else(|| {
            Error::Network("Failed to confirm validations".to_string())
        })?;

        // Generate a private key for the certificate
        let private_key = create_rsa_key(2048);
        let private_key_pem = String::from_utf8(
            private_key.private_key_to_pem_pkcs8()
                .map_err(|e| Error::Other(format!("Failed to encode private key: {}", e)))?
        )
        .map_err(|e| Error::Other(format!("Invalid UTF-8 in private key: {}", e)))?;

        // Finalize the order and get the cert order
        let cert_order = csr_order
            .finalize(&private_key_pem, 5000)
            .map_err(|e| Error::Network(format!("Failed to finalize order: {}", e)))?;

        // Download the certificate
        let cert = cert_order
            .download_and_save_cert()
            .map_err(|e| Error::Network(format!("Failed to download certificate: {}", e)))?;

        // Save the certificate and private key to our own paths
        let cert_path = self.domain_cert_path(domain);
        let key_path = self.domain_key_path(domain);

        // Save certificate chain
        std::fs::write(&cert_path, cert.certificate())
            .map_err(|e| Error::Io(e))?;

        // Save private key
        std::fs::write(&key_path, cert.private_key())
            .map_err(|e| Error::Io(e))?;

        info!("Certificate saved to: {:?}", cert_path);
        info!("Private key saved to: {:?}", key_path);

        // Parse certificate info
        let cert_info = self.parse_certificate_info(domain, &cert_path)?;

        // Store in cache
        self.certificates
            .write()
            .await
            .insert(domain.to_string(), cert_info.clone());

        info!(
            "Certificate obtained successfully for {} (expires: {})",
            domain, cert_info.expires_at
        );

        Ok(cert_info)
    }

    /// Obtain a certificate for multiple domains (SAN certificate)
    pub async fn obtain_certificate_multi(&self, domains: &[String]) -> Result<CertificateInfo> {
        if domains.is_empty() {
            return Err(Error::Validation("At least one domain is required".to_string()));
        }

        let primary_domain = &domains[0];
        info!(
            "Obtaining SAN certificate for {} domains, primary: {}",
            domains.len(),
            primary_domain
        );

        // Create the persistence layer
        let persist_dir = self.config.cache_dir.join("accounts");
        let persist = FilePersist::new(&persist_dir);

        // Select ACME directory
        let dir_url = if self.config.use_staging {
            DirectoryUrl::LetsEncryptStaging
        } else {
            DirectoryUrl::LetsEncrypt
        };

        // Create the ACME directory
        let dir = Directory::from_url(persist, dir_url)
            .map_err(|e| Error::Network(format!("Failed to create ACME directory: {}", e)))?;

        // Load the account
        let account = dir
            .account(&self.config.email)
            .map_err(|e| Error::Network(format!("Failed to load ACME account: {}", e)))?;

        // Convert domains to &str slice for new_order
        let alt_names: Vec<&str> = domains[1..].iter().map(|s| s.as_str()).collect();

        // Create a new order for all domains
        let mut order = account
            .new_order(primary_domain, &alt_names)
            .map_err(|e| Error::Network(format!("Failed to create certificate order: {}", e)))?;

        // Get the authorizations (challenges)
        let auths = order
            .authorizations()
            .map_err(|e| Error::Network(format!("Failed to get authorizations: {}", e)))?;

        // Handle each authorization
        for auth in &auths {
            // Check if we actually need to do the challenge
            if !auth.need_challenge() {
                info!("Authorization for {} is already valid, skipping challenge", auth.domain_name());
                continue;
            }

            // Get the HTTP-01 challenge
            let challenge = auth.http_challenge();

            let token = challenge.http_token().to_string();
            let proof = challenge.http_proof();

            info!("Handling HTTP-01 challenge for domain: {}", auth.domain_name());
            debug!("Challenge token: {}", token);

            // Save the challenge token
            self.save_challenge_token(auth.domain_name(), &token, &proof).await?;

            // Validate the challenge (timeout in milliseconds)
            let validation_result = challenge.validate(5000);
            
            // Clean up the challenge token regardless of result
            self.remove_challenge_token(auth.domain_name(), &token).await?;

            validation_result.map_err(|e| {
                Error::Network(format!("Challenge validation failed: {}", e))
            })?;

            info!("Authorization successful for domain: {}", auth.domain_name());
        }

        // Refresh the order to get updated status
        order.refresh().map_err(|e| {
            Error::Network(format!("Failed to refresh order: {}", e))
        })?;

        // Check if order is validated
        if !order.is_validated() {
            return Err(Error::Network(
                "Order is not validated after challenges".to_string(),
            ));
        }

        // Get the CSR order
        let csr_order = order.confirm_validations().ok_or_else(|| {
            Error::Network("Failed to confirm validations".to_string())
        })?;

        // Generate a private key for the certificate
        let private_key = create_rsa_key(2048);
        let private_key_pem = String::from_utf8(
            private_key.private_key_to_pem_pkcs8()
                .map_err(|e| Error::Other(format!("Failed to encode private key: {}", e)))?
        )
        .map_err(|e| Error::Other(format!("Invalid UTF-8 in private key: {}", e)))?;

        // Finalize the order and get the cert order
        let cert_order = csr_order
            .finalize(&private_key_pem, 5000)
            .map_err(|e| Error::Network(format!("Failed to finalize order: {}", e)))?;

        // Download the certificate
        let cert = cert_order
            .download_and_save_cert()
            .map_err(|e| Error::Network(format!("Failed to download certificate: {}", e)))?;

        // Save the certificate and private key to our own paths
        let cert_path = self.domain_cert_path(primary_domain);
        let key_path = self.domain_key_path(primary_domain);

        // Save certificate chain
        std::fs::write(&cert_path, cert.certificate())
            .map_err(|e| Error::Io(e))?;

        // Save private key
        std::fs::write(&key_path, cert.private_key())
            .map_err(|e| Error::Io(e))?;

        info!("SAN Certificate saved to: {:?}", cert_path);

        // Parse certificate info
        let cert_info = self.parse_certificate_info(primary_domain, &cert_path)?;

        // Store in cache for all domains
        let mut certs = self.certificates.write().await;
        for domain in domains {
            certs.insert(domain.clone(), cert_info.clone());
        }

        info!(
            "SAN Certificate obtained successfully for {} domains (expires: {})",
            domains.len(),
            cert_info.expires_at
        );

        Ok(cert_info)
    }

    /// Check if certificate should be renewed
    fn should_renew(&self, cert_info: &CertificateInfo) -> bool {
        let now = chrono::Utc::now();
        let renewal_threshold = chrono::Duration::days(self.config.renewal_threshold_days as i64);

        cert_info.expires_at - now < renewal_threshold
    }

    /// Get certificate info for a domain
    pub async fn get_certificate_info(&self, domain: &str) -> Result<Option<CertificateInfo>> {
        // Check cache first
        {
            let certs = self.certificates.read().await;
            if let Some(cert_info) = certs.get(domain) {
                return Ok(Some(cert_info.clone()));
            }
        }

        // Try to load from disk
        let cert_path = self.domain_cert_path(domain);
        if !cert_path.exists() {
            return Ok(None);
        }

        let cert_info = self.parse_certificate_info(domain, &cert_path)?;

        // Cache it
        self.certificates
            .write()
            .await
            .insert(domain.to_string(), cert_info.clone());

        Ok(Some(cert_info))
    }

    /// Parse certificate to extract metadata using x509-parser
    fn parse_certificate_info(&self, domain: &str, cert_path: &Path) -> Result<CertificateInfo> {
        use std::io::Read;

        let mut cert_file = std::fs::File::open(cert_path).map_err(Error::from)?;

        let mut cert_pem = Vec::new();
        cert_file
            .read_to_end(&mut cert_pem)
            .map_err(Error::from)?;

        // Parse the certificate using x509-parser
        let (_, cert) = x509_parser::pem::parse_x509_pem(&cert_pem)
            .map_err(|e| Error::Validation(format!("Failed to parse certificate PEM: {:?}", e)))?;

        let x509 = cert.parse_x509()
            .map_err(|e| Error::Validation(format!("Failed to parse X509 certificate: {:?}", e)))?;

        // Extract validity dates
        let validity = &x509.validity();
        
        let issued_at = chrono::DateTime::from_timestamp(validity.not_before.timestamp(), 0)
            .unwrap_or_else(|| chrono::Utc::now());
        
        let expires_at = chrono::DateTime::from_timestamp(validity.not_after.timestamp(), 0)
            .unwrap_or_else(|| chrono::Utc::now() + chrono::Duration::days(90));

        // Extract serial number
        let serial_number = x509.serial.to_string();

        Ok(CertificateInfo {
            domain: domain.to_string(),
            certificate_path: cert_path.to_path_buf(),
            private_key_path: self.domain_key_path(domain),
            expires_at,
            issued_at,
            serial_number,
        })
    }

    /// Get certificate paths
    pub fn domain_cert_path(&self, domain: &str) -> PathBuf {
        self.config
            .cache_dir
            .join(format!("{}-cert.pem", domain.replace('*', "wildcard")))
    }

    pub fn domain_key_path(&self, domain: &str) -> PathBuf {
        self.config
            .cache_dir
            .join(format!("{}-key.pem", domain.replace('*', "wildcard")))
    }

    /// Save HTTP-01 challenge token
    /// 
    /// The challenge token needs to be served at:
    /// http://<domain>/.well-known/acme-challenge/<token>
    pub async fn save_challenge_token(&self, domain: &str, token: &str, proof: &str) -> Result<()> {
        let challenge_path = self
            .config
            .cache_dir
            .join("challenges")
            .join(domain)
            .join(token);

        if let Some(parent) = challenge_path.parent() {
            std::fs::create_dir_all(parent).map_err(Error::from)?;
        }

        std::fs::write(&challenge_path, proof).map_err(Error::from)?;

        // Also store in memory for quick access
        self.challenge_tokens
            .write()
            .await
            .insert(token.to_string(), proof.to_string());

        info!(
            "Saved challenge token for {} at {:?}",
            domain, challenge_path
        );
        Ok(())
    }

    /// Remove HTTP-01 challenge token
    pub async fn remove_challenge_token(&self, domain: &str, token: &str) -> Result<()> {
        let challenge_path = self
            .config
            .cache_dir
            .join("challenges")
            .join(domain)
            .join(token);

        if challenge_path.exists() {
            std::fs::remove_file(&challenge_path).map_err(Error::from)?;
        }

        // Remove from memory cache
        self.challenge_tokens.write().await.remove(token);

        debug!("Removed challenge token for {}: {}", domain, token);
        Ok(())
    }

    /// Get challenge proof for a token (used by the HTTP challenge handler)
    pub async fn get_challenge_proof(&self, token: &str) -> Option<String> {
        self.challenge_tokens.read().await.get(token).cloned()
    }

    /// Start background certificate renewal task
    pub async fn start_renewal_task(self: Arc<Self>) -> Result<()> {
        if !self.config.auto_renew {
            info!("Certificate auto-renewal is disabled");
            return Ok(());
        }

        info!("Starting certificate renewal background task");

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(86_400)); // Check daily

            loop {
                interval.tick().await;

                if let Err(e) = self.check_and_renew_certificates().await {
                    error!("Certificate renewal task failed: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Check and renew certificates that are expiring
    async fn check_and_renew_certificates(&self) -> Result<()> {
        info!("Checking certificates for renewal");

        for domain in &self.config.domains {
            if let Ok(Some(cert_info)) = self.get_certificate_info(domain).await {
                if self.should_renew(&cert_info) {
                    info!("Certificate for {} is expiring, renewing", domain);

                    match self.obtain_certificate(domain).await {
                        Ok(_) => info!("Successfully renewed certificate for {}", domain),
                        Err(e) => error!("Failed to renew certificate for {}: {}", domain, e),
                    }
                } else {
                    debug!("Certificate for {} is still valid", domain);
                }
            } else {
                // No certificate exists, obtain one
                info!("No certificate found for {}, obtaining new one", domain);

                match self.obtain_certificate(domain).await {
                    Ok(_) => info!("Successfully obtained certificate for {}", domain),
                    Err(e) => error!("Failed to obtain certificate for {}: {}", domain, e),
                }
            }
        }

        Ok(())
    }

    /// Get all configured domains
    pub fn domains(&self) -> &[String] {
        &self.config.domains
    }

    /// Check if using staging environment
    pub fn is_staging(&self) -> bool {
        self.config.use_staging
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_lets_encrypt_config_validation() {
        let config = LetsEncryptConfig {
            email: "admin@example.com".to_string(),
            domains: vec!["example.com".to_string()],
            ..Default::default()
        };

        assert!(config.validate().is_ok());

        let invalid = LetsEncryptConfig {
            email: String::new(),
            domains: vec![],
            ..Default::default()
        };

        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_certificate_paths() {
        let config = LetsEncryptConfig {
            email: "admin@example.com".to_string(),
            domains: vec!["example.com".to_string()],
            cache_dir: PathBuf::from("/tmp/certs"),
            ..Default::default()
        };

        let manager = LetsEncryptManager::new(config).unwrap();

        assert_eq!(
            manager.domain_cert_path("example.com"),
            PathBuf::from("/tmp/certs/example.com-cert.pem")
        );
        assert_eq!(
            manager.domain_key_path("example.com"),
            PathBuf::from("/tmp/certs/example.com-key.pem")
        );
    }

    #[test]
    fn test_wildcard_domain_paths() {
        let config = LetsEncryptConfig {
            email: "admin@example.com".to_string(),
            domains: vec!["*.example.com".to_string()],
            cache_dir: PathBuf::from("/tmp/certs"),
            ..Default::default()
        };

        let manager = LetsEncryptManager::new(config).unwrap();

        assert_eq!(
            manager.domain_cert_path("*.example.com"),
            PathBuf::from("/tmp/certs/wildcard.example.com-cert.pem")
        );
    }
}
