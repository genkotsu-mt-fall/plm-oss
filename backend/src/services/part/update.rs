use crate::errors::app_error::AppError;
use crate::errors::validation::extract_validation_errors;
use crate::models::part::{NewPart, Part};

use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

pub async fn update_part(pool: &PgPool, id: Uuid, updated_part: NewPart) -> Result<Part, AppError> {
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
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        error!("DB error during updating part: {}", e);
        AppError::DatabaseError(format!("Failed to update part: {}", e))
    })?;

    match part {
        Some(part) => {
            info!("Part updated successfully: {}", part.id);
            Ok(part)
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
