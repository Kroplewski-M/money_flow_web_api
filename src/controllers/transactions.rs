use actix_web::{
    HttpRequest, HttpResponse, Responder, delete, get, middleware::from_fn, post, put, web,
};
use uuid::Uuid;

use crate::{
    middleware::auth,
    models::{
        shared::AppState,
        transactions::{CreateTransactionRequest, UpdateTransactionsRequest},
    },
    services::transactions,
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
pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    transaction: web::Json<CreateTransactionRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let user = get_authenticated_user(&req, &state.db).await?;
    let result = transactions::create_transaction_for_user(&state.db, &user.id, &transaction).await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"success": "true"}))),
        Err(err) => Ok(HttpResponse::build(err.as_http_status())
            .json(serde_json::json!({"success": false, "error": err.to_string()}))),
    }
}

#[get("/{id}")]
pub async fn show(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<Uuid>,
) -> Result<impl Responder, actix_web::Error> {
    let user = get_authenticated_user(&req, &state.db).await?;
    let result = transactions::get_transaction_for_user(&state.db, &user.id, &id).await;
    match result {
        Ok(res) => {
            if let Some(tran) = &res {
                return Ok(HttpResponse::Ok().json(tran));
            }
            Ok(HttpResponse::NotFound().json(
                serde_json::json!({"success": false, "error": "transaction not found for user"}),
            ))
        }
        Err(err) => Ok(HttpResponse::build(err.as_http_status())
            .json(serde_json::json!({"success": false, "error": err.to_string()}))),
    }
}

#[put("")]
pub async fn edit(
    state: web::Data<AppState>,
    req: HttpRequest,
    transaction: web::Json<UpdateTransactionsRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let user = get_authenticated_user(&req, &state.db).await?;
    let result = transactions::edit_transaction_for_user(&state.db, &user.id, &transaction).await;
    match result {
        Ok(res) => {
            if let Some(tran) = &res {
                return Ok(HttpResponse::Ok().json(tran));
            }
            Ok(HttpResponse::NotFound().json(
                serde_json::json!({"success": false, "error": "transaction not found for user"}),
            ))
        }
        Err(err) => Ok(HttpResponse::build(err.as_http_status())
            .json(serde_json::json!({"success": false, "error": err.to_string()}))),
    }
}
#[delete("/{id}")]
pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<Uuid>,
) -> Result<impl Responder, actix_web::Error> {
    let user = get_authenticated_user(&req, &state.db).await?;
    let result = transactions::delete_transaction_for_user(&state.db, &user.id, &id).await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"success": "true"}))),
        Err(err) => Ok(HttpResponse::build(err.as_http_status())
            .json(serde_json::json!({"success": false, "error": err.to_string()}))),
    }
}
