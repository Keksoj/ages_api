use crate::{
    config::db::connection,
    models::person::{Person, ReceivedPerson},
    toolbox::{errors::CustomError, uid_extractor::get_uid_from_request},
};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde_json::json;

// GET HOST/persons
pub async fn find_all(request: HttpRequest) -> Result<HttpResponse, CustomError> {
    let uid = get_uid_from_request(&request)?;
    let conn = connection()?;
    let persons = Person::find_all(uid, &conn)?;
    Ok(HttpResponse::Ok().json(persons))
}

// GET HOST/{id}
pub async fn find(person_id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let conn = connection()?;
    let person = Person::find_by_id(person_id.into_inner(), &conn)?;
    Ok(HttpResponse::Ok().json(person))
}

// POST HOST/persons
pub async fn create(
    query_content: web::Json<ReceivedPerson>,
    request: HttpRequest,
) -> Result<HttpResponse, CustomError> {
    let uid = get_uid_from_request(&request)?;

    debug!(
        "We receided a post request with this content: {:?}",
        query_content
    );
    let received_person = query_content.clone();
    
    let conn = connection()?;
    let response_data = Person::create(received_person, uid, &conn)?;
    Ok(HttpResponse::Ok().json(response_data))
}

// PUT HOST/persons/
pub async fn update(
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
    let todo = Person::update(updated_person, cloned_content.id, &conn)?;
    Ok(HttpResponse::Ok().json(todo))
}

// DELETE HOST/person/{id}
pub async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let conn = connection()?;
    let deleted_person = Person::delete(id.into_inner(), &conn)?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_person })))
}
