use crate::{
    config::db::DbConnection,
    models::user::{LoginInfoDTO, User},
    toolbox::errors::CustomError,
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
    pub fn generate_token(login: LoginInfoDTO) -> Result<String, CustomError> {
        let now = Utc::now().timestamp_millis() / 1000; //seconds
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: login.username,
            login_session: login.login_session,
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
    pub fn verify_token(
        token_data: &TokenData<Self>,
        conn: &DbConnection,
    ) -> Result<String, String> {
        if User::is_valid_login_session(&token_data.claims, conn) {
            Ok(token_data.claims.user.to_string())
        } else {
            Err("Invalid token".to_string())
        }
    }
    pub fn get_user_id(
        token_data: &TokenData<Self>,
        conn: &DbConnection,
    ) -> Result<i32, CustomError> {
        User::find_id_by_login_session(&token_data.claims, conn)
    }
}
