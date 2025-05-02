use crate::auth::domain::{LoginRequest, LoginResponse, SignupResponse};
use crate::auth::{domain::SignupRequest, service as auth_service};
use crate::responses::success::SuccessResponse;

use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::errors::app_error::AppError;

pub async fn signup(
    State(pool): State<PgPool>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<SuccessResponse<SignupResponse>>, AppError> {
    let signup_response = auth_service::signup(&pool, payload).await?;
    Ok(Json(SuccessResponse::created(signup_response)))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<SuccessResponse<LoginResponse>>, AppError> {
    let login_response = auth_service::login(&pool, payload).await?;
    Ok(Json(SuccessResponse::ok(login_response)))
}
