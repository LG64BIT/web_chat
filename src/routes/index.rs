use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::models::group::Group;
use crate::models::group::UserGroups;
use crate::utils::AppState;
use crate::{
    models::user::User,
    schema::{groups, groups_users, users},
};
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse, Responder};
use diesel::result::Error;
use diesel::QueryDsl;

pub async fn handle(state: Data<AppState>, req: HttpRequest) -> impl Responder {
    if let Ok(user) = User::is_logged(&req) {
        let connection = state.get_pg_connection();
        let data: Result<Vec<Group>, Error> = users::table
            .inner_join(groups_users::table.inner_join(groups::table))
            .filter(users::id.eq(&user.id))
            .select(groups::all_columns)
            .load::<Group>(&connection);
        let data = data.ok();
        let info = UserGroups {
            user: user,
            groups: data,
        };
        HttpResponse::Ok().json(info)
    } else {
        HttpResponse::Forbidden().finish()
    }
    // let user_cookie = match req.cookie("jwt") {
    //     Some(cookie) => cookie,
    //     None => return HttpResponse::Forbidden().finish()
    // };
    //let user_jwt = user_cookie.value().to_string();
}
