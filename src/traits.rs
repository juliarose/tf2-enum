pub use strum::{IntoEnumIterator, EnumCount};

/// The `defindex` value for an item attribute.
pub trait Attribute {
    const DEFINDEX: u32;
}

/// The `defindex` values for a set of item attributes.
pub trait Attributes {
    const DEFINDEX: &'static [u32];
}
