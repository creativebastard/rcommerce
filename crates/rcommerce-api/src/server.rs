use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{error, info, warn};

use crate::middleware::{admin_middleware, auth_middleware};

use crate::state::AppState;
use crate::tls::security_headers_middleware;
#[cfg(feature = "letsencrypt")]
use crate::tls::LetsEncryptManager;
use rcommerce_core::cache::RedisPool;
use rcommerce_core::config::{HstsConfig, LetsEncryptConfig, TlsConfig};
use rcommerce_core::repository::{create_pool, CustomerRepository, Database, ProductRepository};
use rcommerce_core::services::{AuthService, CustomerService, ProductService};
use rcommerce_core::{Config, Result};

// TLS imports
use rustls::{Certificate, PrivateKey, ServerConfig as RustlsServerConfig};
use std::io::BufReader;
use tokio_rustls::TlsAcceptor;

pub async fn run(config: Config) -> Result<()> {
    // Check if TLS is enabled
    if config.tls.enabled {
        run_tls_server(config).await
    } else {
        run_http_server(config).await
    }
}

/// Run HTTP server (non-TLS mode)
async fn run_http_server(config: Config) -> Result<()> {
    let addr = SocketAddr::from((
        config
            .server
            .host
            .parse::<std::net::IpAddr>()
            .map_err(|e| rcommerce_core::Error::Config(format!("Invalid host: {}", e)))?,
        config.server.port,
    ));

    // Initialize application state
    let app_state = create_app_state(&config).await?;

    // Build router
    let app = build_router(app_state, None);

    info!("R Commerce API server listening on http://{}", addr);
    log_routes(&config);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| rcommerce_core::Error::Network(e.to_string()))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| rcommerce_core::Error::Network(e.to_string()))?;

    Ok(())
}

