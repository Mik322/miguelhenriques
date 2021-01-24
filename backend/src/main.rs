use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .wrap(Cors::permissive())
            .service(hello)
        )
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
