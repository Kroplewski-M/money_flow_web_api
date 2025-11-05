use actix_web::{
    HttpRequest, HttpResponse, Responder, delete, get, middleware::from_fn, post, put, web,
};

use crate::{data, middleware::auth, models::shared::AppState, utils::get_authenticated_user};

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
pub async fn index(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder, actix_web::Error> {
    let user = get_authenticated_user(&req, &state.db).await?;
    let categories = data::categories::get_categories_for_user(&state.db, &user.id).await;

    match categories {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(err) => Ok(HttpResponse::build(err.as_http_status())
            .json(serde_json::json!({"success": false, "error": err.to_string()}))),
    }
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