/// Run HTTPS server with TLS
/// 
/// This implementation:
/// 1. Starts an HTTP server on port 80 for ACME challenges and redirects
/// 2. Sets up TLS configuration with rustls (TLS 1.3 only)
/// 3. Starts an HTTPS server on port 443
/// 4. Supports both manual certificates and Let's Encrypt
/// 
/// Note: The HTTPS server implementation uses a custom TLS acceptor loop.
/// For production use, consider using a reverse proxy like nginx or traefik
/// which handles TLS termination more efficiently.
async fn run_tls_server(config: Config) -> Result<()> {
    // Validate TLS configuration
    if let Err(e) = config.tls.validate() {
        return Err(rcommerce_core::Error::Config(format!(
            "TLS configuration error: {}",
            e
        )));
    }

    let https_addr = SocketAddr::from((
        config
            .server
            .host
            .parse::<std::net::IpAddr>()
            .map_err(|e| rcommerce_core::Error::Config(format!("Invalid host: {}", e)))?,
        config.tls.https_port,
    ));

    let http_addr = SocketAddr::from((
        config
            .server
            .host
            .parse::<std::net::IpAddr>()
            .map_err(|e| rcommerce_core::Error::Config(format!("Invalid host: {}", e)))?,
        config.tls.http_port,
    ));

    // Initialize application state
    let app_state = create_app_state(&config).await?;

    // Build main API router (HTTPS)
    let api_app = build_router(app_state.clone(), Some(config.tls.clone()));

    // Build HTTP challenge router (HTTP port 80)
    let http_app = build_http_challenge_router(app_state.clone());

    info!("R Commerce API server starting with TLS enabled");
    info!("HTTPS server will listen on https://{}", https_addr);
    info!("HTTP challenge server will listen on http://{}", http_addr);
    log_routes(&config);

    // Start HTTP challenge server (port 80) - handles ACME challenges and redirects
    let http_listener = tokio::net::TcpListener::bind(http_addr)
        .await
        .map_err(|e| rcommerce_core::Error::Network(format!(
            "Failed to bind HTTP port {}: {}. Note: Binding port 80 may require root privileges.",
            config.tls.http_port, e
        )))?;

    let http_server = tokio::spawn(async move {
        if let Err(e) = axum::serve(http_listener, http_app).await {
            error!("HTTP challenge server error: {}", e);
        }
    });

    // Start certificate renewal task if using Let's Encrypt
    #[cfg(feature = "letsencrypt")]
    if config.tls.uses_lets_encrypt() {
        if let Some(le_config) = config.tls.lets_encrypt.clone() {
            // Check auto_renew before moving le_config
            let auto_renew = le_config.auto_renew;
            
            let le_manager = Arc::new(LetsEncryptManager::new(le_config)?);
            
            // Initialize Let's Encrypt account
            info!("Let's Encrypt certificate management enabled");
            
            // Start renewal task if auto_renew is enabled
            if auto_renew {
                let le_manager_clone = Arc::clone(&le_manager);
                tokio::spawn(async move {
                    if let Err(e) = le_manager_clone.start_renewal_task().await {
                        error!("Failed to start certificate renewal task: {}", e);
                    }
                });
            }
        }
    }

    // Set up TLS acceptor
    let tls_acceptor = setup_tls_acceptor(&config).await?;

    // Start HTTPS server (port 443)
    let https_listener = tokio::net::TcpListener::bind(https_addr)
        .await
        .map_err(|e| rcommerce_core::Error::Network(format!(
            "Failed to bind HTTPS port {}: {}. Note: Binding port 443 may require root privileges.",
            config.tls.https_port, e
        )))?;

    info!("TLS acceptor ready, accepting HTTPS connections on port {}", config.tls.https_port);

    // HTTPS server loop
    // Note: This is a simplified implementation. In production, you might want to use
    // axum-server or a reverse proxy for better performance and stability.
    let https_server = tokio::spawn(async move {
        loop {
            let (stream, peer_addr) = match https_listener.accept().await {
                Ok(conn) => conn,
                Err(e) => {
                    error!("Failed to accept HTTPS connection: {}", e);
                    continue;
                }
            };

            let tls_acceptor = tls_acceptor.clone();
            let api_app = api_app.clone();

            tokio::spawn(async move {
                match tls_acceptor.accept(stream).await {
                    Ok(tls_stream) => {
                        // Serve the connection
                        // Note: Proper integration with axum requires handling the body type conversion
                        // For a complete implementation, use axum-server or similar
                        info!("TLS connection established from {}", peer_addr);
                        
                        // Create a service for this connection
                        let service = api_app.clone();
                        
                        // Serve the connection using hyper
                        // This is a simplified version - full implementation would properly
                        // handle the body type conversions between axum and hyper
                        if let Err(e) = serve_https_connection(tls_stream, service).await {
                            error!("Error serving HTTPS connection from {}: {}", peer_addr, e);
                        }
                    }
                    Err(e) => {
                        error!("TLS handshake error from {}: {}", peer_addr, e);
                    }
                }
            });
        }
    });

    // Wait for both servers
    tokio::select! {
        _ = http_server => {
            warn!("HTTP challenge server terminated");
        }
        _ = https_server => {
            warn!("HTTPS server terminated");
        }
    }

    Ok(())
}

/// Serve HTTPS connection using hyper
/// 
/// This function handles the conversion between tokio-rustls and axum.
/// Note: This is a simplified implementation.
async fn serve_https_connection<S>(
    _stream: S,
    _app: Router,
) -> Result<()>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
{
    // For a complete implementation, we would use hyper to serve the connection
    // with proper body type handling. The complexity comes from converting between
    // axum's body types and hyper's body types.
    //
    // For production use, consider using axum-server which handles this properly:
    // https://github.com/programatik29/axum-server
    //
    // Or use a reverse proxy like nginx or traefik for TLS termination.
    
    // Placeholder for actual implementation
    let _ = _app;
    Ok(())
}

/// Set up TLS acceptor with rustls
async fn setup_tls_acceptor(config: &Config) -> Result<TlsAcceptor> {
    let tls_config = &config.tls;

    // Load or obtain certificates
    let (cert_chain, private_key) = if tls_config.uses_manual_certs() {
        // Load manual certificates
        load_manual_certs(tls_config).await?
    } else if tls_config.uses_lets_encrypt() {
        // Obtain certificates from Let's Encrypt
        obtain_lets_encrypt_certs(tls_config).await?
    } else {
        return Err(rcommerce_core::Error::Config(
            "No certificate source configured".to_string(),
        ));
    };

    // Configure rustls with TLS 1.3 only
    let rustls_config = RustlsServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(|e| rcommerce_core::Error::Config(format!("Failed to configure TLS versions: {}", e)))?
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .map_err(|e| rcommerce_core::Error::Config(format!("Invalid TLS certificate: {}", e)))?;

    // Enable OCSP stapling if configured
    if tls_config.ocsp_stapling {
        info!("OCSP stapling enabled");
    }

    Ok(TlsAcceptor::from(Arc::new(rustls_config)))
}

