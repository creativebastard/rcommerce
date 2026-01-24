use acme_lib::persist::{Persist, FilePersist};
use acme_lib::{Account, Certificate, Directory, DirectoryUrl};
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

use crate::{Result, Error};
use super::config::{LetsEncryptConfig, TlsConfig};

/// Let's Encrypt certificate manager
pub struct LetsEncryptManager {
    config: LetsEncryptConfig,
    persist: FilePersist,
    account: Option<Account>,
    certificates: Arc<RwLock<HashMap<String, CertificateInfo>>>,
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
            .map_err(|e| Error::config(format!("Failed to create cert cache dir: {}", e)))?;
        
        let persist = FilePersist::new(&config.cache_dir);
        
        Ok(Self {
            config,
            persist,
            account: None,
            certificates: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Initialize Let's Encrypt account
    pub async fn init_account(&mut self) -> Result<()> {
        info!("Initializing Let's Encrypt account for {}", self.config.email);
        
        let directory_url = if self.config.use_staging {
            DirectoryUrl::LetsEncryptStaging
        } else {
            DirectoryUrl::LetsEncrypt
        };
        
        let dir = Directory::from_url(self.persist.clone(), directory_url)
            .map_err(|e| Error::config(format!("Failed to create ACME directory: {}", e)))?;
        
        let account = dir.register_account(
            vec![self.config.email.clone()],
            true, // Accept terms of service
        )
        .map_err(|e| Error::config(format!("Failed to register ACME account: {}", e)))?;
        
        self.account = Some(account);
        info!("Let's Encrypt account created successfully");
        
        Ok(())
    }
    
    /// Obtain or renew certificate for a domain
    pub async fn obtain_certificate(&self, domain: &str) -> Result<CertificateInfo> {
        info!("Obtaining certificate for domain: {}", domain);
        
        let account = self.account.as_ref()
            .ok_or_else(|| Error::config("Let's Encrypt account not initialized"))?;
        
        // Check if we already have a valid certificate
        if let Some(cert_info) = self.get_certificate_info(domain).await? {
            if !self.should_renew(&cert_info) {
                info!("Certificate for {} is still valid", domain);
                return Ok(cert_info);
            }
            
            info!("Certificate for {} needs renewal", domain);
        }
        
        // Create a new order for the domain
        let mut new_order = account.new_order(domain, false)
            .map_err(|e| Error::payment(format!("Failed to create certificate order: {}", e)))?;
        
        // Authorize the order (handle HTTP-01 challenge)
        let auths = new_order.authorizations()
            .map_err(|e| Error::payment(format!("Failed to get authorizations: {}", e)))?;
        
        for auth in auths {
            let challenge = auth.http_challenge();
            
            // Save the challenge token for the HTTP server to serve
            let token = challenge.http_token();
            let proof = challenge.http_proof()
                .map_err(|e| Error::payment(format!("Failed to get challenge proof: {}", e)))?;
            
            self.save_challenge_token(domain, token, &proof).await?;
            
            // Start the challenge validation
            challenge.validate()
                .map_err(|e| Error::payment(format!("Failed to validate challenge: {}", e)))?;
            
            // Wait for challenge to complete
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
            loop {
                let status = challenge.poll_status()
                    .map_err(|e| Error::payment(format!("Failed to poll challenge status: {}", e)))?;
                
                if status.is_valid() {
                    info!("Challenge validated for {}", domain);
                    break;
                } else if status.is_invalid() {
                    return Err(Error::payment("Challenge validation failed".to_string()));
                }
                
                if std::time::Instant::now() > deadline {
                    return Err(Error::payment("Challenge validation timeout".to_string()));
                }
                
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            
            // Clean up challenge token
            self.remove_challenge_token(domain, token).await?;
        }
        
        // Finalize the order and get the certificate
        let cert = new_order.finalize()
            .map_err(|e| Error::payment(format!("Failed to finalize order: {}", e)))?;
        
        // Download the certificate
        let cert_pem = cert.certificate()
            .map_err(|e| Error::payment(format!("Failed to download certificate: {}", e)))?
            .ok_or_else(|| Error::payment("No certificate returned".to_string()))?
            .to_pem()
            .map_err(|e| Error::payment(format!("Failed to convert certificate to PEM: {}", e)))?;
        
        // Save certificate and private key
        let cert_path = self.domain_cert_path(domain);
        let key_path = self.domain_key_path(domain);
        
        // Extract private key from order
        let private_key_pem = new_order.certificate().unwrap().private_key_pem()
            .map_err(|e| Error::payment(format!("Failed to get private key: {}", e)))?;
        
        use std::io::Write;
        let mut cert_file = std::fs::File::create(&cert_path)
            .map_err(|e| Error::io(format!("Failed to create cert file: {}", e)))?;
        cert_file.write_all(cert_pem.as_bytes())
            .map_err(|e| Error::io(format!("Failed to write cert: {}", e)))?;
        
        let mut key_file = std::fs::File::create(&key_path)
            .map_err(|e| Error::io(format!("Failed to create key file: {}", e)))?;
        key_file.write_all(private_key_pem.as_bytes())
            .map_err(|e| Error::io(format!("Failed to write key: {}", e)))?;
        
        // Parse certificate to get metadata
        let cert_info = self.parse_certificate_info(domain, &cert_path)?;
        
        // Store in cache
        self.certificates.write().await.insert(domain.to_string(), cert_info.clone());
        
        info!("Certificate obtained for {} (expires: {})", domain, cert_info.expires_at);
        Ok(cert_info)
    }
    
    /// Check if certificate should be renewed
    fn should_renew(&self, cert_info: &CertificateInfo) -> bool {
        let now = chrono::Utc::now();
        let renewal_threshold = chrono::Duration::days(self.config.renewal_threshold_days as i64);
        
        cert_info.expires_at - now < renewal_threshold
    }
    
    /// Get certificate info for a domain
    async fn get_certificate_info(&self, domain: &str) -> Result<Option<CertificateInfo>> {
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
        self.certificates.write().await.insert(domain.to_string(), cert_info.clone());
        
        Ok(Some(cert_info))
    }
    
    /// Parse certificate to extract metadata
    fn parse_certificate_info(&self, domain: &str, cert_path: &Path) -> Result<CertificateInfo> {
        use std::io::Read;
        
        let mut cert_file = std::fs::File::open(cert_path)
            .map_err(|e| Error::io(format!("Failed to open certificate: {}", e)))?;
        
        let mut cert_pem = String::new();
        cert_file.read_to_string(&mut cert_pem)
            .map_err(|e| Error::io(format!("Failed to read certificate: {}", e)))?;
        
        // Parse the certificate (extract expiration date, serial number, etc.)
        // This is simplified - in production, use a proper X.509 parser
        
        // For now, assume the certificate is valid for 90 days from now
        let issued_at = chrono::Utc::now();
        let expires_at = issued_at + chrono::Duration::days(90);
        
        Ok(CertificateInfo {
            domain: domain.to_string(),
            certificate_path: cert_path.to_path_buf(),
            private_key_path: self.domain_key_path(domain),
            expires_at,
            issued_at,
            serial_number: format!("sn-{}", uuid::Uuid::new_v4()), // Placeholder
        })
    }
    
    /// Get certificate paths
    fn domain_cert_path(&self, domain: &str) -> PathBuf {
        self.config.cache_dir.join(format!("{}-cert.pem", domain.replace('*', "wildcard")))
    }
    
    fn domain_key_path(&self, domain: &str) -> PathBuf {
        self.config.cache_dir.join(format!("{}-key.pem", domain.replace('*', "wildcard")))
    }
    
    /// Save HTTP-01 challenge token
    async fn save_challenge_token(&self, domain: &str, token: &str, proof: &str) -> Result<()> {
        let challenge_path = self.config.cache_dir.join("challenges").join(domain).join(token);
        
        if let Some(parent) = challenge_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| Error::io(format!("Failed to create challenge dir: {}", e)))?;
        }
        
        std::fs::write(&challenge_path, proof)
            .map_err(|e| Error::io(format!("Failed to write challenge token: {}", e)))?;
        
        info!("Saved challenge token for {} at {:?}", domain, challenge_path);
        Ok(())
    }
    
    /// Remove HTTP-01 challenge token
    async fn remove_challenge_token(&self, domain: &str, token: &str) -> Result<()> {
        let challenge_path = self.config.cache_dir.join("challenges").join(domain).join(token);
        
        if challenge_path.exists() {
            std::fs::remove_file(&challenge_path)
                .map_err(|e| Error::io(format!("Failed to remove challenge token: {}", e)))?;
        }
        
        Ok(())
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
                }
            }
        }
        
        Ok(())
    }
}

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    
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
}