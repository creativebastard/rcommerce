use axum::{extract::State, http::StatusCode, middleware, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::middleware::auth_rate_limit_middleware;
use crate::state::AppState;
use rcommerce_core::{models::CreateCustomerRequest, Error};

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Login response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub customer: CustomerInfo,
}

/// Customer info in auth responses
#[derive(Debug, Serialize)]
pub struct CustomerInfo {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

/// Register request
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
}

/// Register response
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub customer: CustomerInfo,
    pub message: String,
}

/// Refresh token request
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Refresh token response
#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

/// Password reset request
#[derive(Debug, Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
}

/// Password reset confirm request
#[derive(Debug, Deserialize)]
pub struct PasswordResetConfirmRequest {
    pub token: String,
    pub password: String,
}

/// Password reset response
#[derive(Debug, Serialize)]
pub struct PasswordResetResponse {
    pub message: String,
    pub token: Option<String>, // Only for demo/testing
}

/// Login endpoint
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Error> {
    // Find customer by email
    let customer = state
        .customer_service
        .find_by_email(&payload.email)
        .await?
        .ok_or_else(|| Error::unauthorized("Invalid email or password"))?;

    // Verify password
    let password_hash = customer
        .password_hash
        .as_ref()
        .ok_or_else(|| Error::unauthorized("Invalid email or password"))?;

    let (valid, needs_rehash) = state
        .auth_service
        .verify_password(&payload.password, password_hash)?;

    if !valid {
        return Err(Error::unauthorized("Invalid email or password"));
    }

    // Rehash password if it was using legacy format (PHPass or bcrypt)
    if needs_rehash {
        let new_hash = state.auth_service.hash_password(&payload.password)?;
        if let Err(e) = state.customer_service.update_password_hash(customer.id, &new_hash).await {
            tracing::warn!("Failed to rehash password for customer {}: {}", customer.id, e);
            // Don't fail login if rehash fails, just log the warning
        } else {
            tracing::info!("Password rehashed with Argon2id for customer {}", customer.id);
        }
    }

    // Generate tokens with role-based permissions
    let access_token = state
        .auth_service
        .generate_access_token(customer.id, &customer.email, &customer.role)?;
    let refresh_token = state.auth_service.generate_refresh_token(customer.id)?;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: 24 * 3600, // 24 hours
        customer: CustomerInfo {
            id: customer.id,
            email: customer.email,
            first_name: customer.first_name,
            last_name: customer.last_name,
        },
    }))
}

/// Register endpoint
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), Error> {
    // Validate password strength
    if payload.password.len() < 8 {
        return Err(Error::validation("Password must be at least 8 characters"));
    }

    // Hash password
    let password_hash = state.auth_service.hash_password(&payload.password)?;

    // Create customer with password
    let customer = state
        .customer_service
        .create_customer_with_password(
            CreateCustomerRequest {
                email: payload.email,
                first_name: payload.first_name,
                last_name: payload.last_name,
                phone: payload.phone,
                accepts_marketing: false,
                currency: rcommerce_core::models::Currency::USD,
            },
            password_hash,
        )
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            customer: CustomerInfo {
                id: customer.id,
                email: customer.email,
                first_name: customer.first_name,
                last_name: customer.last_name,
            },
            message: "Registration successful. Please log in.".to_string(),
        }),
    ))
}

/// Refresh access token
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>, Error> {
    // Verify refresh token
    let claims = state.auth_service.verify_token(&payload.refresh_token)?;

    // Ensure it's a refresh token
    if claims.token_type != rcommerce_core::services::TokenType::Refresh {
        return Err(Error::unauthorized("Invalid token type"));
    }

    // Fetch customer to get their current role
    let customer = state
        .customer_service
        .find_by_id(claims.sub)
        .await?
        .ok_or_else(|| Error::unauthorized("Customer not found"))?;

    // Generate new access token with role-based permissions
    let access_token = state
        .auth_service
        .generate_access_token(claims.sub, &customer.email, &customer.role)?;

    Ok(Json(RefreshTokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: 24 * 3600, // 24 hours
    }))
}

/// Request password reset
/// Generates a reset token and sends it via email (or returns for demo)
pub async fn request_password_reset(
    State(state): State<AppState>,
    Json(payload): Json<PasswordResetRequest>,
) -> Result<Json<PasswordResetResponse>, Error> {
    // Find customer by email
    let customer = state
        .customer_service
        .find_by_email(&payload.email)
        .await?;

    // Always return success even if email not found (security)
    if customer.is_none() {
        tracing::info!("Password reset requested for non-existent email: {}", payload.email);
        return Ok(Json(PasswordResetResponse {
            message: "If the email exists, a reset link has been sent".to_string(),
            // In demo mode, return token directly
            token: None,
        }));
    }

    let customer = customer.unwrap();
    
    // Generate reset token (short-lived JWT)
    let reset_token = state
        .auth_service
        .generate_password_reset_token(customer.id, &customer.email)?;

    // TODO: Send email with reset link
    tracing::info!("Password reset token for {}: {}", customer.email, reset_token);

    Ok(Json(PasswordResetResponse {
        message: "Password reset instructions sent".to_string(),
        // In demo mode, return token directly for testing
        token: Some(reset_token),
    }))
}

/// Confirm password reset
/// Validates token and updates password
pub async fn confirm_password_reset(
    State(state): State<AppState>,
    Json(payload): Json<PasswordResetConfirmRequest>,
) -> Result<Json<PasswordResetResponse>, Error> {
    // Validate password strength
    if payload.password.len() < 8 {
        return Err(Error::validation("Password must be at least 8 characters"));
    }

    // Verify reset token
    let claims = state.auth_service.verify_password_reset_token(&payload.token)?;

    // Hash new password
    let password_hash = state.auth_service.hash_password(&payload.password)?;

    // Update customer password
    state.customer_service
        .update_password_hash(claims.sub, &password_hash)
        .await?;

    tracing::info!("Password reset successful for customer {}", claims.sub);

    Ok(Json(PasswordResetResponse {
        message: "Password reset successful. Please log in with your new password.".to_string(),
        token: None,
    }))
}

/// Public auth routes (no API key required)
/// These routes handle initial authentication
/// NOTE: All routes have strict rate limiting to prevent abuse
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/refresh", post(refresh_token))
        .route("/auth/password-reset/confirm", post(confirm_password_reset))
        .layer(middleware::from_fn(auth_rate_limit_middleware))
}

/// Protected auth routes (API key required)
/// These routes require service-to-service authentication
pub fn protected_router() -> Router<AppState> {
    Router::new()
        // Password reset request requires API key to prevent abuse
        .route("/auth/password-reset", post(request_password_reset))
}
