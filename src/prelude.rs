//! Provides a prelude for easy importing of commonly used items.
//! 
//! # Examples
//! ```
//! use tf2_enum::prelude::*;
//!
//! let quality = Quality::Strange;
//! println!("Item quality: {:?}", quality);
//! 
//! let spell = FootprintsSpell::HeadlessHorseshoes;
//! println!("{}: {}", FootprintsSpell::DEFINDEX, spell as u32);
//! ```

// Traits
pub use crate::{
    Attribute,
    Attributes,
    AttributeSet,
    TryFromIntAttributeValue,
    Colored,
    HasItemDefindex,
};

// Enums and types
pub use crate::{
    Class,
    FootprintsSpell,
    Grade,
    ItemLevel,
    ItemSlot,
    KillEaterScoreType,
    KillstreakTier,
    Killstreaker,
    Origin,
    Paint,
    PaintSpell,
    Quality,
    Sheen,
    Spell,
    StrangePart,
    Wear,
};

// Sets
pub use crate::{
    SpellSet,
    StrangePartSet,
};

// External crates
pub use strum::{EnumCount, IntoEnumIterator};
pub use num_enum::{IntoPrimitive, TryFromPrimitive};
