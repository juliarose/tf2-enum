use strum_macros::{Display, EnumString, EnumIter, EnumCount};

/// Capability. Not meant for serialization.
#[derive(
    Debug,
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
pub enum Capability {
    /// Whether the item can be gift wrapped.
    #[strum(serialize = "can gift wrap")]
    CanGiftWrap,
    /// Whether the item can be renamed.
    #[strum(serialize = "nameable")]
    Nameable,
    /// Whether the item can have a craft number.
    #[strum(serialize = "can craft mark")]
    CanCraftMark,
    /// Whether the item can have strange parts.
    #[strum(serialize = "strange parts")]
    StrangeParts,
    /// Whether the item can be restored.
    #[strum(serialize = "can be restored")]
    CanBeRestored,
    /// Whether the item can be strangified
    #[strum(serialize = "can strangify")]
    CanStrangify,
    /// Whether the item can be card upgraded.
    #[strum(serialize = "can card upgrade")]
    CanCardUpgrade,
    /// Whether the item can be consumed.
    #[strum(serialize = "can consume")]
    CanConsume,
    /// Whether the item can be killstreakified.
    #[strum(serialize = "can killstreakify")]
    CanKillstreakify,
    /// Whether the item can be crafted if purchased.
    #[strum(serialize = "can craft if purchased")]
    CanCraftIfPurchased,
    /// Whether the item can be painted.
    #[strum(serialize = "paintable")]
    Paintable,
    /// Whether the item can have a craft count.
    #[strum(serialize = "can craft count")]
    CanCraftCount,
}
