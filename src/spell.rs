use crate::error::TryFromSpellError;
use crate::{
    Attribute,
    Attributes,
    AttributeDef,
    AttributeValue,
    DescriptionFormat,
    EffectType,
    ItemAttribute,
    TryFromIntAttributeValue,
};
use crate::econ_attributes::{
    HalloweenDeathGhosts,
    HalloweenGreenFlames,
    HalloweenPumpkinExplosions,
    HalloweenVoiceModulation,
};
use std::fmt;
use std::str::FromStr;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::{Display, EnumCount, EnumIter, EnumString};

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
#[allow(missing_docs)]
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
    
    /// Gets the attribute ID used to identify this spell. This will only return a value for
    /// footprints spells and paint spells.
    pub fn attribute_id(&self) -> Option<u32> {
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
            // bool - "has a spell"
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

impl Attributes for Spell {
    const DEFINDEX: &'static [u32] = &[
        1004,
        1005,
        1006,
        1007,
        1008,
        1009,
    ];
    const USES_FLOAT_VALUE: bool = true;
    /// Represents the "set_item_tint_rgb_override", "halloween_footstep_type",
    /// "halloween_voice_modulation", "halloween_pumpkin_explosions", "halloween_green_flames",
    /// and "halloween_death_ghosts" attributes.
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
    /// // 1 means true in this context
    /// assert_eq!(Spell::Exorcism.attribute_float_value(), Some(1.));
    /// ```
    fn attribute_float_value(&self) -> Option<f32> {
        Some(match self {
            Self::DieJob => 0.0,
            Self::ChromaticCorruption => 1.0,
            Self::PutrescentPigmentation => 2.0,
            Self::SpectralSpectrum => 3.0,
            Self::SinisterStaining => 4.0,
            Self::TeamSpiritFootprints => 1.0,
            Self::HeadlessHorseshoes => 2.0,
            Self::CorpseGrayFootprints => 3100495.0,
            Self::ViolentVioletFootprints => 5322826.0,
            Self::BruisedPurpleFootprints => 8208497.0,
            Self::GangreenFootprints => 8421376.0,
            Self::RottenOrangeFootprints => 13595446.0,
            // bool - "has a spell"
            _ => 1.0,
        })
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

impl From<Spell> for ItemAttribute {
    fn from(val: Spell) -> Self {
        ItemAttribute {
            defindex: val.attribute_defindex(),
            value: val.attribute_value(),
            float_value: val.attribute_float_value(),
        }
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

/// Paint spell.
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
    Deserialize_repr,
    Serialize_repr,
    EnumString,
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
)]
#[repr(u32)]
#[strum(serialize_all = "title_case")]
#[allow(missing_docs)]
pub enum PaintSpell {
    DieJob = 0,
    ChromaticCorruption = 1,
    PutrescentPigmentation = 2,
    SpectralSpectrum = 3,
    SinisterStaining = 4,
}

impl Attribute for PaintSpell {
    const DEFINDEX: u32 = 1004;
    const USES_FLOAT_VALUE: bool = true;
    /// Represents the "set_item_tint_rgb_override" attribute.
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
    
    fn attribute_float_value(&self) -> Option<f32> {
        Some((*self as u32) as f32)
    }
}

impl TryFromIntAttributeValue for PaintSpell {
    fn try_from_attribute_value(_v: AttributeValue) -> Option<Self> {
        None
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

impl From<PaintSpell> for ItemAttribute {
    fn from(val: PaintSpell) -> Self {
        ItemAttribute {
            defindex: PaintSpell::DEFINDEX,
            value: val.attribute_value(),
            float_value: val.attribute_float_value(),
        }
    }
}

/// Footprints spell.
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
    Deserialize_repr,
    Serialize_repr,
    EnumString,
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
)]
#[repr(u32)]
#[strum(serialize_all = "title_case")]
#[allow(missing_docs)]
pub enum FootprintsSpell {
    TeamSpiritFootprints = 1,
    HeadlessHorseshoes = 2,
    CorpseGrayFootprints = 3100495,
    ViolentVioletFootprints = 5322826,
    BruisedPurpleFootprints = 8208497,
    GangreenFootprints = 8421376,
    RottenOrangeFootprints = 13595446,
}

impl Attribute for FootprintsSpell {
    const DEFINDEX: u32 = 1005;
    const USES_FLOAT_VALUE: bool = true;
    /// Represents the "halloween_footstep_type" attribute.
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
    
    fn attribute_float_value(&self) -> Option<f32> {
        Some((*self as u32) as f32)
    }
}

impl TryFromIntAttributeValue for FootprintsSpell {}

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

impl From<FootprintsSpell> for ItemAttribute {
    fn from(val: FootprintsSpell) -> Self {
        ItemAttribute {
            defindex: PaintSpell::DEFINDEX,
            value: val.attribute_value(),
            float_value: val.attribute_float_value(),
        }
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
