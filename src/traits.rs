pub use strum::{IntoEnumIterator, EnumCount};

/// The `defindex` value for an item attribute.
pub trait Attribute: Sized {
    const DEFINDEX: u32;
}

/// The `defindex` values for a set of item attributes.
pub trait Attributes: Sized {
    const DEFINDEX: &'static [u32];
}
