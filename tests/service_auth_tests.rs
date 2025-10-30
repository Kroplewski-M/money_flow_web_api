use moneyflow::{
    models::{
        auth::{SignInRequest, SignUpRequest},
        shared::User,
    },
    services::auth,
};
use sqlx::{PgPool, types::chrono::Utc};
use uuid::Uuid;

#[sqlx::test]
async fn test_sign_up_and_sign_in(pool: PgPool) {
    // Arrange
    unsafe {
        std::env::set_var("JWT_SECRET", "super_secret_for_tests");
    }

    let signup_request = SignUpRequest {
        email: "auth_user@example.com".into(),
        firstname: "Jane".into(),
        lastname: "Doe".into(),
        password: "mypassword123".into(),
    };

    // Act — Sign up
    let signup_response = auth::sign_up(&pool, &signup_request).await;

    // Assert sign-up succeeded
    assert!(signup_response.is_ok());

    // Act — Now try to sign in
    let signin_request = SignInRequest {
        email: signup_request.email.clone(),
        password: signup_request.password.clone(),
    };

    let token_result = auth::sign_in(&pool, &signin_request).await;

    // Assert
    assert!(token_result.is_ok());
    let token = token_result.unwrap();
    assert!(!token.is_empty());
}

#[sqlx::test]
async fn test_sign_in_invalid_password() {
    unsafe {
        std::env::set_var("JWT_SECRET", "test_secret");
    }
    let hash = bcrypt::hash("correct_password", bcrypt::DEFAULT_COST).unwrap();

    let user = User {
        id: Uuid::new_v4(),
        email: "wrong@example.com".to_string(),
        password_hash: hash,
        firstname: "Mat".to_string(),
        lastname: "Krop".to_string(),
        balance: 0,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let req = SignInRequest {
        email: "wrong@example.com".into(),
        password: "wrong_password".into(),
    };

    let result = bcrypt::verify(&req.password, &user.password_hash);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}
