use crate::error::TryFromSpellError;
use crate::{Attribute, AttributeValue, AttributeDef, Attributes, EffectType, DescriptionFormat};
use crate::econ_attributes::{
    HalloweenVoiceModulation,
    HalloweenPumpkinExplosions,
    HalloweenGreenFlames,
    HalloweenDeathGhosts,
};
use std::fmt;
use std::str::FromStr;
use strum::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde::{Serialize, Deserialize, Serializer};
use serde::de::{self, Visitor, Deserializer};

/// Spell.
/// 
/// As defined by the schema these wouldn't normally be grouped together as different types of
/// spells fall under different attributes, but in practice they are often treated as if they are.
#[derive(
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
    Clone,
    Copy,
)]
#[strum(serialize_all = "title_case")]
pub enum Spell {
    TeamSpiritFootprints,
    HeadlessHorseshoes,
    CorpseGrayFootprints,
    ViolentVioletFootprints,
    BruisedPurpleFootprints,
    GangreenFootprints,
    RottenOrangeFootprints,
    DieJob,
    ChromaticCorruption,
    PutrescentPigmentation,
    SpectralSpectrum,
    SinisterStaining,
    // Allow conversion from "Voices From Below" but serialize as "Voices from Below".
    #[strum(serialize = "Voices From Below", serialize = "Voices from Below")]
    VoicesFromBelow,
    PumpkinBombs,
    HalloweenFire,
    Exorcism,
}

impl Spell {
    /// The attribute `defindex` for paint spells.
    pub const DEFINDEX_PAINT: u32 = 1004;
    /// The attribute `defindex` for footprints spells.
    pub const DEFINDEX_FOOTPRINTS: u32 = 1005;
    /// The attribute `defindex` for voices from below spell.
    pub const DEFINDEX_VOICES_FROM_BELOW: u32 = 1006;
    /// The attribute `defindex` for pumpkin bombs spell.
    pub const DEFINDEX_PUMPKIN_BOMBS: u32 = 1007;
    /// The attribute `defindex` for halloween fire spell.
    pub const DEFINDEX_HALLOWEEN_FIRE: u32 = 1008;
    /// The attribute `defindex` for exorcism spell.
    pub const DEFINDEX_EXORCISM: u32 = 1009;
    
    /// Gets the attribute `defindex` of this spell.
    pub fn attribute_defindex(&self) -> u32 {
        match self {
            Self::DieJob |
            Self::ChromaticCorruption |
            Self::PutrescentPigmentation |
            Self::SpectralSpectrum |
            Self::SinisterStaining => Self::DEFINDEX_PAINT,
            Self::TeamSpiritFootprints |
            Self::GangreenFootprints |
            Self::CorpseGrayFootprints |
            Self::ViolentVioletFootprints |
            Self::RottenOrangeFootprints |
            Self::BruisedPurpleFootprints |
            Self::HeadlessHorseshoes => Self::DEFINDEX_FOOTPRINTS,
            Self::VoicesFromBelow => Self::DEFINDEX_VOICES_FROM_BELOW,
            Self::PumpkinBombs => Self::DEFINDEX_PUMPKIN_BOMBS,
            Self::HalloweenFire => Self::DEFINDEX_HALLOWEEN_FIRE,
            Self::Exorcism => Self::DEFINDEX_EXORCISM,
        }
    }
    
    /// Gets the attribute float value a `u32`.
    pub fn attribute_float_value_u32(&self) -> Option<u32> {
        match self {
            Self::DieJob => Some(0),
            Self::ChromaticCorruption => Some(1),
            Self::PutrescentPigmentation => Some(2),
            Self::SpectralSpectrum => Some(3),
            Self::SinisterStaining => Some(4),
            Self::TeamSpiritFootprints => Some(1),
            Self::HeadlessHorseshoes => Some(2),
            Self::CorpseGrayFootprints => Some(3100495),
            Self::ViolentVioletFootprints => Some(5322826),
            Self::BruisedPurpleFootprints => Some(8208497),
            Self::GangreenFootprints => Some(8421376),
            Self::RottenOrangeFootprints => Some(13595446),
            _ => None,
        }
    }

    /// Checks if this spell is a paint spell.
    pub fn is_paint_spell(&self) -> bool {
        matches!(
            self,
            Self::DieJob |
            Self::ChromaticCorruption |
            Self::PutrescentPigmentation |
            Self::SpectralSpectrum |
            Self::SinisterStaining,
        )
    }

