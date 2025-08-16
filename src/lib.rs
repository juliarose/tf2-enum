//! Provides enumerated types for models related to the Team Fortress 2 item schema.
//! 
//! For the most part, definitions here are relatively stable and don't have any new values added
//! or changed. However, Valve can implement changes to the item schema at any time, which may
//! affect some of the values defined here.
//! 
//! Definitions for things that are often updated like items, attributes, skins, and particles are
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

pub mod econ_attributes;
pub mod error;

mod attribute_def;
mod attribute_value;
mod capability;
mod class;
mod craft_class;
mod craft_material_type;
mod description_format;
mod effect_type;
mod grade;
mod item_level;
mod item_slot;
mod kill_eater_score_type;
mod killstreak_tier;
mod killstreaker;
mod origin;
mod paint;
mod quality;
mod sheen;
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
pub use traits::{Attribute, Attributes, AttributeSet, Colored, ItemDefindex};

// Enum re-exports
pub use attribute_def::AttributeDef;
pub use attribute_value::AttributeValue;
pub use capability::Capability;
pub use class::Class;
pub use craft_class::CraftClass;
pub use craft_material_type::CraftMaterialType;
pub use description_format::DescriptionFormat;
pub use effect_type::EffectType;
pub use grade::Grade;
pub use item_level::{ItemLevel, Level};
pub use item_slot::ItemSlot;
pub use kill_eater_score_type::KillEaterScoreType;
pub use killstreak_tier::KillstreakTier;
pub use killstreaker::Killstreaker;
pub use origin::Origin;
pub use paint::Paint;
pub use quality::Quality;
pub use sheen::Sheen;
pub use spell::{FootprintsSpell, PaintSpell, Spell};
pub use spell_set::{SpellSet, SpellSetIterator};
pub use stock_weapon::StockWeapon;
pub use strange_part::StrangePart;
pub use strange_part_set::{StrangePartSet, StrangePartSetIterator};
pub use wear::Wear;
