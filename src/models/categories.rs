use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize)]
pub struct Category {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub balance: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, message = "title is required"))]
    pub title: String,
    pub description: String,
}
#[derive(Deserialize, Validate)]
pub struct EditCategoryRequest {
    pub id: Uuid,
    #[validate(length(min = 1, message = "title is required"))]
    pub title: String,
    pub description: String,
}
