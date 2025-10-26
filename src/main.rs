use actix_web::{App, HttpServer};

use crate::controllers::{auth, me};
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(auth::configure)
            .configure(me::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
