use serde::{Deserialize, Serialize};
use crate::blocks::BlockPosition;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PreWorld {
    pub blocks: Vec<BlockPosition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WorldEnvironment{
    // The time of day. 0 is sunrise, 6000 is mid day, 12000 is sunset, 18000 is mid night, 24000 is the next day's 0. This value keeps counting past 24000 and does not reset to 0.
    pub day_time: i64,
    // #[serde(with = "serde_enum_as_integer")] TODO
    // pub difficulty: Difficulty,
    // pub game_rules: GameRuleRegistry,
    #[serde(rename = "allowCommands")]
    pub allow_commands: bool,
}