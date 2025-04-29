use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub code: u16,
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    ValidationError(String),
    NotFound(String),
    DatabaseError(String),
    // InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            // AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        error!("Error occurred: {} - {}", status, message);

        let body = Json(ErrorResponse {
            error: ErrorDetail {
                code: status.as_u16(),
                message,
            },
        });

        (status, body).into_response()
    }
}

#[derive(Serialize, Debug)]
pub struct ValidationErrorResponse {
    pub errors: Vec<FieldError>,
}

#[derive(Serialize, Debug)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

fn extract_validation_errors(errors: ValidationErrors) -> ValidationErrorResponse {
    let mut field_errors = Vec::new();

    for (field, errors) in errors.field_errors() {
        for error in errors {
            let message = error
                .message
                .clone()
                .unwrap_or_else(|| std::borrow::Cow::from("Invalid input"));

            field_errors.push(FieldError {
                field: field.to_string(),
                message: message.to_string(),
            });
        }
    }
    ValidationErrorResponse {
        errors: field_errors,
    }
}

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

// #[axum::debug_handler]
pub async fn create_part(
    State(pool): State<PgPool>,
    Json(new_part): Json<NewPart>,
) -> Result<Json<Part>, AppError> {
    new_part.validate().map_err(|e| {
        let errors = extract_validation_errors(e);
        error!("Validation failed during create_part: {:?}", errors);
        AppError::ValidationError(serde_json::to_string(&errors).unwrap())
    })?;

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
        error!("Failed to insert part: {}", e);
        AppError::DatabaseError(format!("DB insert failed: {}", e))
    })?;

    info!("New part created successfully: {}", part.id);
    Ok(Json(part))
}

// #[axum::debug_handler]
pub async fn get_parts(State(pool): State<PgPool>) -> Result<Json<Vec<Part>>, AppError> {
    let parts = sqlx::query_as!(
        Part,
        r#"SELECT id, part_number, name, description, kind, created_at, updated_at
        FROM parts
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!("Failed to fetch parts: {}", e);
        AppError::DatabaseError(format!("DB select failed: {}", e))
    })?;

    info!("Fetched {} parts from database", parts.len());
    Ok(Json(parts))
}

// #[axum::debug_handler]
pub async fn get_part(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Part>, AppError> {
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
        error!("Failed to fetch part: {}", e);
        AppError::DatabaseError(format!("Failed to fetch part: {}", e))
    })?;

    match part {
        Some(part) => {
            info!("Part found: {}", part.id);
            Ok(Json(part))
        }
        None => {
            info!("Part not found: {}", id);
            // Err(StatusCode::NOT_FOUND)
            Err(AppError::NotFound(format!("Part not found: {}", id)))
        }
    }
}

// #[axum::debug_handler]
pub async fn update_part(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(updated_part): Json<NewPart>,
) -> Result<Json<Part>, AppError> {
    updated_part.validate().map_err(|e| {
        let errors = extract_validation_errors(e);
        error!("Validation failed during update_part: {:?}", errors);
        AppError::ValidationError(serde_json::to_string(&errors).unwrap())
    })?;

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
        error!("Failed to update part: {}", e);
        AppError::DatabaseError(format!("Failed to update part: {}", e))
    })?;

    match part {
        Some(part) => {
            info!("Part updated successfully: {}", part.id);
            Ok(Json(part))
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
) -> Result<StatusCode, AppError> {
    let result = sqlx::query!(r#"DELETE FROM parts WHERE id = $1"#, id)
        .execute(&pool)
        .await
        .map_err(|e| {
            error!("Failed to delete part: {}", e);
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
        Ok(StatusCode::NO_CONTENT)
    }
}
