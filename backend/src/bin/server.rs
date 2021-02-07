extern crate backend;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use backend::{handlers, middlewares::authentication_middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = backend::create_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Cors::permissive())
            .wrap(authentication_middleware::Authentication)
            .service(handlers::get_projects)
            .service(handlers::add_project)
            .service(handlers::remove_project)
            .service(handlers::send_email)
            .service(handlers::login)
            .service(handlers::logout)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
