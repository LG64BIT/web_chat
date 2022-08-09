//!Jason web token module, relying on [jsonwebtoken] crate
use chrono::{Duration, Utc};
use jsonwebtoken::*;
use serde::{Deserialize, Serialize};

use crate::{errors::ShopError, models::user::User};
#[derive(Debug, Serialize, Deserialize)]

/// Main structure for encoding user info into token
pub struct UserClaims {
    pub id: String,
    pub username: String,
    pub exp: i64,
    pub iat: i64,
}
/// Function for encoding token from [User] struct.
pub fn generate(user: &User) -> Result<String, ShopError> {
    let secret = dotenv::var("JWT_SECRET_KEY").unwrap_or_else(|_| "".into());
    let duration = dotenv::var("JWT_LIFETIME_IN_SECONDS")
        .unwrap_or_else(|_| "300".into())
        .parse()?;
    let exp = Utc::now() + Duration::seconds(duration);
    let claims = UserClaims {
        id: String::from(&user.id),
        username: String::from(&user.username),
        exp: exp.timestamp(),
        iat: Utc::now().timestamp(),
    };
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&secret.as_bytes()),
    )?)
}
///Function for verifing token and returning user instance from token
pub fn verify(token: String) -> Result<User, ShopError> {
    let secret = dotenv::var("JWT_SECRET_KEY");
    let secret = secret.unwrap_or_else(|_| "".into());
    let token_data = decode::<UserClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(User::from_jwt(&token_data.claims))
}
