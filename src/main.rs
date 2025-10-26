use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env;

use crate::controllers::{auth, me};
use crate::models::shared::AppState;
mod controllers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_connection = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
    let pool = sqlx::PgPool::connect(&db_connection)
        .await
        .expect("could not make a connection to the database");

    let state = web::Data::new(AppState { db: pool });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(auth::configure)
            .configure(me::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
