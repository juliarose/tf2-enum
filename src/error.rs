//! Provides error types.

use crate::Spell;
use std::fmt;

pub use strum::ParseError;
pub use num_enum::TryFromPrimitiveError;

/// An error when attempting to convert a spell into a sub-set of spells (footprints or paint).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TryFromSpellError {
    /// The defindex that was attempted to be converted.
    pub defindex: u32,
    /// The spell that was attempted to be converted.
    pub value: Spell,
}

impl fmt::Display for TryFromSpellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No value matching `{}` for attribute `{}`", self.value, self.defindex)
    }
}

impl std::error::Error for TryFromSpellError {}

/// An error inserting into an attribute set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InsertError {
    /// The set was full.
    Full,
    /// The value already exists in the set.
    Duplicate,
}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InsertError::Full => write!(f, "Attribute set is full"),
            InsertError::Duplicate => write!(f, "Attribute already exists in set"),
        }
    }
}

impl std::error::Error for InsertError {}