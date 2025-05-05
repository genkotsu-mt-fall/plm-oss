use sqlx::PgPool;

use crate::{auth::password::hash_password, errors::app_error::AppError, models::user::User};
use tracing::{error, info};

pub async fn create_user_with_role(
    pool: &PgPool,
    login_name: &str,
    password: &str,
    role: &str,
) -> Result<(), AppError> {
    let existing = sqlx::query!(r#"SELECT id FROM users WHERE login_name = $1"#, login_name,)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            error!("DB error during fetching users: {}", e);
            AppError::DatabaseError("Signup failed: query user from the database.".to_string())
        })?;

    if existing.is_some() {
        // return Err(AppError::Conflict(
        //     "This login name is already taken.".to_string(),
        // ));
        info!("This login name is already exist.");
        return Ok(());
    }

    let hash = hash_password(password)?;

    sqlx::query_as!(
        User,
        r#"INSERT INTO users
        (login_name, password_hash, role)
        VALUES ($1, $2, $3)
        RETURNING id, login_name, password_hash, role, created_at"#,
        login_name,
        &hash,
        role
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        error!("Signup failed: insert user into database: {}", e);
        AppError::DatabaseError("Signup failed: create user account.".to_string())
    })?;
    Ok(())
}
