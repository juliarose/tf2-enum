use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, EnumIter, EnumCount};

/// Craft material type.
#[derive(Debug, Deserialize, Serialize, Hash, Eq, PartialEq, Ord, PartialOrd, Display, EnumString, EnumIter, EnumCount, Clone, Copy)]
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