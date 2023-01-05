use strum_macros::{Display, EnumString, EnumIter};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};
use std::fmt;
use crate::{Attribute, Attributes};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Spell {
    Footprints(FootprintsSpell),
    Paint(PaintSpell),
    VoicesFromBelow,
    PumpkinBombs,
    HalloweenFire,
    Exorcism,
}

impl Spell {
    
    pub const DEFINDEX_PAINT: u32 = 1004;
    pub const DEFINDEX_FOOTPRINTS: u32 = 1005;
    pub const DEFINDEX_VOICES_FROM_BELOW: u32 = 1006;
    pub const DEFINDEX_PUMPKIN_BOMBS: u32 = 1007;
    pub const DEFINDEX_HALLOWEEN_FIRE: u32 = 1008;
    pub const DEFINDEX_EXORCISM: u32 = 1009;
    
    pub fn attribute_defindex(&self) -> u32 {
        match self {
            Spell::Paint(_) => Self::DEFINDEX_PAINT,
            Spell::Footprints(_) => Self::DEFINDEX_FOOTPRINTS,
            Spell::VoicesFromBelow => Self::DEFINDEX_VOICES_FROM_BELOW,
            Spell::PumpkinBombs => Self::DEFINDEX_PUMPKIN_BOMBS,
            Spell::HalloweenFire => Self::DEFINDEX_HALLOWEEN_FIRE,
            Spell::Exorcism => Self::DEFINDEX_EXORCISM,
        }
    }
}

impl Attributes for Spell {
    const DEFINDEX: &'static [u32] = &[1004, 1005, 1006, 1007, 1008, 1009];
}

impl fmt::Display for Spell {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Spell::Paint(spell) => write!(f, "{}", spell),
            Spell::Footprints(spell) => write!(f, "{}", spell),
            Spell::VoicesFromBelow => write!(f, "Voices From Below"),
            Spell::PumpkinBombs => write!(f, "Pumpkin Bombs"),
            Spell::HalloweenFire => write!(f, "Halloween Fire"),
            Spell::Exorcism => write!(f, "Exorcism"),
        }
    }
}

impl std::str::FromStr for Spell {
    type Err = ::strum::ParseError;

    fn from_str(s: &str) -> Result<Spell, Self::Err> {
        match s {
            "Die Job" => Result::Ok(Spell::Paint(PaintSpell::DieJob)),
            "Chromatic Corruption" => Result::Ok(Spell::Paint(PaintSpell::ChromaticCorruption)),
            "Putrescent Pigmentation" => Result::Ok(Spell::Paint(PaintSpell::PutrescentPigmentation)),
            "Spectral Spectrum" => Result::Ok(Spell::Paint(PaintSpell::SpectralSpectrum)),
            "Sinister Staining" => Result::Ok(Spell::Paint(PaintSpell::SinisterStaining)),
            "Team Spirit Footprints" => Result::Ok(Spell::Footprints(FootprintsSpell::TeamSpiritFootprints)),
            "Gangreen Footprints" => Result::Ok(Spell::Footprints(FootprintsSpell::GangreenFootprints)),
            "Corpse Gray Footprints" => Result::Ok(Spell::Footprints(FootprintsSpell::CorpseGrayFootprints)),
            "Violent Violet Footprints" => Result::Ok(Spell::Footprints(FootprintsSpell::ViolentVioletFootprints)),
            "Rotten Orange Footprints" => Result::Ok(Spell::Footprints(FootprintsSpell::RottenOrangeFootprints)),
            "Bruised Purple Footprints" => Result::Ok(Spell::Footprints(FootprintsSpell::BruisedPurpleFootprints)),
            "Headless Horseshoes" => Result::Ok(Spell::Footprints(FootprintsSpell::HeadlessHorseshoes)),
            "Voices From Below" => Result::Ok(Spell::VoicesFromBelow),
            "Voices from Below" => Result::Ok(Spell::VoicesFromBelow),
            "Pumpkin Bombs" => Result::Ok(Spell::PumpkinBombs),
            "Halloween Fire" => Result::Ok(Spell::HalloweenFire),
            "Exorcism" => Result::Ok(Spell::Exorcism),
            _ => Result::Err(strum::ParseError::VariantNotFound),
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, EnumIter, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
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

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, EnumIter, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    
    #[test]
    fn from_str() {
        assert_eq!(Spell::from_str("Headless Horseshoes").unwrap(), Spell::Footprints(FootprintsSpell::HeadlessHorseshoes));
    }
    
    #[test]
    fn to_string() {
        assert_eq!(Spell::Footprints(FootprintsSpell::HeadlessHorseshoes).to_string(), "Headless Horseshoes");
    }
    
    #[test]
    fn from_repr() {
        assert_eq!(FootprintsSpell::try_from(2).unwrap(), FootprintsSpell::HeadlessHorseshoes);
    }
}
