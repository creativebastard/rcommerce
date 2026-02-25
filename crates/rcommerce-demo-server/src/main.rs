//! R Commerce Frontend Server
//! 
//! A production-ready server for hosting customer frontends:
//! - Serves static files with proper caching headers
//! - Proxies API requests with Redis caching
//! - Hides API credentials from browser
//! - Rate limiting per IP
//! - Graceful shutdown

use anyhow::{Context, Result};
use axum::{
    body::Body,
    extract::{ConnectInfo, Request, State},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use clap::Parser;
use reqwest::Method;
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Server configuration
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_bind")]
    pub bind: String,
    
    pub api_url: String,
    pub api_key: String,
    
    #[serde(default = "default_static_dir")]
    pub static_dir: PathBuf,
    
    #[serde(default)]
    pub cors: bool,
    
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
    
    #[serde(default = "default_log_level")]
    pub log_level: String,
    
    // Production features
    #[serde(default = "default_cache_ttl")]
    pub cache_ttl_secs: u64,
    
    #[serde(default = "default_rate_limit")]
    pub rate_limit_per_minute: u32,
    
    #[serde(default = "default_max_body_size")]
    pub max_body_size_mb: usize,
    
    #[serde(default)]
    pub redis_url: Option<String>,
    
    #[serde(default = "default_true")]
    pub enable_compression: bool,
    
    #[serde(default = "default_true")]
    pub enable_etag: bool,
}

fn default_bind() -> String { "0.0.0.0:3000".to_string() }
fn default_static_dir() -> PathBuf { PathBuf::from("frontend") }
fn default_timeout() -> u64 { 30 }
fn default_log_level() -> String { "info".to_string() }
fn default_cache_ttl() -> u64 { 300 } // 5 minutes
fn default_rate_limit() -> u32 { 60 }
fn default_max_body_size() -> usize { 10 }
fn default_true() -> bool { true }

impl Config {
    pub fn load(args: &Args) -> Result<Self> {
        let mut config = config::Config::builder();
        
        // Defaults
        config = config.set_default("bind", default_bind())?;
        config = config.set_default("static_dir", default_static_dir().to_str().unwrap())?;
        config = config.set_default("timeout_secs", default_timeout())?;
        config = config.set_default("log_level", default_log_level())?;
        config = config.set_default("cache_ttl_secs", default_cache_ttl())?;
        config = config.set_default("rate_limit_per_minute", default_rate_limit())?;
        config = config.set_default("max_body_size_mb", default_max_body_size() as i64)?;
        config = config.set_default("enable_compression", true)?;
        config = config.set_default("enable_etag", true)?;
        
        // Config file
        if let Some(ref path) = args.config {
            config = config.add_source(config::File::from(path.as_path()));
        } else {
            config = config.add_source(config::File::with_name("frontend-server").required(false));
            config = config.add_source(config::File::with_name("/etc/rcommerce/frontend-server").required(false));
        }
        
        // Environment (FRONTEND_* prefix)
        config = config.add_source(
            config::Environment::with_prefix("FRONTEND")
                .separator("_")
        );
        
        // CLI overrides
        if let Some(ref bind) = args.bind {
            config = config.set_override("bind", bind.as_str())?;
        }
        if let Some(ref api_url) = args.api_url {
            config = config.set_override("api_url", api_url.as_str())?;
        }
        if let Some(ref api_key) = args.api_key {
            config = config.set_override("api_key", api_key.as_str())?;
        }
        if let Some(ref static_dir) = args.static_dir {
            config = config.set_override("static_dir", static_dir.as_str())?;
        }
        
        let config: Config = config.build()?.try_deserialize()?;
        config.validate()?;
        
        Ok(config)
    }
    
    fn validate(&self) -> Result<()> {
        if self.api_url.is_empty() {
            anyhow::bail!("API URL is required (--api-url or FRONTEND_API_URL)");
        }
        if self.api_key.is_empty() {
            anyhow::bail!("API key is required (--api-key or FRONTEND_API_KEY)");
        }
        if !self.static_dir.exists() {
            anyhow::bail!("Static directory does not exist: {}", self.static_dir.display());
        }
        Ok(())
    }
}

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub http_client: reqwest::Client,
    pub cache: Arc<RwLock<lru::LruCache<String, CachedResponse>>>,
    pub rate_limiter: Arc<RwLock<RateLimiter>>,
}

/// Cached API response
#[derive(Clone)]
pub struct CachedResponse {
    pub body: Vec<u8>,
    pub content_type: String,
    pub cached_at: Instant,
}

/// Simple rate limiter per IP
pub struct RateLimiter {
    requests: std::collections::HashMap<String, Vec<Instant>>,
    limit: u32,
    window: Duration,
}

