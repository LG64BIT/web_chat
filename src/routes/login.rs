use crate::{
    errors::ShopError,
    models::user::{NewUser, User},
    utils::AppState,
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

/// Login user
///
/// # HTTP request
/// Request must be in [Json] format
/// ## Header
/// * jwt: [String] - JWT autorization token
/// ## Body
/// * username: [String] - minimum 5 characters long
/// * password: [String] - minimum 8 characters long
///
/// # HTTP response
/// ##Header
/// * Success code: 200
/// *
/// ## Body
/// * Response is in [Json] format
/// ```
/// {
///     "id": "f7169845-4de5-470e-bb76-7117d4620d8c"
///     "username": "test_user"
/// }
/// ```
/// Error code: 208, 400, 404, 500
pub async fn handle(state: Data<AppState>, user: Json<NewUser>) -> Result<HttpResponse, ShopError> {
    let connection = state.get_pg_connection()?;
    let (valid, token) = User::authenticate(&connection, &user.username, &user.password)?;
    Ok(HttpResponse::Ok().append_header(("jwt", token)).json(valid))
}
