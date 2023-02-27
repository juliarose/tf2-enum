use strum_macros::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, EnumIter, EnumCount, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Quality {
    Normal = 0,
    Genuine = 1,
    #[strum(serialize = "rarity2")]
    Rarity2 = 2,
    Vintage = 3,
    #[strum(serialize = "rarity3")]
    Rarity3 = 4,
    Unusual = 5,
    Unique = 6,
    Community = 7,
    Valve = 8,
    #[strum(serialize = "Self-Made")]
    SelfMade = 9,
    Customized = 10,
    Strange = 11,
    Completed = 12,
    Haunted = 13,
    #[strum(serialize = "Collector's")]
    Collectors = 14,
    #[strum(serialize = "Decorated Weapon")]
    DecoratedWeapon = 15,
}

impl Quality {
    pub fn color(&self) -> u32 {
        match self {
            Quality::Normal => 0xB2B2B2,
            Quality::Genuine => 0x4D7455,
            Quality::Rarity2 => 0xFFFFFF,
            Quality::Vintage => 0x476291,
            Quality::Rarity3 => 0xFFFFFF,
            Quality::Unusual => 0x8650AC,
            Quality::Unique => 0xFFD700,
            Quality::Community => 0x70B04A,
            Quality::Valve => 0x56083F,
            Quality::SelfMade => 0x70B04A,
            Quality::Customized => 0xFFFFFF,
            Quality::Strange => 0xCF6A32,
            Quality::Completed => 0xFFFFFF,
            Quality::Haunted => 0x38F3AB,
            Quality::Collectors => 0xAA0000,
            Quality::DecoratedWeapon => 0xFAFAFA,
        }
    }
    
    pub fn from_color(color: u32) -> Option<Self> {
        match color {
            0xB2B2B2 => Some(Quality::Normal),
            0x4D7455 => Some(Quality::Genuine),
            0x476291 => Some(Quality::Vintage),
            0x8650AC => Some(Quality::Unusual),
            0xFFD700 => Some(Quality::Unique),
            0x56083F => Some(Quality::Valve),
            0x70B04A => Some(Quality::SelfMade),
            0xCF6A32 => Some(Quality::Strange),
            0x38F3AB => Some(Quality::Haunted),
            0xAA0000 => Some(Quality::Collectors),
            0xFAFAFA => Some(Quality::DecoratedWeapon),
            _ => None,
        }
    }
    
    pub fn from_color_str(color: &str) -> Option<Self> {
        let len = color.len();
        let mut color = color;
        
        if len == 7 && color.starts_with('#') {
            color = &color[1..len];
        } else if len != 6 {
            return None;
        }
        
        let color = u32::from_str_radix(color, 16).ok()?;
        
        Self::from_color(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    
    #[test]
    fn from_color() {
        assert_eq!(Some(Quality::Strange), Quality::from_color(0xCF6A32));
    }
    
    #[test]
    fn to_color() {
        assert_eq!(Quality::Strange.color(), 0xCF6A32);
    }

    #[test]
    fn converts_to_primitive() {
        assert_eq!(11 as u32, Quality::Strange as u32);
    }
    
    #[test]
    fn converts_string_to_quality() {
        assert_eq!(Quality::DecoratedWeapon, Quality::from_str("Decorated Weapon").unwrap());
    }
    
    #[test]
    fn displays_as_string() {
        assert_eq!("Collector's", &format!("{}", Quality::Collectors));
    }
}