use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, EnumIter, EnumCount};

/// Craft material type.
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
#[strum(serialize_all = "snake_case")]
pub enum CraftMaterialType {
    #[strum(serialize = "weapon")]
    #[serde(rename = "weapon")]
    Weapon,
    #[strum(serialize = "hat")]
    #[serde(rename = "hat")]
    Hat,
    #[strum(serialize = "craft_bar")]
    #[serde(rename = "craft_bar")]
    CraftBar,
    #[strum(serialize = "haunted_hat")]
    #[serde(rename = "haunted_hat")]
    HauntedHat,
    #[strum(serialize = "tool")]
    #[serde(rename = "tool")]
    Tool,
    #[strum(serialize = "craft_token")]
    #[serde(rename = "craft_token")]
    CraftToken,
    #[strum(serialize = "supply_crate")]
    #[serde(rename = "supply_crate")]
    SupplyCrate,
    #[strum(serialize = "craft_material_burned")]
    #[serde(rename = "craft_material_burned")]
    CraftMaterialBurned,
    #[strum(serialize = "craft_material_voodoocursed")]
    #[serde(rename = "craft_material_voodoocursed")]
    CraftMaterialVoodooCursed,
    #[strum(serialize = "strangepart")]
    #[serde(rename = "strangepart")]
    StrangePart,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    
    #[test]
    fn serializes_craft_material_type() {
        let craft_material_type = CraftMaterialType::Weapon;
        let serialized = serde_json::to_string(&craft_material_type).unwrap();
        
        assert_eq!(serialized, r#""weapon""#);
        
        let craft_material_type = CraftMaterialType::from_str("weapon").unwrap();
        
        assert_eq!(craft_material_type, CraftMaterialType::Weapon);
    }
}
