use crate::errors::ShopError;
use crate::models::group::Group;
use crate::models::user::User;
use crate::utils::AppState;
use actix_web::web::{Data, Path};
use actix_web::{HttpRequest, HttpResponse};
use uuid::Uuid;

/// Removes everyone from group and deletes it
///
/// # HTTP request
/// URL param {group_id} - group id to delete
/// ## Header
/// * jwt: [String] - JWT autorization token
///
/// #HTTP response
/// Success code: 200
///
/// Error code: 400, 403, 500
pub async fn handle(
    state: Data<AppState>,
    req: HttpRequest,
    group_id: Path<Uuid>,
) -> Result<HttpResponse, ShopError> {
    let user = User::is_logged(&req)?;
    let connection = state.get_pg_connection()?;
    if !user.is_group_owner(&connection, &group_id.to_string())? {
        return Err(ShopError::NoPermission(
            "No permission for deleting that group!".to_string(),
        ));
    }
    Group::delete(&connection, &group_id.to_string())?;
    Ok(HttpResponse::Ok().json("Successfully removed group!"))
}
