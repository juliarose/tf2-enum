use strum_macros::{Display, EnumString, EnumIter};

#[derive(Debug, Hash, Eq, PartialEq, Display, EnumString, EnumIter, Clone)]
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
    #[strum(serialize = "Spectral Spectrum")]
    SpectralSpectrum,
    #[strum(serialize = "Putrescent Pigmentation")]
    PutrescentPigmentation,
    #[strum(serialize = "Sinister Staining")]
    SinisterStaining,
    #[strum(serialize = "Chromatic Corruption")]
    ChromaticCorruption,
    #[strum(serialize = "Voices From Below")]
    VoicesFromBelow,
    #[strum(serialize = "Exorcism")]
    Exorcism,
    #[strum(serialize = "Halloween Fire")]
    HalloweenFire,
    #[strum(serialize = "Pumpkin Bombs")]
    PumpkinBombs,
}