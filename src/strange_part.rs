use crate::{KillEaterScoreType, Attributes};
use strum_macros::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Strange part. `repr` values are mapped to their `kill_eater_score_type` attribute value. Strings
/// are the name of the `kill_eater_score_type`, **not** the name of the strange part.
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
    #[strum(serialize = "Defender Kills")]
    DefenderKills = 47,
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
    #[strum(serialize = "Player Hits")]
    PlayerHits = 94,
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
    
    /// Gets the name of the strange part for this [`StrangePart`].
    pub fn strange_part_name(&self) -> &'static str {
        match self {
            Self::ScoutsKilled => "Strange Part: Scouts Killed",
            Self::SnipersKilled => "Strange Part: Snipers Killed",
            Self::DemomenKilled => "Strange Part: Demomen Killed",
            Self::HeaviesKilled => "Strange Part: Heavies Killed",
            Self::PyrosKilled => "Strange Part: Pyros Killed",
            Self::SpiesKilled => "Strange Part: Spies Killed",
            Self::EngineersKilled => "Strange Part: Engineers Killed",
            Self::MedicsKilled => "Strange Part: Medics Killed",
            Self::BuildingsDestroyed => "Strange Part: Buildings Destroyed",
            Self::ProjectilesReflected => "Strange Part: Projectiles Reflected",
            Self::HeadshotKills => "Strange Part: Headshot Kills",
            Self::AirborneEnemyKills => "Strange Part: Airborne Enemies Killed",
            Self::GibKills => "Strange Part: Gib Kills",
            Self::KillsUnderAFullMoon => "Strange Part: Full Moon Kills",
            Self::Dominations => "Strange Part: Domination Kills",
            Self::Revenges => "Strange Part: Revenge Kills",
            Self::PosthumousKills => "Strange Part: Posthumous Kills",
            Self::TeammatesExtinguished => "Strange Part: Teammates Extinguished",
            Self::CriticalKills => "Strange Part: Critical Kills",
            Self::KillsWhileExplosiveJumping => "Strange Part: Kills While Explosive Jumping",
            Self::SappersRemoved => "Strange Part: Sappers Destroyed",
            Self::CloakedSpiesKilled => "Strange Part: Cloaked Spies Killed",
            Self::MedicsKilledThatHaveFullUberCharge => "Strange Part: Medics Killed That Have Full ÜberCharge",
            Self::RobotsDestroyed => "Strange Part: Robots Destroyed",
            Self::DefenderKills => "Strange Part: Defender Kills",
            Self::SubmergedEnemyKills => "Strange Part: Underwater Kills",
            Self::KillsWhileInvulnUberCharged => "Strange Part: Kills While Übercharged",
            Self::TanksDestroyed => "Strange Part: Tanks Destroyed",
            Self::LongDistanceKills => "Strange Part: Long-Distance Kills",
            Self::KillsDuringVictoryTime => "Strange Part: Kills During Victory Time",
            Self::RobotSpiesDestroyed => "Strange Part: Robot Spies Destroyed",
            Self::TauntKills => "Strange Part: Kills with a Taunt Attack",
            Self::UnusualWearingPlayerKills => "Strange Part: Unusual-Wearing Player Kills",
            Self::BurningPlayerKills => "Strange Part: Burning Enemy Kills",
            Self::KillstreaksEnded => "Strange Part: Killstreaks Ended",
            Self::FreezecamTauntAppearances => "Strange Cosmetic Part: Freezecam Taunt Appearances",
            Self::DamageDealt => "Strange Part: Damage Dealt",
            Self::FiresSurvived => "Strange Cosmetic Part: Fires Survived",
            Self::AlliedHealingDone => "Strange Part: Allied Healing Done",
            Self::PointBlankKills => "Strange Part: Point-Blank Kills",
            Self::RobotsKilledDuringHalloween => "Strange Part: Robots Destroyed During Halloween",
            Self::KillsDuringHalloween => "Strange Part: Halloween Kills",
            Self::KillsWhileLowHealth => "Strange Part: Low-Health Kills",
            Self::GiantRobotsDestroyed => "Strange Part: Giant Robots Destroyed",
            Self::Kills => "Strange Cosmetic Part: Kills",
            Self::FullHealthKills => "Strange Part: Full Health Kills",
            Self::SoldiersKilled => "Strange Part: Soldiers Killed",
            Self::RobotScoutsDestroyed => "Strange Part: Robot Scouts Destroyed",
            Self::TauntingPlayerKills => "Strange Part: Taunting Player Kills",
            Self::Assists => "Strange Cosmetic Part: Assists",
            Self::NotCritNorMiniCritKills => "Strange Part: Not Crit nor MiniCrit Kills",
            Self::PlayerHits => "Strange Part: Player Hits",
        }
    }
    
    /// Gets the related [`StrangePart`] by its strange part name, if it exists.
    pub fn from_strange_part_name(name: &str) -> Option<StrangePart> {
        match name {
            "Strange Part: Scouts Killed" => Some(Self::ScoutsKilled),
            "Strange Part: Snipers Killed" => Some(Self::SnipersKilled),
            "Strange Part: Demomen Killed" => Some(Self::DemomenKilled),
            "Strange Part: Heavies Killed" => Some(Self::HeaviesKilled),
            "Strange Part: Pyros Killed" => Some(Self::PyrosKilled),
            "Strange Part: Spies Killed" => Some(Self::SpiesKilled),
            "Strange Part: Engineers Killed" => Some(Self::EngineersKilled),
            "Strange Part: Medics Killed" => Some(Self::MedicsKilled),
            "Strange Part: Buildings Destroyed" => Some(Self::BuildingsDestroyed),
            "Strange Part: Projectiles Reflected" => Some(Self::ProjectilesReflected),
            "Strange Part: Headshot Kills" => Some(Self::HeadshotKills),
            "Strange Part: Airborne Enemies Killed" => Some(Self::AirborneEnemyKills),
            "Strange Part: Gib Kills" => Some(Self::GibKills),
            "Strange Part: Full Moon Kills" => Some(Self::KillsUnderAFullMoon),
            "Strange Part: Domination Kills" => Some(Self::Dominations),
            "Strange Part: Revenge Kills" => Some(Self::Revenges),
            "Strange Part: Posthumous Kills" => Some(Self::PosthumousKills),
            "Strange Part: Teammates Extinguished" => Some(Self::TeammatesExtinguished),
            "Strange Part: Critical Kills" => Some(Self::CriticalKills),
            "Strange Part: Kills While Explosive Jumping" => Some(Self::KillsWhileExplosiveJumping),
            "Strange Part: Sappers Destroyed" => Some(Self::SappersRemoved),
            "Strange Part: Cloaked Spies Killed" => Some(Self::CloakedSpiesKilled),
            "Strange Part: Medics Killed That Have Full ÜberCharge" => Some(Self::MedicsKilledThatHaveFullUberCharge),
            "Strange Part: Robots Destroyed" => Some(Self::RobotsDestroyed),
            "Strange Part: Defender Kills" => Some(Self::DefenderKills),
            "Strange Part: Underwater Kills" => Some(Self::SubmergedEnemyKills),
            "Strange Part: Kills While Übercharged" => Some(Self::KillsWhileInvulnUberCharged),
            "Strange Part: Tanks Destroyed" => Some(Self::TanksDestroyed),
            "Strange Part: Long-Distance Kills" => Some(Self::LongDistanceKills),
            "Strange Part: Kills During Victory Time" => Some(Self::KillsDuringVictoryTime),
            "Strange Part: Robot Spies Destroyed" => Some(Self::RobotSpiesDestroyed),
            "Strange Part: Kills with a Taunt Attack" => Some(Self::TauntKills),
            "Strange Part: Unusual-Wearing Player Kills" => Some(Self::UnusualWearingPlayerKills),
            "Strange Part: Burning Enemy Kills" => Some(Self::BurningPlayerKills),
            "Strange Part: Killstreaks Ended" => Some(Self::KillstreaksEnded),
            "Strange Cosmetic Part: Freezecam Taunt Appearances" => Some(Self::FreezecamTauntAppearances),
            "Strange Part: Damage Dealt" => Some(Self::DamageDealt),
            "Strange Cosmetic Part: Fires Survived" => Some(Self::FiresSurvived),
            "Strange Part: Allied Healing Done" => Some(Self::AlliedHealingDone),
            "Strange Part: Point-Blank Kills" => Some(Self::PointBlankKills),
            "Strange Part: Robots Destroyed During Halloween" => Some(Self::RobotsKilledDuringHalloween),
            "Strange Part: Halloween Kills" => Some(Self::KillsDuringHalloween),
            "Strange Part: Low-Health Kills" => Some(Self::KillsWhileLowHealth),
            "Strange Part: Giant Robots Destroyed" => Some(Self::GiantRobotsDestroyed),
            "Strange Cosmetic Part: Kills" => Some(Self::Kills),
            "Strange Part: Full Health Kills" => Some(Self::FullHealthKills),
            "Strange Part: Soldiers Killed" => Some(Self::SoldiersKilled),
            "Strange Part: Robot Scouts Destroyed" => Some(Self::RobotScoutsDestroyed),
            "Strange Part: Taunting Player Kills" => Some(Self::TauntingPlayerKills),
            "Strange Cosmetic Part: Assists" => Some(Self::Assists),
            "Strange Part: Not Crit nor MiniCrit Kills" => Some(Self::NotCritNorMiniCritKills),
            "Strange Part: Player Hits" => Some(Self::PlayerHits),
            _ => None,
        }
    }
    
    /// Is this strange part for cosmetics?
    pub fn is_cosmetic_part(&self) -> bool {
        matches!(self, Self::Kills | Self::FullHealthKills | Self::Assists | Self::FreezecamTauntAppearances)
    }
    
    /// Gets the `defindex` for the [`StrangePart`].
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
            Self::DefenderKills => 6035,
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
            Self::PlayerHits => 6064,
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
            6035 => Some(Self::DefenderKills),
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
            6064 => Some(Self::PlayerHits),
            _ => None,
        }
    }
    
    /// Checks if the defindex belongs to a strange part.
    pub fn defindex_is_strange_part(defindex: u32) -> bool {
        Self::from_defindex(defindex).is_some()
    }
    
    /// Converts this [`StrangePart`] into its related [`KillEaterScoreType`], if it exists.
    pub fn kill_eater_score_type(&self) -> Option<KillEaterScoreType> {
        KillEaterScoreType::try_from(*self).ok()
    }
}

