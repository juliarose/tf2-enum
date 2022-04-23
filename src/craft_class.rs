use strum_macros::{Display, EnumString, EnumIter};

#[derive(Debug, Hash, Eq, PartialEq, Display, EnumString, EnumIter, Clone, Copy)]
pub enum CraftClass {
    #[strum(serialize = "weapon")]
    Weapon,
    #[strum(serialize = "hat")]
    Hat,
    #[strum(serialize = "craft_bar")]
    CraftBar,
    #[strum(serialize = "haunted_hat")]
    HauntedHat,
    #[strum(serialize = "tool")]
    Tool,
    #[strum(serialize = "craft_token")]
    CraftToken,
    #[strum(serialize = "supply_crate")]
    SupplyCrate,
}