use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use validator::Validate;

use crate::{
    models::user::{NewUser, UserError},
    utils::AppState,
};

pub async fn handle(state: Data<AppState>, user: Json<NewUser>) -> Result<HttpResponse, UserError> {
    //let connection = utils::establish_connection();
    let connection = state.get_pg_connection();
    match user.validate() {
        Ok(_) => (),
        Err(_) => return Err(UserError::InvalidCredentials),
    };
    match NewUser::create(&connection, &user.username, &user.password) {
        Ok(created) => Ok(HttpResponse::Ok().json(created)),
        Err(e) => Err(e),
    }
}
