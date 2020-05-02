use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Branch {
    pub name: String,
    pub is_remote: bool,
    pub is_head: bool,
    pub upstream_name: Option<String>,
}

impl Branch {
    pub fn local(name: String, is_head: bool, upstream_name: Option<String>) -> Self {
        Branch {
            name: name,
            is_remote: false,
            is_head: is_head,
            upstream_name: upstream_name,
        }
    }

    pub fn remote(name: String, is_head: bool, upstream_name: Option<String>) -> Self {
        Branch {
            name: name,
            is_remote: true,
            is_head: is_head,
            upstream_name: upstream_name,
        }
    }
}
