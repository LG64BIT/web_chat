use super::user::User;
use crate::diesel::ExpressionMethods;
use crate::{
    errors::ShopError,
    schema::{groups, groups_users},
};
use diesel::{PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

/// Struct for representing chat group
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    #[serde(skip_serializing)]
    pub owner_id: String,
    pub name: String,
}
/// Struct received from request, used for creating new group
#[derive(Debug, Deserialize, validator::Validate)]
pub struct NewGroup {
    #[validate(length(min = 3))]
    pub name: String,
}
/// Struct for inserting new group into database
#[derive(Insertable, Debug, Deserialize, validator::Validate)]
#[table_name = "groups"]
pub struct InsertableNewGroup {
    pub name: String,
    pub owner_id: String,
}
/// Struct received from request for joining to group
#[derive(Deserialize)]
pub struct JoinableGroup {
    pub id: String,
}
/// Struct for holding [User] and all his joined groups, if any
#[derive(Debug, Serialize)]
pub struct UserGroups {
    pub user: User,
    pub groups: Option<Vec<Group>>,
}
impl Group {
    pub fn delete(connection: &PgConnection, group_id: &str) -> Result<(), ShopError> {
        diesel::delete(groups_users::table)
            .filter(groups_users::group_id.eq(group_id))
            .execute(connection)?;
        diesel::delete(groups::table)
            .filter(groups::id.eq(group_id))
            .execute(connection)?;
        Ok(())
    }
}
