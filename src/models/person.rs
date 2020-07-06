use actix_web::web;
use diesel::prelude::*;

use crate::{
    config::db::Pool, schema::persons,
    toolbox::errors::CustomError,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable, Clone, Debug)]
#[table_name = "persons"]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub birthdate: i64,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable, Clone, Debug)]
#[table_name = "persons"]
pub struct InsertablePerson {
    pub name: String,
    pub birthdate: i64,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReceivedPerson {
    pub name: String,
    pub birthdate: i64,
}

// to avoid confusion with column name 'user_id', we spell it 'uid'
impl Person {
    pub fn find_all(uid: i32, pool: &web::Data<Pool>) -> Result<Vec<Self>, CustomError> {
        let conn = pool.get()?;
        let persons = persons::table
            .filter(persons::user_id.eq(uid))
            .get_results(&conn)?;
        Ok(persons)
    }

    pub fn find_by_id(
        uid: i32,
        person_id: i32,
        pool: &web::Data<Pool>,
    ) -> Result<Self, CustomError> {
        let conn = pool.get()?;
        let person = persons::table
            .filter(persons::id.eq(person_id))
            .filter(persons::user_id.eq(uid))
            .first(&conn)?;
        Ok(person)
    }

    pub fn create(
        uid: i32,
        received_person: ReceivedPerson,
        pool: &web::Data<Pool>,
    ) -> Result<Self, CustomError> {
        let conn = pool.get()?;
        let insertable_person = InsertablePerson {
            name: received_person.name,
            birthdate: received_person.birthdate,
            user_id: uid,
        };

        let person = diesel::insert_into(persons::table)
            .values(insertable_person)
            .get_result(&conn)?;
        Ok(person)
    }

    pub fn update(
        uid: i32,
        updated_person: Person,
        pool: &web::Data<Pool>,
    ) -> Result<Self, CustomError> {
        let conn = pool.get()?;

        let person = diesel::update(persons::table)
            .filter(persons::id.eq(updated_person.id))
            .filter(persons::user_id.eq(uid))
            .set(updated_person)
            .get_result(&conn)?;
        Ok(person)
    }

    pub fn delete(
        uid: i32,
        person_id: i32,
        pool: &web::Data<Pool>,
    ) -> Result<Self, CustomError> {
        let conn = pool.get()?;

        let deleted_person = diesel::delete(persons::table)
            .filter(persons::id.eq(person_id))
            .filter(persons::user_id.eq(uid))
            .get_result(&conn)?;
        Ok(deleted_person)
    }

    pub fn delete_all_with_uid(
        uid: i32,
        pool: &web::Data<Pool>,
    ) -> Result<usize, CustomError> {
        let conn = pool.get()?;

        let number_of_deleted_persons = diesel::delete(persons::table)
            .filter(persons::user_id.eq(uid))
            .execute(&conn)?;
        Ok(number_of_deleted_persons)
    }
}
