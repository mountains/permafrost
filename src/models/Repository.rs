use crate::database::schema::repositories;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Repository {
    pub uuid: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub path: String,
    pub group_uuid: Option<Vec<u8>>,
}

#[derive(Insertable)]
#[table_name = "repositories"]
pub struct NewRepository<'a> {
    pub uuid: &'a Vec<u8>,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: Option<&'a NaiveDateTime>,
    pub path: &'a String,
    pub group_uuid: Option<&'a Vec<u8>>,
}
