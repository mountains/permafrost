use crate::ressources::Branches::Branch;
use git2::{Branch as GitBranch, BranchType, Repository};
use rocket_contrib::json::Json;

#[openapi]
#[get("/v1/git/branches")]
pub fn branches() -> Json<Vec<Branch>> {
    let _repo = match Repository::open("/mnt/Dev/@mountains/permafrost") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let mut _branches = Vec::new();
    let repo_branches = match _repo.branches(None) {
        Ok(repo_branches) => repo_branches,
        Err(e) => panic!("failed to get branches: {}", e),
    };
    for val in repo_branches {
        let repo_branch = match val {
            Ok(branch) => branch,
            Err(e) => panic!("failed to get branch: {}", e),
        };
        _branches.push(get_branch_from_git_branch(repo_branch));
    }
    Json(_branches)
}

fn get_branch_from_git_branch(data: (GitBranch, BranchType)) -> Branch {
    let branch = data.0;
    let repo_branch_name = match branch.name() {
        Ok(repo_branch_name) => repo_branch_name,
        Err(e) => panic!("failed to get branch name: {}", e),
    };

    return match data.1 {
        BranchType::Local => Branch::local(
            repo_branch_name.unwrap().to_string(),
            false,
            get_upstream_branch_name(branch, data.1),
        ),
        BranchType::Remote => Branch::remote(repo_branch_name.unwrap().to_string(), false, None),
    };
}

fn get_upstream_branch_name(branch: GitBranch, branch_type: BranchType) -> Option<String> {
    match branch_type {
        BranchType::Local => {
            let upstream_branch = match branch.upstream() {
                Ok(upstream_branch) => upstream_branch,
                Err(_) => return None,
            };
            let upstream_branch_name = match upstream_branch.name() {
                Ok(upstream_branch_name) => upstream_branch_name,
                Err(e) => panic!("failed to get upstream branch name: {}", e),
            };
            return Some(upstream_branch_name.unwrap().to_string());
        }
        BranchType::Remote => return None,
    }
}
