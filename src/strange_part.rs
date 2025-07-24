use crate::Attributes;
use strum_macros::{Display, EnumString, EnumIter};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Strange part. `repr` values are mapped to their `kill_eater_score_type` attribute value.  
#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Display, EnumString, EnumIter, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum StrangePart {
    #[strum(serialize = "Scouts Killed")]
    ScoutsKilled = 10,
    #[strum(serialize = "Snipers Killed")]
    SnipersKilled = 11,
    #[strum(serialize = "Demomen Killed")]
    DemomenKilled = 13,
    #[strum(serialize = "Heavies Killed")]
    HeaviesKilled = 14,
    #[strum(serialize = "Pyros Killed")]
    PyrosKilled = 15,
    #[strum(serialize = "Spies Killed")]
    SpiesKilled = 16,
    #[strum(serialize = "Engineers Killed")]
    EngineersKilled = 17,
    #[strum(serialize = "Medics Killed")]
    MedicsKilled = 18,
    #[strum(serialize = "Buildings Destroyed")]
    BuildingsDestroyed = 19,
    #[strum(serialize = "Projectiles Reflected")]
    ProjectilesReflected = 20,
    #[strum(serialize = "Headshot Kills")]
    HeadshotKills = 21,
    #[strum(serialize = "Airborne Enemy Kills")]
    AirborneEnemyKills = 22,
    #[strum(serialize = "Gib Kills")]
    GibKills = 23,
    #[strum(serialize = "Kills Under A Full Moon")]
    KillsUnderAFullMoon = 27,
    #[strum(serialize = "Dominations")]
    Dominations = 28,
    #[strum(serialize = "Revenges")]
    Revenges = 30,
    #[strum(serialize = "Posthumous Kills")]
    PosthumousKills = 31,
    #[strum(serialize = "Teammates Extinguished")]
    TeammatesExtinguished = 32,
    #[strum(serialize = "Critical Kills")]
    CriticalKills = 33,
    #[strum(serialize = "Kills While Explosive-Jumping")]
    KillsWhileExplosiveJumping = 34,
    #[strum(serialize = "Sappers Removed")]
    SappersRemoved = 36,
    #[strum(serialize = "Cloaked Spies Killed")]
    CloakedSpiesKilled = 37,
    #[strum(serialize = "Medics Killed That Have Full ÜberCharge")]
    MedicsKilledThatHaveFullUberCharge = 38,
    #[strum(serialize = "Robots Destroyed")]
    RobotsDestroyed = 39,
    #[strum(serialize = "Defenders Killed")]
    DefendersKilled = 47,
    #[strum(serialize = "Submerged Enemy Kills")]
    SubmergedEnemyKills = 48,
    #[strum(serialize = "Kills While Invuln ÜberCharged")]
    KillsWhileInvulnUberCharged = 49,
    #[strum(serialize = "Tanks Destroyed")]
    TanksDestroyed = 61,
    #[strum(serialize = "Long-Distance Kills")]
    LongDistanceKills = 62,
    #[strum(serialize = "Kills during Victory Time")]
    KillsDuringVictoryTime = 67,
    #[strum(serialize = "Robot Spies Destroyed")]
    RobotSpiesDestroyed = 74,
    #[strum(serialize = "Taunt Kills")]
    TauntKills = 77,
    #[strum(serialize = "Unusual-Wearing Player Kills")]
    UnusualWearingPlayerKills = 78,
    #[strum(serialize = "Burning Player Kills")]
    BurningPlayerKills = 79,
    #[strum(serialize = "Killstreaks Ended")]
    KillstreaksEnded = 80,
    #[strum(serialize = "Freezecam Taunt Appearances")]
    FreezecamTauntAppearances = 81,
    #[strum(serialize = "Damage Dealt")]
    DamageDealt = 82,
    #[strum(serialize = "Fires Survived")]
    FiresSurvived = 83,
    #[strum(serialize = "Allied Healing Done")]
    AlliedHealingDone = 84,
    #[strum(serialize = "Point Blank Kills")]
    PointBlankKills = 85,
    #[strum(serialize = "Robots Killed During Halloween")]
    RobotsKilledDuringHalloween = 46,
    #[strum(serialize = "Kills During Halloween")]
    KillsDuringHalloween = 45,
    #[strum(serialize = "Kills While Low Health")]
    KillsWhileLowHealth = 44,
    #[strum(serialize = "Giant Robots Destroyed")]
    GiantRobotsDestroyed = 40,
    #[strum(serialize = "Kills")]
    Kills = 87,
    #[strum(serialize = "Full Health Kills")]
    FullHealthKills = 88,
    #[strum(serialize = "Soldiers Killed")]
    SoldiersKilled = 12,
    #[strum(serialize = "Robot Scouts Destroyed")]
    RobotScoutsDestroyed = 68,
    #[strum(serialize = "Taunting Player Kills")]
    TauntingPlayerKills = 89,
    #[strum(serialize = "Assists")]
    Assists = 95,
    #[strum(serialize = "Not Crit nor MiniCrit Kills")]
    NotCritNorMiniCritKills = 93,
    #[strum(serialize = "Players Hit")]
    PlayersHit = 94,
}

