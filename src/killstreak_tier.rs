use crate::Attribute;
use strum_macros::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Killstreak tier.
#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Display, EnumString, EnumIter, EnumCount, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum KillstreakTier {
    #[strum(serialize = "Killstreak")]
    Killstreak = 1,
    #[strum(serialize = "Specialized Killstreak")]
    Specialized = 2,
    #[strum(serialize = "Professional Killstreak")]
    Professional = 3,
}

impl Attribute for KillstreakTier {
    const DEFINDEX: u32 = 2025;
}