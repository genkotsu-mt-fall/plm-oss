use crate::auth::domain::Claims;
use crate::errors::app_error::AppError;
use crate::errors::validation::extract_validation_errors;
use crate::part::domain::{NewPart, Part};

use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

use super::auth::ensure_admin_or_owner;

pub async fn update_part(
    claims: Claims,
    pool: &PgPool,
    id: Uuid,
    updated_part: NewPart,
) -> Result<Part, AppError> {
    updated_part
        .validate()
        .map_err(|e| AppError::ValidationError(extract_validation_errors(e)))?;

    ensure_admin_or_owner(claims, pool, id).await?;

    let part = sqlx::query_as!(
        Part,
        r#"UPDATE parts
        SET part_number = $1,
            name = $2,
            description = $3,
            kind = $4,
            updated_at = NOW()
        WHERE id = $5
        RETURNING id, part_number, name, description, kind, created_at, created_by, updated_at
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
        AppError::DatabaseError("Failed to update part".to_string())
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
