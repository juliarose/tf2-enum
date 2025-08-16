use crate::{Attribute, AttributeValue, AttributeDef, EffectType, DescriptionFormat};
use strum::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Killstreak tier.
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
pub enum KillstreakTier {
    #[strum(serialize = "Killstreak")]
    Killstreak = 1,
    #[strum(serialize = "Specialized Killstreak")]
    Specialized = 2,
    #[strum(serialize = "Professional Killstreak")]
    Professional = 3,
}

/// killstreak_tier
impl Attribute for KillstreakTier {
    const DEFINDEX: u32 = 2025;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2025,
        name: "killstreak tier",
        attribute_class: Some("killstreak_tier"),
        description_string: Some("Killstreaks Active"),
        description_format: Some(DescriptionFormat::ValueIsAdditive),
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
