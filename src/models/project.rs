use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Insertable, Queryable, Deserialize, Serialize, Clone, Debug)]
#[diesel(table_name=crate::models::schema::projects)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
}
#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[diesel(table_name=crate::models::schema::projects)]
pub struct NewProject {
    pub name: String,
    pub description: String,
}
