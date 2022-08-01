use actix::Actor;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::web::Data;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use dotenv::dotenv;
use models::lobby::Lobby;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate redis;

pub mod jwt;
pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let chat_server = Lobby::default().start(); //create and spin up a lobby
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(utils::initialize()))
            .wrap(Logger::default())
            .service(web::scope("/").configure(routes::router))
            .default_service(web::to(|| HttpResponse::Ok()))
            .app_data(Data::new(chat_server.clone())) //register the lobby
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
