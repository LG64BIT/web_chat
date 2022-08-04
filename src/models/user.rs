use crate::diesel::prelude::*;
use crate::jwt::{self, UserClaims};
use crate::schema::{groups_users, users};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use bcrypt::verify;
use derive_more::Display;
use diesel::result::Error;
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

/// Custom error type for handling user errors
#[derive(Debug, Display, Clone)]
pub enum UserError {
    AlreadyExistsError,
    NotFoundError,
    ConnectionError,
    InvalidCredentials,
}

/// Implementing ResponseError so that UserError can be returned in handler functions
impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::AlreadyExistsError => StatusCode::ALREADY_REPORTED,
            UserError::NotFoundError => StatusCode::NOT_FOUND,
            UserError::ConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::InvalidCredentials => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

impl User {
    /// Get [User] by username from database
    pub fn get_by_username(connection: &PgConnection, username: &str) -> Result<Self, UserError> {
        let result = users::table
            .select(users::all_columns)
            .filter(users::username.eq(username))
            .load::<Self>(connection);
        if result.is_err() {
            return Err(UserError::ConnectionError);
        }
        let result = result.unwrap();
        if result.len() != 1 {
            return Err(UserError::NotFoundError);
        }
        Ok(User {
            id: result[0].id.clone(),
            username: result[0].username.clone(),
            password: result[0].password.clone(),
        })
    }
    /// Get username: [String] of user with provided id: [String]
    pub fn get_username(connection: &PgConnection, id: &str) -> Result<String, UserError> {
        let username = match users::table
            .select(users::username)
            .filter(users::id.eq(id))
            .load::<String>(connection)
        {
            Ok(u) => u[0].to_string(),
            Err(_) => return Err(UserError::ConnectionError),
        };
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
    /// * error: [UserError]
    pub fn authenticate(
        connection: &PgConnection,
        username: &str,
        password: &str,
    ) -> Result<(User, String), UserError> {
        let user = User::get_by_username(&connection, &username)?;
        if !verify(password, &user.password).unwrap() {
            return Err(UserError::InvalidCredentials);
        }
        let token = user.generate_jwt();
        Ok((user, token))
    }
    /// Method for generating token: [String] on current user object
    pub fn generate_jwt(&self) -> String {
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
    /// * error: [UserError]
    pub fn is_logged(req: &HttpRequest) -> Result<User, UserError> {
        let user_jwt = match req.headers().get("jwt") {
            Some(jwt) => jwt.to_str().unwrap(),
            None => return Err(UserError::InvalidCredentials),
        };
        match jwt::verify(String::from(user_jwt)) {
            Ok(user) => Ok(user),
            Err(_) => Err(UserError::InvalidCredentials),
        }
    }
    /// Method on User object, joins self to provided group
    /// # Returns
    /// ## On success
    /// * ()
    /// ## On faliure
    /// * error: [Error]
    pub fn join_group(&self, connection: &PgConnection, group_id: &str) -> Result<(), Error> {
        let new_user_group = (
            groups_users::user_id.eq(self.id.clone()),
            groups_users::group_id.eq(group_id),
        );
        match diesel::insert_into(groups_users::table)
            .values(&new_user_group)
            .execute(connection)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
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
    /// * error: [UserError]
    pub fn create(
        connection: &PgConnection,
        username: &str,
        pass: &str,
    ) -> Result<User, UserError> {
        if !User::is_available_username(&connection, username) {
            return Err(UserError::AlreadyExistsError);
        }
        let user = Self {
            username: username.to_string(),
            password: bcrypt::hash(&pass, bcrypt::DEFAULT_COST).unwrap(),
        };
        match diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(connection)
        {
            Ok(user) => Ok(user),
            Err(_) => Err(UserError::ConnectionError),
        }
    }
}
