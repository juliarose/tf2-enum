use strum_macros::{Display, EnumString};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum Wear {
    #[strum(serialize = "Factory New")]
    FactoryNew = 1,
    #[strum(serialize = "Minimal Wear")]
    MinimalWear = 2,
    #[strum(serialize = "Field-Tested")]
    FieldTested = 3,
    #[strum(serialize = "Well-Worn")]
    WellWorn = 4,
    #[strum(serialize = "Battle Scarred")]
    BattleScarred = 5
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum KillstreakTier {
    #[strum(serialize = "Professional Killstreak")]
    Professional = 3,
    #[strum(serialize = "Specialized Killstreak")]
    Specialized = 2,
    #[strum(serialize = "Killstreak")]
    Basic = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum Quality {
    Normal = 0,
    Genuine = 1,
    Rarity2 = 2,
    Vintage = 3,
    Rarity4 = 4,
    Unusual = 5,
    Unique = 6,
    Community = 7,
    Valve = 8,
    #[strum(serialize = "Self-Made")]
    SelfMade = 9,
    Customized = 10,
    Strange = 11,
    Completed = 12,
    Haunted = 13,
    #[strum(serialize = "Collector's")]
    Collectors = 14,
    #[strum(serialize = "Decorated Weapon")]
    DecoratedWeapon = 15,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum Sheen {
    #[strum(serialize = "Team Shine")]
    TeamShine = 1,
    #[strum(serialize = "Deadly Daffodil")]
    DeadlyDaffodil = 2,
    #[strum(serialize = "Manndarin")]
    Manndarin = 3,
    #[strum(serialize = "Mean Green")]
    MeanGreen = 4,
    #[strum(serialize = "Agonizing Emerald")]
    AgonizingEmerald = 5,
    #[strum(serialize = "Villainous Violet")]
    VillainousViolet = 6,
    #[strum(serialize = "Hot Rod")]
    HotRod = 7,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u16)]
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

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum Rarity {
    Civilian = 1,
    Freelance = 2,
    Mercenary = 3,
    Commando = 4,
    Assassin = 5,
    Elite = 6,
}

#[derive(Debug, Hash, Eq, PartialEq, Display, EnumString, Clone)]
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

#[derive(Debug, Hash, Eq, PartialEq, Display, EnumString, Clone)]
pub enum Class {
    Scout,
    Soldier,
    Pyro,
    Demoman,
    Heavy,
    Engineer,
    Medic,
    Sniper,
    Spy,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::str::FromStr;

    #[test]
    fn converts_quality_to_primitive() {
        let quality = Quality::Strange;
        let quality_num: u8 = quality.into();
        
        assert_eq!(11, quality_num);
    }
    
    #[test]
    fn converts_string_to_quality() {
        let quality = Quality::from_str("Decorated Weapon").unwrap();
        
        assert_eq!(Quality::DecoratedWeapon, quality);
    }
    
    #[test]
    fn displays_quality() {
        let quality = Quality::Collectors;
        
        assert_eq!("Collector's", &format!("{}", quality));
    }
}