use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
struct Project {
    id: u32,
    image_name: Option<String>,
    name: String,
    description: String
}


fn get_projects_vec() -> Vec<Project> {
    let mut projects = Vec::with_capacity(2);
    projects.push(Project {id: 1, name: String::from("Site"), image_name: Some(String::from("Site.png")), description: String::from("This website")});
    projects.push(Project {id: 2, name: String::from("QuarentineLife"), image_name: None, description: String::from("A forum created for a school project")});

    projects
}

#[get("/projects")]
async fn get_projects() -> impl Responder {
    let projects = get_projects_vec();
    HttpResponse::Ok().json(projects)
}

#[get("/project/{project_id}")]
async fn get_project(web::Path(project_id): web::Path<usize>) -> HttpResponse {
    let projects = get_projects_vec();

    match projects.get(project_id-1) {
        Some(project) => HttpResponse::Ok().json(project.clone()),
        None => HttpResponse::NotFound().json("Project not Found".to_string())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .wrap(Cors::permissive())
            .service(
                web::scope("/get")
                    .service(get_projects)
                    .service(get_project)
            )
        )
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
