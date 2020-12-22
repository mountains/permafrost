use crate::requests::Reference::Reference;
use crate::ressources::Commits::Commit;
use git2::{Oid, Repository};
use rocket_contrib::json::Json;

#[openapi]
#[post(
    "/v1/git/<uuid>/commits",
    format = "application/json",
    data = "<reference>"
)]
pub fn commits(reference: Json<Reference>, uuid: String) -> Json<Vec<Commit>> {
    let _repo = match Repository::open("/mnt/Dev/@mountains/permafrost") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let mut _commits = match _repo.revwalk() {
        Ok(commits) => commits,
        Err(e) => panic!("failed to get commits: {}", e),
    };
    match _commits.push_ref(&reference.name) {
        Ok(_) => {}
        Err(e) => panic!("Invalid reference: {}", e),
    };
    let mut _commits_list = Vec::new();
    for id in _commits {
        let commit = match id {
            Ok(commit) => commit,
            Err(e) => panic!("failed to get commit: {}", e),
        };
        _commits_list.push(Commit {
            sha: hex::encode(commit.as_bytes()),
            body: get_commit_message(&_repo, &commit),
        });
    }

    Json(_commits_list)
}

fn get_commit_message(_repo: &Repository, &oid: &Oid) -> Option<String> {
    let commit = match _repo.find_commit(oid) {
        Ok(commit) => commit,
        Err(_) => panic!("Can no get commit: {}", oid),
    };
    return match commit.message() {
        Some(value) => Some(value.to_string()),
        None => None,
    };
}
