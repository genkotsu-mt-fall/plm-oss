use crate::{auth::domain::Claims, errors::app_error::AppError};

use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use super::auth::ensure_part_owner;

pub async fn delete_part(claims: Claims, pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    ensure_part_owner(claims, pool, id).await?;

    let result = sqlx::query!(r#"DELETE FROM parts WHERE id = $1"#, id)
        .execute(pool)
        .await
        .map_err(|e| {
            error!("DB error during deleting part: {}", e);
            AppError::DatabaseError("Failed to delete part".to_string())
        })?;

    if result.rows_affected() == 0 {
        info!("Part not found for deletion: {}", id);
        Err(AppError::NotFound(format!(
            "Part not found for deletion: {}",
            id
        )))
    } else {
        info!("Part deleted successfully: {}", id);
        Ok(())
    }
}
