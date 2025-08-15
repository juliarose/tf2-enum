use crate::{EffectType, DescriptionFormat};

/// Attribute values for an item attribute.
pub trait Attribute: Sized {
    /// The defindex.
    const DEFINDEX: u32;
    /// The name.
    const NAME: &str;
    /// The attribute class.
    const ATTRIBUTE_CLASS: &str;
    /// The description string.
    const DESCRIPTION_STRING: Option<&str>;
    /// The description format.
    const DESCRIPTION_FORMAT: Option<DescriptionFormat>;
    /// The effect type.
    const EFFECT_TYPE: EffectType;
    /// Whether the attribute is hidden.
    const HIDDEN: bool;
    /// Whether the value for the attribute is stored as an integer.
    const STORED_AS_INTEGER: bool;
}

/// Associated attribute values for a set of item attributes.
pub trait Attributes: Sized {
    /// The defindex.
    const DEFINDEX: &[u32];
    /// The name.
    const NAME: &[&str];
    /// The attribute class.
    const ATTRIBUTE_CLASS: &[&str];
    /// The description string.
    const DESCRIPTION_STRING: &[Option<&str>];
    /// The description format.
    const DESCRIPTION_FORMAT: &[Option<DescriptionFormat>];
    /// The effect type.
    const EFFECT_TYPE: &[EffectType];
    /// Whether the attribute is hidden.
    const HIDDEN: &[bool];
    /// Whether the value for the attribute is stored as an integer.
    const STORED_AS_INTEGER: &[bool];
}

/// Definitions which are associated with colors.
pub trait Colored: Sized {
    /// Gets the color.
    fn color(&self) -> u32;
    
    /// Attempts to convert a hexadecimal color.
    fn from_color(color: u32) -> Option<Self>;
    
    /// Converts this into a hexademical color string in the format "#FFFFFF".
    fn color_str(&self) -> String {
        format!("#{:06X}", self.color())
    }
    
    /// Attempts to convert a hexadecimal color string.
    fn from_color_str(color: &str) -> Option<Self> {
        let len = color.len();
        let mut color = color;
        
        if len == 7 && color.starts_with('#') {
            color = &color[1..len];
        } else if len != 6 {
            return None;
        }
        
        let color = u32::from_str_radix(color, 16).ok()?;
        
        Self::from_color(color)
    }
}

/// Definitions which are associated with an item defindex.
pub trait ItemDefindex: Sized {
    /// Gets the `defindex`.
    fn defindex(&self) -> u32;
    
    /// Converts a `defindex` into its related item, if it exists.
    fn from_defindex(defindex: u32) -> Option<Self>;
}
