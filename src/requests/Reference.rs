use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    #[schemars(example = "example_name")]
    pub name: String,
}

pub fn example_name() -> &'static str {
    "refs/heads/main"
}

impl Reference {
    pub fn fq(name: String) -> Self {
        Reference { name: name }
    }
}
