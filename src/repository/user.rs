use crate::models::schema::users::dsl::*;
use crate::models::user::{NewUser, User};
use crate::repository::database::Database;
use diesel::prelude::*;

pub trait Users {
    fn get_users(&self) -> Vec<User>;
    fn get_user(&self, by_username: &str) -> Option<User>;
    fn create_user(&self, user: NewUser) -> Result<usize, diesel::result::Error>;
    fn delete_user(&self, by_id: i32) -> Result<usize, diesel::result::Error>;
    fn update_user(&self, user: User) -> Result<usize, diesel::result::Error>;
}

impl Users for Database {
    fn get_users(&self) -> Vec<User> {
        users
            .load::<User>(&mut self.pool.get().unwrap())
            .expect("Failed to get users")
    }

    fn get_user(&self, by_username: &str) -> Option<User> {
        users
            .filter(username.eq(by_username))
            .first::<User>(&mut self.pool.get().unwrap())
            .ok()
    }

    fn create_user(&self, user: NewUser) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(users)
            .values(&user)
            .execute(&mut self.pool.get().unwrap())
    }

    fn delete_user(&self, by_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(users.filter(id.eq(by_id))).execute(&mut self.pool.get().unwrap())
    }

    fn update_user(&self, user: User) -> Result<usize, diesel::result::Error> {
        diesel::update(users.filter(id.eq(user.id)))
            .set(&user)
            .execute(&mut self.pool.get().unwrap())
    }
}