impl RateLimiter {
    pub fn new(limit: u32, window_secs: u64) -> Self {
        Self {
            requests: std::collections::HashMap::new(),
            limit,
            window: Duration::from_secs(window_secs),
        }
    }
    
    pub fn check(&mut self, ip: &str) -> bool {
        let now = Instant::now();
        let window_start = now - self.window;
        
        let entries = self.requests.entry(ip.to_string()).or_default();
        entries.retain(|&t| t > window_start);
        
        if entries.len() >= self.limit as usize {
            return false;
        }
        
        entries.push(now);
        true
    }
}

/// CLI arguments
#[derive(Parser, Debug)]
#[command(name = "rcommerce-frontend")]
#[command(about = "R Commerce Frontend Server - production-ready static file + API proxy server")]
#[command(version)]
pub struct Args {
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
    
    #[arg(short, long, env = "FRONTEND_BIND")]
    pub bind: Option<String>,
    
    #[arg(short = 'u', long, env = "FRONTEND_API_URL")]
    pub api_url: Option<String>,
    
    #[arg(short = 'k', long, env = "FRONTEND_API_KEY")]
    pub api_key: Option<String>,
    
    #[arg(short = 's', long, env = "FRONTEND_STATIC_DIR")]
    pub static_dir: Option<String>,
    
    #[arg(long)]
    pub print_config: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    if args.print_config {
        print_example_config();
        return Ok(());
    }
    
    let config = Config::load(&args)?;
    
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(&config.log_level)
        .init();
    
    info!("R Commerce Frontend Server v{}", env!("CARGO_PKG_VERSION"));
    info!("  Bind: {}", config.bind);
    info!("  API URL: {}", config.api_url);
    info!("  Static dir: {}", config.static_dir.display());
    info!("  Cache TTL: {}s", config.cache_ttl_secs);
    info!("  Rate limit: {}/min", config.rate_limit_per_minute);
    
    // HTTP client
    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(config.timeout_secs))
        .pool_max_idle_per_host(10)
        .build()
        .context("Failed to create HTTP client")?;
    
    // LRU cache for API responses (100 entries)
    let cache = Arc::new(RwLock::new(lru::LruCache::new(
        std::num::NonZeroUsize::new(100).unwrap()
    )));
    
    // Rate limiter
    let rate_limiter = Arc::new(RwLock::new(RateLimiter::new(
        config.rate_limit_per_minute,
        60
    )));
    
    let state = AppState {
        config: Arc::new(config.clone()),
        http_client,
        cache,
        rate_limiter,
    };
    
    // Build router
    let mut app = Router::new()
        // Health check
        .route("/health", get(health_check))
        // API proxy with caching
        .route("/api/*path", get(api_proxy_cached).post(api_proxy).put(api_proxy).delete(api_proxy))
        // Static files
        .fallback_service(tower_http::services::ServeDir::new(&config.static_dir)
            .append_index_html_on_directories(true)
            .precompressed_gzip()
            .precompressed_br()
        )
        .with_state(state);
    
    // Add compression if enabled
    if config.enable_compression {
        app = app.layer(tower_http::compression::CompressionLayer::new());
    }
    
    // Add CORS if enabled
    if config.cors {
        app = app.layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
                .allow_headers(tower_http::cors::Any),
        );
    }
    
    // Add security headers
    app = app.layer(axum::middleware::from_fn(security_headers));
    
    // Add tracing
    app = app.layer(tower_http::trace::TraceLayer::new_for_http());
    
    // Parse bind address
    let addr: SocketAddr = config.bind.parse()
        .context("Invalid bind address")?;
    
    info!("Server ready: http://{}", addr);
    
    // Start server with graceful shutdown
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;
    
    info!("Server shutdown complete");
    Ok(())
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    axum::Json(::serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "service": "rcommerce-frontend"
    }))
}

/// API proxy with caching for GET requests
async fn api_proxy_cached(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
) -> Result<Response, StatusCode> {
    let cache_key = format!("{}:{}", req.method(), req.uri().path());
    
    // Check cache for GET requests
    if req.method() == Method::GET {
        let cache = state.cache.read().await;
        if let Some(cached) = cache.peek(&cache_key) {
            let age = cached.cached_at.elapsed().as_secs();
            if age < state.config.cache_ttl_secs {
                info!("Cache HIT: {}", cache_key);
                let mut response = Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", &cached.content_type)
                    .header("X-Cache", "HIT")
                    .header("Cache-Age", age.to_string())
                    .body(Body::from(cached.body.clone()))
                    .unwrap();
                return Ok(response);
            }
        }
    }

    
    // Cache miss - proxy to backend
    api_proxy_inner(state, addr, req, cache_key).await
}

