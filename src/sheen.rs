use crate::{Attribute, AttributeValue, AttributeDef, EffectType, DescriptionFormat};
use strum::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Sheen.
#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Display,
    EnumString,
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
    Clone,
    Copy,
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

/// Represents the "killstreak_idleeffect" attribute.
impl Attribute for Sheen {
    const DEFINDEX: u32 = 2014;
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
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        Some((*self as u32) as f64)
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
        fn attribute_float_value<A: Attribute>(attribute: A) -> Option<f64> {
            attribute.attribute_float_value()
        }
        
        assert_eq!(attribute_float_value(Sheen::DeadlyDaffodil), Some(2.0));
    }
}