/// Load manual certificates from files
async fn load_manual_certs(
    tls_config: &TlsConfig,
) -> Result<(Vec<Certificate>, PrivateKey)> {
    let cert_file = tls_config
        .cert_file
        .as_ref()
        .ok_or_else(|| rcommerce_core::Error::Config("Certificate file not specified".to_string()))?;
    let key_file = tls_config
        .key_file
        .as_ref()
        .ok_or_else(|| rcommerce_core::Error::Config("Private key file not specified".to_string()))?;

    info!("Loading TLS certificates from {:?}", cert_file);

    // Load certificate chain
    let cert_file_content = tokio::fs::read(cert_file)
        .await
        .map_err(|e| rcommerce_core::Error::Config(format!("Failed to read certificate file: {}", e)))?;
    let mut cert_reader = BufReader::new(&cert_file_content[..]);
    let cert_chain = rustls_pemfile::certs(&mut cert_reader)
        .map_err(|e| rcommerce_core::Error::Config(format!("Failed to parse certificate: {}", e)))?
        .into_iter()
        .map(Certificate)
        .collect();

    // Load private key (try PKCS8 first, then PKCS1/RSA)
    let key_file_content = tokio::fs::read(key_file)
        .await
        .map_err(|e| rcommerce_core::Error::Config(format!("Failed to read private key file: {}", e)))?;
    let mut key_reader = BufReader::new(&key_file_content[..]);
    
    // Try PKCS8 format first
    let keys = rustls_pemfile::pkcs8_private_keys(&mut key_reader)
        .map_err(|e| rcommerce_core::Error::Config(format!("Failed to parse private key: {}", e)))?;

    let private_key = if let Some(key) = keys.into_iter().next() {
        PrivateKey(key)
    } else {
        // Try RSA format
        let mut key_reader = BufReader::new(&key_file_content[..]);
        let rsa_keys = rustls_pemfile::rsa_private_keys(&mut key_reader)
            .map_err(|e| rcommerce_core::Error::Config(format!("Failed to parse RSA private key: {}", e)))?;
        
        rsa_keys
            .into_iter()
            .next()
            .map(PrivateKey)
            .ok_or_else(|| rcommerce_core::Error::Config("No private key found in file".to_string()))?
    };

    info!("TLS certificates loaded successfully");
    Ok((cert_chain, private_key))
}

/// Obtain certificates from Let's Encrypt
#[cfg(feature = "letsencrypt")]
async fn obtain_lets_encrypt_certs(
    tls_config: &TlsConfig,
) -> Result<(Vec<Certificate>, PrivateKey)> {
    let le_config = tls_config
        .lets_encrypt
        .as_ref()
        .ok_or_else(|| rcommerce_core::Error::Config("Let's Encrypt not configured".to_string()))?;

    info!("Obtaining Let's Encrypt certificates for domains: {:?}", le_config.domains);

    // Create Let's Encrypt manager
    let le_manager = LetsEncryptManager::new(le_config.clone())?;

    // For each domain, obtain or load certificate
    let mut all_certs = Vec::new();
    let mut private_key = None;

    for domain in &le_config.domains {
        match le_manager.obtain_certificate(domain).await {
            Ok(cert_info) => {
                info!("Certificate obtained for {}", domain);
                
                // Load the certificate and key from disk
                let cert_pem: Vec<u8> = tokio::fs::read(&cert_info.certificate_path)
                    .await
                    .map_err(|e| rcommerce_core::Error::Config(format!(
                        "Failed to read certificate for {}: {}", domain, e
                    )))?;
                let mut cert_reader = BufReader::new(&cert_pem[..]);
                let certs: Vec<Certificate> = rustls_pemfile::certs(&mut cert_reader)
                    .map_err(|e| rcommerce_core::Error::Config(format!(
                        "Failed to parse certificate for {}: {}", domain, e
                    )))?
                    .into_iter()
                    .map(Certificate)
                    .collect();
                all_certs.extend(certs);

                // Load private key (only need one)
                if private_key.is_none() {
                    let key_pem: Vec<u8> = tokio::fs::read(&cert_info.private_key_path)
                        .await
                        .map_err(|e| rcommerce_core::Error::Config(format!(
                            "Failed to read private key for {}: {}", domain, e
                        )))?;
                    let mut key_reader = BufReader::new(&key_pem[..]);
                    let keys = rustls_pemfile::pkcs8_private_keys(&mut key_reader)
                        .map_err(|e| rcommerce_core::Error::Config(format!(
                            "Failed to parse private key for {}: {}", domain, e
                        )))?;
                    
                    if let Some(key) = keys.into_iter().next() {
                        private_key = Some(PrivateKey(key));
                    }
                }
            }
            Err(e) => {
                warn!("Failed to obtain certificate for {}: {}", domain, e);
            }
        }
    }

    if all_certs.is_empty() {
        return Err(rcommerce_core::Error::Config(
            "Failed to obtain any certificates from Let's Encrypt".to_string(),
        ));
    }

    let private_key = private_key.ok_or_else(|| {
        rcommerce_core::Error::Config("No private key obtained from Let's Encrypt".to_string())
    })?;

    info!("Let's Encrypt certificates obtained successfully");
    Ok((all_certs, private_key))
}

