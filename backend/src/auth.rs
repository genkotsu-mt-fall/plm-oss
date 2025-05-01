use axum::{
    body::Body,
    http::{Request, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn jwt_auth(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());

    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    ) {
        Ok(_token_data) => Ok(next.run(req).await),
        Err(err) => {
            error!("Invalid JWT: {}", err);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
