use crate::{
    config::db::Pool,
    jwt::user_token::UserToken,
    models::{
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
    received_user: web::Json<ReceivedUser>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, CustomError> {
    let conn = pool.get()?;

    match User::signup(received_user.0, &conn) {
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
    json_login: web::Json<ReceivedUser>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, CustomError> {
    let conn = pool.get()?;
    debug!("We received this login request: {:#?}", json_login);
    let received_login = json_login.0;

    let logged_user = User::login(&received_login, &conn)?;
    let json_token_response = UserToken::generate_token_response(&logged_user)?;
    Ok(HttpResponse::Ok().json(ResponseBody::new(
        "Login successfull, here, have a jwt token.",
        json_token_response,
    )))
}

// PUT /auth/update
pub async fn update(
    json_user: web::Json<User>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, CustomError> {
    let conn = pool.get()?;
    let user_to_update = json_user.0;
    let updated_user = User::update(&user_to_update, &conn)?;
    Ok(HttpResponse::Ok().json(ResponseBody::new(
        "Successfullu updated the user:",
        updated_user,
    )))
}

// DELETE /auth/delete
pub async fn delete(
    request: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, CustomError> {
    let uid = get_uid_from_request(&request)?;
    let conn = pool.get()?;

    let deleted_persons = Person::delete_all_wit_uid(uid, &conn)?;
    let deleted_user = User::delete(uid, &conn)?;

    return Ok(HttpResponse::Ok().json(ResponseBody::new(
        "We deleted: ",
        format!(
            "The user'{}', the {} associated persons",
            deleted_user.username, deleted_persons
        ),
    )));
}
