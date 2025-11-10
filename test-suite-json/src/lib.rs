use serde::{Deserialize, Serialize};
use crate::action::Action;
use crate::assert::AssertState;
use crate::pre_world::PreWorld;

pub mod pre_world;
pub mod blocks;
pub mod assert;
pub mod inventory;
pub mod action;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SimulationRun {
    pub description: String,
    pub required: bool,
    pub pre_world: Option<PreWorld>,
    pub timeline: Vec<Tick>,
    pub assert_state: Option<AssertState>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Tick {
    pub tick: i64,
    pub action: Option<Action>,
    pub assert_state: Option<AssertState>,
}