use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use validator::Validate;
use crate::{models::user::NewUser, utils::AppState};

pub async fn handle(state: Data<AppState>, user: Json<NewUser>) -> HttpResponse {

    let connection = state.get_pg_connection();
    match user.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::InternalServerError().json(e),
    };
    match NewUser::create(&connection, &user.username, &user.password) {
        Ok(created) => HttpResponse::Ok().json(created),
        Err(e) => HttpResponse::from_error(e)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        models::user::NewUser,
        utils::{get_connection_pool, StaticData},
    };
    use std::sync::Arc;
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_register_short_password() {
        let db_pool = get_connection_pool();
        let state = AppState {
            static_data: Arc::new(StaticData { db: db_pool }),
        };
        let app =
        test::init_service(App::new().route("/", web::get().to(handle))).await;
        let user = NewUser {
            username: String::from("legit_user"),
            password: String::from("123"),
        };
        let req = test::TestRequest::get().app_data(state).set_payload(serde_json::to_string(&user).unwrap()).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_register_invalid_password() {
        let db_pool = get_connection_pool();
        let state = AppState {
            static_data: Arc::new(StaticData { db: db_pool }),
        };
        let app =
        test::init_service(App::new().route("/", web::get().to(handle))).await;
        let user = NewUser {
            username: String::from("legit_user"),
            password: String::from("        "), // 8 spaces password
        };
        let req = test::TestRequest::get().app_data(state).set_payload(serde_json::to_string(&user).unwrap()).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_register_short_username() {
        let db_pool = get_connection_pool();
        let state = AppState {
            static_data: Arc::new(StaticData { db: db_pool }),
        };
        let app =
        test::init_service(App::new().route("/", web::get().to(handle))).await;
        let user = NewUser {
            username: String::from("usr"),
            password: String::from("legit_pass"),
        };
        let req = test::TestRequest::get().app_data(state).set_payload(serde_json::to_string(&user).unwrap()).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }
}