use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, EnumIter};

#[derive(Debug, Deserialize, Serialize, Hash, Eq, PartialEq, Display, EnumString, EnumIter, Clone, Copy)]
pub enum ItemSlot {
    #[strum(serialize = "melee")]
    Melee,
    #[strum(serialize = "primary")]
    Primary,
    #[strum(serialize = "secondary")]
    Secondary,
    #[strum(serialize = "pda")]
    PDA,
    #[strum(serialize = "pda2")]
    PDA2,
    #[strum(serialize = "building")]
    Building,
    #[strum(serialize = "misc")]
    Misc,
    #[strum(serialize = "taunt")]
    Taunt,
    #[strum(serialize = "action")]
    Action,
    #[strum(serialize = "utility")]
    Utility,
    #[strum(serialize = "quest")]
    Quest,
}