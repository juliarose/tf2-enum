use crate::Colored;
use strum_macros::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Grade.
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
    EnumString,
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
    Clone,
    Copy,
)]
#[repr(u32)]
pub enum Grade {
    Civilian = 1,
    Freelance = 2,
    Mercenary = 3,
    Commando = 4,
    Assassin = 5,
    Elite = 6,
}

impl Colored for Grade {
    /// Gets the related color of this grade as a hexadecimal color.
    fn color(&self) -> u32 {
        match self {
            Self::Civilian => 0xB0C3D9,
            Self::Freelance => 0x5E98D9,
            Self::Mercenary => 0x4B69FF,
            Self::Commando => 0x8847FF,
            Self::Assassin => 0xD32CE6,
            Self::Elite => 0xEB4B4B,
        }
    }
    
    /// Converts a hexadecimal color into a [`Grade`].
    /// 
    /// # Examples
    /// ```
    /// use tf2_enum::{Grade, Colored};
    /// 
    /// assert_eq!(Grade::from_color(0xEB4B4B).unwrap(), Grade::Elite);
    /// ```
    fn from_color(color: u32) -> Option<Self> {
        match color {
            0xB0C3D9 => Some(Self::Civilian),
            0x5E98D9 => Some(Self::Freelance),
            0x4B69FF => Some(Self::Mercenary),
            0x8847FF => Some(Self::Commando),
            0xD32CE6 => Some(Self::Assassin),
            0xEB4B4B => Some(Self::Elite),
            _ => None,
        }
    }
}
