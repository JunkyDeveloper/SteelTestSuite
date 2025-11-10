use serde::{Deserialize, Serialize};
use crate::blocks::BlockProperties;
use crate::inventory::InventorySlot;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AssertState {
    pub block_id: String,
    pub cords: Vec<i32>,
    pub properties: BlockProperties,
    pub inventory: Option<Vec<InventorySlot>>,
}