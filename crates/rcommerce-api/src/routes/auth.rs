use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

    let valid = state
        .auth_service
        .verify_password(&payload.password, password_hash)?;

    if !valid {
        return Err(Error::unauthorized("Invalid email or password"));
    }

    // Generate tokens
    let access_token = state
        .auth_service
        .generate_access_token(customer.id, &customer.email)?;
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

    // Generate new access token
    let access_token = state
        .auth_service
        .generate_access_token(claims.sub, &claims.email)?;

    Ok(Json(RefreshTokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: 24 * 3600, // 24 hours
    }))
}

/// Router for auth routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/refresh", post(refresh_token))
}
