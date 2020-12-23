use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewRepositorySuccessResponse {
    #[schemars(example = "example_success")]
    pub success: bool,
    #[schemars(example = "example_message")]
    pub message: Option<String>,
    #[schemars(example = "example_message")]
    pub repository_uuid: String,
}

pub fn example_success() -> bool {
    true
}

pub fn example_message() -> &'static str {
    "Success !"
}

impl NewRepositorySuccessResponse {
    pub fn created(repository_uuid: String) -> Self {
        NewRepositorySuccessResponse {
            success: true,
            message: Some("Created !".to_string()),
            repository_uuid: repository_uuid,
        }
    }
}
