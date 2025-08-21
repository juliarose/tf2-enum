use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString};

/// Craft material type.
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
    Deserialize,
    Serialize,
    EnumString,
    EnumIter,
    EnumCount,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum CraftMaterialType {
    Weapon,
    Hat,
    CraftBar,
    HauntedHat,
    Tool,
    CraftToken,
    SupplyCrate,
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
        
        let craft_material_type = CraftMaterialType::from_str("craft_material_voodoocursed").unwrap();

        assert_eq!(craft_material_type, CraftMaterialType::CraftMaterialVoodooCursed);
        
        let craft_material_type = CraftMaterialType::from_str("strangepart").unwrap();
        
        assert_eq!(craft_material_type, CraftMaterialType::StrangePart);
    }
}
