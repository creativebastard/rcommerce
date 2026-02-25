//! R Commerce Frontend Server

use anyhow::{Context, Result};
use axum::{
    body::Body,
    extract::{ConnectInfo, Request, State},
    http::{HeaderValue, StatusCode},
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

mod cache;
use cache::{Cache, CacheBackend, CachedResponse};

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
    #[serde(default = "default_cache_ttl")]
    pub cache_ttl_secs: u64,
    #[serde(default = "default_rate_limit")]
    pub rate_limit_per_minute: u32,
    #[serde(default)]
    pub redis_url: Option<String>,
    #[serde(default = "default_redis_ttl")]
    pub redis_ttl_secs: u64,
    #[serde(default = "default_true")]
    pub enable_edge_caching: bool,
    #[serde(default = "default_static_cache_days")]
    pub static_cache_days: u32,
    #[serde(default = "default_api_cache_seconds")]
    pub api_cache_seconds: u32,
}

fn default_bind() -> String { "0.0.0.0:3000".to_string() }
fn default_static_dir() -> PathBuf { PathBuf::from("frontend") }
fn default_timeout() -> u64 { 30 }
fn default_log_level() -> String { "info".to_string() }
fn default_cache_ttl() -> u64 { 300 }
fn default_rate_limit() -> u32 { 60 }
fn default_redis_ttl() -> u64 { 600 }
fn default_static_cache_days() -> u32 { 30 }
fn default_api_cache_seconds() -> u32 { 300 }
fn default_true() -> bool { true }

impl Config {
    pub fn load(args: &Args) -> Result<Self> {
        let mut config = config::Config::builder();
        config = config.set_default("bind", default_bind())?;
        config = config.set_default("static_dir", default_static_dir().to_str().unwrap())?;
        config = config.set_default("timeout_secs", default_timeout() as i64)?;
        config = config.set_default("log_level", default_log_level())?;
        config = config.set_default("cache_ttl_secs", default_cache_ttl() as i64)?;
        config = config.set_default("rate_limit_per_minute", default_rate_limit() as i64)?;
        config = config.set_default("redis_ttl_secs", default_redis_ttl() as i64)?;
        config = config.set_default("static_cache_days", default_static_cache_days() as i64)?;
        config = config.set_default("api_cache_seconds", default_api_cache_seconds() as i64)?;
        config = config.set_default("enable_edge_caching", true)?;
        
        if let Some(ref path) = args.config {
            config = config.add_source(config::File::from(path.as_path()));
        } else {
            config = config.add_source(config::File::with_name("frontend-server").required(false));
            config = config.add_source(config::File::with_name("/etc/rcommerce/frontend-server").required(false));
        }
        
        config = config.add_source(config::Environment::with_prefix("FRONTEND").separator("_"));
        
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
            anyhow::bail!("API URL is required");
        }
        if self.api_key.is_empty() {
            anyhow::bail!("API key is required");
        }
        if !self.static_dir.exists() {
            anyhow::bail!("Static directory does not exist: {}", self.static_dir.display());
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub http_client: reqwest::Client,
    pub cache: Arc<Cache>,
    pub rate_limiter: Arc<RwLock<RateLimiter>>,
}

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

#[derive(Parser, Debug)]
#[command(name = "rcommerce-frontend")]
#[command(about = "R Commerce Frontend Server with Redis & edge caching")]
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
    
    tracing_subscriber::fmt()
        .with_env_filter(&config.log_level)
        .init();
    
    info!("R Commerce Frontend Server v{}", env!("CARGO_PKG_VERSION"));
    info!("  Bind: {}", config.bind);
    info!("  API URL: {}", config.api_url);
    info!("  Static dir: {}", config.static_dir.display());
    
