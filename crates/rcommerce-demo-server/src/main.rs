//! R Commerce Demo Server
//! 
//! A lightweight server that:
//! - Serves static demo frontend files
//! - Proxies API requests to R Commerce backend with injected API key
//! - Hides API credentials from browser

use anyhow::{Context, Result};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use clap::Parser;
use reqwest::Method;
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tracing::{error, info, warn};

/// Demo server configuration
#[derive(Debug, Clone, serde::Deserialize)]
struct Config {
    /// Server bind address
    #[serde(default = "default_bind")]
    bind: String,
    
    /// R Commerce API base URL
    api_url: String,
    
    /// API key for service-to-service authentication
    api_key: String,
    
    /// Path to static files directory
    #[serde(default = "default_static_dir")]
    static_dir: PathBuf,
    
    /// Enable CORS (for development)
    #[serde(default)]
    cors: bool,
    
    /// Request timeout in seconds
    #[serde(default = "default_timeout")]
    timeout_secs: u64,
    
    /// Log level
    #[serde(default = "default_log_level")]
    log_level: String,
}

fn default_bind() -> String {
    "0.0.0.0:3000".to_string()
}

fn default_static_dir() -> PathBuf {
    PathBuf::from("demo-frontend")
}

fn default_timeout() -> u64 {
    30
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Config {
    /// Load configuration from file, env, or CLI
    fn load(args: &Args) -> Result<Self> {
        let mut config = config::Config::builder();
        
        // Start with defaults
        config = config.set_default("bind", default_bind())?;
        config = config.set_default("static_dir", default_static_dir().to_str().unwrap())?;
        config = config.set_default("timeout_secs", default_timeout())?;
        config = config.set_default("log_level", default_log_level())?;
        
        // Load from config file if specified
        if let Some(ref path) = args.config {
            config = config.add_source(config::File::from(path.as_path()));
        } else {
            // Try default config locations
            config = config.add_source(
                config::File::with_name("demo-server").required(false)
            );
            config = config.add_source(
                config::File::with_name("/etc/rcommerce/demo-server").required(false)
            );
        }
        
        // Override with environment variables (RC_DEMO_*)
        config = config.add_source(
            config::Environment::with_prefix("RC_DEMO")
                .separator("_")
        );
        
        // Override with CLI args
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
        if args.cors {
            config = config.set_override("cors", true)?;
        }
        
        let config: Config = config.build()?.try_deserialize()?;
        config.validate()?;
        
        Ok(config)
    }
    
    fn validate(&self) -> Result<()> {
        if self.api_url.is_empty() {
            anyhow::bail!("API URL is required (set --api-url or RC_DEMO_API_URL)");
        }
        if self.api_key.is_empty() {
            anyhow::bail!("API key is required (set --api-key or RC_DEMO_API_KEY)");
        }
        if !self.static_dir.exists() {
            anyhow::bail!("Static directory does not exist: {}", self.static_dir.display());
        }
        Ok(())
    }
}

/// Application state shared across handlers
#[derive(Clone)]
struct AppState {
    config: Arc<Config>,
    http_client: reqwest::Client,
}

/// CLI arguments
#[derive(Parser, Debug)]
#[command(name = "rcommerce-demo")]
#[command(about = "R Commerce Demo Server - serves frontend and proxies API requests")]
#[command(version)]
struct Args {
    /// Config file path
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    
    /// Bind address (e.g., 0.0.0.0:3000)
    #[arg(short, long, env = "RC_DEMO_BIND")]
    bind: Option<String>,
    
    /// R Commerce API URL (e.g., http://localhost:8080)
    #[arg(short = 'u', long, env = "RC_DEMO_API_URL")]
    api_url: Option<String>,
    
    /// API key for R Commerce (keep secret!)
    #[arg(short = 'k', long, env = "RC_DEMO_API_KEY")]
    api_key: Option<String>,
    
    /// Static files directory
    #[arg(short = 's', long, env = "RC_DEMO_STATIC_DIR")]
    static_dir: Option<String>,
    
    /// Enable CORS (for development)
    #[arg(long)]
    cors: bool,
    
    /// Print example config file
    #[arg(long)]
    print_config: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Print example config and exit
    if args.print_config {
        print_example_config();
        return Ok(());
    }
    
    // Load configuration
    let config = Config::load(&args)?;
    
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(&config.log_level)
        .init();
    
    info!("Starting R Commerce Demo Server");
    info!("  Bind: {}", config.bind);
    info!("  API URL: {}", config.api_url);
    info!("  Static dir: {}", config.static_dir.display());
    
    // Create HTTP client for proxying
    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(config.timeout_secs))
        .build()
        .context("Failed to create HTTP client")?;
    
    let state = AppState {
        config: Arc::new(config.clone()),
        http_client,
    };
    
    // Build router
    let mut app = Router::new()
        // API proxy routes - all /api/* requests go to R Commerce
        .route("/api/*path", get(api_proxy).post(api_proxy).put(api_proxy).delete(api_proxy).patch(api_proxy))
        // Static files - serve everything else from static directory
        .fallback_service(tower_http::services::ServeDir::new(&config.static_dir))
        .with_state(state);
    
    // Add CORS if enabled
    if config.cors {
        app = app.layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
                .allow_headers(tower_http::cors::Any),
        );
    }
    
    // Add tracing
    app = app.layer(tower_http::trace::TraceLayer::new_for_http());
    
    // Parse bind address
    let addr: SocketAddr = config.bind.parse()
        .context("Invalid bind address")?;
    
    info!("Server listening on http://{}", addr);
    
    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// Proxy API requests to R Commerce backend with injected API key
