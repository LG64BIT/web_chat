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

#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, Display, Clone)]
pub enum UserError {
    AlreadyExistsError,
    NotFoundError,
    ConnectionError,
    InvalidCredentials,
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::AlreadyExistsError => StatusCode::ALREADY_REPORTED,
            UserError::NotFoundError => StatusCode::NOT_FOUND,
            UserError::ConnectionError => StatusCode::REQUEST_TIMEOUT,
            UserError::InvalidCredentials => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

//Get user by username from database
impl User {
    pub fn get(connection: &PgConnection, username: &str) -> Result<Self, UserError> {
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

    fn is_available_username(connection: &PgConnection, username: &str) -> bool {
        let result = User::get(connection, username);
        if result.is_err() {
            return true;
        }
        return false;
    }

    //remove from database
    // pub fn remove(username: &str) {
    //     let connection = utils::establish_connection();
    //     diesel::delete(users::table.filter(users::username.eq(username)))
    //         .execute(&connection)
    //         .expect("Error deleting file");
    // }

    pub fn authenticate(
        connection: &PgConnection,
        username: &str,
        password: &str,
    ) -> Result<(User, String), UserError> {
        let user = User::get(&connection, &username)?;
        if !verify(password, &user.password).unwrap() {
            return Err(UserError::InvalidCredentials);
        }
        let token = user.generate_jwt();
        Ok((user, token))
    }

    pub fn generate_jwt(&self) -> String {
        crate::jwt::generate(&self)
    }

    pub fn from_jwt(claims: &UserClaims) -> Self {
        User {
            id: String::from(&claims.id),
            username: String::from(&claims.username),
            password: String::new(),
        }
    }

    pub fn is_logged(req: &HttpRequest) -> Result<User, UserError> {
        //let user_cookie = match req.cookie("jwt") {
        //     Some(cookie) => cookie,
        //     None => return false,
        // };
        //let user_jwt = user_cookie.value().to_string();
        let user_jwt = match req.headers().get("jwt") {
            Some(jwt) => jwt.to_str().unwrap(),
            None => return Err(UserError::InvalidCredentials),
        };
        match jwt::verify(String::from(user_jwt)) {
            Ok(user) => Ok(user),
            Err(_) => Err(UserError::InvalidCredentials),
        }
    }

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

//insert into database
#[derive(Insertable, Debug, Serialize, Deserialize, validator::Validate)]
#[table_name = "users"]
pub struct NewUser {
    #[validate(length(min = "MIN_USERNAME_LENGTH"))]
    pub username: String,
    #[validate(length(min = "MIN_PASSWORD_LENGTH"))]
    pub password: String,
}

impl NewUser {
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
