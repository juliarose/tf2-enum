use crate::StockWeapon;
use serde::{Deserialize, Serialize};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::{Display, EnumCount, EnumIter, EnumString};

/// Class.
#[derive(
    Debug,
    Deserialize,
    Serialize,
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
#[allow(missing_docs)]
#[repr(u32)]
pub enum Class {
    #[serde(alias = "scout")]
    Scout = 1,
    #[serde(alias = "sniper")]
    Sniper = 2,
    #[serde(alias = "soldier")]
    Soldier = 3,
    #[serde(alias = "demoman")]
    Demoman = 4,
    #[serde(alias = "medic")]
    Medic = 5,
    #[serde(alias = "heavy")]
    Heavy = 6,
    #[serde(alias = "pyro")]
    Pyro = 7,
    #[serde(alias = "spy")]
    Spy = 8,
    #[serde(alias = "engineer")]
    Engineer = 9,
}

impl Class {
    /// Gets the set of stock weapons available to the class.
    pub fn stock_weapons(&self) -> &'static [StockWeapon] {
        StockWeapon::class_stock_weapons(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn serializes() {
        let json = r#""Scout""#;
        let serialized = serde_json::to_string(&Class::Scout).unwrap();
        assert_eq!(serialized, json);
    }

    #[test]
    fn deserializes() {
        let json = r#""Scout""#;
        let deserialized: Class = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, Class::Scout);

        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, json);
    }
}