impl StrangePart {
    /// Gets the `kill_eater_score_type` attribute value for this [`StrangePart`].
    pub fn score_type(&self) -> u32 {
        u32::from(*self)
    }
    
    /// Converts a `kill_eater_score_type` attribute value into a [`StrangePart`].
    pub fn from_score_type(score_type: u32) -> Option<Self> {
        Self::try_from(score_type).ok()
    }
    
    /// Gets the `defindex` related to this [`StrangePart`].
    pub fn defindex(&self) -> u32 {
        match self {
            Self::ScoutsKilled => 6003,
            Self::SnipersKilled => 6005,
            Self::DemomenKilled => 6001,
            Self::HeaviesKilled => 6000,
            Self::PyrosKilled => 6006,
            Self::SpiesKilled => 6008,
            Self::EngineersKilled => 6004,
            Self::MedicsKilled => 6007,
            Self::BuildingsDestroyed => 6009,
            Self::ProjectilesReflected => 6010,
            Self::HeadshotKills => 6011,
            Self::AirborneEnemyKills => 6012,
            Self::GibKills => 6013,
            Self::KillsUnderAFullMoon => 6015,
            Self::Dominations => 6016,
            Self::Revenges => 6018,
            Self::PosthumousKills => 6019,
            Self::TeammatesExtinguished => 6020,
            Self::CriticalKills => 6021,
            Self::KillsWhileExplosiveJumping => 6022,
            Self::SappersRemoved => 6025,
            Self::CloakedSpiesKilled => 6024,
            Self::MedicsKilledThatHaveFullUberCharge => 6023,
            Self::RobotsDestroyed => 6026,
            Self::DefendersKilled => 6035,
            Self::SubmergedEnemyKills => 6036,
            Self::KillsWhileInvulnUberCharged => 6037,
            Self::TanksDestroyed => 6038,
            Self::LongDistanceKills => 6039,
            Self::KillsDuringVictoryTime => 6041,
            Self::RobotSpiesDestroyed => 6048,
            Self::TauntKills => 6051,
            Self::UnusualWearingPlayerKills => 6052,
            Self::BurningPlayerKills => 6053,
            Self::KillstreaksEnded => 6054,
            Self::FreezecamTauntAppearances => 6055,
            Self::DamageDealt => 6056,
            Self::FiresSurvived => 6057,
            Self::AlliedHealingDone => 6058,
            Self::PointBlankKills => 6059,
            Self::RobotsKilledDuringHalloween => 6034,
            Self::KillsDuringHalloween => 6033,
            Self::KillsWhileLowHealth => 6032,
            Self::GiantRobotsDestroyed => 6028,
            Self::Kills => 6060,
            Self::FullHealthKills => 6061,
            Self::SoldiersKilled => 6002,
            Self::RobotScoutsDestroyed => 6042,
            Self::TauntingPlayerKills => 6062,
            Self::Assists => 6065,
            Self::NotCritNorMiniCritKills => 6063,
            Self::PlayersHit => 6064,
        }
    }
    
