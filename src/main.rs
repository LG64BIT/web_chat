/*!
API for basic web-sockets realtime chat. It is multy threaded, supports multiple group chats with multiple users per chat.
# Overview
Crate provides basic authentification functionalities (header auth with stateless [jwt])

*/
use actix::Actor;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::web::Data;
use actix_web::App;
use actix_web::HttpServer;
use dotenv::dotenv;
use models::lobby::Lobby;

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_migrations;

embed_migrations!("migrations");

pub mod errors;
mod jwt;
pub mod models;
pub mod routes;
mod schema;
pub mod utils;

///Program entrance point
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let chat_server = Lobby::default().start();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(utils::initialize()))
            .wrap(Logger::default())
            .service(web::scope("/").configure(routes::router))
            .app_data(Data::new(chat_server.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
