use crate::{
    auth::{
        domain::{SignupRequest, SignupResponse},
        // password::hash_password,
    },
    // models::user::User,
};

use sqlx::PgPool;
// use tracing::error;

use crate::errors::app_error::AppError;

use super::user_create::create_user_with_role;

pub async fn signup(pool: &PgPool, payload: SignupRequest) -> Result<SignupResponse, AppError> {
    // let existing = sqlx::query!(
    //     r#"SELECT id FROM users WHERE login_name = $1"#,
    //     payload.login_name,
    // )
    // .fetch_optional(pool)
    // .await
    // .map_err(|e| {
    //     error!("DB error during fetching users: {}", e);
    //     AppError::DatabaseError("Signup failed: query user from the database.".to_string())
    // })?;

    // if existing.is_some() {
    //     return Err(AppError::Conflict(
    //         "This login name is already taken.".to_string(),
    //     ));
    // }

    // let hash = hash_password(&payload.password)?;

    // let user = sqlx::query_as!(
    //     User,
    //     r#"INSERT INTO users
    //     (login_name, password_hash)
    //     VALUES ($1, $2)
    //     RETURNING id, login_name, password_hash, role, created_at"#,
    //     payload.login_name,
    //     &hash
    // )
    // .fetch_one(pool)
    // .await
    // .map_err(|e| {
    //     error!("Signup failed: insert user into database: {}", e);
    //     AppError::DatabaseError("Signup failed: create user account.".to_string())
    // })?;
    create_user_with_role(pool, &payload.login_name, &payload.password, "user").await?;

    Ok(SignupResponse {
        login_name: payload.login_name,
    })
}
