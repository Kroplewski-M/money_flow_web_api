use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::{
        chrono::{DateTime, Utc},
        uuid,
    },
};
use thiserror::Error;
pub struct AppState {
    pub db: sqlx::PgPool,
}
#[derive(Serialize)]
pub struct ServiceResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing)]
    pub status: StatusCode,
}
#[derive(Debug, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub password_hash: String,
    pub balance: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: uuid::Uuid,
    pub role: String,
    pub exp: u64,
}
#[derive(Debug, Error)]
pub enum SignInError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Token creation failed: {0}")]
    TokenCreationError(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Password verification failed: {0}")]
    BcryptError(String),
}
impl ResponseError for SignInError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SignInError::InvalidCredentials => {
                HttpResponse::Unauthorized().json(serde_json::json!({ "error": self.to_string() }))
            }
            SignInError::DatabaseError(_) => HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Database error" })),
            SignInError::BcryptError(_) => HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Password verification failed" })),
            SignInError::TokenCreationError(_) => HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Token creation failed" })),
        }
    }
}
