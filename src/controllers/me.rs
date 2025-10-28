use actix_web::{Responder, get, middleware::from_fn, post, web};

use crate::middleware::auth;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/me")
            .wrap(from_fn(auth::verified_jwt))
            .service(profile)
            .service(update_profile),
    );
}

#[get("")]
pub async fn profile() -> impl Responder {
    "profile"
}
#[post("")]
pub async fn update_profile() -> impl Responder {
    "update profile"
}
