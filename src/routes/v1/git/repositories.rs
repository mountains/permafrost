use crate::connection::Connection;
use diesel::{self, prelude::*};
use rocket_contrib::json::Json;

use crate::models::Repository::Repository;
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
