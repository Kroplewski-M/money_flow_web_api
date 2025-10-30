use crate::{
    models::{
        auth::{SignInRequest, SignUpRequest},
        shared::{AppState, SignInError},
    },
    services,
};
use actix_web::{
    HttpResponse, Responder, post,
    web::{self},
};
use validator::Validate;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_in).service(sign_up);
}

#[post("/auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    if let Err(errors) = data.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({"errors": errors}));
    }
    let result = services::auth::sign_up(&state.db, &data).await;
    match result {
        Ok(id) => HttpResponse::Created().json(serde_json::json!({"success": true, "user_id": id})),
        Err(err) => HttpResponse::build(err.as_http_status())
            .json(serde_json::json!({"success": false, "error": err.to_string()})),
    }
}

#[post("/auth/sign-in")]
pub async fn sign_in(
    state: web::Data<AppState>,
    data: web::Json<SignInRequest>,
) -> Result<impl Responder, SignInError> {
    if let Err(errors) = data.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({"errors": errors})));
    }

    let result = services::auth::sign_in(&state.db, &data).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"token": result})))
}
