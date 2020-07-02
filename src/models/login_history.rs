use diesel::prelude::*;

use crate::{
    config::db::DbConnection,
    models::user::User,
    schema::login_history::{self, dsl::*},
    toolbox::errors::CustomError,
};
use chrono::{DateTime, Utc};

#[derive(Identifiable, Associations, Queryable)]
#[belongs_to(User)]
#[table_name = "login_history"]
pub struct LoginHistory {
    pub id: i32,
    pub user_id: i32,
    pub login_timestamp: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "login_history"]
pub struct LoginHistoryInsertableDTO {
    pub user_id: i32,
    pub login_timestamp: DateTime<Utc>,
}

impl LoginHistory {
    pub fn create(
        un: &str,
        conn: &DbConnection,
    ) -> Result<LoginHistoryInsertableDTO, CustomError> {
        let user = User::find_user_by_username(un, conn)?;
        let login_history_insertable_dto = LoginHistoryInsertableDTO {
            user_id: user.id,
            login_timestamp: Utc::now(),
        };
        Ok(login_history_insertable_dto)
    }

    pub fn save_login_history(
        insert_record: LoginHistoryInsertableDTO,
        conn: &DbConnection,
    ) -> QueryResult<usize> {
        diesel::insert_into(login_history)
            .values(&insert_record)
            .execute(conn)
    }
}
