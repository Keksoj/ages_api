use crate::{
    config::db::Pool,
    models::person::{Person, ReceivedPerson},
    toolbox::{errors::CustomError, uid_extractor::get_uid_from_request},
};
use actix_web::{web, HttpRequest, HttpResponse, Result};

// GET HOST/persons
pub async fn find_all(
    request: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let uid = get_uid_from_request(&request)?;
    let persons = Person::find_all(uid, &pool)?;
    Ok(HttpResponse::Ok().json(persons))
}

// GET HOST/{id}
pub async fn find(
    person_id: web::Path<i32>,
    request: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let uid = get_uid_from_request(&request)?;
    let person = Person::find_by_id(uid, person_id.into_inner(), &pool)?;
    Ok(HttpResponse::Ok().json(person))
}

// POST HOST/persons
pub async fn create(
    query_content: web::Json<ReceivedPerson>,
    request: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    debug!(
        "We receided a post request with this content: {:?}",
        query_content
    );
    let uid = get_uid_from_request(&request)?;
    let received_person = query_content.clone();
    let created_person = Person::create(uid, received_person, &pool)?;
    Ok(HttpResponse::Ok().json(created_person))
}

// PUT HOST/persons/
pub async fn update(
    request: HttpRequest,
    query_content: web::Json<Person>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    info!(
        "We receided an update request with this content: {:?}",
        query_content
    );
    let uid = get_uid_from_request(&request)?;

    let person_to_update = query_content.clone();
    let updated_person = Person::update(uid, person_to_update, &pool)?;
    Ok(HttpResponse::Ok().json(updated_person))
}

// DELETE HOST/person/{id}
pub async fn delete(
    id: web::Path<i32>,
    request: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let uid = get_uid_from_request(&request)?;
    let deleted_person = Person::delete(uid, id.into_inner(), &pool)?;
    Ok(HttpResponse::Ok().body(format!("Deleted the person '{}'", deleted_person.name)))
}
