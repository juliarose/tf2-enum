//! # tf2-enum
//! 
//! Provides enumerated types for models related to the Team Fortress 2 item schema.
//! 
//! ## Usage
//! 
//! ```
//! use tf2_enum::{Quality, KillstreakTier};
//! use std::str::FromStr;
//! 
//! assert_eq!(
//!     "Unusual".parse::<Quality>().unwrap(),
//!     Quality::Unusual,
//! );
//! assert_eq!(
//!     Quality::Unusual as u32,
//!     5,
//! );
//! assert_eq!(
//!     KillstreakTier::Professional.to_string(),
//!     "Professional Killstreak",
//! );
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

pub use traits::{IntoEnumIterator, EnumCount, Attribute, Attributes};
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

pub use num_enum;
pub use strum;
pub use strum_macros;
