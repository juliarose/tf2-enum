use crate::{
    Attribute,
    AttributeDef,
    DescriptionFormat,
    EffectType,
    ItemAttribute,
    TryFromIntAttributeValue,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::{Display, EnumCount, EnumIter, EnumString};

/// Sheen.
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
    Serialize_repr,
    Deserialize_repr,
    EnumString,
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
)]
#[repr(u32)]
#[strum(serialize_all = "title_case")]
#[allow(missing_docs)]
pub enum Sheen {
    TeamShine = 1,
    DeadlyDaffodil = 2,
    Manndarin = 3,
    MeanGreen = 4,
    AgonizingEmerald = 5,
    VillainousViolet = 6,
    HotRod = 7,
}

impl Attribute for Sheen {
    const DEFINDEX: u32 = 2014;
    const USES_FLOAT_VALUE: bool = true;
    /// Represents the "killstreak_idleeffect" attribute.
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2014,
        name: "killstreak idleeffect",
        attribute_class: Some("killstreak_idleeffect"),
        description_string: Some("Sheen: %s1"),
        description_format: Some(DescriptionFormat::ValueIsKillstreakIdleEffectIndex),
        effect_type: EffectType::Positive,
        hidden: false,
        stored_as_integer: false,
    };
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f32> {
        Some((*self as u32) as f32)
    }
}

impl TryFromIntAttributeValue for Sheen {}

impl From<Sheen> for ItemAttribute {
    fn from(val: Sheen) -> Self {
        ItemAttribute {
            defindex: Sheen::DEFINDEX,
            value: val.attribute_value(),
            float_value: val.attribute_float_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_display_and_parse() {
        assert_eq!(Sheen::from_str("Hot Rod").unwrap(), Sheen::HotRod);
        assert_eq!(Sheen::HotRod.to_string(), "Hot Rod");
    }

    #[test]
    fn test_serde_repr() {
        let s = serde_json::to_string(&Sheen::MeanGreen).unwrap();
        assert_eq!(s, "4");
        let d: Sheen = serde_json::from_str("4").unwrap();
        assert_eq!(d, Sheen::MeanGreen);
    }
    
    #[test]
    fn accepts_attibute_argument() {
        fn attribute_float_value<A: Attribute>(attribute: A) -> Option<f32> {
            attribute.attribute_float_value()
        }
        
        assert_eq!(attribute_float_value(Sheen::DeadlyDaffodil), Some(2.0));
    }
}
