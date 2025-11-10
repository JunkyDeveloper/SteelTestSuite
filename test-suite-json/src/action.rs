use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub method: String,
    pub slot: i32,
    pub item: String,
}