use strum_macros::{Display, EnumString};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum Rarity {
    Civilian = 1,
    Freelance = 2,
    Mercenary = 3,
    Commando = 4,
    Assassin = 5,
    Elite = 6,
}