use crate::{
    config::db::connection,
    jwt::user_token::UserToken,
    models::user::{LoginDTO, User, UserDTO},
    toolbox::{errors::CustomError, response::ResponseBody},
};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde_json::json;

// POST HOST/auth/signup
pub async fn signup(
    user_dto: web::Json<UserDTO>,
) -> Result<HttpResponse, CustomError> {
    let conn = connection()?;

    match User::signup(user_dto.0, &conn) {
        Ok(message) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(&message, "".to_string())))
        }
        Err(_error) => Ok(HttpResponse::InternalServerError()
            .reason("Database issue")
            .finish()),
    }
}

// POST HOST/auth/login
pub async fn login(
    login_dto: web::Json<LoginDTO>,
) -> Result<HttpResponse, CustomError> {
    // gotta convert those CustomError into ServiceError
    let conn = connection()?;
    let login_info = User::login(login_dto.0, &conn)?;
    let jwt = UserToken::generate_token(login_info)?;
    let token_response = serde_json::from_value(json!({
        "token": jwt,
        "token_type": "bearer"
    }))?;
    Ok(HttpResponse::Ok().json(ResponseBody::new(
        "Login successfull, here, have a jwt token.",
        token_response,
    )))
}

// #[post("/auth/logout")]
pub async fn logout(request: HttpRequest) -> Result<HttpResponse, CustomError> {

    let authen_header = match request.headers().get("Authorization") {
        Some(authen_header) => authen_header,
        None => {
            return Ok(HttpResponse::BadRequest()
                .json(ResponseBody::new("Message token is missing", "")))
        }
    };

    let authen_str = authen_header.to_str()?;
    if !authen_str.starts_with("bearer") {
        return Err(CustomError::new(
            500,
            "The authentication header doesn't start with 'bearer'".to_string(),
        ));
    }

    let user_token = authen_str[6..authen_str.len()].trim();
    let token_data = UserToken::decode_token(user_token.to_string())?;
    let conn = connection()?;

    let user_name = UserToken::verify_token(&token_data, &conn)?;

    match User::logout(&user_name, &conn) {
        Ok(()) => {
            return Ok(
                HttpResponse::Ok().json(ResponseBody::new("logout successful", ""))
            )
        }
        Err(_custom_error) => {
            return Ok(HttpResponse::InternalServerError()
                .reason("logout unsuccessful")
                .finish())
        }
    }
}
