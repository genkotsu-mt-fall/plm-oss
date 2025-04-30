use crate::errors::app_error::AppError;
use crate::models::part::{NewPart, Part};
use crate::responses::success::SuccessResponse;
use crate::services::part_service::PartService;

use axum::{Json, extract::Path, extract::State};
use sqlx::PgPool;
use uuid::Uuid;

// #[axum::debug_handler]
pub async fn create_part(
    State(pool): State<PgPool>,
    Json(new_part): Json<NewPart>,
) -> Result<Json<SuccessResponse<Part>>, AppError> {
    let part = PartService::create_part(&pool, new_part).await?;
    Ok(Json(SuccessResponse::created(part)))
}

// #[axum::debug_handler]
pub async fn get_parts(
    State(pool): State<PgPool>,
) -> Result<Json<SuccessResponse<Vec<Part>>>, AppError> {
    let parts = PartService::get_parts(&pool).await?;
    Ok(Json(SuccessResponse::ok(parts)))
}

// #[axum::debug_handler]
pub async fn get_part(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<SuccessResponse<Part>>, AppError> {
    let part = PartService::get_part(&pool, id).await?;
    Ok(Json(SuccessResponse::ok(part)))
}

// #[axum::debug_handler]
pub async fn update_part(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(updated_part): Json<NewPart>,
) -> Result<Json<SuccessResponse<Part>>, AppError> {
    let part = PartService::update_part(&pool, id, updated_part).await?;
    Ok(Json(SuccessResponse::ok(part)))
}

// #[axum::debug_handler]
pub async fn delete_part(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<SuccessResponse<()>>, AppError> {
    PartService::delete_part(&pool, id).await?;
    Ok(Json(SuccessResponse::no_content()))
}
