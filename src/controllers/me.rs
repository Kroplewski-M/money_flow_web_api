use actix_web::{HttpRequest, HttpResponse, Responder, get, middleware::from_fn, post, web};

use crate::{middleware::auth, models::shared::AppState, services::user, utils::get_user_id};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/me")
            .wrap(from_fn(auth::verified_jwt))
            .service(profile)
            .service(update_profile),
    );
}

#[get("")]
pub async fn profile(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = get_user_id(req);
    let user = user::get_user_from_id(&state.db, user_id).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(&user),
        Err(err) => HttpResponse::build(err.as_http_status())
            .json(serde_json::json!({"success": false, "error": err.to_string()})),
    }
}
#[post("")]
pub async fn update_profile() -> impl Responder {
    "update profile"
}
