use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize, ToSchema)]
pub struct Part {
    pub id: Uuid,
    pub part_number: String,
    pub name: String,
    pub description: Option<String>,
    pub kind: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct NewPart {
    #[validate(length(min = 1, message = "part_number must not be empty"))]
    pub part_number: String,
    #[validate(length(min = 1, message = "name must not be empty"))]
    pub name: String,
    pub description: Option<String>,
    pub kind: Option<String>,
}

#[cfg(test)]
mod tests {
    use validator::Validate;

    use super::NewPart;

    #[test]
    fn test_valid_new_part() {
        let new_part = NewPart {
            part_number: "ABC-123".to_string(),
            name: "Test Part".to_string(),
            description: Some("A test part".to_string()),
            kind: Some("TypeA".to_string()),
        };
        assert!(new_part.validate().is_ok())
    }

    #[test]
    fn test_invalid_empty_part_number() {
        let new_part = NewPart {
            part_number: "".to_string(),
            name: "Test Part".to_string(),
            description: None,
            kind: None,
        };
        assert!(new_part.validate().is_err())
    }

    #[test]
    fn test_invalid_empty_name() {
        let new_part = NewPart {
            part_number: "ABC-123".to_string(),
            name: "".to_string(),
            description: None,
            kind: None,
        };
        assert!(new_part.validate().is_err())
    }
}
