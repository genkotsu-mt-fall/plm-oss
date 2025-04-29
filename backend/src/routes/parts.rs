use crate::errors::app_error::AppError;
use crate::errors::validation::extract_validation_errors;
use crate::models::part::{NewPart, Part};
use crate::responses::success::SuccessResponse;

use axum::{Json, extract::Path, extract::State};
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

// #[axum::debug_handler]
pub async fn create_part(
    State(pool): State<PgPool>,
    Json(new_part): Json<NewPart>,
) -> Result<Json<SuccessResponse<Part>>, AppError> {
    new_part
        .validate()
        .map_err(|e| AppError::ValidationError(extract_validation_errors(e)))?;

    let part = sqlx::query_as!(
        Part,
        r#"INSERT INTO parts (id, part_number, name, description, kind)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, part_number, name, description, kind, created_at, updated_at"#,
        Uuid::new_v4(),
        new_part.part_number,
        new_part.name,
        new_part.description,
        new_part.kind
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("DB error during part insertion: {}", e);
        AppError::DatabaseError(format!("DB insert failed: {}", e))
    })?;

    info!("Part created successfully: {}", part.id);
    Ok(Json(SuccessResponse::created(part)))
}

// #[axum::debug_handler]
pub async fn get_parts(
    State(pool): State<PgPool>,
) -> Result<Json<SuccessResponse<Vec<Part>>>, AppError> {
    let parts = sqlx::query_as!(
        Part,
        r#"SELECT id, part_number, name, description, kind, created_at, updated_at
        FROM parts
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!("DB error during fetching parts: {}", e);
        AppError::DatabaseError(format!("DB select failed: {}", e))
    })?;

    info!("Fetched {} parts successfully", parts.len());
    Ok(Json(SuccessResponse::ok(parts)))
}

// #[axum::debug_handler]
pub async fn get_part(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<SuccessResponse<Part>>, AppError> {
    let part = sqlx::query_as!(
        Part,
        r#"SELECT id, part_number, name, description, kind, created_at, updated_at
        FROM parts
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!("DB error during fetching part: {}", e);
        AppError::DatabaseError(format!("Failed to fetch part: {}", e))
    })?;

    match part {
        Some(part) => {
            info!("Part found: {}", part.id);
            Ok(Json(SuccessResponse::ok(part)))
        }
        None => {
            info!("Part not found: {}", id);
            Err(AppError::NotFound(format!("Part not found: {}", id)))
        }
    }
}

// #[axum::debug_handler]
pub async fn update_part(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(updated_part): Json<NewPart>,
) -> Result<Json<SuccessResponse<Part>>, AppError> {
    updated_part
        .validate()
        .map_err(|e| AppError::ValidationError(extract_validation_errors(e)))?;

    let part = sqlx::query_as!(
        Part,
        r#"UPDATE parts
        SET part_number = $1,
            name = $2,
            description = $3,
            kind = $4,
            updated_at = NOW()
        WHERE id = $5
        RETURNING id, part_number, name, description, kind, created_at, updated_at
        "#,
        updated_part.part_number,
        updated_part.name,
        updated_part.description,
        updated_part.kind,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!("DB error during updating part: {}", e);
        AppError::DatabaseError(format!("Failed to update part: {}", e))
    })?;

    match part {
        Some(part) => {
            info!("Part updated successfully: {}", part.id);
            Ok(Json(SuccessResponse::ok(part)))
        }
        None => {
            info!("Part not found for update: {}", id);
            Err(AppError::NotFound(format!(
                "Part not found for update: {}",
                id
            )))
        }
    }
}

// #[axum::debug_handler]
pub async fn delete_part(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<SuccessResponse<()>>, AppError> {
    let result = sqlx::query!(r#"DELETE FROM parts WHERE id = $1"#, id)
        .execute(&pool)
        .await
        .map_err(|e| {
            error!("DB error during deleting part: {}", e);
            AppError::DatabaseError(format!("Failed to delete part: {}", e))
        })?;

    if result.rows_affected() == 0 {
        info!("Part not found for deletion: {}", id);
        Err(AppError::NotFound(format!(
            "Part not found for deletion: {}",
            id
        )))
    } else {
        info!("Part deleted successfully: {}", id);
        Ok(Json(SuccessResponse::no_content()))
    }
}
