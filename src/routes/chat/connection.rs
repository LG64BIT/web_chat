use crate::diesel::ExpressionMethods;
use crate::errors::ShopError;
use crate::utils::AppState;
use crate::{diesel::RunQueryDsl, schema::groups_users};
use crate::{
    models::{group::Group, lobby::Lobby, user::User, ws::WsConn},
    schema::{groups, users},
};
use actix::Addr;
use actix_web::{
    web::{self, Data, Payload},
    HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use diesel::QueryDsl;
use uuid::Uuid;

/// Enters selected chat group
/// # HTTP request
/// ## Header
/// * jwt: [String] - JWT autorization token
///
/// # HTTP response
/// Success code: 101
/// Switching to web-socket protocol v13
///
/// Error code: 403, 500
pub async fn handle(
    state: Data<AppState>,
    req: HttpRequest,
    stream: Payload,
    group_id: web::Path<Uuid>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, ShopError> {
    let user = User::is_logged(&req)?;
    let connection = state.get_pg_connection()?;
    let result = users::table
        .inner_join(groups_users::table.inner_join(groups::table))
        .filter(users::id.eq(&user.id))
        .filter(groups::id.eq(&group_id.to_string()))
        .select(groups::all_columns)
        .load::<Group>(&connection)?;
    if result.len() == 0 {
        return Err(ShopError::NoPermission(
            "No permission for that action".to_string(),
        ));
    }
    let ws = WsConn::new(*group_id, srv.get_ref().clone(), Uuid::parse_str(&user.id)?);
    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}
