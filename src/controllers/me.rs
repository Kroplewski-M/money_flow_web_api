use actix_web::{HttpRequest, Responder, get, middleware::from_fn, post, web};

use crate::{middleware::auth, utils::get_user_id};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/me")
            .wrap(from_fn(auth::verified_jwt))
            .service(profile)
            .service(update_profile),
    );
}

#[get("")]
pub async fn profile(req: HttpRequest) -> impl Responder {
    let user_id = get_user_id(req);
    format!("profile - user id {}", &user_id)
}
#[post("")]
pub async fn update_profile() -> impl Responder {
    "update profile"
}
