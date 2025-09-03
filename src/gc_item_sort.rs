use serde::{Deserialize, Serialize};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::{Display, EnumCount, EnumIter};

/// GC item sort. Used when sending a `CMsgSortItems` request.
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
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
    Clone,
    Copy,
)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum GCItemSort {
    /// This won't do anything, but can be used as a safe "header" value
    NoSort = 0,
    /// Sorts items by name.
    SortByName = 1,
    /// Sorts items by defindex.
    SortByDefIndex = 2,
    /// Sorts items by quality.
    SortByRarity = 3,
    /// Sorts items by type.
    SortByType = 4,
    /// Sorts items by date acquired.
    SortByDate = 5,
    /// Not sure.
    GameSpecificBase = 100,
}
