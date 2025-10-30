use std::env;

use jsonwebtoken::{EncodingKey, Header, errors::Error};
use sqlx::{PgPool, types::chrono};
use uuid::Uuid;

use crate::{
    data,
    models::{
        auth::{SignInRequest, SignUpRequest},
        shared::{Claims, ServiceStatus, SignInError, User},
    },
};

pub async fn sign_up(pool: &PgPool, request: &SignUpRequest) -> Result<Uuid, ServiceStatus> {
    let email = request.email.as_str();
    let exists = data::user::exists_with_email(pool, email).await;
    if exists {
        return Err(ServiceStatus::Conflict);
    }
    let result = data::user::create_user(pool, request).await;
    match result {
        Ok(id) => return Ok(id),
        Err(_) => Err(ServiceStatus::InternalError),
    }
}
pub async fn sign_in(pool: &PgPool, request: &SignInRequest) -> Result<String, SignInError> {
    let email = request.email.as_str();
    let user = data::user::get_user_from_email(pool, email).await;

    let user = user
        .map_err(|err| SignInError::DatabaseError(err))?
        .ok_or(SignInError::InvalidCredentials)?;

    let valid_password = bcrypt::verify(&request.password, &user.password_hash)
        .map_err(|e| SignInError::BcryptError(e.to_string()))?;

    if !valid_password {
        return Err(SignInError::InvalidCredentials);
    }

    let claims = create_claims_for_user(user);
    let token = try_create_token_for_claims(claims)
        .map_err(|e| SignInError::TokenCreationError(e.to_string()))?;
    Ok(token)
}

fn create_claims_for_user(user: User) -> Claims {
    Claims {
        sub: user.id,
        role: "user".to_string(),
        exp: (chrono::Utc::now().timestamp() as u64) + 60 * 60 * 24 * 7,
    }
}
fn try_create_token_for_claims(claims: Claims) -> Result<String, Error> {
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET is missing")
                .as_bytes(),
        ),
    )
}
