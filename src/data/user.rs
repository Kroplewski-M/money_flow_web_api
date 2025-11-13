use bcrypt::{DEFAULT_COST, hash};
use sqlx::types::uuid;
use uuid::Uuid;

use crate::models::{auth::SignUpRequest, shared::User, user::UpdateProfileReq};

pub async fn exists_with_email(pool: &sqlx::PgPool, email: &str) -> bool {
    let row = sqlx::query!(
        "SELECT EXISTS(SELECT id FROM users WHERE email = $1) as exists",
        email
    )
    .fetch_one(pool)
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
pub async fn get_user_from_email(
    pool: &sqlx::PgPool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    let result = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(pool)
        .await;
    match result {
        Ok(user) => Ok(user),
        Err(err) => {
            tracing::error!("Failed to fetch user by email'{}': {:?}", email, err);
            Err(err)
        }
    }
}
pub async fn get_user_from_id(pool: &sqlx::PgPool, id: &Uuid) -> Result<Option<User>, sqlx::Error> {
    let result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(pool)
        .await;
    match result {
        Ok(user) => Ok(user),
        Err(err) => {
            tracing::error!("Failed to fetch user by id '{}': {:?}", id, err);
            Err(err)
        }
    }
}
pub async fn update_user_from_id(
    pool: &sqlx::PgPool,
    user_id: &Uuid,
    update_details: &UpdateProfileReq,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET firstname = $1,
            lastname = $2
        WHERE id = $3
    "#,
        update_details.firstname,
        update_details.lastname,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn update_user_balance(
    pool: &sqlx::PgPool,
    user_id: &Uuid,
    cost: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE users SET balance = balance - $1 WHERE id = $2",
        cost,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
