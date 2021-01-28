use serde::Serialize;
use crate::schema::projects::{self, dsl::*};
use crate::Pool;
use diesel::dsl::{delete, insert_into};
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use crate::handlers::InputProject;
use actix_web::web;


#[derive(Serialize, Clone, Queryable)]
pub struct Project {
    pub id: i32,
    pub image_name: Option<String>,
    pub name: String,
    pub description: String
}

#[derive(Insertable)]
#[table_name="projects"]
pub struct NewProject<'a> {
    pub image_name: Option<&'a str>,
    pub name: &'a str,
    pub description: &'a str
}

impl Project {
    pub fn get_all_projects(pool: web::Data<Pool>) -> Result<Vec<Project>, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let items = projects.load::<Project>(&conn)?;
        Ok(items)
    }

    pub fn remove_project(pool: web::Data<Pool>, project_id: i32) -> Result<usize, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let count = delete(projects.find(project_id)).execute(&conn).unwrap();
        Ok(count)
    }
}

impl<'a> NewProject<'a> {
    pub fn add_single_project(pool: web::Data<Pool>, item: web::Json<InputProject>) -> Result<Project, diesel::result::Error> {
        let conn = pool.get().unwrap();

        let new_project = NewProject {
            image_name: item.image_name.as_deref(),
            name: &item.name,
            description: &item.description
        };
        let res = insert_into(projects).values(&new_project).get_result(&conn)?;
        Ok(res)
    }
}