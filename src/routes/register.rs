use crate::{errors::ShopError, models::user::NewUser, utils::AppState};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use validator::Validate;
/// Register the user with username and password
/// # HTTP request
/// Request must be in [Json] format
/// ## Header
/// * jwt: [String] - JWT autorization token
/// ## Body
/// * username: [String] - minimum 5 characters long
/// * password: [String] - minimum 8 characters long
///
/// # HTTP response
/// * Success code: 200
/// * Response is in [Json] format
/// ```
/// {
///     "id": "f7169845-4de5-470e-bb76-7117d4620d8c"
///     "username":"test_user"
/// }
/// ```
/// Error code: 208, 400, 500
pub async fn handle(state: Data<AppState>, user: Json<NewUser>) -> Result<HttpResponse, ShopError> {
    let connection = state.get_pg_connection()?;
    user.validate()?;
    let created = NewUser::create(&connection, &user.username, &user.password)?;
    Ok(HttpResponse::Ok().json(created))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        models::user::NewUser,
        utils::{get_connection_pool, StaticData},
    };
    use actix_web::{test, web, App};
    use std::sync::Arc;

    #[actix_web::test]
    async fn test_register_short_password() {
        let db_pool = get_connection_pool();
        let state = AppState {
            static_data: Arc::new(StaticData { db: db_pool }),
        };
        let app = test::init_service(App::new().route("/", web::get().to(handle))).await;
        let user = NewUser {
            username: String::from("legit_user"),
            password: String::from("123"),
        };
        let req = test::TestRequest::get()
            .app_data(state)
            .set_payload(serde_json::to_string(&user).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_register_invalid_password() {
        let db_pool = get_connection_pool();
        let state = AppState {
            static_data: Arc::new(StaticData { db: db_pool }),
        };
        let app = test::init_service(App::new().route("/", web::get().to(handle))).await;
        let user = NewUser {
            username: String::from("legit_user"),
            password: String::from("        "), // 8 spaces password
        };
        let req = test::TestRequest::get()
            .app_data(state)
            .set_payload(serde_json::to_string(&user).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_register_short_username() {
        let db_pool = get_connection_pool();
        let state = AppState {
            static_data: Arc::new(StaticData { db: db_pool }),
        };
        let app = test::init_service(App::new().route("/", web::get().to(handle))).await;
        let user = NewUser {
            username: String::from("usr"),
            password: String::from("legit_pass"),
        };
        let req = test::TestRequest::get()
            .app_data(state)
            .set_payload(serde_json::to_string(&user).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }
}
