use strum_macros::{Display, EnumString};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum Wear {
    #[strum(serialize = "Factory New")]
    FactoryNew = 1,
    #[strum(serialize = "Minimal Wear")]
    MinimalWear = 2,
    #[strum(serialize = "Field-Tested")]
    FieldTested = 3,
    #[strum(serialize = "Well-Worn")]
    WellWorn = 4,
    #[strum(serialize = "Battle Scarred")]
    BattleScarred = 5
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum KillstreakTier {
    #[strum(serialize = "Professional Killstreak")]
    Professional = 3,
    #[strum(serialize = "Specialized Killstreak")]
    Specialized = 2,
    #[strum(serialize = "Killstreak")]
    Basic = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum Quality {
    Normal = 0,
    Genuine = 1,
    Vintage = 3,
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

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum Rarity {
    Civilian = 1,
    Freelance = 2,
    Mercenary = 3,
    Commando = 4,
    Assassin = 5,
    Elite = 6,
}

#[derive(Debug, PartialEq, Display, EnumString, Clone)]
pub enum Class {
    Scout,
    Soldier,
    Pyro,
    Demoman,
    Heavy,
    Engineer,
    Medic,
    Sniper,
    Spy,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::str::FromStr;

    #[test]
    fn converts_quality_to_primitive() {
        let quality = Quality::Strange;
        let quality_num: u8 = quality.into();
        
        assert_eq!(11, quality_num);
    }
    
    #[test]
    fn converts_string_to_quality() {
        let quality = Quality::from_str("Decorated Weapon").unwrap();
        
        assert_eq!(Quality::DecoratedWeapon, quality);
    }
    
    #[test]
    fn displays_quality() {
        let quality = Quality::Collectors;
        
        assert_eq!("Collector's", &format!("{}", quality));
    }
}