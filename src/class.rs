use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, EnumIter, EnumCount};

/// Class.
#[derive(Debug, Deserialize, Serialize, Hash, Eq, PartialEq, Ord, PartialOrd, Display, EnumString, EnumIter, EnumCount, Clone, Copy)]
pub enum Class {
    #[serde(alias = "scout")]
    Scout,
    #[serde(alias = "soldier")]
    Soldier,
    #[serde(alias = "pyro")]
    Pyro,
    #[serde(alias = "demoman")]
    Demoman,
    #[serde(alias = "heavy")]
    Heavy,
    #[serde(alias = "engineer")]
    Engineer,
    #[serde(alias = "medic")]
    Medic,
    #[serde(alias = "sniper")]
    Sniper,
    #[serde(alias = "spy")]
    Spy,
}