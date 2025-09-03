use crate::{Rarity, Colored};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::{Display, EnumCount, EnumIter, EnumString};

/// Grade. The repr values for each variant aren't associated with anything from the schema. Use
/// [`Rarity`] if you need internal values. Instead, [`Grade`] is used for display purposes and each
/// variant is ranked from lowest to highest tier.
/// 
/// The types can be neatly-converted from one another.
/// 
/// # Examples
/// ```
/// use tf2_enum::{Grade, Rarity};
/// 
/// assert_eq!(Grade::from(Rarity::Ancient), Grade::Elite);
/// 
/// let rarity: Rarity = Grade::Elite.into();
/// 
/// assert_eq!(rarity, Rarity::Ancient);
/// ```
#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Display,
    Serialize_repr,
    Deserialize_repr,
    EnumString,
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
)]
#[repr(u32)]
#[allow(missing_docs)]
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

impl From<Rarity> for Grade {
    fn from(rarity: Rarity) -> Self {
        match rarity {
            Rarity::Common => Self::Civilian,
            Rarity::Uncommon => Self::Freelance,
            Rarity::Rare => Self::Mercenary,
            Rarity::Mythical => Self::Commando,
            Rarity::Legendary => Self::Assassin,
            Rarity::Ancient => Self::Elite,
        }
    }
}
