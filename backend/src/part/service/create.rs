use crate::errors::app_error::AppError;
use crate::errors::validation::extract_validation_errors;
use crate::part::domain::{NewPart, Part};

use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

pub async fn create_part(pool: &PgPool, new_part: NewPart) -> Result<Part, AppError> {
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
    .fetch_one(pool)
    .await
    .map_err(|e| {
        error!("DB error during part insertion: {}", e);
        AppError::DatabaseError("DB insert failed".to_string())
    })?;

    info!("Part created successfully: {}", part.id);
    Ok(part)
}
