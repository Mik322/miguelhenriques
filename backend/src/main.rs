#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;

mod schema;
mod models;
mod handlers;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create Pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = create_pool();

    HttpServer::new(move ||
        App::new()
            .data(pool.clone())
            .wrap(Cors::permissive())
            .service(
                web::scope("/get")
                    .service(handlers::get_projects)
                    .service(handlers::get_project)
            )
            .service(handlers::add_project)
        )
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