impl Attributes for StrangePart {
    const DEFINDEX: &'static [u32] = &[380, 382, 384];
}

impl TryFrom<KillEaterScoreType> for StrangePart {
    type Error = num_enum::TryFromPrimitiveError<StrangePart>;
    
    fn try_from(part: KillEaterScoreType) -> Result<Self, Self::Error> {
        StrangePart::try_from(part as u32)
    }
}

impl TryFrom<&KillEaterScoreType> for StrangePart {
    type Error = num_enum::TryFromPrimitiveError<StrangePart>;
    
    fn try_from(part: &KillEaterScoreType) -> Result<Self, Self::Error> {
        StrangePart::try_from(*part as u32)
    }
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
    
    #[test]
    fn serializes_player_hits_correctly() {
        let player_hits = StrangePart::PlayerHits;
        let serialized = player_hits.to_string();

        assert_eq!(serialized, "Player Hits");
        assert_eq!(StrangePart::from_str("Player Hits").unwrap(), StrangePart::PlayerHits);
    }
    
    #[test]
    fn serializes_defender_kills_correctly() {
        let defender_kills = StrangePart::DefenderKills;
        let serialized = defender_kills.to_string();

        assert_eq!(serialized, "Defender Kills");
        assert_eq!(StrangePart::from_str("Defender Kills").unwrap(), StrangePart::DefenderKills);
    }
}
