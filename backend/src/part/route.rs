use crate::errors::app_error::AppError;
use crate::errors::validation::ValidationErrorResponse;
use crate::part::domain::{NewPart, Part};
use crate::part::service::{
    create_part as service_create_part, delete_part as service_delete_part,
    get_part as service_get_part, get_parts as service_get_parts,
    update_part as service_update_part,
};
use crate::responses::error::ErrorResponse;
use crate::responses::success::SuccessResponse;
// use crate::services::part_service::PartService;

use axum::{Json, extract::Path, extract::State};
use sqlx::PgPool;
use uuid::Uuid;

// #[axum::debug_handler]
#[utoipa::path(post, path = "/parts", request_body = NewPart, responses(
    (status = 201, description = "Part created successfully", body = SuccessResponse<Part>),
    (status = 400, description = "Validation error", body = ValidationErrorResponse),
    (status = 401, description = "Unauthorized error", body = ErrorResponse),
    (status = 500, description = "Database error", body = ErrorResponse),
), tags = ["parts"], security(("bearerAuth" = [])))]
pub async fn create_part(
    State(pool): State<PgPool>,
    Json(new_part): Json<NewPart>,
) -> Result<Json<SuccessResponse<Part>>, AppError> {
    let part = service_create_part(&pool, new_part).await?;
    Ok(Json(SuccessResponse::created(part)))
}

// #[axum::debug_handler]
#[utoipa::path(get, path = "/parts", responses(
    (status = 200, description = "Fetched parts successfully", body = SuccessResponse<Vec<Part>>),
    (status = 401, description = "Unauthorized error", body = ErrorResponse),
    (status = 500, description = "Database error", body = ErrorResponse),
), tags = ["parts"], security(("bearerAuth" = [])))]
pub async fn get_parts(
    State(pool): State<PgPool>,
) -> Result<Json<SuccessResponse<Vec<Part>>>, AppError> {
    let parts = service_get_parts(&pool).await?;
    Ok(Json(SuccessResponse::ok(parts)))
}

// #[axum::debug_handler]
#[utoipa::path(get, path = "/parts/{id}", params(("id" = Uuid, Path, description = "Part ID to fetch")),  responses(
    (status = 200, description = "Fetched part successfully", body = SuccessResponse<Part>),
    (status = 401, description = "Unauthorized error", body = ErrorResponse),
    (status = 404, description = "NotFound error", body = ErrorResponse),
    (status = 500, description = "Database error", body = ErrorResponse),
), tags = ["parts"], security(("bearerAuth" = [])))]
pub async fn get_part(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<SuccessResponse<Part>>, AppError> {
    let part = service_get_part(&pool, id).await?;
    Ok(Json(SuccessResponse::ok(part)))
}

// #[axum::debug_handler]
#[utoipa::path(put, path = "/parts/{id}", params(("id" = Uuid, Path, description = "Part ID to update")) , request_body = NewPart, responses(
    (status = 200, description = "Part updated successfully", body = SuccessResponse<Part>),
    (status = 400, description = "Validation error", body = ValidationErrorResponse),
    (status = 401, description = "Unauthorized error", body = ErrorResponse),
    (status = 404, description = "NotFound error", body = ErrorResponse),
    (status = 500, description = "Database error", body = ErrorResponse),
), tags = ["parts"], security(("bearerAuth" = [])))]
pub async fn update_part(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(updated_part): Json<NewPart>,
) -> Result<Json<SuccessResponse<Part>>, AppError> {
    let part = service_update_part(&pool, id, updated_part).await?;
    Ok(Json(SuccessResponse::ok(part)))
}

// #[axum::debug_handler]
#[utoipa::path(delete, path = "/parts/{id}", params(("id" = Uuid, Path, description = "Part ID to delete")) , responses(
    (status = 204, description = "Part deleted successfully"),
    (status = 401, description = "Unauthorized error", body = ErrorResponse),
    (status = 404, description = "NotFound error", body = ErrorResponse),
    (status = 500, description = "Database error", body = ErrorResponse),
), tags = ["parts"], security(("bearerAuth" = [])))]
pub async fn delete_part(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<SuccessResponse<()>>, AppError> {
    service_delete_part(&pool, id).await?;
    Ok(Json(SuccessResponse::no_content()))
}
