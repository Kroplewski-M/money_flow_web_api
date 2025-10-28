use crate::{
    models::{
        auth::{SignInRequest, SignUpRequest},
        shared::{AppState, SignInError},
    },
    services,
};
use actix_web::{HttpResponse, Responder, post, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_in).service(sign_up);
}

#[post("/auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    let result = services::auth::sign_up(&state.db, &data).await;
    HttpResponse::build(result.status).json(&result)
}

#[post("/auth/sign-in")]
pub async fn sign_in(
    state: web::Data<AppState>,
    data: web::Json<SignInRequest>,
) -> Result<impl Responder, SignInError> {
    let result = services::auth::sign_in(&state.db, &data).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"token": result})))
}
