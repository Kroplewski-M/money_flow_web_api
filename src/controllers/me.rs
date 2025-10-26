use actix_web::{Responder, get, post, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(profile).service(update_profile);
}

#[get("/me")]
pub async fn profile() -> impl Responder {
    "profile"
}
#[post("/me")]
pub async fn update_profile() -> impl Responder {
    "update profile"
}
