use crate::{
    config::db::connection,
    jwt::user_token::UserToken,
    models::person::{Person, ReceivedPerson},
    toolbox::{errors::CustomError, response::ResponseBody},
};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde_json::json;

// GET HOST/persons
pub async fn find_all(request: HttpRequest) -> Result<HttpResponse, CustomError> {
    // parsing the user token to find the user id
    // should be done in the middleware, the user id should be accessible anywhere
    let authen_header = match request.headers().get("Authorization") {
        Some(authen_header) => authen_header,
        None => {
            return Ok(HttpResponse::BadRequest()
                .json(ResponseBody::new("Something went very wrong", "")));
            // because the middleware should have checked this already
        }
    };
    let authen_str = authen_header.to_str()?;
    if !authen_str.starts_with("bearer") {
        return Err(CustomError::new(
            500,
            "The authentication header doesn't start with 'bearer'".to_string(),
        ));
    }
    let token = authen_str[6..authen_str.len()].trim();
    let token_data = UserToken::decode_token(token.to_string())?;
    let conn = connection()?;
    let user_id = UserToken::get_user_id(&token_data, &conn)?;

    let persons = Person::find_all(user_id, &conn)?;
    Ok(HttpResponse::Ok().json(persons))
}

// GET HOST/{id}
pub async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let conn = connection()?;
    let person = Person::find_by_id(id.into_inner(), &conn)?;
    Ok(HttpResponse::Ok().json(person))
}

// POST HOST/persons
pub async fn create(
    query_content: web::Json<ReceivedPerson>,
) -> Result<HttpResponse, CustomError> {
    debug!(
        "We receided a post request with this content: {:?}",
        query_content
    );
    let cloned_content = query_content.clone();
    let received_person = ReceivedPerson {
        name: cloned_content.name,
        birthdate: cloned_content.birthdate,
    };
    let conn = connection()?;
    let response_data = Person::create(received_person, &conn)?;
    Ok(HttpResponse::Ok().json(response_data))
}

// PUT HOST/persons/{id}
pub async fn update(
    id: web::Path<i32>,
    query_content: web::Json<Person>,
) -> Result<HttpResponse, CustomError> {
    info!(
        "We receided an update request with this content: {:?}",
        query_content
    );
    let cloned_content = query_content.clone();
    let updated_person = Person {
        id: cloned_content.id,
        name: cloned_content.name,
        birthdate: cloned_content.birthdate,
        user_id: cloned_content.user_id,
    };
    let conn = connection()?;

    let todo = Person::update(updated_person, id.into_inner(), &conn)?;
    Ok(HttpResponse::Ok().json(todo))
}

// DELETE HOST/person/{id}
pub async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let conn = connection()?;
    let deleted_person = Person::delete(id.into_inner(), &conn)?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_person })))
}
