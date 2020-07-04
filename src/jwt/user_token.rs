use crate::{
    config::db::DbConnection, models::user::User, toolbox::errors::CustomError,
};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // seconds

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64, // issued at (posix)
    pub exp: i64, // expires at (posix)
    pub user: String,
    pub login_session: String,
}

impl UserToken {
    pub fn generate_token(
        username: &str,
        login_session: &str,
    ) -> Result<String, CustomError> {
        let now = Utc::now().timestamp_millis() / 1000; //seconds
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: username.to_string(),
            login_session: login_session.to_string(),
        };
        let jwt = jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&KEY),
        )?;
        Ok(jwt)
    }

    pub fn decode_token(
        token: String,
    ) -> jsonwebtoken::errors::Result<TokenData<Self>> {
        jsonwebtoken::decode(
            &token,
            &DecodingKey::from_secret(&KEY),
            &Validation::default(),
        )
    }

    pub fn token_is_still_valid(
        token_data: &TokenData<Self>,
        // conn: &DbConnection,
    ) -> bool {
        let now = Utc::now().timestamp_millis() / 1000; //seconds
        if token_data.claims.exp < now {
            true
        } else {
            false
        }

    }
    pub fn get_user_id(
        token_data: &TokenData<Self>,
        conn: &DbConnection,
    ) -> Result<i32, CustomError> {
        User::find_id_by_login_session(&token_data.claims, conn)
    }
}

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
