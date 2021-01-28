use crate::schema::login_history::{self, dsl::*};
use diesel::{PgConnection, insert_into};
use diesel::RunQueryDsl;
use chrono::{NaiveDateTime, Utc};
use super::user::User;

#[derive(Queryable)]
pub struct LoginHistory {
    pub id: i32,
    pub user_id: i32,
    pub login_timestamp: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="login_history"]
pub struct NewLoginHistory {
    pub user_id: i32,
    pub login_timestamp: NaiveDateTime
}

impl LoginHistory {
    pub fn add_login_history(conn: &PgConnection, user: &User) -> Result<usize, diesel::result::Error> {
        let utc = Utc::now();
        let new_lh = NewLoginHistory {
            user_id: user.id,
            login_timestamp: utc.naive_utc()
        };

        insert_into(login_history)
            .values(&new_lh)
            .execute(conn)

    }
}