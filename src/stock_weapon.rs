use crate::{Class, HasItemDefindex, ItemSlot};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};

/// Stock weapons.
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
pub enum StockWeapon {
    #[strum(serialize = "Bat")]
    Bat,
    #[strum(serialize = "Bottle")]
    Bottle,
    #[strum(serialize = "Fire Axe")]
    FireAxe,
    #[strum(serialize = "Kukri")]
    Kukri,
    #[strum(serialize = "Knife")]
    Knife,
    #[strum(serialize = "Fists")]
    Fists,
    #[strum(serialize = "Shovel")]
    Shovel,
    #[strum(serialize = "Wrench")]
    Wrench,
    #[strum(serialize = "Bonesaw")]
    Bonesaw,
    #[strum(serialize = "Shotgun")]
    Shotgun,
    #[strum(serialize = "Scattergun")]
    Scattergun,
    #[strum(serialize = "Sniper Rifle")]
    SniperRifle,
    #[strum(serialize = "Minigun")]
    Minigun,
    #[strum(serialize = "SMG")]
    SMG,
    #[strum(serialize = "Syringe Gun")]
    SyringeGun,
    #[strum(serialize = "Rocket Launcher")]
    RocketLauncher,
    #[strum(serialize = "Grenade Launcher")]
    GrenadeLauncher,
    #[strum(serialize = "Stickybomb Launcher")]
    StickybombLauncher,
    #[strum(serialize = "Flame Thrower")]
    FlameThrower,
    #[strum(serialize = "Pistol")]
    Pistol,
    #[strum(serialize = "Revolver")]
    Revolver,
    #[strum(serialize = "Construction PDA")]
    ConstructionPDA,
    #[strum(serialize = "Destruction PDA")]
    DestructionPDA,
    #[strum(serialize = "Disguise Kit")]
    DisguiseKit,
    #[strum(serialize = "PDA")]
    PDA,
    #[strum(serialize = "Medi Gun")]
    MediGun,
    #[strum(serialize = "Invis Watch")]
    InvisWatch,
}

impl HasItemDefindex for StockWeapon {
    /// Gets the defindex for this item. If there are multiple, the first is returned.
    /// 
    /// If you want the defindex associated with inventory items, use
    /// [`StockWeapon::econ_defindex`].
    fn defindex(&self) -> u32 {
        self.defindexes()
            .first()
            .copied()
            .unwrap() // safety - we know there is at least one defindex
    }
    
    /// Attempts to create a [`StockWeapon`] from a defindex. Excludes definitions for skinned
    /// items.
    fn from_defindex(defindex: u32) -> Option<Self> {
        match defindex {
            0 | 190 => Some(Self::Bat),
            1 | 191 => Some(Self::Bottle),
            2 | 192 => Some(Self::FireAxe),
            3 | 193 => Some(Self::Kukri),
            4 | 194 => Some(Self::Knife),
            5 | 195 => Some(Self::Fists),
            6 | 196 => Some(Self::Shovel),
            7 | 197 => Some(Self::Wrench),
            8 | 198 => Some(Self::Bonesaw),
            9 | 10 | 11 | 12 | 199 => Some(Self::Shotgun),
            13 | 200 => Some(Self::Scattergun),
            14 | 201 => Some(Self::SniperRifle),
            15 | 202 => Some(Self::Minigun),
            16 | 203 => Some(Self::SMG),
            17 | 204 => Some(Self::SyringeGun),
            18 | 205 => Some(Self::RocketLauncher),
            19 | 206 => Some(Self::GrenadeLauncher),
            20 | 207 => Some(Self::StickybombLauncher),
            21 | 208 => Some(Self::FlameThrower),
            22 | 23 | 209 => Some(Self::Pistol),
            24 | 210 => Some(Self::Revolver),
            25 | 737 => Some(Self::ConstructionPDA),
            26 => Some(Self::DestructionPDA),
            27 => Some(Self::DisguiseKit),
            28 => Some(Self::PDA),
            29 | 211 => Some(Self::MediGun),
            30 | 212 => Some(Self::InvisWatch),
            _ => None,
        }
    }
}

