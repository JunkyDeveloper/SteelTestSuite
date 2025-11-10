use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlockPosition {
    pub cords: Vec<i32>,
    pub block_id: String,
    pub properties: BlockProperties,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlockProperties {
    pub burning: Option<bool>,
    pub waterlogged: Option<bool>,
}