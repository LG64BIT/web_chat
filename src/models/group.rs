use super::user::User;
use crate::schema::groups;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    #[serde(skip_serializing)]
    pub owner_id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct NewGroup {
    #[validate(length(min = 3))]
    pub name: String,
}

#[derive(Insertable, Debug, Deserialize, validator::Validate)]
#[table_name = "groups"]
pub struct InsertableNewGroup {
    pub name: String,
    pub owner_id: String,
}

#[derive(Deserialize)]
pub struct JoinableGroup {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct UserGroups {
    pub user: User,
    pub groups: Option<Vec<Group>>,
}
