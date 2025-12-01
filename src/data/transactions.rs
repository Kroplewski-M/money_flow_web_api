use uuid::Uuid;

use crate::models::{
    shared::ServiceErrorStatus,
    transactions::{CreateTransactionRequest, Transaction, UpdateTransactionsRequest},
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
pub async fn edit_transaction_for_user(
    pool: &sqlx::PgPool,
    user_id: &Uuid,
    transaction: &UpdateTransactionsRequest,
) -> Result<Option<Transaction>, ServiceErrorStatus> {
    let updated = sqlx::query_as!(
        Transaction,
        "UPDATE transactions 
                  SET memo = $1, description = $2 
                  WHERE id = $3 AND user_id = $4
         RETURNING *",
        transaction.memo,
        transaction.description,
        transaction.id,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update category for user {}: {:?}", user_id, e);
        ServiceErrorStatus::InternalError
    })?;
    Ok(updated)
}
