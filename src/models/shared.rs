use std::fmt;

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
#[derive(Debug, Clone)]
pub enum ServiceStatus {
    Success,
    Conflict,
    NotFound,
    InternalError,
}
impl ServiceStatus {
    pub fn as_http_status(&self) -> StatusCode {
        match self {
            ServiceStatus::Success => StatusCode::OK,
            ServiceStatus::Conflict => StatusCode::CONFLICT,
            ServiceStatus::NotFound => StatusCode::NOT_FOUND,
            ServiceStatus::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ServiceStatus::Success => "Success",
            ServiceStatus::Conflict => "Conflict: resource already exists",
            ServiceStatus::NotFound => "Not found: resource does not exist",
            ServiceStatus::InternalError => "Internal server error",
        };
        write!(f, "{message}")
    }
}
#[derive(Debug, FromRow, Serialize)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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
