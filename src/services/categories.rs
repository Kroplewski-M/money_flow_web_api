use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    data,
    models::{
        categories::{Category, CreateCategoryRequest, EditCategoryRequest},
        shared::ServiceErrorStatus,
    },
};

pub async fn get_categories_for_user(
    pool: &PgPool,
    user_id: &Uuid,
) -> Result<Vec<Category>, ServiceErrorStatus> {
    data::categories::get_categories_for_user(pool, user_id).await
}
pub async fn get_category_for_user(
    pool: &PgPool,
    user_id: &Uuid,
    category_id: &Uuid,
) -> Result<Option<Category>, ServiceErrorStatus> {
    data::categories::get_category_for_user(pool, user_id, category_id).await
}
pub async fn create_category_for_user(
    pool: &PgPool,
    user_id: &Uuid,
    category: &CreateCategoryRequest,
) -> Result<(), ServiceErrorStatus> {
    data::categories::create_category_for_user(pool, user_id, category).await
}
pub async fn edit_category_for_user(
    pool: &PgPool,
    user_id: &Uuid,
    category: &EditCategoryRequest,
) -> Result<Category, ServiceErrorStatus> {
    data::categories::edit_category_for_user(pool, user_id, category).await
}
