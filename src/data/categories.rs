use uuid::Uuid;

use crate::models::{
    categories::{Category, CreateCategoryRequest, EditCategoryRequest},
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
pub async fn get_category_for_user(
    pool: &sqlx::PgPool,
    user_id: &Uuid,
    category_id: &Uuid,
) -> Result<Option<Category>, ServiceErrorStatus> {
    let category = sqlx::query_as!(
        Category,
        "SELECT * FROM categories WHERE user_id = $1 AND id = $2",
        user_id,
        category_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| ServiceErrorStatus::InternalError)?;
    Ok(category)
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
    .map_err(|e| {
        tracing::error!("Failed to insert category for user {}: {:?}", user_id, e);
        ServiceErrorStatus::InternalError
    })?;

    Ok(())
}
pub async fn edit_category_for_user(
    pool: &sqlx::PgPool,
    user_id: &Uuid,
    category: &EditCategoryRequest,
) -> Result<Category, ServiceErrorStatus> {
    let updated = sqlx::query_as!(
        Category,
        "UPDATE categories SET title = $1, description = $2
                  WHERE id = $3 AND user_id = $4
        RETURNING *",
        category.title,
        category.description,
        category.id,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update category for user {}: {:?}", user_id, e);
        ServiceErrorStatus::InternalError
    })?;
    Ok(updated)
}
pub async fn delete_category_for_user(
    pool: &sqlx::PgPool,
    user_id: &Uuid,
    category_id: &Uuid,
) -> Result<(), ServiceErrorStatus> {
    sqlx::query!(
        "DELETE FROM categories WHERE id = $1 AND user_id = $2",
        category_id,
        user_id
    )
    .execute(pool)
    .await
    .map_err(|_| ServiceErrorStatus::InternalError)?;
    Ok(())
}
pub async fn update_category_balance_for_user(
    pool: &sqlx::PgPool,
    user_id: &Uuid,
    category_id: &Uuid,
    cost: i64,
    type_name: &str,
) -> Result<(), ServiceErrorStatus> {
    let amount = match type_name.to_uppercase().as_str() {
        "CREDIT" => cost, // add to balance
        "DEBIT" => -cost, // subtract from balance
        _ => return Err(ServiceErrorStatus::InternalError),
    };
    sqlx::query!(
        "UPDATE categories SET balance = balance + $1 WHERE id = $2 AND user_id = $3",
        amount,
        category_id,
        user_id
    )
    .execute(pool)
    .await
    .map_err(|_| ServiceErrorStatus::InternalError)?;
    Ok(())
}
