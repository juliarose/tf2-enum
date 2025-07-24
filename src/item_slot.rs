use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, EnumIter, EnumCount};

/// Item slot.
#[derive(Debug, Deserialize, Serialize, Hash, Eq, PartialEq, Ord, PartialOrd, Display, EnumString, EnumIter, EnumCount, Clone, Copy)]
pub enum ItemSlot {
    #[strum(serialize = "melee")]
    #[serde(rename = "melee")]
    Melee,
    #[strum(serialize = "primary")]
    #[serde(rename = "primary")]
    Primary,
    #[strum(serialize = "secondary")]
    #[serde(rename = "secondary")]
    Secondary,
    #[strum(serialize = "pda")]
    #[serde(rename = "pda")]
    PDA,
    #[strum(serialize = "pda2")]
    #[serde(rename = "pda2")]
    PDA2,
    #[strum(serialize = "building")]
    #[serde(rename = "building")]
    Building,
    #[strum(serialize = "misc")]
    #[serde(rename = "misc")]
    Misc,
    #[strum(serialize = "taunt")]
    #[serde(rename = "taunt")]
    Taunt,
    #[strum(serialize = "action")]
    #[serde(rename = "action")]
    Action,
    #[strum(serialize = "utility")]
    #[serde(rename = "utility")]
    Utility,
    #[strum(serialize = "quest")]
    #[serde(rename = "quest")]
    Quest,
}
