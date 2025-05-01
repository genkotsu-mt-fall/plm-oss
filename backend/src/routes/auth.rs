use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{Json, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

use crate::auth::Claims;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub _email: String,
    pub _password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    if payload._email != "user@example.com" || payload._password != "password123" {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(60 * 60))
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: payload._email.clone(),
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse { token }))
}