async fn api_proxy(
    State(state): State<AppState>,
    req: Request,
) -> Result<impl IntoResponse, StatusCode> {
    let path = req.uri().path();
    let method = req.method().clone();
    
    // Build target URL
    let target_url = format!("{}{}", state.config.api_url, path);
    
    info!("Proxying {} {} -> {}", method, path, target_url);
    
    // Build proxied request
    let mut proxy_req = state.http_client.request(method.clone(), &target_url);
    
    // Copy relevant headers from original request (but not Authorization - we'll inject our own)
    let headers = req.headers();
    for (name, value) in headers.iter() {
        let name_str = name.as_str().to_lowercase();
        // Skip hop-by-hop headers and auth (we'll add our own)
        if !is_hop_by_hop_header(&name_str) && name_str != "authorization" {
            proxy_req = proxy_req.header(name.clone(), value.clone());
        }
    }
    
    // Inject API key for service-to-service auth
    proxy_req = proxy_req.header(
        "Authorization",
        format!("Bearer {}", state.config.api_key)
    );
    
    // Forward body for non-GET requests
    if method != Method::GET && method != Method::HEAD {
        let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
            .await
            .map_err(|e| {
                error!("Failed to read request body: {}", e);
                StatusCode::BAD_REQUEST
            })?;
        proxy_req = proxy_req.body(body_bytes);
    }
    
    // Send request to backend
    let backend_resp = proxy_req.send().await.map_err(|e| {
        error!("Backend request failed: {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    // Build response
    let status = StatusCode::from_u16(backend_resp.status().as_u16())
        .unwrap_or(StatusCode::OK);
    
    let mut response_builder = Response::builder().status(status);
    
    // Copy headers from backend (excluding hop-by-hop)
    for (name, value) in backend_resp.headers().iter() {
        if !is_hop_by_hop_header(&name.as_str().to_lowercase()) {
            response_builder = response_builder.header(name.clone(), value.clone());
        }
    }
    
    // Get response body
    let body_bytes = backend_resp.bytes().await.map_err(|e| {
        error!("Failed to read backend response: {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    Ok(response_builder.body(Body::from(body_bytes)).unwrap())
}

/// Check if a header is hop-by-hop (should not be forwarded)
fn is_hop_by_hop_header(name: &str) -> bool {
    matches!(name, 
        "connection" | "keep-alive" | "proxy-authenticate" | 
        "proxy-authorization" | "te" | "trailers" | "transfer-encoding" | "upgrade"
    )
}

/// Print example configuration file
fn print_example_config() {
    println!(r#"# R Commerce Demo Server Configuration
# Save this as demo-server.toml

# Server bind address
bind = "0.0.0.0:3000"

# R Commerce API base URL
api_url = "http://localhost:8080"

# API key for service-to-service authentication
# Get this from: rcommerce api-key create --name "Demo" --scopes "products:read,orders:write"
api_key = "ak_yourprefix.yoursecret"

# Path to static files directory
static_dir = "demo-frontend"

# Enable CORS (for development)
cors = false

# Request timeout in seconds
timeout_secs = 30

# Log level (trace, debug, info, warn, error)
log_level = "info"
"#);
}