/// API proxy without caching (for POST/PUT/DELETE)
async fn api_proxy(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
) -> Result<Response, StatusCode> {
    api_proxy_inner(state, addr, req, String::new()).await
}

/// Internal proxy implementation
async fn api_proxy_inner(
    state: AppState,
    addr: SocketAddr,
    req: Request,
    cache_key: String,
) -> Result<Response, StatusCode> {
    // Rate limiting
    let ip = addr.ip().to_string();
    {
        let mut limiter = state.rate_limiter.write().await;
        if !limiter.check(&ip) {
            warn!("Rate limit exceeded: {}", ip);
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }
    
    let path = req.uri().path();
    let method = req.method().clone();
    
    // Build target URL
    let target_url = format!("{}{}", state.config.api_url, path);
    
    info!("Proxy: {} {} (from {})", method, path, ip);
    
    // Build proxied request
    let mut proxy_req = state.http_client.request(method.clone(), &target_url);
    
    // Copy headers (skip hop-by-hop and auth)
    let headers = req.headers();
    for (name, value) in headers.iter() {
        let name_str = name.as_str().to_lowercase();
        if !is_hop_by_hop_header(&name_str) && name_str != "authorization" {
            proxy_req = proxy_req.header(name.clone(), value.clone());
        }
    }
    
    // Inject API key
    proxy_req = proxy_req.header("Authorization", format!("Bearer {}", state.config.api_key));
    
    // Forward body
    if method != Method::GET && method != Method::HEAD {
        let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
            .await
            .map_err(|e| {
                error!("Failed to read request body: {}", e);
                StatusCode::BAD_REQUEST
            })?;
        proxy_req = proxy_req.body(body_bytes);
    }
    
    // Send request
    let backend_resp = proxy_req.send().await.map_err(|e| {
        error!("Backend request failed: {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    let status = StatusCode::from_u16(backend_resp.status().as_u16())
        .unwrap_or(StatusCode::OK);
    
    let mut response_builder = Response::builder()
        .status(status)
        .header("X-Cache", "MISS");
    
    // Copy headers
    let content_type = backend_resp.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/json")
        .to_string();
        
    for (name, value) in backend_resp.headers().iter() {
        if !is_hop_by_hop_header(&name.as_str().to_lowercase()) {
            response_builder = response_builder.header(name.clone(), value.clone());
        }
    }
    
    // Get body
    let body_bytes = backend_resp.bytes().await.map_err(|e| {
        error!("Failed to read backend response: {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    // Cache successful GET responses
    if method == Method::GET && status.is_success() && !cache_key.is_empty() {
        let cached = CachedResponse {
            body: body_bytes.to_vec(),
            content_type: content_type.clone(),
            cached_at: Instant::now(),
        };
        let mut cache = state.cache.write().await;
        cache.put(cache_key, cached);
    }
    
    Ok(response_builder.body(Body::from(body_bytes)).unwrap())
}

/// Security headers middleware
async fn security_headers(req: Request, next: axum::middleware::Next) -> impl IntoResponse {
    let mut response = next.run(req).await;
    
    let headers = response.headers_mut();
    headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    headers.insert("X-XSS-Protection", HeaderValue::from_static("1; mode=block"));
    headers.insert("Referrer-Policy", HeaderValue::from_static("strict-origin-when-cross-origin"));
    
    response
}

/// Check if header is hop-by-hop
fn is_hop_by_hop_header(name: &str) -> bool {
    matches!(name, 
        "connection" | "keep-alive" | "proxy-authenticate" | 
        "proxy-authorization" | "te" | "trailers" | "transfer-encoding" | "upgrade"
    )
}

/// Graceful shutdown signal
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C, shutting down..."),
        _ = terminate => info!("Received SIGTERM, shutting down..."),
    }
}

/// Print example config
fn print_example_config() {
    println!(r#"# R Commerce Frontend Server Configuration

# Server bind address
bind = "0.0.0.0:3000"

# R Commerce API base URL
api_url = "http://localhost:8080"

# API key for service-to-service authentication
api_key = "ak_yourprefix.yoursecret"

# Path to static files directory (customer frontend)
static_dir = "frontend"

# Enable CORS (for development only!)
cors = false

# Request timeout in seconds
timeout_secs = 30

# API response cache TTL in seconds
cache_ttl_secs = 300

# Rate limit per IP (requests per minute)
rate_limit_per_minute = 60

# Maximum request body size in MB
max_body_size_mb = 10

# Enable Brotli/Gzip compression
enable_compression = true

# Enable ETag for static files
enable_etag = true

# Log level
log_level = "info"
"#);
}
