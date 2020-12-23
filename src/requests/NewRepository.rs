use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewRepositoryRequest {
    #[schemars(example = "example_path")]
    pub path: String,
}

pub fn example_path() -> &'static str {
    "/mnt/Dev/@mountains/permafrost"
}

impl NewRepositoryRequest {
    pub fn create(path: String) -> Self {
        NewRepositoryRequest { path: path }
    }
}
