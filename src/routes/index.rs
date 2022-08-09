use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::errors::ShopError;
use crate::models::group::Group;
use crate::models::group::UserGroups;
use crate::utils::AppState;
use crate::{
    models::user::User,
    schema::{groups, groups_users, users},
};
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use diesel::result::Error;
use diesel::QueryDsl;

/// Gets currently logged in (self) user info
/// # HTTP request
/// Request must be in [Json](actix_web::web::Json) format
/// ## Header
/// * jwt: [String] - JWT autorization token
///
/// # HTTP response
/// * Success code: 200
/// * Response is in [Json](actix_web::web::Json) format
/// ```
/// {
///  "user": {
///      "id": "f7169845-4de5-470e-bb76-7117d4620d8c",
///      "username": "test_user"
///  },
///     "groups": [
///         {
///             "id": "9780f090-82a7-47dc-a64a-c4b1ad3c978d",
///             "name": "group_1"
///         },
///         {
///             "id": "d819befb-c975-4a0d-bdcd-b619848f1b5b",
///             "name": "group_2"
///         }
///     ]
///  }
/// ```
/// Error code: 403
pub async fn handle(state: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, ShopError> {
    if let Ok(user) = User::is_logged(&req) {
        let connection = state.get_pg_connection()?;
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
        return Ok(HttpResponse::Ok().json(info));
    } else {
        Err(ShopError::NoPermission(
            "No permission for that action".to_string(),
        ))
    }
}
