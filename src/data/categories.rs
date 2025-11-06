use uuid::Uuid;

use crate::models::{
    categories::{Category, CreateCategoryRequest},
    shared::ServiceErrorStatus,
};

pub async fn get_categories_for_user(
    pool: &sqlx::PgPool,
    user_id: &Uuid,
) -> Result<Vec<Category>, ServiceErrorStatus> {
    let categories = sqlx::query_as!(
        Category,
        "SELECT * FROM categories WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|_| ServiceErrorStatus::InternalError)?;

    Ok(categories)
}
pub async fn create_category_for_user(
    pool: &sqlx::PgPool,
    user_id: &Uuid,
    category: &CreateCategoryRequest,
) -> Result<(), ServiceErrorStatus> {
    sqlx::query!(
        "INSERT INTO categories (user_id,title,description) VALUES ($1,$2,$3)",
        user_id,
        category.title,
        category.description
    )
    .execute(pool)
    .await
    .map_err(|_| ServiceErrorStatus::InternalError)?;

    Ok(())
}
