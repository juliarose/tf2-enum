use crate::{
    Attribute,
    AttributeValue,
    AttributeDef,
    TryFromAttributeValueU32,
    EffectType,
    DescriptionFormat,
    ItemAttribute,
};
use strum::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Killstreaker.
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
pub enum Killstreaker {
    FireHorns = 2002,
    CerebralDischarge = 2003,
    Tornado = 2004,
    Flames = 2005,
    Singularity = 2006,
    Incinerator = 2007,
    #[strum(serialize = "Hypno-Beam")]
    HypnoBeam = 2008,
}

/// Represents the "killstreak_effect" attribute.
impl Attribute for Killstreaker {
    const DEFINDEX: u32 = 2013;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2013,
        name: "killstreak effect",
        attribute_class: Some("killstreak_effect"),
        description_string: Some("Killstreaker: %s1"),
        description_format: Some(DescriptionFormat::ValueIsKillstreakEffectIndex),
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

impl TryFromAttributeValueU32 for Killstreaker {}

impl From<Killstreaker> for ItemAttribute {
    fn from(val: Killstreaker) -> Self {
        ItemAttribute {
            defindex: Killstreaker::DEFINDEX,
            value: val.attribute_value(),
            float_value: val.attribute_float_value(),
        }
    }
}
