use crate::{
    config::db::Pool,
    models::person::{Person, ReceivedPerson},
    toolbox::{errors::CustomError, uid_extractor::get_uid_from_request},
};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde_json::json;

// GET HOST/persons
pub async fn find_all(
    request: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, CustomError> {
    let uid = get_uid_from_request(&request)?;
    let conn = pool.get()?;
    let persons = Person::find_all(uid, &conn)?;
    Ok(HttpResponse::Ok().json(persons))
}

// GET HOST/{id}
pub async fn find(
    person_id: web::Path<i32>,
    request: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, CustomError> {
    let uid = get_uid_from_request(&request)?;
    let conn = pool.get()?;
    let person = Person::find_by_id(uid, person_id.into_inner(), &conn)?;
    Ok(HttpResponse::Ok().json(person))
}

// POST HOST/persons
pub async fn create(
    query_content: web::Json<ReceivedPerson>,
    request: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, CustomError> {
    debug!(
        "We receided a post request with this content: {:?}",
        query_content
    );
    let uid = get_uid_from_request(&request)?;
    let received_person = query_content.clone();
    let conn = pool.get()?;
    let response_data = Person::create(uid, received_person, &conn)?;
    Ok(HttpResponse::Ok().json(response_data))
}

// PUT HOST/persons/
pub async fn update(
    request: HttpRequest,
    query_content: web::Json<Person>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, CustomError> {
    info!(
        "We receided an update request with this content: {:?}",
        query_content
    );
    let uid = get_uid_from_request(&request)?;

    let updated_person = query_content.clone();
    let conn = pool.get()?;
    let todo = Person::update(uid, updated_person, &conn)?;
    Ok(HttpResponse::Ok().json(todo))
}

// DELETE HOST/person/{id}
pub async fn delete(
    id: web::Path<i32>,
    request: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, CustomError> {
    let uid = get_uid_from_request(&request)?;
    let conn = pool.get()?;
    let deleted_person = Person::delete(uid, id.into_inner(), &conn)?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_person })))
}
