use crate::errors::app_error::AppError;
use crate::models::part::Part;

use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

pub async fn get_parts(pool: &PgPool) -> Result<Vec<Part>, AppError> {
    let parts = sqlx::query_as!(
        Part,
        r#"SELECT id, part_number, name, description, kind, created_at, updated_at
        FROM parts
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!("DB error during fetching parts: {}", e);
        AppError::DatabaseError("DB select failed".to_string())
    })?;

    info!("Fetched {} parts successfully", parts.len());
    Ok(parts)
}

pub async fn get_part(pool: &PgPool, id: Uuid) -> Result<Part, AppError> {
    let part = sqlx::query_as!(
        Part,
        r#"SELECT id, part_number, name, description, kind, created_at, updated_at
        FROM parts
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        error!("DB error during fetching part: {}", e);
        AppError::DatabaseError("Failed to fetch part".to_string())
    })?;

    match part {
        Some(part) => {
            info!("Part found: {}", part.id);
            Ok(part)
        }
        None => {
            info!("Part not found: {}", id);
            Err(AppError::NotFound(format!("Part not found: {}", id)))
        }
    }
}
