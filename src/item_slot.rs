use crate::StockWeapon;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, EnumIter, EnumCount};

/// Item slot.
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
pub enum ItemSlot {
    #[strum(serialize = "melee")]
    #[serde(rename = "melee")]
    Melee,
    #[strum(serialize = "primary")]
    #[serde(rename = "primary")]
    Primary,
    #[strum(serialize = "secondary")]
    #[serde(rename = "secondary")]
    Secondary,
    #[strum(serialize = "pda")]
    #[serde(rename = "pda")]
    PDA,
    #[strum(serialize = "pda2")]
    #[serde(rename = "pda2")]
    PDA2,
    #[strum(serialize = "building")]
    #[serde(rename = "building")]
    Building,
    #[strum(serialize = "misc")]
    #[serde(rename = "misc")]
    Misc,
    #[strum(serialize = "taunt")]
    #[serde(rename = "taunt")]
    Taunt,
    #[strum(serialize = "action")]
    #[serde(rename = "action")]
    Action,
    #[strum(serialize = "utility")]
    #[serde(rename = "utility")]
    Utility,
    #[strum(serialize = "quest")]
    #[serde(rename = "quest")]
    Quest,
}

impl ItemSlot {
    /// Gets the stock weapons available for this item slot.
    pub fn stock_weapons(&self) -> &'static [StockWeapon] {
        match self {
            ItemSlot::Primary => &[
                StockWeapon::Scattergun,
                StockWeapon::FlameThrower,
                StockWeapon::SyringeGun,
                StockWeapon::RocketLauncher,
                StockWeapon::GrenadeLauncher,
                StockWeapon::SniperRifle,
                StockWeapon::Minigun,
                StockWeapon::Revolver,
                // The Shotgun is both a primary and secondary weapon.
                StockWeapon::Shotgun,
            ],
            ItemSlot::Secondary => &[
                StockWeapon::SMG,
                StockWeapon::StickybombLauncher,
                StockWeapon::Pistol,
                StockWeapon::MediGun,
                StockWeapon::Shotgun,
            ],
            ItemSlot::Melee => &[
                StockWeapon::Bat,
                StockWeapon::Bottle,
                StockWeapon::FireAxe,
                StockWeapon::Kukri,
                StockWeapon::Knife,
                StockWeapon::Fists,
                StockWeapon::Shovel,
                StockWeapon::Wrench,
                StockWeapon::Bonesaw,
                StockWeapon::Bottle,
            ],
            ItemSlot::PDA => &[
                StockWeapon::DisguiseKit,
                StockWeapon::ConstructionPDA,
            ],
            ItemSlot::PDA2 => &[
                StockWeapon::InvisWatch,
                StockWeapon::DestructionPDA,
            ],
            ItemSlot::Building => &[
                StockWeapon::PDA,
            ],
            // No stock weapons for these slots
            ItemSlot::Misc | ItemSlot::Taunt | ItemSlot::Action | ItemSlot::Utility | ItemSlot::Quest => &[],
        }
    }
}