    // Initialize cache
    let cache_backend: Arc<dyn CacheBackend> = match &config.redis_url {
        Some(redis_url) => {
            info!("  Cache: Redis ({redis_url})");
            match cache::RedisCache::new(redis_url, config.redis_ttl_secs).await {
                Ok(redis) => Arc::new(redis),
                Err(e) => {
                    warn!("Failed to connect to Redis: {}. Using in-memory cache.", e);
                    Arc::new(cache::MemoryCache::new(1000, config.cache_ttl_secs))
                }
            }
        }
        None => {
            info!("  Cache: In-memory (LRU)");
            Arc::new(cache::MemoryCache::new(1000, config.cache_ttl_secs))
        }
    };
    
    let cache = Arc::new(Cache::new(cache_backend));
    
    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(config.timeout_secs))
        .pool_max_idle_per_host(10)
        .build()
        .context("Failed to create HTTP client")?;
    
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
    
    // Build router: API routes first, then static files fallback
    let mut app = Router::new()
        .route("/health", get(health_check))
        .route("/api/*path", get(api_proxy_cached).post(api_proxy).put(api_proxy).delete(api_proxy))
        .fallback(get(serve_static))
        .with_state(state);
    
    if config.cors {
        app = app.layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(tower_http::cors::Any),
        );
    }
    
    app = app.layer(axum::middleware::from_fn(security_headers));
    app = app.layer(tower_http::trace::TraceLayer::new_for_http());
    
    let addr: SocketAddr = config.bind.parse()
        .context("Invalid bind address")?;
    
    info!("Server ready: http://{}", addr);
    
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

// Static file handler with edge caching headers
async fn serve_static(
    State(state): State<AppState>,
    req: Request,
) -> impl IntoResponse {
    let path = req.uri().path().trim_start_matches('/');
    let file_path = state.config.static_dir.join(if path.is_empty() { "index.html" } else { path });
    
    let actual_path = if file_path.exists() && file_path.is_file() {
        file_path
    } else {
        // SPA fallback
        let index = state.config.static_dir.join("index.html");
        if index.exists() {
            index
        } else {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .unwrap();
        }
    };
    
    let content = match tokio::fs::read(&actual_path).await {
        Ok(data) => data,
        Err(_) => return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Failed to read file"))
            .unwrap(),
    };
    
    let content_type = mime_guess::from_path(&actual_path)
        .first_or_octet_stream()
        .to_string();
    
    let mut response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", content_type);
    
    // Add CloudFlare/edge caching headers
    if state.config.enable_edge_caching {
        let ext = actual_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        
        match ext {
            "css" | "js" | "woff" | "woff2" | "ttf" | "ico" | 
            "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" => {
                let max_age = state.config.static_cache_days * 24 * 60 * 60;
                response = response
                    .header("Cache-Control", format!("public, max-age={max_age}, immutable"))
                    .header("CDN-Cache-Control", format!("max-age={max_age}"))
                    .header("CloudFlare-CDN-Cache-Control", format!("max-age={max_age}"));
            }
            "html" => {
                response = response
                    .header("Cache-Control", "public, max-age=60, stale-while-revalidate=300")
                    .header("Vary", "Accept-Encoding");
            }
            _ => {
                response = response.header("Cache-Control", "public, max-age=3600");
            }
        }
    }
    
    response.body(Body::from(content)).unwrap()
}

async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    let cache_status = state.cache.health_check().await;
    let status = if cache_status { "healthy" } else { "degraded" };
    let cache_type = if (*state.config).redis_url.is_some() { "redis" } else { "memory" };
    
    axum::Json(serde_json::json!({
        "status": status,
        "version": env!("CARGO_PKG_VERSION"),
        "service": "rcommerce-frontend",
        "cache": cache_type
    }))
}