    /// Checks if this spell is a footprints spell.
    pub fn is_footprints_spell(&self) -> bool {
        matches!(
            self,
            Self::TeamSpiritFootprints |
            Self::HeadlessHorseshoes |
            Self::CorpseGrayFootprints |
            Self::ViolentVioletFootprints |
            Self::BruisedPurpleFootprints |
            Self::GangreenFootprints |
            Self::RottenOrangeFootprints,
        )
    }
}

/// Represents the "set_item_tint_rgb_override", "halloween_footstep_type",
/// "halloween_voice_modulation", "halloween_pumpkin_explosions", "halloween_green_flames",
/// and "halloween_death_ghosts" attributes.
impl Attributes for Spell {
    const DEFINDEX: &[u32] = &[
        1004,
        1005,
        1006,
        1007,
        1008,
        1009,
    ];
    const ATTRIBUTES: &'static [AttributeDef] = &[
        AttributeDef {
            defindex: 1004,
            name: "SPELL: set item tint RGB",
            attribute_class: Some("set_item_tint_rgb_override"),
            description_string: Some("%s1"),
            description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
            effect_type: EffectType::Positive,
            hidden: false,
            stored_as_integer: false,
        },
        AttributeDef {
            defindex: 1005,
            name: "SPELL: set Halloween footstep type",
            attribute_class: Some("halloween_footstep_type"),
            description_string: Some("%s1"),
            description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
            effect_type: EffectType::Positive,
            hidden: false,
            stored_as_integer: false,
        },
        AttributeDef {
            defindex: 1006,
            name: "SPELL: Halloween voice modulation",
            attribute_class: Some("halloween_voice_modulation"),
            description_string: Some("Voices from Below"),
            description_format: Some(DescriptionFormat::ValueIsAdditive),
            effect_type: EffectType::Positive,
            hidden: false,
            stored_as_integer: false,
        },
        AttributeDef {
            defindex: 1007,
            name: "SPELL: Halloween pumpkin explosions",
            attribute_class: Some("halloween_pumpkin_explosions"),
            description_string: Some("Pumpkin Bombs"),
            description_format: Some(DescriptionFormat::ValueIsAdditive),
            effect_type: EffectType::Positive,
            hidden: false,
            stored_as_integer: false,
        },
        AttributeDef {
            defindex: 1008,
            name: "SPELL: Halloween green flames",
            attribute_class: Some("halloween_green_flames"),
            description_string: Some("Halloween Fire"),
            description_format: Some(DescriptionFormat::ValueIsAdditive),
            effect_type: EffectType::Positive,
            hidden: false,
            stored_as_integer: false,
        },
        AttributeDef {
            defindex: 1009,
            name: "SPELL: Halloween death ghosts",
            attribute_class: Some("halloween_death_ghosts"),
            description_string: Some("Exorcism"),
            description_format: Some(DescriptionFormat::ValueIsAdditive),
            effect_type: EffectType::Positive,
            hidden: false,
            stored_as_integer: false,
        },
    ];
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    /// Gets the value of an attribute belonging to a group of spells.
    /// 
    /// Footprints and paint spells share a common attribute but have specific values that
    /// correspond to which spell is being referenced that can be used to identify the spell.
    /// 
    /// # Examples
    /// ```
    /// use tf2_enum::{Spell, Attributes};
    /// 
    /// assert_eq!(Spell::DieJob.attribute_float_value(), Some(0.));
    /// assert_eq!(Spell::HeadlessHorseshoes.attribute_float_value(), Some(2.));
    /// assert_eq!(Spell::Exorcism.attribute_float_value(), None);
    /// ```
    fn attribute_float_value(&self) -> Option<f64> {
        self.attribute_float_value_u32().map(|v| v as f64)
    }
}

struct SpellVisitor;

impl<'de> Visitor<'de> for SpellVisitor {
    type Value = Spell;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }
    
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Spell::from_str(value).map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Spell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SpellVisitor)
    }
}

impl Serialize for Spell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl TryInto<u32> for Spell {
    type Error = ();
    
    fn try_into(self) -> Result<u32, Self::Error> {
        self.attribute_float_value_u32().ok_or(())
    }
}

