use crate::Attribute;
use strum_macros::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Sheen.
#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Display, EnumString, EnumIter, EnumCount, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Sheen {
    #[strum(serialize = "Team Shine")]
    TeamShine = 1,
    #[strum(serialize = "Deadly Daffodil")]
    DeadlyDaffodil = 2,
    #[strum(serialize = "Manndarin")]
    Manndarin = 3,
    #[strum(serialize = "Mean Green")]
    MeanGreen = 4,
    #[strum(serialize = "Agonizing Emerald")]
    AgonizingEmerald = 5,
    #[strum(serialize = "Villainous Violet")]
    VillainousViolet = 6,
    #[strum(serialize = "Hot Rod")]
    HotRod = 7,
}

impl Attribute for Sheen {
    const DEFINDEX: u32 = 2014;
}
