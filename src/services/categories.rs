use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    data,
    models::{categories::Category, shared::ServiceErrorStatus},
};

pub async fn get_categories_for_user(
    pool: &PgPool,
    user_id: &Uuid,
) -> Result<Vec<Category>, ServiceErrorStatus> {
    let categories = data::categories::get_categories_for_user(pool, user_id).await;

    match categories {
        Ok(res) => Ok(res),
        Err(_) => Err(ServiceErrorStatus::InternalError),
    }
}
