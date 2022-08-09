use crate::diesel::prelude::*;
use crate::errors::ShopError;
use crate::jwt::{self, UserClaims};
use crate::schema::{groups_users, users};
use actix_web::HttpRequest;
use bcrypt::verify;
use serde::{Deserialize, Serialize};

pub const MIN_USERNAME_LENGTH: u8 = 5;
pub const MIN_PASSWORD_LENGTH: u8 = 8;

/// Main struct for manipulating with user data
#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl User {
    /// Get [User] by username from database
    pub fn get_by_username(connection: &PgConnection, username: &str) -> Result<Self, ShopError> {
        let result = users::table
            .select(users::all_columns)
            .filter(users::username.eq(username))
            .first::<Self>(connection)?;
        Ok(User {
            id: result.id.clone(),
            username: result.username.clone(),
            password: result.password.clone(),
        })
    }
    /// Get username: [String] of user with provided id: [String]
    pub fn get_username(connection: &PgConnection, id: &str) -> Result<String, ShopError> {
        let username = users::table
            .select(users::username)
            .filter(users::id.eq(id))
            .first::<String>(connection)?;
        Ok(username)
    }
    /// Check if username: [String] is available for use
    fn is_available_username(connection: &PgConnection, username: &str) -> bool {
        let result = User::get_by_username(connection, username);
        if result.is_err() {
            return true;
        }
        return false;
    }
    /// Check if user is registered
    /// # Returns
    /// ## On success
    /// * Tuple of registered user and generated token, (user: [User], token: [String])
    /// ## On faliure
    /// * error: [ShopError]
    pub fn authenticate(
        connection: &PgConnection,
        username: &str,
        password: &str,
    ) -> Result<(User, String), ShopError> {
        let user = User::get_by_username(&connection, &username)?;
        if !verify(password, &user.password)? {
            return Err(ShopError::NoPermission(
                "No permission for that action".to_string(),
            ));
        }
        let token = user.generate_jwt()?;
        Ok((user, token))
    }
    /// Method for generating token: [String] on current user object
    fn generate_jwt(&self) -> Result<String, ShopError> {
        crate::jwt::generate(&self)
    }
    /// Function for creating [User] struct from [UserClaims] struct
    pub fn from_jwt(claims: &UserClaims) -> Self {
        User {
            id: String::from(&claims.id),
            username: String::from(&claims.username),
            password: String::new(),
        }
    }
    ///Check if user is currently logged in
    /// # Returns
    /// ## On success
    /// * Currently logged in user: [User]
    /// ## On faliure
    /// * error: [ShopError]
    pub fn is_logged(req: &HttpRequest) -> Result<User, ShopError> {
        let user_jwt = match req.headers().get("jwt") {
            Some(jwt) => jwt.to_str()?,
            None => return Err(ShopError::InvalidInput),
        };
        Ok(jwt::verify(String::from(user_jwt))?)
    }
    /// Method on User object, joins self to provided group
    /// # Returns
    /// ## On success
    /// * number of inserted rows: [usize]
    /// ## On faliure
    /// * error: [ShopError]
    pub fn join_group(
        &self,
        connection: &PgConnection,
        group_id: &str
    ) -> Result<usize, ShopError> {
        Ok(diesel::insert_into(groups_users::table)
            .values((
                groups_users::user_id.eq(self.id.clone()),
                groups_users::group_id.eq(group_id),
            ))
            .execute(connection)?)
    }

    pub fn is_group_owner(
        &self,
        connection: &PgConnection,
        group_id: &str
    ) -> Result<bool, ShopError> {
        let owner = groups_users::table
            .select(groups_users::id)
            .filter(groups_users::group_id.eq(group_id))
            .filter(groups_users::user_id.eq(&self.id))
            .load::<String>(connection)?;
        if owner.len() != 1 {
            return Ok(false);
        }
        Ok(true)
    }
}

/// Struct for validating and inserting [User] into database
#[derive(Insertable, Debug, Serialize, Deserialize, validator::Validate)]
#[table_name = "users"]
pub struct NewUser {
    #[validate(length(min = "MIN_USERNAME_LENGTH"))]
    pub username: String,
    #[validate(length(min = "MIN_PASSWORD_LENGTH"))]
    pub password: String,
}

impl NewUser {
    /// Function that crates new user record in database
    /// # Returns
    /// ## On success
    /// * Newly created user: [User]
    /// ## On faliure
    /// * error: [ShopError]
    pub fn create(
        connection: &PgConnection,
        username: &str,
        pass: &str,
    ) -> Result<User, ShopError> {
        if !User::is_available_username(&connection, username) {
            return Err(ShopError::AlreadyExistsError);
        }
        let user = Self {
            username: username.to_string(),
            password: bcrypt::hash(&pass, bcrypt::DEFAULT_COST).unwrap(),
        };
        Ok(diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(connection)?)
    }
}