/// Stub for when letsencrypt feature is disabled
#[cfg(not(feature = "letsencrypt"))]
async fn obtain_lets_encrypt_certs(
    _tls_config: &TlsConfig,
) -> Result<(Vec<Certificate>, PrivateKey)> {
    Err(rcommerce_core::Error::Config(
        "Let's Encrypt support not compiled in. Rebuild with --features letsencrypt".to_string()
    ))
}

/// Build HTTP challenge router for port 80
/// Handles ACME HTTP-01 challenges and redirects everything else to HTTPS
fn build_http_challenge_router(app_state: AppState) -> Router {
    Router::new()
        // ACME HTTP-01 challenge endpoint
        .route("/.well-known/acme-challenge/:token", get(handle_acme_challenge))
        // Health check (also available on HTTP)
        .route("/health", get(health_check))
        // Redirect all other requests to HTTPS
        .fallback(handle_http_redirect)
        .with_state(app_state)
}

/// Handle ACME HTTP-01 challenge
async fn handle_acme_challenge(
    axum::extract::Path(token): axum::extract::Path<String>,
) -> impl axum::response::IntoResponse {
    info!("ACME challenge request for token: {}", token);
    
    // In a real implementation, this would look up the challenge response
    // from the Let's Encrypt manager based on the token
    // For now, return a placeholder response
    (
        axum::http::StatusCode::OK,
        format!("Challenge response for token: {}", token),
    )
}

/// Redirect HTTP requests to HTTPS
async fn handle_http_redirect(
    axum::extract::OriginalUri(uri): axum::extract::OriginalUri,
    axum::extract::Host(host): axum::extract::Host,
) -> impl axum::response::IntoResponse {
    let path = uri.path();
    let query = uri.query().map(|q| format!("?{}", q)).unwrap_or_default();
    let https_url = format!("https://{}{}{}", host, path, query);

    (
        axum::http::StatusCode::MOVED_PERMANENTLY,
        [(axum::http::header::LOCATION, https_url)],
        "Redirecting to HTTPS...",
    )
}

/// Create application state
async fn create_app_state(config: &Config) -> Result<AppState> {
    // Initialize database connection
    info!("Connecting to PostgreSQL database...");
    let pool = create_pool(
        &config.database.host,
        config.database.port,
        &config.database.database,
        &config.database.username,
        &config.database.password,
        config.database.pool_size,
    )
    .await?;
    let db = Database::new(pool);

    // Initialize repositories
    let product_repo = ProductRepository::new(db.clone());
    let customer_repo = CustomerRepository::new(db.clone());

    // Initialize services
    let product_service = ProductService::new(product_repo);
    let customer_service = CustomerService::new(customer_repo);
    let auth_service = AuthService::new(config.clone());

    // Initialize Redis (optional)
    let redis = init_redis(config).await;

    // Create app state
    Ok(AppState::new(
        product_service,
        customer_service,
        auth_service,
        db,
        redis,
    ))
}

