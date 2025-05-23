use axum::http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;
use validator::ValidationErrors;

#[derive(Serialize, Debug, ToSchema)]
pub struct ValidationErrorResponse {
    #[schema(example = false)]
    pub success: bool,
    #[schema(example = 400)]
    pub code: u16,
    pub errors: Vec<FieldError>,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

pub fn extract_validation_errors(errors: ValidationErrors) -> ValidationErrorResponse {
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
        success: false,
        code: StatusCode::BAD_REQUEST.as_u16(),
        errors: field_errors,
    }
}
