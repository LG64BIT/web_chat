use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::errors::ShopError;
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
) -> Result<HttpResponse, ShopError> {
    let user = User::is_logged(&req)?;
    let connection = state.get_pg_connection()?;
    let group_count = groups::table
        .select(groups::id)
        .filter(groups::id.eq(&group.id))
        .load::<String>(&connection)?;
    if group_count.is_empty() {
        return Err(ShopError::NotFoundError("Group not found".to_string()));
    }
    let user_count = groups_users::table
        .select(groups_users::id)
        .filter(groups_users::group_id.eq(&group.id))
        .filter(groups_users::user_id.eq(&user.id))
        .load::<String>(&connection)?;
    if user_count.len() != 0 {
        return Err(ShopError::AlreadyExistsError);
    }
    user.join_group(&connection, &group.id)?;
    return Ok(HttpResponse::Ok().json("Successfully joined!"));
}
