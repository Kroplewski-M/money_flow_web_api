use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    data::{self, user::get_user_from_id},
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
    //checking user balance
    let user = get_user_from_id(pool, user_id).await.unwrap().unwrap();
    if user.balance < transaction.amount && transaction.type_name == "DEBIT" {
        return Err(ServiceErrorStatus::BadRequest(String::from(
            "User does not have enough funds",
        )));
    }
    let category =
        data::categories::get_category_for_user(pool, user_id, &transaction.category_id).await;
    if let Ok(Some(cat)) = &category
        && cat.balance < transaction.amount
        && transaction.type_name == "DEBIT"
    {
        return Err(ServiceErrorStatus::BadRequest(String::from(
            "category does not have enough funds",
        )));
    }
    //getting category and creating transaction
    match category {
        Ok(Some(category)) if category.user_id == *user_id => {
            data::transactions::create_transaction_for_user(pool, user_id, transaction).await?;
            data::user::update_user_balance(pool, user_id, transaction.amount)
                .await
                .map_err(|_| ServiceErrorStatus::InternalError)?;
            data::categories::update_category_balance_for_user(
                pool,
                user_id,
                &category.id,
                transaction.amount,
            )
            .await
            .map_err(|_| ServiceErrorStatus::InternalError)?;
            Ok(())
        }
        Ok(_) => Err(ServiceErrorStatus::NotFound), // Category doesn't exist
        Err(e) => Err(e),                           // DB-level error mapped earlier
    }
}
