use crate::repository::database::Database;
use crate::repository::projects::Projects;
use crate::views::page::page;
use actix_web::{get, web};
use maud::{html, Markup};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/projects").service(show).service(show_one));
}

#[get("")]
async fn show(db: web::Data<Database>) -> Markup {
    let projects = db.get_projects();

    page(
        "Projects list",
        html! {
            @for project in &projects {
                ul {
                    li {
                        a href={"/projects/" (project.id) } {
                            (project.name)
                        }
                    }
                }
            }
        },
    )
}

#[get("/{id}")]
async fn show_one(db: web::Data<Database>, path: web::Path<i32>) -> Markup {
    let project = db.get_project(path.into_inner()).unwrap();

    page(
        "Project details",
        html! {
            h1 { (project.name) }
            p { (project.description) }
        },
    )
}
