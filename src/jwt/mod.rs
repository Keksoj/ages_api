use crate::{models::user::User, toolbox::errors::CustomError};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // seconds

// This is to be used within the API
#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64, // issued at (posix)
    pub exp: i64, // expires at (posix)
    pub username: String,
    pub uid: i32, // user id
}

impl UserToken {
    pub fn decode_from_string(token: String) -> Result<Self, CustomError> {
        let token_data = jsonwebtoken::decode::<UserToken>(
            &token,
            &DecodingKey::from_secret(&KEY),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }

    pub fn is_still_valid(&self) -> bool {
        let now = Utc::now().timestamp_millis() / 1000; //seconds
        now < self.exp
    }
}

// this is to be sent to the client
#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: String, // usually "Bearer"
}

impl TokenResponse {
    pub fn new(token_string: String) -> Self {
        Self {
            token: token_string,
            token_type: "Bearer".to_string(),
        }
    }
}

pub fn generate_token_response(user: &User) -> Result<TokenResponse, CustomError> {
    let now = Utc::now().timestamp_millis() / 1000; //seconds
    let payload = UserToken {
        iat: now,
        exp: now + ONE_WEEK,
        username: user.username.to_string(),
        uid: user.id,
    };
    let jwt_string = jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(&KEY),
    )?;
    let token_response = TokenResponse::new(jwt_string);
    Ok(token_response)
}
