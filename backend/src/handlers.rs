use super::models::project::{NewProject, Project};
use super::Pool;
use crate::services::email_service::email_sender;
use crate::{models::user::UserLoginData, services::account_service};
use actix_web::{delete, get, post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputProject {
    pub image_name: Option<String>,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct InputEmail {
    pub name: String,
    pub from: String,
    pub subject: String,
    pub text: String,
}

#[get("/get/projects")]
pub async fn get_projects(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || Project::get_all_projects(pool))
        .await
        .map(|project| HttpResponse::Ok().json(project))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[post("/add/project")]
pub async fn add_project(
    pool: web::Data<Pool>,
    item: web::Json<InputProject>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || NewProject::add_single_project(pool, item))
            .await
            .map(|project| HttpResponse::Ok().json(project))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[delete("/delete/project/{project_id}")]
pub async fn remove_project(
    pool: web::Data<Pool>,
    web::Path(project_id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Project::remove_project(pool, project_id))
            .await
            .map(|count| HttpResponse::Ok().json(count))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[post("/send_email")]
pub async fn send_email(email: web::Json<InputEmail>) -> HttpResponse {
    match email_sender(email) {
        Ok(_) => HttpResponse::Ok().json("Email sent"),
        _ => HttpResponse::InternalServerError().json("There was a problem sending the email"),
    }
}

#[post("/login")]
pub async fn login(login_data: web::Json<UserLoginData>, pool: web::Data<Pool>) -> HttpResponse {
    match account_service::login(pool, login_data) {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(se) => se.response(),
    }
}
