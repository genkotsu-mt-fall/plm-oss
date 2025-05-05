use crate::auth::domain::{SignupRequest, SignupResponse};

use sqlx::PgPool;

use crate::errors::app_error::AppError;

use super::user_create::create_user_with_role;

pub async fn signup(pool: &PgPool, payload: SignupRequest) -> Result<SignupResponse, AppError> {
    create_user_with_role(pool, &payload.login_name, &payload.password, "user").await?;

    Ok(SignupResponse {
        login_name: payload.login_name,
    })
}
