use argon2::{Argon2, PasswordHasher, PasswordVerifier, PasswordHash};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use crate::errors::AppError;


#[derive(Serialize,Deserialize)]
pub struct Claims{
    pub sub:i32,
    pub ext:usize,
}

pub struct AuthUser{
    pub user_id:i32,
}

pub fn hash_password(password:&str)->Result<String,AppError>{
    let salt=SaltString::generate(&mut OsRng);
    let argon2=Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt)
    .map(|h| h.to_string())
    .map_err(|e| AppError::Internal(e.to_string()))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn create_jwt(user_id:i32,secret:&str)->Result<String,AppError>{
    let expiration=chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Valid Timestamp")
        .timestamp() as  usize;

    let claims=Claims{sub:user_id,ext:expiration};
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).map_err(|e| AppError::Internal(e.to_string()))
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized("missing authorization header".to_string()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized("invalid authorization header".to_string()))?;

        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("invalid or expired token".to_string()))?
        .claims;

        Ok(AuthUser { user_id: claims.sub })
    }
}