use crate::error::TryFromSpellError;
use crate::{Attribute, Attributes};
use std::fmt;
use std::str::FromStr;
use strum_macros::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde::{Serialize, Deserialize, Serializer};
use serde::de::{self, Visitor, Deserializer};

/// Spell.
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Display, EnumString, EnumIter, EnumCount, Clone, Copy)]
pub enum Spell {
    #[strum(serialize = "Team Spirit Footprints")]
    TeamSpiritFootprints,
    #[strum(serialize = "Gangreen Footprints")]
    GangreenFootprints,
    #[strum(serialize = "Corpse Gray Footprints")]
    CorpseGrayFootprints,
    #[strum(serialize = "Violent Violet Footprints")]
    ViolentVioletFootprints,
    #[strum(serialize = "Rotten Orange Footprints")]
    RottenOrangeFootprints,
    #[strum(serialize = "Bruised Purple Footprints")]
    BruisedPurpleFootprints,
    #[strum(serialize = "Headless Horseshoes")]
    HeadlessHorseshoes,
    #[strum(serialize = "Die Job")]
    DieJob,
    #[strum(serialize = "Chromatic Corruption")]
    ChromaticCorruption,
    #[strum(serialize = "Putrescent Pigmentation")]
    PutrescentPigmentation,
    #[strum(serialize = "Spectral Spectrum")]
    SpectralSpectrum,
    #[strum(serialize = "Sinister Staining")]
    SinisterStaining,
    // Allow conversion from "Voices from Below" but serialize as "Voices From Below".
    #[strum(serialize="Voices from Below", serialize = "Voices From Below")]
    VoicesFromBelow,
    #[strum(serialize = "Pumpkin Bombs")]
    PumpkinBombs,
    #[strum(serialize = "Halloween Fire")]
    HalloweenFire,
    #[strum(serialize = "Exorcism")]
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
    
    /// Gets the value of an attribute belonging to a group of spells.
    /// 
    /// Footprints and paint spells share a common attribute but have specific values that
    /// correspond to which spell is being referenced that can be used to identify the spell.
    /// 
    /// # Examples
    /// ```
    /// use tf2_enum::Spell;
    /// 
    /// assert_eq!(Spell::DieJob.attribute_value(), Some(0));
    /// assert_eq!(Spell::HeadlessHorseshoes.attribute_value(), Some(2));
    /// assert_eq!(Spell::Exorcism.attribute_value(), None);
    /// ```
    pub fn attribute_value(&self) -> Option<u32> {
        match self {
            Self::DieJob => Some(0),
            Self::ChromaticCorruption => Some(1),
            Self::PutrescentPigmentation => Some(2),
            Self::SpectralSpectrum => Some(3),
            Self::SinisterStaining => Some(4),
            Self::TeamSpiritFootprints => Some(1),
            Self::GangreenFootprints => Some(8421376),
            Self::CorpseGrayFootprints => Some(3100495),
            Self::ViolentVioletFootprints => Some(5322826),
            Self::RottenOrangeFootprints => Some(13595446),
            Self::BruisedPurpleFootprints => Some(8208497),
            Self::HeadlessHorseshoes => Some(2),
            _ => None,
        }
    }
}

impl Attributes for Spell {
    const DEFINDEX: &'static [u32] = &[1004, 1005, 1006, 1007, 1008, 1009];
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
        self.attribute_value().ok_or(())
    }
}

/// Paint spell.
#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Display, EnumString, EnumIter, EnumCount, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum PaintSpell {
    #[strum(serialize = "Die Job")]
    DieJob = 0,
    #[strum(serialize = "Chromatic Corruption")]
    ChromaticCorruption = 1,
    #[strum(serialize = "Putrescent Pigmentation")]
    PutrescentPigmentation = 2,
    #[strum(serialize = "Spectral Spectrum")]
    SpectralSpectrum = 3,
    #[strum(serialize = "Sinister Staining")]
    SinisterStaining = 4,
}

impl Attribute for PaintSpell {
    const DEFINDEX: u32 = 1004;
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

impl TryFrom<Spell> for PaintSpell {
    type Error = TryFromSpellError;
    
    fn try_from(value: Spell) -> Result<Self, Self::Error> {
        match value {
            Spell::DieJob => Ok(PaintSpell::DieJob),
            Spell::ChromaticCorruption => Ok(PaintSpell::ChromaticCorruption),
            Spell::PutrescentPigmentation => Ok(PaintSpell::PutrescentPigmentation),
            Spell::SpectralSpectrum => Ok(PaintSpell::SpectralSpectrum),
            Spell::SinisterStaining => Ok(PaintSpell::SinisterStaining),
            _ => Err(TryFromSpellError {
                defindex: Self::DEFINDEX,
                value
            }),
        }
    }
}

/// Footprints spell.
#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Display, EnumString, EnumIter, EnumCount, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum FootprintsSpell {
    #[strum(serialize = "Team Spirit Footprints")]
    TeamSpiritFootprints = 1,
    #[strum(serialize = "Gangreen Footprints")]
    GangreenFootprints = 8421376,
    #[strum(serialize = "Corpse Gray Footprints")]
    CorpseGrayFootprints = 3100495,
    #[strum(serialize = "Violent Violet Footprints")]
    ViolentVioletFootprints = 5322826,
    #[strum(serialize = "Rotten Orange Footprints")]
    RottenOrangeFootprints = 13595446,
    #[strum(serialize = "Bruised Purple Footprints")]
    BruisedPurpleFootprints = 8208497,
    #[strum(serialize = "Headless Horseshoes")]
    HeadlessHorseshoes = 2,
}

impl Attribute for FootprintsSpell {
    const DEFINDEX: u32 = 1005;
}

impl From<FootprintsSpell> for Spell {
    fn from(val: FootprintsSpell) -> Self {
        match val {
            FootprintsSpell::TeamSpiritFootprints => Spell::TeamSpiritFootprints,
            FootprintsSpell::GangreenFootprints => Spell::GangreenFootprints,
            FootprintsSpell::CorpseGrayFootprints => Spell::CorpseGrayFootprints,
            FootprintsSpell::ViolentVioletFootprints => Spell::ViolentVioletFootprints,
            FootprintsSpell::RottenOrangeFootprints => Spell::RottenOrangeFootprints,
            FootprintsSpell::BruisedPurpleFootprints => Spell::BruisedPurpleFootprints,
            FootprintsSpell::HeadlessHorseshoes => Spell::HeadlessHorseshoes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    
    #[test]
    fn from_str() {
        assert_eq!(Spell::from_str("Headless Horseshoes").unwrap(), Spell::HeadlessHorseshoes);
    }
    
    #[test]
    fn serialize_spell() {
        #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
        struct SpellAttribute {
            spell: Spell,
        }
        
        let attribute = SpellAttribute {
            spell: Spell::HeadlessHorseshoes,
        };
        let json = serde_json::to_string(&attribute).unwrap();
        
        assert_eq!(json, "{\"spell\":\"Headless Horseshoes\"}");
        assert_eq!(serde_json::from_str::<SpellAttribute>(&json).unwrap(), attribute);
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
        assert_eq!(Spell::VoicesFromBelow.to_string(), "Voices From Below");
        assert_eq!(Spell::from_str("Voices from Below").unwrap(), Spell::VoicesFromBelow);
        assert_eq!(Spell::from_str("Voices From Below").unwrap(), Spell::VoicesFromBelow);
    }
}
