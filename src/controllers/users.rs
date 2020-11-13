use crate::{
    config::db::Pool,
    jwt::user_token::UserToken,
    models::{
        person::Person,
        user::{ReceivedUser, User},
    },
    toolbox::uid_extractor::get_uid_from_request,
};
use actix_web::{web, HttpRequest, HttpResponse, Result};

// POST HOST/auth/signup
pub async fn signup(
    received_user: web::Json<ReceivedUser>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let registered_user = User::signup(received_user.0, &pool)?;
    Ok(HttpResponse::Ok().body(format!(
        "Sucessfully registered the user '{}'",
        registered_user.username
    )))
}

// POST HOST/auth/login
pub async fn login(
    json_login: web::Json<ReceivedUser>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    debug!("We received this login request: {:#?}", json_login);
    let received_login = json_login.0;

    let logged_user = User::login(&received_login, &pool)?;
    let json_token_response = UserToken::generate_token_response(&logged_user)?;
    Ok(HttpResponse::Ok().json(json_token_response))
}

// PUT /auth/update
pub async fn update(
    json_user: web::Json<ReceivedUser>,
    request: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let uid = get_uid_from_request(&request)?;

    let received_user = json_user.0;
    let updated_user = User::update(uid, received_user, &pool)?;
    Ok(HttpResponse::Ok().body(format!(
        "Successfully updated the user '{}'",
        updated_user.username
    )))
}

// DELETE /auth/delete
pub async fn delete(request: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let uid = get_uid_from_request(&request)?;

    let deleted_persons = Person::delete_all_with_uid(uid, &pool)?;
    let deleted_user = User::delete(uid, &pool)?;

    return Ok(HttpResponse::Ok().body(format!(
        "We deleted the user'{}', the {} associated persons",
        deleted_user.username, deleted_persons
    )));
}
