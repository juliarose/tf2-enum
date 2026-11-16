//! Provides enumerated types for attributes related to the Team Fortress 2 item schema.
//! 
//! Definitions are generally stable, though Valve occasionally changes spelling or adds new values.
//! Frequently updated values like items, skins, and particles are not included.
//! 
//! Includes some additional utilities, such as [`SpellSet`] and [`StrangePartSet`] or working with
//! sets of spells and strange parts respectively.
//! 
//! ## Usage
//! 
//! ```
//! use tf2_enum::{Quality, Spell, IntoEnumIterator};
//! use std::str::FromStr;
//! 
//! assert_eq!("Unusual".parse::<Quality>().unwrap(), Quality::Unusual);
//! assert_eq!(Quality::Unusual as u32, 5);
//! assert_eq!(Spell::HalloweenFire.to_string(), "Halloween Fire");
//! 
//! /// Iterate over all quality values.
//! for quality in Quality::iter() {
//!     println!("{quality}");
//! }
//! ```
//! 
//! If you are using [sqlx](https://crates.io/crates/sqlx) (v0.8) with PostgreSQL, you can enable
//! the `sqlx-postgres-0_8` feature. This adds the appropriate bindings for Postgres databases for
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

#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
mod sqlx_macros;

#[cfg(feature = "sqlx-postgres-0_8")]
pub use sqlx_08 as sqlx;

#[cfg(all(feature = "sqlx-postgres-0_9", not(feature = "sqlx-postgres-0_8")))]
pub use sqlx_09 as sqlx;

#[cfg(all(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
compile_error!("features `sqlx-postgres-0_8` and `sqlx-postgres-0_9` cannot be enabled together");

#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_display_postgres!(Capability);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_display_postgres!(Class);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_display_postgres!(CraftClass);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_display_postgres!(CraftMaterialType);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_display_postgres!(DescriptionFormat);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_display_postgres!(DropType);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_display_postgres!(EffectType);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_display_postgres!(Grade);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_display_postgres!(HolidayRestriction);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_display_postgres!(ItemLevel);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_display_postgres!(ItemSlot);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_display_postgres!(Spell);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_display_postgres!(StockWeapon);

#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(Flags);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(FootprintsSpell);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(GCItemSort);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(KillEaterScoreType);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(KillstreakTier);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(Killstreaker);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(Origin);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(Paint);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(PaintSpell);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(Quality);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(Rarity);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(Sheen);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(StrangePart);
#[cfg(any(feature = "sqlx-postgres-0_8", feature = "sqlx-postgres-0_9"))]
impl_sqlx_enum_repr_postgres!(Wear);
