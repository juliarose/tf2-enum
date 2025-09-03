 use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString};

/// Drop type.
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
#[allow(missing_docs)]
pub enum DropType {
    Drop,
    None,
}
