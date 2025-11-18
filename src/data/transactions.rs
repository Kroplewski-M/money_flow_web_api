use uuid::Uuid;

use crate::models::{
    shared::ServiceErrorStatus,
    transactions::{CreateTransactionRequest, Transaction},
};

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

pub async fn get_transaction_for_user(
    pool: &sqlx::PgPool,
    user_id: &Uuid,
    id: &Uuid,
) -> Result<Option<Transaction>, ServiceErrorStatus> {
    let transaction = sqlx::query_as!(
        Transaction,
        "SELECT * FROM transactions WHERE user_id = $1 AND id = $2",
        user_id,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| ServiceErrorStatus::InternalError)?;
    Ok(transaction)
}

pub async fn create_transaction_for_user(
    pool: &sqlx::PgPool,
    user_id: &Uuid,
    transaction: &CreateTransactionRequest,
) -> Result<(), ServiceErrorStatus> {
    sqlx::query!(
        "INSERT INTO transactions (user_id, category_id, type_name, amount, memo, description)
                  VALUES ($1,$2,$3,$4,$5,$6)",
        user_id,
        transaction.category_id,
        transaction.type_name,
        transaction.amount,
        transaction.memo,
        transaction.description
    )
    .execute(pool)
    .await
    .map_err(|_| ServiceErrorStatus::InternalError)?;
    Ok(())
}
