use strum_macros::{Display, EnumString};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::Attribute;

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum KillstreakTier {
    #[strum(serialize = "Killstreak")]
    Basic = 1,
    #[strum(serialize = "Specialized Killstreak")]
    Specialized = 2,
    #[strum(serialize = "Professional Killstreak")]
    Professional = 3,
}

impl Attribute for KillstreakTier {
    const DEFINDEX: u32 = 2025;
}