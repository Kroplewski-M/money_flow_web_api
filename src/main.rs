use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    id: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}
async fn manuall_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!!!")
}
#[get("/params")]
async fn params(query: web::Query<Params>) -> impl Responder {
    let id = &query.id;
    HttpResponse::Ok().body(format!("Id: {}", id))
}
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(params)
            .route("/hey", web::get().to(manuall_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
