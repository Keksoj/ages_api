use diesel::prelude::*;

use crate::{
    config::db::DbConnection,
    models::login_history::LoginHistory,
    schema::users::{self, dsl::*},
    toolbox::errors::CustomError,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Serialize, Deserialize, Identifiable, AsChangeset, Insertable, Queryable, Clone, Debug,
)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub login_session: String,
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
        conn: &DbConnection,
    ) -> Result<String, CustomError> {
        if Self::user_already_exists(&received_user.username, conn) {
            return Ok(format!(
                "User '{}' is already registered",
                &received_user.username
            ));
        }
        let hashed_passwd = hash(&received_user.password, DEFAULT_COST).unwrap();
        let insertable_user = ReceivedUser {
            username: received_user.username,
            password: hashed_passwd,
        };
        diesel::insert_into(users)
            .values(&insertable_user)
            .execute(conn)?;
        Ok("The new user is registered in the database".to_string())
    }

    pub fn login(
        received_login: &ReceivedUser,
        conn: &DbConnection,
    ) -> Result<User, CustomError> {
        if received_login.password.is_empty() {
            return Err(CustomError::new(500, "Password is empty".to_string()));
        }

        let users_with_username = users
            .filter(username.eq(&received_login.username))
            .get_results::<User>(conn)?;

        let matching_user =
            Self::find_matching_user(&received_login.password, users_with_username)?;

        let login_history = LoginHistory::create(&matching_user.id)?;

        LoginHistory::save_login_history(login_history, conn)?;
        let login_session_str = Self::generate_login_session();
        User::update_login_session_to_db(matching_user.id, &login_session_str, conn)?;
        Ok(matching_user)
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

    pub fn logout(uid: &i32, conn: &DbConnection) -> Result<(), CustomError> {
        let user = Self::find_user_by_id(uid, conn)?;
        Self::update_login_session_to_db(user.id, "", conn)?;
        Ok(())
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

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn update_login_session_to_db(
        uid: i32,
        login_session_str: &str,
        conn: &DbConnection,
    ) -> Result<(), CustomError> {
        let user = Self::find_user_by_id(&uid, conn)?;
        diesel::update(users.find(user.id))
            .set(login_session.eq(login_session_str.to_string()))
            .execute(conn)?;
        Ok(())
    }
}
