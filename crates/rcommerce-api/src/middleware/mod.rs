//! Middleware modules for the R Commerce API

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    body::Body,
};

use rcommerce_core::services::AuthService;
use crate::state::AppState;

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
            tracing::debug!("Found Authorization header: {}", &header[..header.len().min(50)]);
            AuthService::extract_bearer_token(header)
                .ok_or_else(|| {
                    tracing::warn!("Invalid Authorization header format");
                    StatusCode::UNAUTHORIZED
                })?
        }
        None => {
            tracing::warn!("No Authorization header found");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    tracing::debug!("Extracted token: {}", &token[..token.len().min(50)]);

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
        Some(header) => AuthService::extract_bearer_token(header)
            .ok_or(StatusCode::UNAUTHORIZED)?,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Verify token
    let claims = state.auth_service.verify_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check for admin permission
    if !claims.permissions.contains(&"admin".to_string()) {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}
