use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct RepositoryGroup {
    pub uuid: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub name: String,
}
