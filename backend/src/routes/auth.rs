use crate::models::user::User;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::error;

use crate::{auth::Claims, errors::app_error::AppError};

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub login_name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    pub login_name: String,
    // pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub login_name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn signup(
    State(pool): State<PgPool>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<SignupResponse>, AppError> {
    let existing = sqlx::query!(
        r#"SELECT id FROM users WHERE login_name = $1"#,
        payload.login_name,
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!("DB error during fetching users: {}", e);
        AppError::DatabaseError("Signup failed: query user from the database.".to_string())
    })?;

    if existing.is_some() {
        return Err(AppError::Conflict(
            "This login name is already taken.".to_string(),
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| {
            error!("Password hashing failed: {}", e);
            AppError::InternalError("Signup failed: hash the password.".to_string())
        })?
        .to_string();

    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users
        (login_name, password_hash)
        VALUES ($1, $2)
        RETURNING id, login_name, password_hash, created_at"#,
        payload.login_name,
        &hash
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("Signup failed: insert user into database: {}", e);
        AppError::DatabaseError("Signup failed: create user account.".to_string())
    })?;

    Ok(Json(SignupResponse {
        login_name: user.login_name,
    }))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // let expected_email = std::env::var("TEST_USER_EMAIL").unwrap_or_default();
    // let expected_password = std::env::var("TEST_USER_PASSWORD").unwrap_or_default();
    // if payload._email != expected_email || payload._password != expected_password {
    //     return Err(StatusCode::UNAUTHORIZED);
    // }

    let user = sqlx::query_as!(
        User,
        r#"SELECT id, login_name, password_hash, created_at
        FROM users
        WHERE login_name = $1"#,
        payload.login_name,
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!("Login failed: fetch user from database: {}", e);
        AppError::DatabaseError("Login failed: fetch user".to_string())
    })?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|e| {
        error!("Login failed: parse stored password hash: {}", e);
        AppError::InternalError("Password hash parse error".to_string())
    })?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized("Invalid login credentials.".to_string()))?;

    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(60 * 60))
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::InternalError("JWT secret is not set.".into()))?;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::InternalError("Login failed: generate JWT token.".to_string()))?;

    Ok(Json(LoginResponse { token }))
}
