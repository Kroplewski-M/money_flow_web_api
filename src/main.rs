use crate::controllers::{auth, categories, me, transactions};
use crate::models::shared::AppState;
use actix_extensible_rate_limit::RateLimiter;
use actix_extensible_rate_limit::backend::SimpleInputFunctionBuilder;
use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env;
use std::time::Duration;
mod controllers;
mod data;
mod middleware;
mod models;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_connection = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
    let pool = sqlx::PgPool::connect(&db_connection)
        .await
        .expect("could not make a connection to the database");

    let state = web::Data::new(AppState { db: pool });
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let rate_limiter_backend = InMemoryBackend::builder().build();

    HttpServer::new(move || {
        App::new()
            .wrap(
                RateLimiter::builder(
                    rate_limiter_backend.clone(),
                    SimpleInputFunctionBuilder::new(Duration::from_secs(60), 50)
                        .real_ip_key()
                        .build(),
                )
                .add_headers()
                .build(),
            )
            .app_data(state.clone())
            .configure(auth::configure)
            .configure(me::configure)
            .configure(categories::configure)
            .configure(transactions::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
