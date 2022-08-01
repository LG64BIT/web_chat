use actix_web::{
    web::{self, ServiceConfig},
    Responder,
};

pub mod chat;
pub mod index;
pub mod login;
pub mod register;

pub async fn test() -> impl Responder {
    "Noice".to_string()
}

pub fn router(conf: &mut ServiceConfig) {
    conf.service(web::resource("/test").route(web::get().to(test)));
    conf.service(web::resource("/register").route(web::post().to(register::handle)));
    conf.service(web::resource("/login").route(web::post().to(login::handle)));
    //conf.service(web::resource("/logout").route(web::get().to(logout::handle)));
    conf.service(web::resource("/self").route(web::get().to(index::handle)));
    conf.service(web::resource("/chat/addGroup").route(web::post().to(chat::add::handle)));
    conf.service(web::resource("/chat/joinGroup").route(web::post().to(chat::join::handle)));
    conf.service(
        web::resource("/chat/enter/{group_id}").route(web::get().to(chat::connection::handle)),
    );
}
