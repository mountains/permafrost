use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryRessource {
    #[schemars(example = "example_path")]
    pub path: String,
    #[schemars(example = "example_uuid")]
    pub uuid: String,
}

pub fn example_path() -> &'static str {
    "/mnt/Dev/@mountains/permafrost"
}

pub fn example_uuid() -> &'static str {
    "94f21484-ef1a-42d8-8f28-eb1c4ce2a3ac"
}
