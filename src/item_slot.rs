use crate::StockWeapon;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumIter, EnumCount};

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
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ItemSlot {
    Melee,
    Primary,
    Secondary,
    #[strum(serialize = "pda")]
    #[serde(rename = "pda")]
    PDA,
    #[strum(serialize = "pda2")]
    #[serde(rename = "pda2")]
    PDA2,
    Building,
    Misc,
    Taunt,
    Action,
    Utility,
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
            _ => &[],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_slot_stock_weapons() {
        let primary_weapons = ItemSlot::Primary.stock_weapons();
        assert!(primary_weapons.contains(&StockWeapon::Scattergun));
        assert!(primary_weapons.contains(&StockWeapon::RocketLauncher));
    }
    
    #[test]
    fn serializes() {
        let json = serde_json::to_string(&ItemSlot::Primary).unwrap();
        assert_eq!(json, "\"primary\"");
        
        
        let json = serde_json::to_string(&ItemSlot::PDA).unwrap();
        assert_eq!(json, "\"pda\"");
        let json = serde_json::to_string(&ItemSlot::PDA2).unwrap();
        assert_eq!(json, "\"pda2\"");
    }
}
