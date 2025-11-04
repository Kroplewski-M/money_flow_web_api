use actix_web::{HttpMessage, HttpRequest};
use uuid::Uuid;

use crate::{data, models::shared::User};

pub fn get_user_id(req: &HttpRequest) -> Uuid {
    let ext = req.extensions();
    *ext.get::<Uuid>().unwrap()
}
pub async fn get_authenticated_user(
    req: &HttpRequest,
    pool: &sqlx::PgPool,
) -> Result<User, actix_web::Error> {
    let user_id = get_user_id(req); // assume this extracts a Uuid from the JWT

    let user = data::user::get_user_from_id(pool, &user_id)
        .await
        .map_err(|e| {
            tracing::error!("Database error fetching user {user_id}: {e:?}");
            actix_web::error::ErrorInternalServerError("Database error")
        })?
        .ok_or_else(|| {
            tracing::warn!("User not found for valid token: {user_id}");
            actix_web::error::ErrorUnauthorized("Invalid token")
        })?;

    Ok(user)
}
