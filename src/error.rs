use crate::Spell;
use std::fmt;

/// An error when attempting to convert a spell into a sub-set of spells (footprints or paint).
#[derive(Debug)]
pub struct TryFromSpellError {
    pub defindex: u32,
    pub value: Spell,
}

impl fmt::Display for TryFromSpellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No value matching `{}` for attribute `{}`", self.value, self.defindex)
    }
}

impl std::error::Error for TryFromSpellError {}
