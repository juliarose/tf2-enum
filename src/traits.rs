
/// Attribute values for an item attribute.
pub trait Attribute: Sized {
    const DEFINDEX: u32;
    const NAME: &str;
    const ATTRIBUTE_CLASS: &str;
    const DESCRIPTION_STRING: Option<&str>;
    const DESCRIPTION_FORMAT: Option<&str>;
    const EFFECT_TYPE: &str;
    const HIDDEN: bool;
    const STORED_AS_INTEGER: bool;
}

/// Associated attribute values for a set of item attributes.
pub trait Attributes: Sized {
    const DEFINDEX: &[u32];
    const NAME: &[&str];
    const ATTRIBUTE_CLASS: &[&str];
    const DESCRIPTION_STRING: &[Option<&str>];
    const DESCRIPTION_FORMAT: &[Option<&str>];
    const EFFECT_TYPE: &[&str];
    const HIDDEN: &[bool];
    const STORED_AS_INTEGER: &[bool];
}
