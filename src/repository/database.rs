use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Database {
    pub pool: DbPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let result = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool");

        Database { pool: result }
    }
}
