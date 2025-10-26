use bcrypt::{DEFAULT_COST, hash};
use sqlx::types::uuid;

use crate::models::auth::SignUpRequest;

pub async fn exists_with_email(db: &sqlx::PgPool, email: &str) -> bool {
    let row = sqlx::query!(
        "SELECT EXISTS(SELECT id FROM users WHERE email = $1) as exists",
        email
    )
    .fetch_one(db)
    .await
    .unwrap();

    row.exists.unwrap_or(false)
}
pub async fn create_user(
    db: &sqlx::PgPool,
    user: &SignUpRequest,
) -> Result<uuid::Uuid, Box<dyn std::error::Error>> {
    let hashed_password = hash(&user.password, DEFAULT_COST)?;

    let record = sqlx::query!(
        r#"
        INSERT INTO users (email, firstname, lastname, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        user.email,
        user.firstname,
        user.lastname,
        hashed_password,
    )
    .fetch_one(db)
    .await?;

    Ok(record.id)
}
