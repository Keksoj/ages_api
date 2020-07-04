use crate::{jwt::user_token::UserToken, toolbox::errors::CustomError};
use actix_web::HttpRequest;

// ideally this should be done in the middleware and written in the app state
pub fn get_uid_from_request(request: &HttpRequest) -> Result<i32, CustomError> {
    let authen_header = match request.headers().get("Authorization") {
        Some(authen_header) => authen_header,
        None => {
            return Err(
                // because the middleware should have checked this already
                CustomError::new(400, "Something went very wrong".to_string()),
            );
        }
    };
    let authen_str = authen_header.to_str()?;
    if !authen_str.starts_with("bearer") && !authen_str.starts_with("Bearer") {
        return Err(CustomError::new(
            400,
            "The authentication header doesn't start with 'bearer'".to_string(),
        ));
    }
    let token = authen_str[6..authen_str.len()].trim();
    let token_data = UserToken::decode_token(token.to_string())?;
    let uid = token_data.claims.uid;
    Ok(uid)
}
