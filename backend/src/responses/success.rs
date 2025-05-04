use axum::http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SuccessResponse<T> {
    #[schema(example = true)]
    pub success: bool,
    pub code: u16,
    pub data: T,
}

impl<T> SuccessResponse<T> {
    pub fn ok(data: T) -> Self {
        SuccessResponse {
            success: true,
            code: StatusCode::OK.as_u16(),
            data,
        }
    }

    pub fn created(data: T) -> Self {
        SuccessResponse {
            success: true,
            code: StatusCode::CREATED.as_u16(),
            data,
        }
    }

    pub fn no_content() -> Self
    where
        T: Default,
    {
        SuccessResponse {
            success: true,
            code: StatusCode::NO_CONTENT.as_u16(),
            data: T::default(),
        }
    }
}