impl StockWeapon {
    /// Gets the set of related defindexes of the weapon. Excludes definitions for skinned items.
    pub fn defindexes(&self) -> &'static [u32] {
        match self {
            Self::Bat => &[0, 190],
            Self::Bottle => &[1, 191],
            Self::FireAxe => &[2, 192],
            Self::Kukri => &[3, 193],
            Self::Knife => &[4, 194],
            Self::Fists => &[5, 195],
            Self::Shovel => &[6, 196],
            Self::Wrench => &[7, 197],
            Self::Bonesaw => &[8, 198],
            Self::Shotgun => &[9, 10, 11, 12, 199],
            Self::Scattergun => &[13, 200],
            Self::SniperRifle => &[14, 201],
            Self::Minigun => &[15, 202],
            Self::SMG => &[16, 203],
            Self::SyringeGun => &[17, 204],
            Self::RocketLauncher => &[18, 205],
            Self::GrenadeLauncher => &[19, 206],
            Self::StickybombLauncher => &[20, 207],
            Self::FlameThrower => &[21, 208],
            Self::Pistol => &[22, 23, 209],
            Self::Revolver => &[24, 210],
            Self::ConstructionPDA => &[25, 737],
            Self::DestructionPDA => &[26],
            Self::DisguiseKit => &[27],
            Self::PDA => &[28],
            Self::MediGun => &[29, 211],
            Self::InvisWatch => &[30, 212],
        }
    }
    
    /// Gets the economy defindex for this weapon (if available).
    /// 
    /// This is the defindex that can be mapped to inventory items. Not available for
    /// [`StockWeapon::DestructionPDA`], [`StockWeapon::PDA`], and [`StockWeapon::DisguiseKit`].
    pub fn econ_defindex(&self) -> Option<u32> {
        match self {
            Self::Bat => Some(190),
            Self::Bottle => Some(191),
            Self::FireAxe => Some(192),
            Self::Kukri => Some(193),
            Self::Knife => Some(194),
            Self::Fists => Some(195),
            Self::Shovel => Some(196),
            Self::Wrench => Some(197),
            Self::Bonesaw => Some(198),
            Self::Shotgun => Some(199),
            Self::Scattergun => Some(200),
            Self::SniperRifle => Some(201),
            Self::Minigun => Some(202),
            Self::SMG => Some(203),
            Self::SyringeGun => Some(204),
            Self::RocketLauncher => Some(205),
            Self::GrenadeLauncher => Some(206),
            Self::StickybombLauncher => Some(207),
            Self::FlameThrower => Some(208),
            Self::Pistol => Some(209),
            Self::Revolver => Some(210),
            _ => None,
        }
    }

    /// Gets the set of classes that can use this weapon.
    pub fn used_by_classes(&self) -> &'static [Class] {
        match self {
            Self::Bat |
            Self::Scattergun => &[Class::Scout],
            Self::Bottle |
            Self::GrenadeLauncher |
            Self::StickybombLauncher => &[Class::Demoman],
            Self::FireAxe |
            Self::FlameThrower => &[Class::Pyro],
            Self::Kukri |
            Self::SniperRifle |
            Self::SMG => &[Class::Sniper],
            Self::Knife |
            Self::Revolver |
            Self::DisguiseKit |
            Self::InvisWatch => &[Class::Spy],
            Self::Fists |
            Self::Minigun => &[Class::Heavy],
            Self::Shovel |
            Self::RocketLauncher => &[Class::Soldier],
            Self::Wrench  |
            Self::ConstructionPDA |
            Self::DestructionPDA |
            Self::PDA => &[Class::Engineer],
            Self::Bonesaw |
            Self::SyringeGun |
            Self::MediGun => &[Class::Medic],
            Self::Shotgun => &[Class::Soldier, Class::Pyro, Class::Engineer, Class::Heavy],
            Self::Pistol => &[Class::Scout, Class::Engineer],
        }
    }
    
    /// Checks if the weapon is used by a specific class.
    pub fn used_by_class(&self, class: Class) -> bool {
        self.used_by_classes().contains(&class)
    }
    
    /// Gets the stock weapons available to a specific class.
    pub fn class_stock_weapons(class: Class) -> &'static [StockWeapon] {
        match class {
            Class::Scout => &[Self::Scattergun, Self::Pistol, Self::Bat],
            Class::Demoman => &[Self::GrenadeLauncher, Self::StickybombLauncher, Self::Bottle],
            Class::Pyro => &[Self::FlameThrower, Self::Shotgun, Self::FireAxe],
            Class::Sniper => &[Self::SniperRifle, Self::SMG, Self::Kukri],
            Class::Spy => &[Self::Revolver, Self::Knife, Self::DisguiseKit, Self::InvisWatch],
            Class::Heavy => &[Self::Minigun, Self::Shotgun, Self::Fists],
            Class::Soldier => &[Self::RocketLauncher, Self::Shotgun, Self::Shovel],
            Class::Engineer => &[Self::Shotgun, Self::Pistol, Self::Wrench, Self::ConstructionPDA, Self::DestructionPDA],
            Class::Medic => &[Self::SyringeGun, Self::MediGun, Self::Bonesaw],
        }
    }
    
    /// Gets the item slot of the weapon for the class.
    /// 
    /// **Note:** [`StockWeapon::Shotgun`] is a secondary weapon for the Soldier and Pyro classes
    /// but a primary weapon for the Engineer class. It's returned as [`ItemSlot::Secondary`] here.
    pub fn item_slot(&self) -> ItemSlot {
        match self {
            Self::Bat => ItemSlot::Melee,
            Self::Scattergun => ItemSlot::Primary,
            Self::Bottle => ItemSlot::Secondary,
            Self::FireAxe => ItemSlot::Melee,
            Self::FlameThrower => ItemSlot::Primary,
            Self::Kukri => ItemSlot::Melee,
            Self::Knife => ItemSlot::Melee,
            Self::Fists => ItemSlot::Melee,
            Self::Shovel => ItemSlot::Melee,
            Self::Wrench => ItemSlot::Melee,
            Self::Bonesaw => ItemSlot::Melee,
            Self::SMG => ItemSlot::Secondary,
            Self::SyringeGun => ItemSlot::Primary,
            Self::RocketLauncher => ItemSlot::Primary,
            Self::GrenadeLauncher => ItemSlot::Primary,
            Self::StickybombLauncher => ItemSlot::Secondary,
            Self::SniperRifle => ItemSlot::Primary,
            Self::Minigun => ItemSlot::Primary,
            Self::Pistol => ItemSlot::Secondary,
            Self::Revolver => ItemSlot::Primary,
            Self::DisguiseKit => ItemSlot::PDA,
            Self::InvisWatch => ItemSlot::PDA2,
            Self::MediGun => ItemSlot::Secondary,
            Self::ConstructionPDA => ItemSlot::PDA,
            Self::DestructionPDA => ItemSlot::PDA2,
            Self::PDA => ItemSlot::Building,
            Self::Shotgun => ItemSlot::Secondary,
        }
    }
}

// Manually implemented since TryFromPrimitive only supports mapping each variant to one number.
impl TryFromPrimitive for StockWeapon {
    const NAME: &str = "StockWeapon";
    type Primitive = u32;
    type Error = TryFromPrimitiveError<StockWeapon>;
    
    fn try_from_primitive(number: u32) -> Result<Self, Self::Error> {
        Self::from_defindex(number)
            .ok_or(TryFromPrimitiveError { number })
    }
}

impl TryFrom<u32> for StockWeapon {
    type Error = TryFromPrimitiveError<StockWeapon>;
    
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_primitive(value)
    }
}

impl TryFrom<&u32> for StockWeapon {
    type Error = TryFromPrimitiveError<StockWeapon>;
    
    fn try_from(value: &u32) -> Result<Self, Self::Error> {
        Self::try_from_primitive(*value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn tries_from_primitive() {
        let weapon = StockWeapon::try_from(0).unwrap();
        
        assert_eq!(weapon, StockWeapon::Bat);
    }
}
