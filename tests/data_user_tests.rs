use moneyflow::{data::user, models::auth::SignUpRequest};
use sqlx::PgPool;

#[sqlx::test]
async fn test_create_and_exists_with_email(pool: PgPool) {
    let request = SignUpRequest {
        email: "test@example.com".into(),
        firstname: "John".into(),
        lastname: "Doe".into(),
        password: "secret123".into(),
    };

    // Create user
    let id = user::create_user(&pool, &request).await.unwrap();
    assert!(!id.is_nil());

    // Check existence
    let exists = user::exists_with_email(&pool, &request.email).await;
    assert!(exists);
}
#[sqlx::test]
async fn test_user_does_not_exist(pool: PgPool) {
    let exists = user::exists_with_email(&pool, "some_email@gmail.com").await;
    assert!(!exists);
}
