use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SignupRequest {
    pub login_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SignupResponse {
    pub login_name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub login_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}
