use diesel::prelude::*;

use crate::{
    config::db::DbConnection,
    jwt::user_token::UserToken,
    models::login_history::LoginHistory,
    schema::users::{self, dsl::*},
    toolbox::errors::CustomError,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Serialize,
    Deserialize,
    Identifiable,
    AsChangeset,
    Insertable,
    Queryable,
    Clone,
    Debug,
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

pub struct LoginInfo {
    pub is_already_logged_in: bool,
    pub login_session: String,
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
        let hashed_passwd =
            hash(&received_user.password, DEFAULT_COST).unwrap();
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
        login_dto: &ReceivedUser,
        conn: &DbConnection,
    ) -> Result<String, CustomError> {
        let user_to_verify = users
            .filter(username.eq(&login_dto.username))
            .get_result::<User>(conn)?;

        if user_to_verify.password.is_empty() {
            return Err(CustomError::new(500, "Password is empty".to_string()));
        }
        if !verify(&login_dto.password, &user_to_verify.password).unwrap() {
            return Err(CustomError::new(
                500,
                "Password doesn't match".to_string(),
            ));
        }

        let login_history = LoginHistory::create(&user_to_verify.id)?;

        LoginHistory::save_login_history(login_history, conn)?;
        let login_session_str = Self::generate_login_session();
        User::update_login_session_to_db(
            &user_to_verify.username,
            &login_session_str,
            conn,
        )?;
        Ok(login_session_str)
    }

    pub fn logout(
        user_name: &str,
        conn: &DbConnection,
    ) -> Result<(), CustomError> {
        let user = Self::find_user_by_username(user_name, conn)?;
        Self::update_login_session_to_db(&user.username, "", conn)?;
        Ok(())
    }

    pub fn is_valid_login_session(
        user_token: &UserToken,
        conn: &DbConnection,
    ) -> bool {
        users
            .filter(username.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn find_id_by_login_session(
        user_token: &UserToken,
        conn: &DbConnection,
    ) -> Result<i32, CustomError> {
        let user = users
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)?;
        Ok(user.id)
    }

    pub fn find_user_by_username(
        un: &str,
        conn: &DbConnection,
    ) -> QueryResult<User> {
        users.filter(username.eq(un)).get_result::<User>(conn)
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
        un: &str,
        login_session_str: &str,
        conn: &DbConnection,
    ) -> Result<(), CustomError> {
        let user = Self::find_user_by_username(un, conn)?;
        diesel::update(users.find(user.id))
            .set(login_session.eq(login_session_str.to_string()))
            .execute(conn)?;
        Ok(())
    }
}
