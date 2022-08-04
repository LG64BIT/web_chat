use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::models::group::JoinableGroup;
use crate::schema::groups_users;
use crate::utils::AppState;
use crate::{models::user::User, schema::groups};
use actix_web::web::Data;
use actix_web::{web::Json, HttpRequest, HttpResponse};
use diesel::QueryDsl;

/// Joins current user to provided group
///
/// # HTTP request
/// Request must be in [Json] format
/// ## Header
/// * jwt: [String] - JWT autorization token
/// ## Body
/// * id: [String] - group id
///
/// # HTTP response
/// Success code: 200
///
/// Error code: 403, 404, 500
pub async fn handle(
    state: Data<AppState>,
    req: HttpRequest,
    group: Json<JoinableGroup>,
) -> HttpResponse {
    let user = match User::is_logged(&req) {
        Ok(user) => user,
        Err(e) => return HttpResponse::Forbidden().json(e.to_string()),
    };
    let connection = state.get_pg_connection();
    let group_count = match groups::table
        .select(groups::id)
        .filter(groups::id.eq(&group.id))
        .load::<String>(&connection)
    {
        Ok(v) => v,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()), //change to this syntax in whole project
    };
    if group_count.is_empty() {
        return HttpResponse::NotFound().finish();
    }
    let user_count = match groups_users::table
        .select(groups_users::id)
        .filter(groups_users::group_id.eq(&group.id))
        .filter(groups_users::user_id.eq(&user.id))
        .load::<String>(&connection)
    {
        Ok(v) => v,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };
    if user_count.len() != 0 {
        return HttpResponse::AlreadyReported().json("User already joined!");
    }
    match user.join_group(&connection, &group.id) {
        Ok(_) => (),
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };
    return HttpResponse::Ok().json("Successfully joined!");
}
