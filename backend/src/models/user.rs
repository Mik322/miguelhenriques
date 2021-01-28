use std::borrow::Borrow;

use serde::{Serialize, Deserialize};
use crate::schema::users::{self, dsl::*};
use crate::Pool;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, update};
use diesel::RunQueryDsl;
use actix_web::web;
use bcrypt::verify;
use uuid::Uuid;

use super::login_history::LoginHistory;

#[derive(Queryable, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub login_session: String
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginData {
    pub username: String,
    pub password: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct UserSession {
    pub username: String,
    pub login_session: String,
}


impl User {
    fn get_user_by_username(conn: &PgConnection, user_username: &str) -> Option<User> {
        let user = users.filter(username.eq(user_username)).first::<User>(conn);
        match user {
            Ok(u) => Some(u),
            _ => None
        }
    }

    pub fn login_user(pool: web::Data<Pool>, user_ld: UserLoginData) -> Option<UserSession> {
        let conn = pool.get().unwrap();
        //Gets the user with the same userme as the inputed user
        if let Some(user) = User::get_user_by_username(&conn, user_ld.username.borrow()) {
            //Returns None if password is empty or its diferent than the stored password
            if user_ld.password.is_empty() || !verify(user_ld.password, &user.password).unwrap() {return None}
            //If theres a problem storing the login record it returns none
            if LoginHistory::add_login_history(&conn, &user).is_err() {return None;}
            //Generates session string and returns the user session if the session string was successefully stored in the db
            let session_str = User::generate_login_session_string();
            if User::update_session_db(&conn, &user.username, &session_str) {
                return Some(UserSession {
                    username: user.username,
                    login_session: session_str
                })
            }

            None
        }
        else {
            None
        }
    }

    fn generate_login_session_string() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    fn update_session_db(conn: &PgConnection, un: &str, session_str: &str) -> bool {
        if let Some(user) = User::get_user_by_username(conn, un) {
            update(users.find(user.id))
                .set(login_session.eq(session_str.to_string()))
                .execute(conn)
                .is_ok()
        } else {
            false
        }
    }
}