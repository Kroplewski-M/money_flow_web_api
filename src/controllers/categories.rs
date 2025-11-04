use actix_web::{Responder, delete, get, middleware::from_fn, post, put, web};

use crate::middleware::auth;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .wrap(from_fn(auth::verified_jwt))
            .service(index)
            .service(create)
            .service(show)
            .service(edit)
            .service(delete),
    );
}
#[get("")]
pub async fn index() -> impl Responder {
    "categories: list"
}
#[post("")]
pub async fn create() -> impl Responder {
    "categories: create"
}
#[get("/show/{id}")]
pub async fn show() -> impl Responder {
    "categories: show"
}
#[put("/edit/{id}")]
pub async fn edit() -> impl Responder {
    "categories: edit"
}
#[delete("/delete/{id}")]
pub async fn delete() -> impl Responder {
    "categories: delete"
}
