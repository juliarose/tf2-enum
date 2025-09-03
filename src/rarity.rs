use crate::Grade;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::{Display, EnumCount, EnumIter};

/// Kill eater score type. Conversion from strings is not supported due to multiple variants
/// having the same string representation. They can still be formatted into strings.
#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Display,
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
    Clone,
    Copy,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[repr(u32)]
#[allow(missing_docs)]
pub enum Rarity {
    Common = 17,
    Uncommon = 18,
    Rare = 19,
    Mythical = 20,
    Legendary = 21,
    Ancient = 22,
}

impl From<Grade> for Rarity {
    fn from(grade: Grade) -> Self {
        match grade {
            Grade::Civilian => Self::Common,
            Grade::Freelance => Self::Uncommon,
            Grade::Mercenary => Self::Rare,
            Grade::Commando => Self::Mythical,
            Grade::Assassin => Self::Legendary,
            Grade::Elite => Self::Ancient,
        }
    }
}

