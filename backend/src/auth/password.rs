use tracing::error;

use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use crate::errors::app_error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            error!("Password hashing failed: {}", e);
            AppError::InternalError("Signup failed: hash the password.".to_string())
        })
        .map(|hash| hash.to_string())
}

pub fn verify_password(password: &str, hashed: &str) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(hashed).map_err(|e| {
        error!("Login failed: parse stored password hash: {}", e);
        AppError::InternalError("Password hash parse error".to_string())
    })?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized("Invalid login credentials.".to_string()))
}
