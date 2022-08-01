use crate::{
    models::user::{NewUser, User, UserError},
    utils::AppState,
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

pub async fn handle(state: Data<AppState>, user: Json<NewUser>) -> Result<HttpResponse, UserError> {
    let connection = state.get_pg_connection();
    match User::authenticate(&connection, &user.username, &user.password) {
        Ok((valid, token)) => {
            Ok(HttpResponse::Ok()
                .append_header(("jwt", token))
                //.cookie(cookie)
                .json(valid))
        }
        Err(e) => Err(e),
    }
}
