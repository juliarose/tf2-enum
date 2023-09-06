use crate::Attributes;
use strum_macros::{Display, EnumString, EnumIter};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

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
    pub fn score_type(&self) -> u32 {
        u32::from(*self)
    }
    
    pub fn from_score_type(score_type: u32) -> Option<Self> {
        Self::try_from(score_type).ok()
    }
    
    pub fn defindex(&self) -> u32 {
        match self {
            StrangePart::ScoutsKilled => 6003,
            StrangePart::SnipersKilled => 6005,
            StrangePart::DemomenKilled => 6001,
            StrangePart::HeaviesKilled => 6000,
            StrangePart::PyrosKilled => 6006,
            StrangePart::SpiesKilled => 6008,
            StrangePart::EngineersKilled => 6004,
            StrangePart::MedicsKilled => 6007,
            StrangePart::BuildingsDestroyed => 6009,
            StrangePart::ProjectilesReflected => 6010,
            StrangePart::HeadshotKills => 6011,
            StrangePart::AirborneEnemyKills => 6012,
            StrangePart::GibKills => 6013,
            StrangePart::KillsUnderAFullMoon => 6015,
            StrangePart::Dominations => 6016,
            StrangePart::Revenges => 6018,
            StrangePart::PosthumousKills => 6019,
            StrangePart::TeammatesExtinguished => 6020,
            StrangePart::CriticalKills => 6021,
            StrangePart::KillsWhileExplosiveJumping => 6022,
            StrangePart::SappersRemoved => 6025,
            StrangePart::CloakedSpiesKilled => 6024,
            StrangePart::MedicsKilledThatHaveFullUberCharge => 6023,
            StrangePart::RobotsDestroyed => 6026,
            StrangePart::DefendersKilled => 6035,
            StrangePart::SubmergedEnemyKills => 6036,
            StrangePart::KillsWhileInvulnUberCharged => 6037,
            StrangePart::TanksDestroyed => 6038,
            StrangePart::LongDistanceKills => 6039,
            StrangePart::KillsDuringVictoryTime => 6041,
            StrangePart::RobotSpiesDestroyed => 6048,
            StrangePart::TauntKills => 6051,
            StrangePart::UnusualWearingPlayerKills => 6052,
            StrangePart::BurningPlayerKills => 6053,
            StrangePart::KillstreaksEnded => 6054,
            StrangePart::FreezecamTauntAppearances => 6055,
            StrangePart::DamageDealt => 6056,
            StrangePart::FiresSurvived => 6057,
            StrangePart::AlliedHealingDone => 6058,
            StrangePart::PointBlankKills => 6059,
            StrangePart::RobotsKilledDuringHalloween => 6034,
            StrangePart::KillsDuringHalloween => 6033,
            StrangePart::KillsWhileLowHealth => 6032,
            StrangePart::GiantRobotsDestroyed => 6028,
            StrangePart::Kills => 6060,
            StrangePart::FullHealthKills => 6061,
            StrangePart::SoldiersKilled => 6002,
            StrangePart::RobotScoutsDestroyed => 6042,
            StrangePart::TauntingPlayerKills => 6062,
            StrangePart::Assists => 6065,
            StrangePart::NotCritNorMiniCritKills => 6063,
            StrangePart::PlayersHit => 6064,
        }
    }
    
    pub fn from_defindex(defindex: u32) -> Option<Self> {
        match defindex {
            6003 => Some(StrangePart::ScoutsKilled),
            6005 => Some(StrangePart::SnipersKilled),
            6001 => Some(StrangePart::DemomenKilled),
            6000 => Some(StrangePart::HeaviesKilled),
            6006 => Some(StrangePart::PyrosKilled),
            6008 => Some(StrangePart::SpiesKilled),
            6004 => Some(StrangePart::EngineersKilled),
            6007 => Some(StrangePart::MedicsKilled),
            6009 => Some(StrangePart::BuildingsDestroyed),
            6010 => Some(StrangePart::ProjectilesReflected),
            6011 => Some(StrangePart::HeadshotKills),
            6012 => Some(StrangePart::AirborneEnemyKills),
            6013 => Some(StrangePart::GibKills),
            6015 => Some(StrangePart::KillsUnderAFullMoon),
            6016 => Some(StrangePart::Dominations),
            6018 => Some(StrangePart::Revenges),
            6019 => Some(StrangePart::PosthumousKills),
            6020 => Some(StrangePart::TeammatesExtinguished),
            6021 => Some(StrangePart::CriticalKills),
            6022 => Some(StrangePart::KillsWhileExplosiveJumping),
            6025 => Some(StrangePart::SappersRemoved),
            6024 => Some(StrangePart::CloakedSpiesKilled),
            6023 => Some(StrangePart::MedicsKilledThatHaveFullUberCharge),
            6026 => Some(StrangePart::RobotsDestroyed),
            6035 => Some(StrangePart::DefendersKilled),
            6036 => Some(StrangePart::SubmergedEnemyKills),
            6037 => Some(StrangePart::KillsWhileInvulnUberCharged),
            6038 => Some(StrangePart::TanksDestroyed),
            6039 => Some(StrangePart::LongDistanceKills),
            6041 => Some(StrangePart::KillsDuringVictoryTime),
            6048 => Some(StrangePart::RobotSpiesDestroyed),
            6051 => Some(StrangePart::TauntKills),
            6052 => Some(StrangePart::UnusualWearingPlayerKills),
            6053 => Some(StrangePart::BurningPlayerKills),
            6054 => Some(StrangePart::KillstreaksEnded),
            6055 => Some(StrangePart::FreezecamTauntAppearances),
            6056 => Some(StrangePart::DamageDealt),
            6057 => Some(StrangePart::FiresSurvived),
            6058 => Some(StrangePart::AlliedHealingDone),
            6059 => Some(StrangePart::PointBlankKills),
            6034 => Some(StrangePart::RobotsKilledDuringHalloween),
            6033 => Some(StrangePart::KillsDuringHalloween),
            6032 => Some(StrangePart::KillsWhileLowHealth),
            6028 => Some(StrangePart::GiantRobotsDestroyed),
            6060 => Some(StrangePart::Kills),
            6061 => Some(StrangePart::FullHealthKills),
            6002 => Some(StrangePart::SoldiersKilled),
            6042 => Some(StrangePart::RobotScoutsDestroyed),
            6062 => Some(StrangePart::TauntingPlayerKills),
            6065 => Some(StrangePart::Assists),
            6063 => Some(StrangePart::NotCritNorMiniCritKills),
            6064 => Some(StrangePart::PlayersHit),
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