use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Insertable, Queryable, Deserialize, Serialize, Clone, Debug)]
#[diesel(table_name=crate::models::schema::users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}
#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[diesel(table_name=crate::models::schema::users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
}
