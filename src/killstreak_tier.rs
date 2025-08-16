use crate::{Attribute, AttributeValue, EffectType, DescriptionFormat};
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
    const NAME: &str = "killstreak tier";
    const ATTRIBUTE_CLASS: Option<&str> = Some("killstreak_tier");
    const DESCRIPTION_STRING: Option<&str> = Some("Killstreaks Active");
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = Some(DescriptionFormat::ValueIsAdditive);
    const EFFECT_TYPE: EffectType = EffectType::Positive;
    const HIDDEN: bool = false;
    const STORED_AS_INTEGER: bool = false;
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        Some((*self as u32) as f64)
    }
}
