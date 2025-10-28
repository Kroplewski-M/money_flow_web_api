use std::env;

use actix_web::http::StatusCode;
use jsonwebtoken::{EncodingKey, Header, errors::Error};
use sqlx::{PgPool, types::chrono};

use crate::{
    data,
    models::{
        auth::{SignInRequest, SignUpRequest},
        shared::{Claims, ServiceResponse, SignInError, User},
    },
};

pub async fn sign_up(pool: &PgPool, request: &SignUpRequest) -> ServiceResponse {
    let email = request.email.as_str();
    let exists = data::user::exists_with_email(pool, email).await;
    if exists {
        return ServiceResponse {
            success: false,
            message: format!("user {email} already exists"),
            status: StatusCode::CONFLICT,
        };
    }
    let result = data::user::create_user(pool, request).await;
    match result {
        Ok(id) => ServiceResponse {
            success: true,
            message: format!("user created succesfully, id: {id}"),
            status: StatusCode::CREATED,
        },
        Err(err) => ServiceResponse {
            success: false,
            message: format!("Error: {err:?}"),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        },
    }
}
pub async fn sign_in(pool: &PgPool, request: &SignInRequest) -> Result<String, SignInError> {
    let email = request.email.as_str();
    let user = data::user::get_user_from_email(pool, email).await;

    let user = user.ok_or(SignInError::InvalidCredentials)?;

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
