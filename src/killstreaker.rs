use crate::{Attribute, AttributeValue, AttributeDef, EffectType, DescriptionFormat};
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
pub enum Killstreaker {
    #[strum(serialize = "Fire Horns")]
    FireHorns = 2002,
    #[strum(serialize = "Cerebral Discharge")]
    CerebralDischarge = 2003,
    #[strum(serialize = "Tornado")]
    Tornado = 2004,
    #[strum(serialize = "Flames")]
    Flames = 2005,
    #[strum(serialize = "Singularity")]
    Singularity = 2006,
    #[strum(serialize = "Incinerator")]
    Incinerator = 2007,
    #[strum(serialize = "Hypno-Beam")]
    HypnoBeam = 2008,
}

/// killstreak_effect
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
