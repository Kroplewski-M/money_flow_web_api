use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    data,
    models::{
        shared::{ServiceStatus, User},
        user::UpdateProfileReq,
    },
};
pub async fn get_user_from_id(pool: &PgPool, id: Uuid) -> Result<User, ServiceStatus> {
    let user = data::user::get_user_from_id(pool, &id)
        .await
        .map_err(|_| ServiceStatus::InternalError)?
        .ok_or(ServiceStatus::NotFound)?;
    Ok(user)
}
pub async fn update_user_from_id(
    pool: &PgPool,
    user_id: &Uuid,
    req: &UpdateProfileReq,
) -> Result<(), ServiceStatus> {
    data::user::update_user_from_id(pool, user_id, req)
        .await
        .map_err(|_| ServiceStatus::InternalError)
}
