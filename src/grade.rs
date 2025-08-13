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

impl Grade {
    /// Gets the related color of this grade as a hexadecimal color.
    pub fn color(&self) -> u32 {
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
    /// use tf2_enum::Grade;
    /// 
    /// assert_eq!(Grade::from_color(0xEB4B4B).unwrap(), Grade::Elite);
    /// ```
    pub fn from_color(color: u32) -> Option<Self> {
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
    
    /// Converts a hexadecimal color string into a [`Grade`].
    /// 
    /// # Examples
    /// ```
    /// use tf2_enum::Grade;
    /// 
    /// assert_eq!(Grade::from_color_str("#EB4B4B").unwrap(), Grade::Elite);
    /// ```
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
