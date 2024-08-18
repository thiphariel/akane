use crate::models::project::{NewProject, Project};
use crate::models::schema::projects::dsl::*;
use crate::repository::database::Database;
use diesel::prelude::*;

pub trait Projects {
    fn get_projects(&self) -> Vec<Project>;
    fn get_project(&self, by_id: i32) -> Option<Project>;
    fn create_project(&self, project: NewProject) -> Result<usize, diesel::result::Error>;
    fn delete_project(&self, by_id: i32) -> Result<usize, diesel::result::Error>;
    fn update_project(&self, project: Project) -> Result<usize, diesel::result::Error>;
}

impl Projects for Database {
    fn get_projects(&self) -> Vec<Project> {
        projects
            .load::<Project>(&mut self.pool.get().unwrap())
            .expect("Failed to get projects")
    }

    fn get_project(&self, by_id: i32) -> Option<Project> {
        projects
            .find(by_id)
            .first::<Project>(&mut self.pool.get().unwrap())
            .ok()
    }

    fn create_project(&self, project: NewProject) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(projects)
            .values(&project)
            .execute(&mut self.pool.get().unwrap())
    }

    fn delete_project(&self, by_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(projects.filter(id.eq(by_id))).execute(&mut self.pool.get().unwrap())
    }

    fn update_project(&self, project: Project) -> Result<usize, diesel::result::Error> {
        diesel::update(projects.filter(id.eq(project.id)))
            .set(&project)
            .execute(&mut self.pool.get().unwrap())
    }
}
