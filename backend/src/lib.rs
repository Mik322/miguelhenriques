#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;
extern crate lettre_email;
extern crate r2d2;
#[macro_use]
extern crate serde_json;
extern crate actix_web;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;

mod errors;
pub mod handlers;
pub mod middlewares;
pub mod models;
mod schema;
mod services;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create Pool")
}
