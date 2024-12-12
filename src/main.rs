mod domain;
mod use_case;
mod infrastructure;
mod service;
mod interface;

use actix_web::{web, App, HttpServer};
use interface::handlers::{list_users_handler, register_user_form, register_user_handler};
use dotenv::dotenv;
use std::env; // このインポートは使用していないので警告が出ます

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(list_users_handler))
            .route("/register", web::get().to(register_user_form))
            .route("/register", web::post().to(register_user_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
