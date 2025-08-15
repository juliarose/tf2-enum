use crate::{Attribute, EffectType, DescriptionFormat};
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
    const NAME: &str = "killstreak effect";
    const ATTRIBUTE_CLASS: &str = "killstreak_effect";
    const DESCRIPTION_STRING: Option<&str> = Some("Killstreaker: %s1");
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = Some(DescriptionFormat::ValueIsKillstreakEffectIndex);
    const EFFECT_TYPE: EffectType = EffectType::Positive;
    const HIDDEN: bool = false;
    const STORED_AS_INTEGER: bool = false;
}
