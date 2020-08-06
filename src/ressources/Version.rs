use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    #[schemars(example = "example_code")]
    pub code: String,
    #[schemars(example = "example_description")]
    pub description: String,
}

pub fn example_code() -> &'static str {
    "v1"
}

pub fn example_description() -> &'static str {
    "A cool name"
}
