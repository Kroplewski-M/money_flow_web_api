use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    data,
    models::{
        categories::{Category, CreateCategoryRequest},
        shared::ServiceErrorStatus,
    },
};

pub async fn get_categories_for_user(
    pool: &PgPool,
    user_id: &Uuid,
) -> Result<Vec<Category>, ServiceErrorStatus> {
    data::categories::get_categories_for_user(pool, user_id)
        .await
        .map_err(|_| ServiceErrorStatus::InternalError)
}
pub async fn create_category_for_user(
    pool: &PgPool,
    user_id: &Uuid,
    category: &CreateCategoryRequest,
) -> Result<(), ServiceErrorStatus> {
    data::categories::create_category_for_user(pool, user_id, category)
        .await
        .map_err(|_| ServiceErrorStatus::InternalError)
}
