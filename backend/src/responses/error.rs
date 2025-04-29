use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub code: u16,
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub message: String,
}
