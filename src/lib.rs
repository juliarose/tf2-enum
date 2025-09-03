//! Provides enumerated types for models related to the Team Fortress 2 item schema. Other
//! utilities relating to the schema are also included, as well as related item and
//! attribute conversion helpers. This crate provides a number of additional attributes mostly
//! related to trading.
//! 
//! For the most part, definitions here are relatively stable and don't have any new values added
//! or changed. However, Valve can implement changes to the item schema at any time, which may
//! affect some of the values defined here.
//! 
//! Definitions for things that are often updated like items, skins, and particles are
//! not included.
//! 
//! ## Usage
//! 
//! ```
//! use tf2_enum::{Quality, Spell, ItemLevel, KillstreakTier, IntoEnumIterator};
//! use std::str::FromStr;
//! 
//! assert_eq!("Unusual".parse::<Quality>().unwrap(), Quality::Unusual);
//! assert_eq!(Quality::Unusual as u32, 5);
//! assert_eq!(Spell::HalloweenFire.to_string(), "Halloween Fire");
//! 
//! let level = ItemLevel::KillEaterRank.score_level(9000);
//! let killstreak_tier = KillstreakTier::Professional;
//! let full_name = format!("{level} {killstreak_tier} Pomson 6000");
//! 
//! assert_eq!(full_name, "Hale's Own Professional Killstreak Pomson 6000");
//! 
//! /// Iterate over all quality values.
//! for quality in Quality::iter() {
//!     println!("{quality}");
//! }
//! ```
//! 
//! If you are using [sqlx](https://crates.io/crates/sqlx) (v0.8) with PostgreSQL, you can enable
//! the `sqlx-postgres` feature. This adds the appropriate bindings for Postgres databases for
//! enums.

#![warn(missing_docs)]

pub mod econ_attributes;
pub mod error;
pub mod prelude;

mod attribute_def;
mod attribute_value;
mod capability;
mod class;
mod craft_class;
mod craft_material_type;
mod description_format;
mod drop_type;
mod effect_type;
mod flags;
mod gc_item_sort;
mod grade;
mod holiday_restriction;
mod item_attribute;
mod item_level;
mod item_slot;
mod kill_eater_score_type;
mod killstreak_tier;
mod killstreaker;
mod origin;
mod paint;
mod quality;
mod rarity;
mod sheen;
mod serialize;
mod spell;
mod spell_set;
mod stock_weapon;
mod strange_part;
mod strange_part_set;
mod traits;
mod wear;

// Traits and utility re-exports
pub use strum::{EnumCount, IntoEnumIterator};
pub use num_enum::{IntoPrimitive, TryFromPrimitive};
pub use traits::{
    Attribute,
    Attributes,
    AttributeSet,
    TryFromIntAttributeValue,
    Colored,
    HasItemDefindex
};

// Enum re-exports
pub use attribute_def::AttributeDef;
pub use attribute_value::AttributeValue;
pub use capability::Capability;
pub use class::Class;
pub use craft_class::CraftClass;
pub use craft_material_type::CraftMaterialType;
pub use description_format::DescriptionFormat;
pub use drop_type::DropType;
pub use effect_type::EffectType;
pub use flags::Flags;
pub use gc_item_sort::GCItemSort;
pub use grade::Grade;
pub use holiday_restriction::HolidayRestriction;
pub use item_attribute::ItemAttribute;
pub use item_level::{ItemLevel, Level};
pub use item_slot::ItemSlot;
pub use kill_eater_score_type::KillEaterScoreType;
pub use killstreak_tier::KillstreakTier;
pub use killstreaker::Killstreaker;
pub use origin::Origin;
pub use paint::Paint;
pub use quality::Quality;
pub use rarity::Rarity;
pub use sheen::Sheen;
pub use spell::{FootprintsSpell, PaintSpell, Spell};
pub use spell_set::{SpellSet, SpellSetIterator};
pub use stock_weapon::StockWeapon;
pub use strange_part::StrangePart;
pub use strange_part_set::{StrangePartSet, StrangePartSetIterator};
pub use wear::Wear;

#[cfg(feature = "sqlx-postgres")]
mod sqlx_macros;

#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_display_postgres!(Capability);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_display_postgres!(Class);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_display_postgres!(CraftClass);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_display_postgres!(CraftMaterialType);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_display_postgres!(DescriptionFormat);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_display_postgres!(DropType);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_display_postgres!(EffectType);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_display_postgres!(Grade);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_display_postgres!(HolidayRestriction);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_display_postgres!(ItemLevel);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_display_postgres!(ItemSlot);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_display_postgres!(Spell);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_display_postgres!(StockWeapon);

#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(Flags);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(FootprintsSpell);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(GCItemSort);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(KillEaterScoreType);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(KillstreakTier);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(Killstreaker);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(Origin);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(Paint);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(PaintSpell);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(Quality);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(Rarity);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(Sheen);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(StrangePart);
#[cfg(feature = "sqlx-postgres")]
impl_sqlx_enum_repr_postgres!(Wear);
