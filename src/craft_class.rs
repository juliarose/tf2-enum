use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, EnumIter, EnumCount};

/// Craft class.
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
#[serde(rename_all = "snake_case")]
pub enum CraftClass {
    Weapon,
    Hat,
    CraftBar,
    HauntedHat,
    Tool,
    CraftToken,
    SupplyCrate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_craft_class_serialization() {
        let weapon = CraftClass::Weapon;
        let hat = CraftClass::Hat;
        let craft_bar = CraftClass::CraftBar;
        let haunted_hat = CraftClass::HauntedHat;
        let tool = CraftClass::Tool;
        let craft_token = CraftClass::CraftToken;
        let supply_crate = CraftClass::SupplyCrate;

        assert_eq!(serde_json::to_string(&weapon).unwrap(), "\"weapon\"");
        assert_eq!(serde_json::to_string(&hat).unwrap(), "\"hat\"");
        assert_eq!(serde_json::to_string(&craft_bar).unwrap(), "\"craft_bar\"");
        assert_eq!(serde_json::to_string(&haunted_hat).unwrap(), "\"haunted_hat\"");
        assert_eq!(serde_json::to_string(&tool).unwrap(), "\"tool\"");
        assert_eq!(serde_json::to_string(&craft_token).unwrap(), "\"craft_token\"");
        assert_eq!(serde_json::to_string(&supply_crate).unwrap(), "\"supply_crate\"");
    }
}
