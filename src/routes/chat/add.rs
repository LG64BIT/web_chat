use crate::diesel::RunQueryDsl;
use crate::errors::ShopError;
use crate::models::group::Group;
use crate::utils::AppState;
use crate::{
    models::{
        group::{InsertableNewGroup, NewGroup},
        user::User,
    },
    schema::groups,
};
use actix_web::web::Data;
use actix_web::{web::Json, HttpRequest, HttpResponse};
use validator::Validate;

/// Adds new group and automatically joins it
///
/// # HTTP request
/// Request must be in [Json] format
/// ## Header
/// * jwt: [String] - JWT autorization token
/// ## Body
/// * name: [String] - group name, minimum 3 characters long
///
/// #HTTP response
/// Success code: 200
///
/// Error code: 400, 403, 500
pub async fn handle(
    state: Data<AppState>,
    req: HttpRequest,
    group: Json<NewGroup>,
) -> Result<HttpResponse, ShopError> {
    let user = User::is_logged(&req)?;
    let connection = state.get_pg_connection()?;
    group.validate()?;
    let insertable_group = InsertableNewGroup {
        name: group.into_inner().name,
        owner_id: user.id.clone(),
    };
    let new_group: Group = diesel::insert_into(groups::table)
        .values(insertable_group)
        .get_result::<Group>(&connection)?;
    user.join_group(&connection, &new_group.id)?;
    Ok(HttpResponse::Ok().json("Successfully added new group!"))
}
