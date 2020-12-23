use super::schema::repositories;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct RepositoryGroup {
    pub uuid: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "repository_groups"]
pub struct NewRepositoryGroup<'a> {
    pub name: &'a str,
}
