use chrono::{Duration, Utc};
use jsonwebtoken::{errors::Error, *};
use serde::{Deserialize, Serialize};

use crate::models::user::User;
#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub id: String,
    pub username: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn generate(user: &User) -> String {
    let secret = dotenv::var("JWT_SECRET_KEY").unwrap_or_else(|_| "".into());
    let duration = dotenv::var("JWT_LIFETIME_IN_SECONDS")
        .unwrap_or_else(|_| "300".into())
        .parse()
        .unwrap();
    let exp = Utc::now() + Duration::seconds(duration);
    let claims = UserClaims {
        id: String::from(&user.id),
        username: String::from(&user.username),
        exp: exp.timestamp(),
        iat: Utc::now().timestamp(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&secret.as_bytes()),
    )
    .unwrap_or_default()
}

pub fn verify(token: String) -> Result<User, Error> {
    let secret = dotenv::var("JWT_SECRET_KEY");
    let secret = secret.unwrap_or_else(|_| "".into());
    let token_data = decode::<UserClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(User::from_jwt(&token_data.claims))
}
