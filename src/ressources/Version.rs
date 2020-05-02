use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all="camelCase")]
pub struct Version {
    pub code: String,
    pub description: String,
}
