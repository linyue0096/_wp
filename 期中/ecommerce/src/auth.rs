use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::models::UserPublic;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub exp: usize,
}

pub fn create_jwt_raw(user: &UserPublic, secret: &str) -> String {
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::days(7)).timestamp() as usize;

    let claims = Claims {
        sub: user.id.clone(),
        username: user.username.clone(),
        email: user.email.clone(),
        role: user.role.clone(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn create_jwt(user: &crate::models::User, secret: &str) -> String {
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::days(7)).timestamp() as usize;

    let claims = Claims {
        sub: user.id.clone(),
        username: user.username.clone(),
        email: user.email.clone(),
        role: user.role.clone(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn verify_jwt(token: &str, secret: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .ok()
    .map(|data| data.claims)
}

pub async fn get_user_from_jar(jar: &CookieJar, state: &AppState) -> Option<UserPublic> {
    let token = jar.get("token")?.value().to_string();
    let claims = verify_jwt(&token, &state.jwt_secret)?;
    Some(UserPublic {
        id: claims.sub,
        username: claims.username,
        email: claims.email,
        role: claims.role,
    })
}