async fn api_proxy_cached(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
) -> Result<Response, StatusCode> {
    let cache_key = format!("api:{}:{}", req.method(), req.uri().path());
    
    if req.method() == Method::GET {
        match state.cache.get(&cache_key).await {
            Ok(Some(cached)) => {
                info!("Cache HIT: {}", cache_key);
                let mut response = Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", cached.content_type)
                    .header("X-Cache", "HIT");
                
                if state.config.enable_edge_caching {
                    response = response.header("Cache-Control", 
                        format!("public, max-age={}", state.config.api_cache_seconds));
                }
                
                return Ok(response.body(Body::from(cached.body)).unwrap());
            }
            Ok(None) => {}
            Err(e) => warn!("Cache error: {}", e),
        }
    }
    
    api_proxy_inner(state, addr, req, cache_key).await
}

async fn api_proxy(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
) -> Result<Response, StatusCode> {
    api_proxy_inner(state, addr, req, String::new()).await
}

async fn api_proxy_inner(
    state: AppState,
    addr: SocketAddr,
    req: Request,
    cache_key: String,
) -> Result<Response, StatusCode> {
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
    let target_url = format!("{}{}", state.config.api_url, path);
    
    info!("Proxy: {} {} (from {})", method, path, ip);
    
    let mut proxy_req = state.http_client.request(method.clone(), &target_url);
    let headers = req.headers();
    
    for (name, value) in headers.iter() {
        let name_str = name.as_str().to_lowercase();
        if name_str != "connection" && name_str != "authorization" {
            proxy_req = proxy_req.header(name.clone(), value.clone());
        }
    }
    
    proxy_req = proxy_req.header("Authorization", format!("Bearer {}", state.config.api_key));
    
    if method != Method::GET && method != Method::HEAD {
        let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        proxy_req = proxy_req.body(body_bytes);
    }
    
    let backend_resp = proxy_req.send().await.map_err(|e| {
        error!("Backend request failed: {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    let status = StatusCode::from_u16(backend_resp.status().as_u16())
        .unwrap_or(StatusCode::OK);
    
    let mut response_builder = Response::builder()
        .status(status)
        .header("X-Cache", "MISS");
    
    let content_type = backend_resp.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/json")
        .to_string();
        
    for (name, value) in backend_resp.headers().iter() {
        let name_str = name.as_str().to_lowercase();
        if !matches!(name_str.as_str(), "connection" | "transfer-encoding") {
            response_builder = response_builder.header(name.clone(), value.clone());
        }
    }
    
    let body_bytes = backend_resp.bytes().await.map_err(|e| {
        error!("Failed to read backend response: {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    if method == Method::GET && status.is_success() && !cache_key.is_empty() {
        let cached = CachedResponse {
            body: body_bytes.to_vec(),
            content_type: content_type.clone(),
            cached_at: chrono::Utc::now(),
            backend: String::new(),
        };
        
        if let Err(e) = state.cache.set(&cache_key, &cached, state.config.cache_ttl_secs).await {
            warn!("Failed to cache response: {}", e);
        }
    }
    
    if state.config.enable_edge_caching && status.is_success() {
        response_builder = response_builder
            .header("Cache-Control", format!("public, max-age={}", state.config.api_cache_seconds))
            .header("CDN-Cache-Control", format!("max-age={}", state.config.api_cache_seconds));
    }
    
    Ok(response_builder.body(Body::from(body_bytes)).unwrap())
}

async fn security_headers(req: Request, next: axum::middleware::Next) -> impl IntoResponse {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    headers.insert("X-XSS-Protection", HeaderValue::from_static("1; mode=block"));
    headers.insert("Referrer-Policy", HeaderValue::from_static("strict-origin-when-cross-origin"));
    response
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
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

fn print_example_config() {
    println!(r#"# R Commerce Frontend Server

bind = "0.0.0.0:3000"
api_url = "http://localhost:8080"
api_key = "ak_yourprefix.yoursecret"
static_dir = "frontend"

# Redis caching
redis_url = "redis://localhost:6379"
redis_ttl_secs = 600
cache_ttl_secs = 300

# Edge caching (CloudFlare)
enable_edge_caching = true
static_cache_days = 30
api_cache_seconds = 300

# Rate limiting
rate_limit_per_minute = 60

# Logging
log_level = "info"
"#);
}
