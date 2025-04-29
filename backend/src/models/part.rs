use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize)]
pub struct Part {
    pub id: Uuid,
    pub part_number: String,
    pub name: String,
    pub description: Option<String>,
    pub kind: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Validate)]
pub struct NewPart {
    #[validate(length(min = 1, message = "part_number must not be empty"))]
    pub part_number: String,
    #[validate(length(min = 1, message = "name must not be empty"))]
    pub name: String,
    pub description: Option<String>,
    pub kind: Option<String>,
}