impl From<PaintSpell> for Spell {
    fn from(val: PaintSpell) -> Self {
        match val {
            PaintSpell::DieJob => Spell::DieJob,
            PaintSpell::ChromaticCorruption => Spell::ChromaticCorruption,
            PaintSpell::PutrescentPigmentation => Spell::PutrescentPigmentation,
            PaintSpell::SpectralSpectrum => Spell::SpectralSpectrum,
            PaintSpell::SinisterStaining => Spell::SinisterStaining,
        }
    }
}

impl From<&PaintSpell> for Spell {
    fn from(val: &PaintSpell) -> Self {
        Self::from(*val)
    }
}

impl From<FootprintsSpell> for Spell {
    fn from(val: FootprintsSpell) -> Self {
        match val {
            FootprintsSpell::TeamSpiritFootprints => Self::TeamSpiritFootprints,
            FootprintsSpell::HeadlessHorseshoes => Self::HeadlessHorseshoes,
            FootprintsSpell::CorpseGrayFootprints => Self::CorpseGrayFootprints,
            FootprintsSpell::ViolentVioletFootprints => Self::ViolentVioletFootprints,
            FootprintsSpell::BruisedPurpleFootprints => Self::BruisedPurpleFootprints,
            FootprintsSpell::GangreenFootprints => Self::GangreenFootprints,
            FootprintsSpell::RottenOrangeFootprints => Self::RottenOrangeFootprints,
        }
    }
}

impl From<&FootprintsSpell> for Spell {
    fn from(val: &FootprintsSpell) -> Self {
        Self::from(*val)
    }
}

impl From<HalloweenVoiceModulation> for Spell {
    fn from(_: HalloweenVoiceModulation) -> Self {
        Spell::VoicesFromBelow
    }
}

impl From<&HalloweenVoiceModulation> for Spell {
    fn from(_: &HalloweenVoiceModulation) -> Self {
        Spell::VoicesFromBelow
    }
}

impl From<HalloweenPumpkinExplosions> for Spell {
    fn from(_: HalloweenPumpkinExplosions) -> Self {
        Spell::PumpkinBombs
    }
}

impl From<&HalloweenPumpkinExplosions> for Spell {
    fn from(_: &HalloweenPumpkinExplosions) -> Self {
        Spell::PumpkinBombs
    }
}

impl From<HalloweenGreenFlames> for Spell {
    fn from(_: HalloweenGreenFlames) -> Self {
        Spell::HalloweenFire
    }
}

impl From<&HalloweenGreenFlames> for Spell {
    fn from(_: &HalloweenGreenFlames) -> Self {
        Spell::HalloweenFire
    }
}

impl From<HalloweenDeathGhosts> for Spell {
    fn from(_: HalloweenDeathGhosts) -> Self {
        Spell::Exorcism
    }
}

impl From<&HalloweenDeathGhosts> for Spell {
    fn from(_: &HalloweenDeathGhosts) -> Self {
        Spell::Exorcism
    }
}

/// Paint spell.
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
pub enum PaintSpell {
    DieJob = 0,
    ChromaticCorruption = 1,
    PutrescentPigmentation = 2,
    SpectralSpectrum = 3,
    SinisterStaining = 4,
}

/// Represents the "set_item_tint_rgb_override" attribute.
impl Attribute for PaintSpell {
    const DEFINDEX: u32 = 1004;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 1004,
        name: "SPELL: set item tint RGB",
        attribute_class: Some("set_item_tint_rgb_override"),
        description_string: Some("%s1"),
        description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
        effect_type: EffectType::Positive,
        hidden: false,
        stored_as_integer: false,
    };
    
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        Some((*self as u32) as f64)
    }
}

impl TryFrom<Spell> for PaintSpell {
    type Error = TryFromSpellError;
    
    fn try_from(value: Spell) -> Result<Self, Self::Error> {
        match value {
            Spell::DieJob => Ok(Self ::DieJob),
            Spell::ChromaticCorruption => Ok(Self::ChromaticCorruption),
            Spell::PutrescentPigmentation => Ok(Self::PutrescentPigmentation),
            Spell::SpectralSpectrum => Ok(Self::SpectralSpectrum),
            Spell::SinisterStaining => Ok(Self::SinisterStaining),
            _ => Err(TryFromSpellError {
                defindex: Self::DEFINDEX,
                value
            }),
        }
    }
}

