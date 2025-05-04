use crate::auth::domain::{LoginRequest, LoginResponse, SignupRequest, SignupResponse};
use crate::auth::service as auth_service;
use crate::responses::error::ErrorResponse;
use crate::responses::success::SuccessResponse;

use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::errors::app_error::AppError;

#[utoipa::path(
    post,
    path = "/signup",
    request_body = SignupRequest,
    responses(
        (status = 201, description = "User created successfully", body = SuccessResponse<SignupResponse>),
        (status = 409, description = "Conflict (already exists)", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["auth"]
)]
pub async fn signup(
    State(pool): State<PgPool>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<SuccessResponse<SignupResponse>>, AppError> {
    let signup_response = auth_service::signup(&pool, payload).await?;
    Ok(Json(SuccessResponse::created(signup_response)))
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = SuccessResponse<LoginResponse>),
        (status = 401, description = "Unauthorized (invalid credentials)", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["auth"]
)]
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<SuccessResponse<LoginResponse>>, AppError> {
    let login_response = auth_service::login(&pool, payload).await?;
    Ok(Json(SuccessResponse::ok(login_response)))
}
