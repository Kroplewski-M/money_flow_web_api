use actix_web::http::StatusCode;
use sqlx::PgPool;

use crate::{
    data,
    models::{
        auth::{SignInRequest, SignUpRequest},
        shared::ServiceResponse,
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
pub async fn sign_in(pool: &PgPool, request: &SignInRequest) {
    let email = request.email.as_str();
    let user = data::user::get_user_from_email(&pool, email).await;
}
