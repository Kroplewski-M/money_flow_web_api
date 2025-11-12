use uuid::Uuid;

use crate::models::{shared::ServiceErrorStatus, transactions::Transaction};

pub async fn get_transactions_for_user(
    pool: &sqlx::PgPool,
    user_id: &Uuid,
) -> Result<Vec<Transaction>, ServiceErrorStatus> {
    let transactions = sqlx::query_as!(
        Transaction,
        "SELECT * FROM transactions WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|_| ServiceErrorStatus::InternalError)?;
    Ok(transactions)
}
