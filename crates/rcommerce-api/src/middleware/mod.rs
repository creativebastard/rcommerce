//! Middleware modules for the R Commerce API

use axum::{
    body::Body,
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

use crate::state::AppState;
use rcommerce_core::services::AuthService;

pub mod scopes;
pub mod api_key_auth;

pub use api_key_auth::{
    ApiKeyAuth, 
    JwtAuth, 
    AuthContext,
    api_key_auth_middleware, 
    combined_auth_middleware
};

/// Rate limiter for auth endpoints (in-memory, per-IP)
#[derive(Clone)]
pub struct AuthRateLimiter {
    /// Max attempts per window
    max_attempts: u32,
    /// Window duration in seconds
    window_secs: u64,
    /// Store: IP -> (attempts, first_attempt_time)
    store: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
}

impl AuthRateLimiter {
    pub fn new(max_attempts: u32, window_secs: u64) -> Self {
        Self {
            max_attempts,
            window_secs,
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Check if the request is allowed and increment counter
    pub async fn check_and_increment(&self, ip: &str) -> bool {
        let mut store = self.store.lock().await;
        let now = Instant::now();
        let window = Duration::from_secs(self.window_secs);

        match store.get_mut(ip) {
            Some((attempts, first_attempt)) => {
                // Check if window has expired
                if now.duration_since(*first_attempt) > window {
                    // Reset window
                    *attempts = 1;
                    *first_attempt = now;
                    true
                } else if *attempts < self.max_attempts {
                    // Increment attempts
                    *attempts += 1;
                    true
                } else {
                    // Rate limit exceeded
                    false
                }
            }
            None => {
                // First attempt from this IP
                store.insert(ip.to_string(), (1, now));
                true
            }
        }
    }

    /// Clean up expired entries (call periodically)
    pub async fn cleanup(&self) {
        let mut store = self.store.lock().await;
        let now = Instant::now();
        let window = Duration::from_secs(self.window_secs);
        store.retain(|_, (_, first_attempt)| now.duration_since(*first_attempt) <= window);
    }
}

/// Auth rate limiting middleware - limits login/register attempts per IP
pub async fn auth_rate_limit_middleware(
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract state from request extensions
    let state = request
        .extensions()
        .get::<Arc<AppState>>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Extract connection info from request extensions
    let addr = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0)
        .unwrap_or_else(|| SocketAddr::from(([0, 0, 0, 0], 0)));

    // Get client IP (use X-Forwarded-For if behind proxy, otherwise use socket addr)
    let ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string());

    // Check rate limit (5 attempts per minute)
    if !state.auth_rate_limiter.check_and_increment(&ip).await {
        tracing::warn!("Auth rate limit exceeded for IP: {}", ip);
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}

/// Authentication middleware - validates JWT tokens
pub async fn auth_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    tracing::debug!("Auth middleware checking request");

    // Get Authorization header
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(header) => {
            tracing::debug!("Found Authorization header");
            AuthService::extract_bearer_token(header).ok_or_else(|| {
                tracing::warn!("Invalid Authorization header format");
                StatusCode::UNAUTHORIZED
            })?
        }
        None => {
            tracing::warn!("No Authorization header found");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    tracing::debug!("Extracted bearer token");

    // Verify token
    match state.auth_service.verify_token(token) {
        Ok(claims) => {
            tracing::debug!("Token verified for customer: {}", claims.sub);
            Ok(next.run(request).await)
        }
        Err(e) => {
            tracing::warn!("Token verification failed: {}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

/// Optional authentication middleware - doesn't fail if no token
pub async fn optional_auth_middleware(
    State(_state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // For now, just pass through - could add user info to request extensions
    next.run(request).await
}

/// Rate limiting middleware
pub async fn rate_limit_middleware(
    State(_state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // TODO: Implement proper rate limiting with Redis
    // For now, pass through all requests
    Ok(next.run(request).await)
}

/// Admin-only middleware
pub async fn admin_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get Authorization header
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(header) => {
            tracing::debug!("Admin middleware: Found Authorization header");
            AuthService::extract_bearer_token(header).ok_or(StatusCode::UNAUTHORIZED)?
        }
        None => {
            tracing::debug!("Admin middleware: No Authorization header found");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Verify token
    let claims = state
        .auth_service
        .verify_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check for admin permission
    if !claims.permissions.contains(&"admin".to_string()) {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}
