pub use strum::{IntoEnumIterator, EnumCount};

pub trait Attribute {
    const DEFINDEX: u32;
}

pub trait Attributes {
    const DEFINDEX: &'static [u32];
}