use actix_web::http::StatusCode;
use serde::Serialize;
use sqlx::{
    prelude::FromRow,
    types::{
        chrono::{DateTime, Utc},
        uuid,
    },
};
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
