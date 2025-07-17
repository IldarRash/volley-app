// main.rs

use actix_web::{App, HttpServer};

// Declare modules to organize the application code.
pub mod api;
pub mod config;
pub mod dto;
pub mod models;
pub mod persistence;
pub mod security;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::user::register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
