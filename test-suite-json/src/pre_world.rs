use serde::{Deserialize, Serialize};
use crate::blocks::BlockPosition;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PreWorld {
    pub blocks: Vec<BlockPosition>,
}