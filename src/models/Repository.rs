use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Repository {
    pub uuid: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub path: String,
    pub group_uuid: Option<Vec<u8>>,
}
