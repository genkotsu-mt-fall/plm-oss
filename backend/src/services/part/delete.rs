use crate::errors::app_error::AppError;

use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

pub async fn delete_part(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
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