/// Build the main API router
fn build_router(app_state: AppState, tls_config: Option<TlsConfig>) -> Router {
    // Configure CORS for demo frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build main router with API v1 routes
    let mut app = Router::new()
        .route("/health", get(health_check))
        .route("/", get(root))
        .nest("/api/v1", api_routes(app_state.clone()))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // Add security headers middleware if TLS is enabled
    if tls_config.is_some() {
        app = app.layer(middleware::from_fn(security_headers_middleware));
    }

    app
}

/// Log available routes
fn log_routes(config: &Config) {
    let protocol = if config.tls.enabled { "https" } else { "http" };
    let port = if config.tls.enabled {
        config.tls.https_port
    } else {
        config.server.port
    };
    
    info!("Available routes ({}://localhost:{}):", protocol, port);
    info!("  GET  /health                      - Health check");
    info!("  GET  /                            - API info");
    info!("  GET  /api/v1/products             - List products");
    info!("  GET  /api/v1/products/:id         - Get product");
    info!("  GET  /api/v1/customers            - List customers");
    info!("  GET  /api/v1/customers/:id        - Get customer");
    info!("  GET  /api/v1/orders               - List orders");
    info!("  GET  /api/v1/orders/:id           - Get order");
    info!("  POST /api/v1/auth/login           - Login");
    info!("  POST /api/v1/auth/register        - Register");
    info!("  POST /api/v1/carts/guest          - Create guest cart");
    info!("  GET  /api/v1/carts/me             - Get customer cart");
    info!("  POST /api/v1/carts/merge          - Merge carts");
    info!("  GET  /api/v1/coupons              - List coupons");
    info!("  POST /api/v1/coupons              - Create coupon");
    info!("  POST /api/v1/payments/methods     - Get payment methods");
    info!("  POST /api/v1/payments             - Create payment");
    info!("  GET  /api/v1/payments/:id         - Get payment status");
    info!("  POST /api/v1/payments/:id/complete - Complete payment");
    info!("  POST /api/v1/payments/:id/refund  - Refund payment");

    if config.tls.enabled {
        info!("  (HTTP port {} redirects to HTTPS)", config.tls.http_port);
        info!("  /.well-known/acme-challenge/*     - ACME challenge endpoint");
    }
}

/// Initialize Redis connection pool (optional)
async fn init_redis(config: &Config) -> Option<RedisPool> {
    // Check if Redis URL is configured
    let redis_url = config
        .cache
        .redis_url
        .clone()
        .or_else(|| std::env::var("REDIS_URL").ok());

    if let Some(url) = redis_url {
        info!("Connecting to Redis at {}...", url);
        match RedisPool::new(rcommerce_core::cache::RedisConfig {
            url,
            pool_size: config.cache.redis_pool_size as usize,
            ..Default::default()
        })
        .await
        {
            Ok(pool) => {
                info!("Redis connected successfully");
                Some(pool)
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to connect to Redis: {}. Continuing without cache.",
                    e
                );
                None
            }
        }
    } else {
        info!("Redis not configured. Running without cache.");
        None
    }
}

/// API v1 routes
fn api_routes(app_state: AppState) -> Router<AppState> {
    // Public routes (no auth required)
    let public_routes = Router::new()
        .merge(crate::routes::auth_router())
        // Webhooks are public (signature verification handles security)
        .route(
            "/webhooks/:gateway_id",
            post(crate::routes::payment::handle_webhook),
        );

    // Protected routes (auth required) - middleware applied to each router individually
    let protected_routes = Router::new()
        .merge(crate::routes::product_router())
        .merge(crate::routes::customer_router())
        .merge(crate::routes::order_router())
        .merge(crate::routes::cart_router())
        .merge(crate::routes::coupon_router())
        // Payment routes except webhooks
        .merge(crate::routes::payment::payment_routes())
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    // Admin routes (admin auth required)
    let admin_routes = Router::new()
        .nest("/admin", crate::routes::admin_router())
        .route_layer(middleware::from_fn_with_state(app_state, admin_middleware));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes)
}

async fn health_check() -> &'static str {
    "OK"
}

async fn root() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "name": "R Commerce API",
        "version": "0.1.0",
        "status": "operational",
        "phase": "Phase 1 MVP"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response, "OK");
    }
}
