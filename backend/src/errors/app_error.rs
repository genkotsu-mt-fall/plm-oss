use crate::errors::validation::ValidationErrorResponse;
use crate::responses::error::{ErrorDetail, ErrorResponse};

use axum::response::{IntoResponse, Response};
use axum::{Json, http::StatusCode};
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    ValidationError(ValidationErrorResponse),
    NotFound(String),
    Conflict(String),
    DatabaseError(String),
    InternalError(String),
    Unauthorized(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let response: Response = match self {
            AppError::ValidationError(errors) => {
                let status = StatusCode::BAD_REQUEST;

                error!(
                    "Validation error ({}): {} error(s) - {:?}",
                    status,
                    errors.errors.len(),
                    errors.errors
                );

                let body = Json(errors);

                (status, body).into_response()
            }
            AppError::NotFound(message) => {
                let status = StatusCode::NOT_FOUND;

                error!("NotFound error ({}): {}", status, message);

                let body = Json(ErrorResponse {
                    success: false,
                    code: status.as_u16(),
                    error: ErrorDetail { message },
                });

                (status, body).into_response()
            }
            AppError::Conflict(message) => {
                let status = StatusCode::CONFLICT;

                error!("Conflict ({}): {}", status, message);

                let body = Json(ErrorResponse {
                    success: false,
                    code: status.as_u16(),
                    error: ErrorDetail { message },
                });

                (status, body).into_response()
            }
            AppError::DatabaseError(message) => {
                let status = StatusCode::INTERNAL_SERVER_ERROR;

                error!("Database error ({}): {}", status, message);

                let body = Json(ErrorResponse {
                    success: false,
                    code: status.as_u16(),
                    error: ErrorDetail { message },
                });

                (status, body).into_response()
            }
            AppError::InternalError(message) => {
                let status = StatusCode::INTERNAL_SERVER_ERROR;

                error!("Internal error ({}): {}", status, message);

                let body = Json(ErrorResponse {
                    success: false,
                    code: status.as_u16(),
                    error: ErrorDetail { message },
                });

                (status, body).into_response()
            }
            AppError::Unauthorized(message) => {
                let status = StatusCode::UNAUTHORIZED;

                error!("Unauthorized error ({}): {}", status, message);

                let body = Json(ErrorResponse {
                    success: false,
                    code: status.as_u16(),
                    error: ErrorDetail { message },
                });

                (status, body).into_response()
            }
        };
        response
    }
}
