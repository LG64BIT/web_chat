use super::user::User;
use crate::schema::groups;
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
