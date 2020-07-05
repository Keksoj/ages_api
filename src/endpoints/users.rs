use crate::{
    config::db::connection,
    jwt::user_token::UserToken,
    models::{
        login_history::LoginHistory,
        person::Person,
        user::{ReceivedUser, User},
    },
    toolbox::{
        errors::CustomError, response::ResponseBody, uid_extractor::get_uid_from_request,
    },
};
use actix_web::{web, HttpRequest, HttpResponse, Result};

// POST HOST/auth/signup
pub async fn signup(
    user_dto: web::Json<ReceivedUser>,
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
pub async fn login(json_login: web::Json<ReceivedUser>) -> Result<HttpResponse> {
    // gotta convert those CustomError into ServiceError
    let conn = connection()?;
    debug!("We received this login request: {:#?}", json_login);
    let received_login = json_login.0;

    let user_to_log = User::login(&received_login, &conn)?;
    let json_token_response = UserToken::generate_token_response(&user_to_log)?;
    Ok(HttpResponse::Ok().json(ResponseBody::new(
        "Login successfull, here, have a jwt token.",
        json_token_response,
    )))
}

// DELETE /auth/delete
pub async fn delete(request: HttpRequest) -> Result<HttpResponse, CustomError> {
    let uid = get_uid_from_request(&request)?;
    let conn = connection()?;

    let deleted_persons = Person::delete_all_wit_uid(uid, &conn)?;
    let deleted_logins = LoginHistory::delete_a_users_history(&uid, &conn)?;
    let deleted_user = User::delete(uid, &conn)?;

    return Ok(HttpResponse::Ok().json(ResponseBody::new(
        "We deleted :",
        format!(
            "The user'{}', the {} associated persons, {} login rows",
            deleted_user.username, deleted_persons, deleted_logins
        ),
    )));
}

// #[post("/auth/logout")]
pub async fn logout(request: HttpRequest) -> Result<HttpResponse, CustomError> {
    let uid = get_uid_from_request(&request)?;

    let conn = connection()?;

    match User::logout(&uid, &conn) {
        Ok(()) => {
            return Ok(HttpResponse::Ok().json(ResponseBody::new("logout successful", "")))
        }
        Err(_custom_error) => {
            return Ok(HttpResponse::InternalServerError()
                .reason("logout unsuccessful")
                .finish())
        }
    }
}
