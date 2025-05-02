use crate::{
    auth::{
        domain::{Claims, LoginRequest, LoginResponse},
        jwt::generate_jwt,
        password::verify_password,
    },
    models::user::User,
};

use sqlx::PgPool;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::error;

use crate::errors::app_error::AppError;

pub async fn login(pool: &PgPool, payload: LoginRequest) -> Result<LoginResponse, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, login_name, password_hash, created_at
        FROM users
        WHERE login_name = $1"#,
        payload.login_name,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        error!("Login failed: fetch user from database: {}", e);
        AppError::DatabaseError("Login failed: fetch user".to_string())
    })?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    verify_password(&payload.password, &user.password_hash)?;

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

    let token = generate_jwt(claims)?;
    Ok(LoginResponse { token })
}
