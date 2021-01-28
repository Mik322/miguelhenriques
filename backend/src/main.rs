#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;
extern crate lettre_email;
extern crate r2d2;
#[macro_use]
extern crate serde_json;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;

mod errors;
mod handlers;
mod models;
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Cors::permissive())
            .service(handlers::get_projects)
            .service(handlers::add_project)
            .service(handlers::remove_project)
            .service(handlers::send_email)
            .service(handlers::login)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
