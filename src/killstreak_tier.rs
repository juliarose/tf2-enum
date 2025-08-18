use crate::{
    Attribute,
    AttributeDef,
    TryFromAttributeValueU32,
    DescriptionFormat,
    EffectType,
    ItemAttribute,
};
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
#[allow(missing_docs)]
pub enum KillstreakTier {
    #[strum(serialize = "Killstreak")]
    Killstreak = 1,
    #[strum(serialize = "Specialized Killstreak")]
    Specialized = 2,
    #[strum(serialize = "Professional Killstreak")]
    Professional = 3,
}

impl Attribute for KillstreakTier {
    const DEFINDEX: u32 = 2025;
    const USES_FLOAT_VALUE: bool = true;
    /// Represents the "killstreak_tier" attribute.
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
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f32> {
        Some((*self as u32) as f32)
    }
}

impl TryFromAttributeValueU32 for KillstreakTier {}

impl From<KillstreakTier> for ItemAttribute {
    fn from(val: KillstreakTier) -> Self {
        ItemAttribute {
            defindex: KillstreakTier::DEFINDEX,
            value: val.attribute_value(),
            float_value: val.attribute_float_value(),
        }
    }
}
