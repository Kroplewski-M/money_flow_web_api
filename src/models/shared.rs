use actix_web::http::StatusCode;
use serde::Serialize;
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
