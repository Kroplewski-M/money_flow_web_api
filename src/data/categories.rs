use uuid::Uuid;

use crate::models::{categories::Category, shared::ServiceErrorStatus};

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
