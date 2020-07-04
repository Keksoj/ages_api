use diesel::prelude::*;

use crate::config::db::DbConnection;
use crate::schema::persons;
use crate::toolbox::errors::CustomError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable, Clone, Debug)]
#[table_name = "persons"]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub birthdate: i64,
    pub user_id: i32,
}

impl Person {
    pub fn find_all(user_id: i32, conn: &DbConnection) -> Result<Vec<Self>, CustomError> {
        let persons = persons::table
            .filter(persons::user_id.eq(user_id))
            .get_results(conn)?;
        Ok(persons)
    }

    pub fn find_by_id(person_id: i32, conn: &DbConnection) -> Result<Self, CustomError> {
        let person = persons::table
            .filter(persons::id.eq(person_id))
            .first(conn)?;
        Ok(person)
    }

    pub fn create(
        received_person: ReceivedPerson,
        conn: &DbConnection,
    ) -> Result<Self, CustomError> {
        debug!(
            "We will insert this person in the database: {:#?}",
            received_person
        );

        let person = diesel::insert_into(persons::table)
            .values(received_person)
            .get_result(conn)?;
        Ok(person)
    }

    pub fn update(
        person_to_update: Person,
        person_id: i32,
        conn: &DbConnection,
    ) -> Result<Self, CustomError> {
        let person = diesel::update(persons::table)
            .filter(persons::id.eq(person_id))
            .set(person_to_update)
            .get_result(conn)?;
        Ok(person)
    }
    pub fn delete(person_id: i32, conn: &DbConnection) -> Result<usize, CustomError> {
        let response = diesel::delete(persons::table.find(person_id)).execute(conn)?;
        Ok(response)
    }
    pub fn delete_all_wit_uid(
        user_id: i32,
        conn: &DbConnection,
    ) -> Result<usize, CustomError> {
        let number_of_deleted_persons = diesel::delete(persons::table)
            .filter(persons::user_id.eq(user_id))
            .execute(conn)?;
        Ok(number_of_deleted_persons)
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable, Clone, Debug)]
#[table_name = "persons"]
pub struct ReceivedPerson {
    pub name: String,
    pub birthdate: i64,
}
