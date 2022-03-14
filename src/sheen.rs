use strum_macros::{Display, EnumString};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
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