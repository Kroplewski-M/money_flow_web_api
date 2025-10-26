use actix_web::{Responder, post, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_in).service(sign_up);
}

#[post("/auth/sign-up")]
pub async fn sign_up() -> impl Responder {
    "sign-up"
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    "sign-in"
}
