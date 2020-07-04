use crate::{config::db::DbConnection, models::user::User, toolbox::errors::CustomError};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // seconds

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64, // issued at (posix)
    pub exp: i64, // expires at (posix)
    pub username: String,
    pub uid: i32,
    // pub login_session: String,
}

impl UserToken {
    pub fn generate_token_string(user: &User) -> Result<String, CustomError> {
        let now = Utc::now().timestamp_millis() / 1000; //seconds
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            username: user.username.to_string(),
            uid: user.id,
            // login_session: login_session.to_string(),
        };
        let jwt_string = jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&KEY),
        )?;
        Ok(jwt_string)
    }
    pub fn generate_token_response(user: &User) -> Result<TokenResponse, CustomError> {
        let jwt_string = Self::generate_token_string(&user)?;
        Ok(TokenResponse::new(jwt_string))
    }

    pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<Self>> {
        jsonwebtoken::decode::<UserToken>(
            &token,
            &DecodingKey::from_secret(&KEY),
            &Validation::default(),
        )
    }

    pub fn is_still_valid(token_data: &TokenData<Self>) -> bool {
        let now = Utc::now().timestamp_millis() / 1000; //seconds
        if now < token_data.claims.exp {
            true
        } else {
            false
        }
    }

    // obsolete
    pub fn get_user_id(
        token_data: &TokenData<Self>,
        conn: &DbConnection,
    ) -> Result<i32, CustomError> {
        let user = User::find_user_by_id(&token_data.claims.uid, conn)?;
        Ok(user.id)
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