    /// Converts a `defindex` into its related [`StrangePart`], if it exists.
    pub fn from_defindex(defindex: u32) -> Option<Self> {
        match defindex {
            6003 => Some(Self::ScoutsKilled),
            6005 => Some(Self::SnipersKilled),
            6001 => Some(Self::DemomenKilled),
            6000 => Some(Self::HeaviesKilled),
            6006 => Some(Self::PyrosKilled),
            6008 => Some(Self::SpiesKilled),
            6004 => Some(Self::EngineersKilled),
            6007 => Some(Self::MedicsKilled),
            6009 => Some(Self::BuildingsDestroyed),
            6010 => Some(Self::ProjectilesReflected),
            6011 => Some(Self::HeadshotKills),
            6012 => Some(Self::AirborneEnemyKills),
            6013 => Some(Self::GibKills),
            6015 => Some(Self::KillsUnderAFullMoon),
            6016 => Some(Self::Dominations),
            6018 => Some(Self::Revenges),
            6019 => Some(Self::PosthumousKills),
            6020 => Some(Self::TeammatesExtinguished),
            6021 => Some(Self::CriticalKills),
            6022 => Some(Self::KillsWhileExplosiveJumping),
            6025 => Some(Self::SappersRemoved),
            6024 => Some(Self::CloakedSpiesKilled),
            6023 => Some(Self::MedicsKilledThatHaveFullUberCharge),
            6026 => Some(Self::RobotsDestroyed),
            6035 => Some(Self::DefendersKilled),
            6036 => Some(Self::SubmergedEnemyKills),
            6037 => Some(Self::KillsWhileInvulnUberCharged),
            6038 => Some(Self::TanksDestroyed),
            6039 => Some(Self::LongDistanceKills),
            6041 => Some(Self::KillsDuringVictoryTime),
            6048 => Some(Self::RobotSpiesDestroyed),
            6051 => Some(Self::TauntKills),
            6052 => Some(Self::UnusualWearingPlayerKills),
            6053 => Some(Self::BurningPlayerKills),
            6054 => Some(Self::KillstreaksEnded),
            6055 => Some(Self::FreezecamTauntAppearances),
            6056 => Some(Self::DamageDealt),
            6057 => Some(Self::FiresSurvived),
            6058 => Some(Self::AlliedHealingDone),
            6059 => Some(Self::PointBlankKills),
            6034 => Some(Self::RobotsKilledDuringHalloween),
            6033 => Some(Self::KillsDuringHalloween),
            6032 => Some(Self::KillsWhileLowHealth),
            6028 => Some(Self::GiantRobotsDestroyed),
            6060 => Some(Self::Kills),
            6061 => Some(Self::FullHealthKills),
            6002 => Some(Self::SoldiersKilled),
            6042 => Some(Self::RobotScoutsDestroyed),
            6062 => Some(Self::TauntingPlayerKills),
            6065 => Some(Self::Assists),
            6063 => Some(Self::NotCritNorMiniCritKills),
            6064 => Some(Self::PlayersHit),
            _ => None,
        }
    }
}

impl Attributes for StrangePart {
    const DEFINDEX: &'static [u32] = &[380, 382, 384];
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    
    #[test]
    fn from_str_converts() {
        assert_eq!(
            StrangePart::MedicsKilledThatHaveFullUberCharge,
            StrangePart::from_str("Medics Killed That Have Full ÜberCharge").unwrap()
        );
    }
}