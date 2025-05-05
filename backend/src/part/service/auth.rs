use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use crate::{auth::domain::Claims, errors::app_error::AppError};

pub async fn ensure_part_owner(claims: Claims, pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let part_owner = sqlx::query_scalar!("SELECT created_by FROM parts WHERE ID = $1", id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!("DB error during ownership check: {}", e);
            AppError::DatabaseError("Ownership check failed".into())
        })?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|e| AppError::InternalError(format!("Invalid UUID in claims: {}", e)))?;

    if part_owner != Some(user_id) {
        Err(AppError::Unauthorized("You do not own this part.".into()))
    } else {
        Ok(())
    }
}
