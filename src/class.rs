use crate::StockWeapon;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumIter, EnumCount};

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
    Clone,
    Copy,
)]
#[allow(missing_docs)]
pub enum Class {
    #[serde(alias = "scout")]
    Scout,
    #[serde(alias = "soldier")]
    Soldier,
    #[serde(alias = "pyro")]
    Pyro,
    #[serde(alias = "demoman")]
    Demoman,
    #[serde(alias = "heavy")]
    Heavy,
    #[serde(alias = "engineer")]
    Engineer,
    #[serde(alias = "medic")]
    Medic,
    #[serde(alias = "sniper")]
    Sniper,
    #[serde(alias = "spy")]
    Spy,
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
