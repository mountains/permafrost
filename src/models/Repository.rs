use super::schema::repositories;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Repository {
    pub uuid: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub path: String,
    pub group_uuid: Option<String>,
}

#[derive(Insertable)]
#[table_name = "repositories"]
pub struct NewRepository<'a> {
    pub path: &'a str,
    pub group_uuid: &'a str,
}
