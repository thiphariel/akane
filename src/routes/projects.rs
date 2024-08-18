use crate::models::project::{NewProject, Project};
use crate::repository::database::Database;
use crate::repository::projects::Projects;
use actix_web::{delete, get, post, put, web, HttpResponse};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(get_projects)
            .service(get_project)
            .service(create_project)
            .service(delete_project)
            .service(update_project),
    );
}

#[get("/projects")]
async fn get_projects(db: web::Data<Database>) -> HttpResponse {
    let projects = db.get_projects();
    HttpResponse::Ok().json(projects)
}

#[get("/projects/{id}")]
async fn get_project(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let project = db.get_project(path.into_inner());
    match project {
        Some(project) => HttpResponse::Ok().json(project),
        None => HttpResponse::NotFound().body("Project not found"),
    }
}

#[post("/projects")]
async fn create_project(db: web::Data<Database>, project: web::Json<NewProject>) -> HttpResponse {
    let project = db.create_project(project.into_inner());
    match project {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError()
            .body("An error has occurred while creating the project"),
    }
}

#[delete("/projects/{id}")]
async fn delete_project(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let project = db.delete_project(path.into_inner());
    match project {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError()
            .body("An error has occurred while deleting the project"),
    }
}

#[put("/projects")]
async fn update_project(db: web::Data<Database>, project: web::Json<Project>) -> HttpResponse {
    let project = db.update_project(project.into_inner());
    match project {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError()
            .body("An error has occurred while updating the project"),
    }
}
