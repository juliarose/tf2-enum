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
//! use tf2_enum::{Quality, Spell, ItemLevel, KillstreakTier};
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
//! ```
pub mod error;

mod quality;
mod class;
mod killstreak_tier;
mod sheen;
mod spell;
mod grade;
mod killstreaker;
mod paint;
mod strange_part;
mod wear;
mod traits;
mod craft_class;
mod item_slot;
mod origin;
mod craft_material_type;
mod kill_eater_score_type;
mod item_level;
mod stock_weapon;
mod capability;
mod effect_type;
mod description_format;
mod spell_set;
mod strange_part_set;

pub use traits::{Attribute, Attributes};
pub use quality::Quality;
pub use class::Class;
pub use killstreak_tier::KillstreakTier;
pub use sheen::Sheen;
pub use spell::{Spell, FootprintsSpell, PaintSpell};
pub use grade::Grade;
pub use killstreaker::Killstreaker;
pub use paint::Paint;
pub use strange_part::StrangePart;
pub use wear::Wear;
pub use craft_class::CraftClass;
pub use item_slot::ItemSlot;
pub use origin::Origin;
pub use craft_material_type::CraftMaterialType;
pub use kill_eater_score_type::KillEaterScoreType;
pub use item_level::{ItemLevel, Level};
pub use stock_weapon::StockWeapon;
pub use capability::Capability;
pub use effect_type::EffectType;
pub use description_format::DescriptionFormat;
pub use spell_set::{SpellSet, SpellSetIterator};
pub use strange_part_set::{StrangePartSet, StrangePartSetIterator};

pub use num_enum;
pub use strum;
pub use strum_macros;
