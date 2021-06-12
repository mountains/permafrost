use crate::connection::Connection;
use diesel::dsl::insert_into;
use diesel::{self, prelude::*};
use rocket::serde::json::Json;

use crate::models::Repository::NewRepository;
use crate::models::Repository::Repository;
use crate::requests::NewRepository::NewRepositoryRequest;
use crate::responses::NewRepositorySuccessResponse::NewRepositorySuccessResponse;
use crate::ressources::Repository::RepositoryRessource;

use uuid::Uuid;

#[openapi]
#[get("/v1/git/repositories")]
pub fn list_repositories(conn: Connection) -> Json<Vec<RepositoryRessource>> {
    use crate::database::schema::repositories::dsl::*;

    let repositories_found = repositories.load::<Repository>(&*conn).unwrap();

    let mut _repositories = Vec::new();
    for repository in repositories_found {
        let _uuid = match Uuid::from_slice(repository.uuid.as_slice()) {
            Ok(_uuid) => _uuid,
            Err(_err) => Uuid::parse_str("000000000000000000000000000000000000").unwrap(),
        };
        _repositories.push(RepositoryRessource {
            uuid: _uuid.to_string(),
            path: repository.path,
        })
    }
    Json(_repositories)
}

#[openapi]
#[post(
    "/v1/git/repositories",
    format = "application/json",
    data = "<request>"
)]
pub fn add_repository(
    request: Json<NewRepositoryRequest>,
    conn: Connection,
) -> Json<NewRepositorySuccessResponse> {
    use crate::database::schema::repositories::dsl::*;

    let new_uuid = Uuid::new_v4();
    let new_repository = NewRepository {
        uuid: &new_uuid.as_bytes().to_vec(),
        created_at: &chrono::Local::now().naive_utc(),
        updated_at: None,
        path: &request.path,
        group_uuid: None,
    };

    insert_into(repositories)
        .values(&new_repository)
        .execute(&*conn)
        .expect("Error");

    Json(NewRepositorySuccessResponse::created(new_uuid.to_string()))
}
