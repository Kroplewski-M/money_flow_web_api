use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub amount: i64,
    pub memo: String,
    pub type_name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    pub category_id: Uuid,
    pub amount: i64,
    pub memo: String,
    pub type_name: String,
    pub description: Option<String>,
}
