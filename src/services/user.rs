use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    data,
    models::shared::{ServiceStatus, User},
};
pub async fn get_user_from_id(pool: &PgPool, id: Uuid) -> Result<User, ServiceStatus> {
    let user = data::user::get_user_from_id(pool, &id)
        .await
        .map_err(|_| ServiceStatus::InternalError)?
        .ok_or(ServiceStatus::NotFound)?;
    Ok(user)
}
