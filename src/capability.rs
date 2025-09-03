use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString};

/// Capability.
#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Display,
    Deserialize,
    Serialize,
    EnumString,
    EnumIter,
    EnumCount,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Capability {
    /// Whether the item can be painted.
    Paintable,
    /// Whether the item can be renamed.
    Nameable,
    /// Whether the item can be crafted if purchased.
    CanCraftIfPurchased,
    /// Whether the item can be gift wrapped.
    CanGiftWrap,
    /// Whether the item can have a craft count.
    CanCraftCount,
    /// Whether the item can have a craft number.
    CanCraftMark,
    /// Whether the item can be restored.
    CanBeRestored,
    /// Whether the item can have strange parts.
    StrangeParts,
    /// Whether the item can be card upgraded.
    CanCardUpgrade,
    /// Whether the item can be strangified
    CanStrangify,
    /// Whether the item can be killstreakified.
    CanKillstreakify,
    /// Whether the item can be consumed.
    CanConsume,
    /// Whether the item can be unusualified.
    CanUnusualify,
    /// Whether the item can be collected.
    CanCollect,
    /// Whether the item can have a custom texture.
    CanCustomizeTexture,
    /// Whether the item can have a spell page.
    CanSpellPage,
    /// Whether the item is decodable.
    Decodable,
    /// Whether the item is duck upgradable.
    DuckUpgradable,
    /// Whether the item is usable.
    Usable,
    /// Whether the item is usable in GC.
    UsableGc,
    /// Whether the item is usable out of game.
    UsableOutOfGame,
}
