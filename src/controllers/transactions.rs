use actix_web::{Responder, delete, get, middleware::from_fn, post, put, web};

use crate::middleware::auth;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/transactions")
            .wrap(from_fn(auth::verified_jwt))
            .service(index)
            .service(create)
            .service(show)
            .service(edit)
            .service(delete),
    );
}
#[get("")]
pub async fn index() -> Result<impl Responder, actix_web::Error> {
    Ok("index")
}
#[post("")]
pub async fn create() -> Result<impl Responder, actix_web::Error> {
    Ok("create")
}

#[get("/{id}")]
pub async fn show() -> Result<impl Responder, actix_web::Error> {
    Ok("show")
}

#[put("")]
pub async fn edit() -> Result<impl Responder, actix_web::Error> {
    Ok("edit")
}
#[delete("/{id}")]
pub async fn delete() -> Result<impl Responder, actix_web::Error> {
    Ok("delete")
}
