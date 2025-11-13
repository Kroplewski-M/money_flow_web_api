use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    data,
    models::{
        shared::ServiceErrorStatus,
        transactions::{CreateTransactionRequest, Transaction},
    },
};

pub async fn get_transactions_for_user(
    pool: &PgPool,
    user_id: &Uuid,
) -> Result<Vec<Transaction>, ServiceErrorStatus> {
    data::transactions::get_transactions_for_user(pool, user_id).await
}
pub async fn create_transaction_for_user(
    pool: &PgPool,
    user_id: &Uuid,
    transaction: &CreateTransactionRequest,
) -> Result<(), ServiceErrorStatus> {
    data::transactions::create_transaction_for_user(pool, user_id, transaction).await
}
