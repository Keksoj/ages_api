use actix_web::web;
use diesel::prelude::*;

use crate::{
    config::db::DbConnection,
    config::db::Pool,
    schema::users::{self, dsl::*},
    toolbox::errors::CustomError,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Identifiable, AsChangeset, Insertable, Queryable, Clone, Debug,
)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug)]
#[table_name = "users"]
pub struct ReceivedUser {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn signup(
        received_user: ReceivedUser,
        pool: &web::Data<Pool>,
    ) -> Result<User, CustomError> {
        let conn = pool.get()?;
        if Self::user_already_exists(&received_user.username, &conn) {
            return Err(CustomError::new(
                202,
                format!("User '{}' is already registered", &received_user.username),
            ));
        }
        let hashed_passwd = hash(&received_user.password, DEFAULT_COST)?;
        let insertable_user = ReceivedUser {
            username: received_user.username,
            password: hashed_passwd,
        };
        let registered_user = diesel::insert_into(users)
            .values(&insertable_user)
            .get_result(&conn)?;
        Ok(registered_user)
    }

    pub fn login(
        received_login: &ReceivedUser,
        pool: &web::Data<Pool>,
    ) -> Result<User, CustomError> {
        let conn = pool.get()?;

        if received_login.password.is_empty() {
            return Err(CustomError::new(500, "Password is empty".to_string()));
        }

        let users_with_username = users
            .filter(username.eq(&received_login.username))
            .get_results::<User>(&conn)?;

        let matching_user =
            Self::find_matching_user(&received_login.password, users_with_username)?;

        Ok(matching_user)
    }

    pub fn update(
        uid: i32,
        new_data: ReceivedUser,
        pool: &web::Data<Pool>,
    ) -> Result<User, CustomError> {
        let conn = pool.get()?;

        let updatable_user_data = User {
            id: uid,
            username: new_data.username,
            password: hash(&new_data.password, DEFAULT_COST)?,
        };

        let user = diesel::update(users::table)
            .filter(users::id.eq(uid))
            .set(updatable_user_data)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn delete(uid: i32, pool: &web::Data<Pool>) -> Result<User, CustomError> {
        let conn = pool.get()?;
        let deleted_user = diesel::delete(users::table)
            .filter(users::id.eq(uid))
            .get_result(&conn)?;
        Ok(deleted_user)
    }

    pub fn find_matching_user(
        passwd: &str,
        users_to_check_against: Vec<Self>,
    ) -> Result<Self, CustomError> {
        for user in users_to_check_against.iter() {
            if verify(&passwd, &user.password).unwrap() {
                return Ok(user.clone());
            }
        }
        return Err(CustomError::new(400, "Password doesn't match".to_string()));
    }

    pub fn find_user_by_id(uid: &i32, conn: &DbConnection) -> QueryResult<User> {
        users.filter(id.eq(uid)).get_result::<User>(conn)
    }

    pub fn user_already_exists(un: &str, conn: &DbConnection) -> bool {
        users
            .filter(username.eq(un))
            .get_result::<User>(conn)
            .is_ok()
    }
}
