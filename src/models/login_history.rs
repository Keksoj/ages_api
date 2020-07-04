use diesel::prelude::*;

use crate::{
    config::db::DbConnection,
    models::user::User,
    schema::{login_history, login_history::dsl::*},
    toolbox::errors::CustomError,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, AsChangeset, Identifiable, Associations, Queryable,
)]
#[belongs_to(User)]
#[table_name = "login_history"]
pub struct LoginHistory {
    pub id: i32,
    pub user_id: i32,
    pub login_timestamp: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "login_history"]
pub struct InsertableLoginHistory {
    pub user_id: i32,
    pub login_timestamp: DateTime<Utc>,
}

impl LoginHistory {
    pub fn create(u_id: &i32) -> Result<InsertableLoginHistory, CustomError> {
        // let user = User::find_user_by_username(un, conn)?;
        let insertable_login_history = InsertableLoginHistory {
            user_id: *u_id,
            login_timestamp: Utc::now(),
        };
        Ok(insertable_login_history)
    }

    pub fn save_login_history(
        insert_record: InsertableLoginHistory,
        conn: &DbConnection,
    ) -> QueryResult<usize> {
        diesel::insert_into(login_history::table)
            .values(&insert_record)
            .execute(conn)
    }

    pub fn user_is_already_logged_in(
        u_id: &i32,
        conn: &DbConnection,
    ) -> Result<bool, CustomError> {
        let the_users_logins = login_history::table
            .filter(login_history::user_id.eq(u_id))
            .get_results::<LoginHistory>(conn)?;
        let a_week_ago = Utc::now() - Duration::weeks(1);
        for login_row in the_users_logins.iter() {
            if login_row.login_timestamp > a_week_ago {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