impl TryFrom<&Spell> for PaintSpell {
    type Error = TryFromSpellError;
    
    fn try_from(value: &Spell) -> Result<Self, Self::Error> {
        Self::try_from(*value)
    }
}

/// Footprints spell.
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
pub enum FootprintsSpell {
    TeamSpiritFootprints = 1,
    HeadlessHorseshoes = 2,
    CorpseGrayFootprints = 3100495,
    ViolentVioletFootprints = 5322826,
    BruisedPurpleFootprints = 8208497,
    GangreenFootprints = 8421376,
    RottenOrangeFootprints = 13595446,
}

/// Represents the "halloween_footstep_type" attribute.
impl Attribute for FootprintsSpell {
    const DEFINDEX: u32 = 1005;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 1005,
        name: "SPELL: set Halloween footstep type",
        attribute_class: Some("halloween_footstep_type"),
        description_string: Some("%s1"),
        description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
        effect_type: EffectType::Positive,
        hidden: false,
        stored_as_integer: false,
    };
    
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        Some((*self as u32) as f64)
    }
}

impl TryFrom<Spell> for FootprintsSpell {
    type Error = TryFromSpellError;
    
    fn try_from(value: Spell) -> Result<Self, Self::Error> {
        match value {
            Spell::TeamSpiritFootprints => Ok(Self::TeamSpiritFootprints),
            Spell::HeadlessHorseshoes => Ok(Self::HeadlessHorseshoes),
            Spell::CorpseGrayFootprints => Ok(Self::CorpseGrayFootprints),
            Spell::ViolentVioletFootprints => Ok(Self::ViolentVioletFootprints),
            Spell::BruisedPurpleFootprints => Ok(Self::BruisedPurpleFootprints),
            Spell::GangreenFootprints => Ok(Self::GangreenFootprints),
            Spell::RottenOrangeFootprints => Ok(Self::RottenOrangeFootprints),
            _ => Err(TryFromSpellError {
                defindex: Self::DEFINDEX,
                value
            }),
        }
    }
}

impl TryFrom<&Spell> for FootprintsSpell {
    type Error = TryFromSpellError;
    
    fn try_from(value: &Spell) -> Result<Self, Self::Error> {
        Self::try_from(*value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    
    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    struct SpellAttribute {
        spell: Spell,
    }
    
    #[test]
    fn from_str() {
        assert_eq!(Spell::from_str("Headless Horseshoes").unwrap(), Spell::HeadlessHorseshoes);
    }
    
    #[test]
    fn serialize_spell() {
        let attribute = SpellAttribute {
            spell: Spell::HeadlessHorseshoes,
        };
        let json = serde_json::to_string(&attribute).unwrap();
        
        assert_eq!(json, "{\"spell\":\"Headless Horseshoes\"}");
        assert_eq!(serde_json::from_str::<SpellAttribute>(&json).unwrap(), attribute);
        assert_eq!(serde_json::to_string(&Spell::HeadlessHorseshoes).unwrap(), "\"Headless Horseshoes\"");
    }
    
    #[test]
    fn deserializes_spell() {
        let json = "{\"spell\":\"Headless Horseshoes\"}";
        let attribute: SpellAttribute = serde_json::from_str(json).unwrap();
        
        assert_eq!(attribute.spell, Spell::HeadlessHorseshoes);
        assert_eq!(serde_json::from_str::<Spell>("\"Headless Horseshoes\"").unwrap(), Spell::HeadlessHorseshoes);
    }
    
    #[test]
    fn to_string() {
        assert_eq!(Spell::HeadlessHorseshoes.to_string(), "Headless Horseshoes");
    }
    
    #[test]
    fn from_repr() {
        assert_eq!(FootprintsSpell::try_from(2).unwrap(), FootprintsSpell::HeadlessHorseshoes);
    }
    
    #[test]
    fn voices_from_below_from_str() {
        assert_eq!(Spell::VoicesFromBelow.to_string(), "Voices from Below");
        assert_eq!(Spell::from_str("Voices from Below").unwrap(), Spell::VoicesFromBelow);
        assert_eq!(Spell::from_str("Voices From Below").unwrap(), Spell::VoicesFromBelow);
    }
    
    #[test]
    fn attribute_slices_are_equal_length() {
        assert_eq!(Spell::DEFINDEX.len(), Spell::ATTRIBUTES.len());
    }
}
