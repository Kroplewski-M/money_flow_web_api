use actix_web::{App, http::StatusCode, test};
use moneyflow::{
    controllers::auth::configure,
    models::{
        auth::{SignInRequest, SignUpRequest},
        shared::AppState,
    },
};
use sqlx::PgPool;

#[sqlx::test]
async fn test_sign_up_and_sign_in_controller(pool: PgPool) {
    // Arrange
    unsafe {
        std::env::set_var("JWT_SECRET", "super_secret_for_tests");
    }

    let app_state = AppState { db: pool.clone() };
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(app_state))
            .configure(configure),
    )
    .await;

    // ----- Sign up -----
    let signup_request = SignUpRequest {
        email: "controller_user@example.com".into(),
        firstname: "Jane".into(),
        lastname: "Doe".into(),
        password: "mypassword123".into(),
    };

    let req = test::TestRequest::post()
        .uri("/auth/sign-up")
        .set_json(&signup_request)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // ----- Sign in -----
    let signin_request = SignInRequest {
        email: signup_request.email.clone(),
        password: signup_request.password.clone(),
    };

    let req = test::TestRequest::post()
        .uri("/auth/sign-in")
        .set_json(&signin_request)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Parse JSON body
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("token").is_some());
}
