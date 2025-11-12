use actix_web::{
    HttpRequest, HttpResponse, Responder, delete, get, middleware::from_fn, post, put, web,
};

use crate::{
    middleware::auth, models::shared::AppState, services::transactions,
    utils::get_authenticated_user,
};

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
pub async fn index(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder, actix_web::Error> {
    let user = get_authenticated_user(&req, &state.db).await?;
    let result = transactions::get_transactions_for_user(&state.db, &user.id).await;
    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(err) => Ok(HttpResponse::build(err.as_http_status())
            .json(serde_json::json!({"success": false, "error": err.to_string()}))),
    }
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
