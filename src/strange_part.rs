use crate::{
    Attributes,
    AttributeValue,
    AttributeDef,
    TryFromAttributeValueU32,
    KillEaterScoreType,
    HasItemDefindex,
    EffectType,
};
use crate::error::TryFromPrimitiveError;
use strum::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

// Avoid repeating strings
const STR_SCOUTS_KILLED: &str = "Strange Part: Scouts Killed";
const STR_SNIPERS_KILLED: &str = "Strange Part: Snipers Killed";
const STR_SOLDIERS_KILLED: &str = "Strange Part: Soldiers Killed";
const STR_DEMOMEN_KILLED: &str = "Strange Part: Demomen Killed";
const STR_HEAVIES_KILLED: &str = "Strange Part: Heavies Killed";
const STR_PYROS_KILLED: &str = "Strange Part: Pyros Killed";
const STR_SPIES_KILLED: &str = "Strange Part: Spies Killed";
const STR_ENGINEERS_KILLED: &str = "Strange Part: Engineers Killed";
const STR_MEDICS_KILLED: &str = "Strange Part: Medics Killed";
const STR_BUILDINGS_DESTROYED: &str = "Strange Part: Buildings Destroyed";
const STR_PROJECTILES_REFLECTED: &str = "Strange Part: Projectiles Reflected";
const STR_HEADSHOT_KILLS: &str = "Strange Part: Headshot Kills";
const STR_AIRBORNE_ENEMY_KILLS: &str = "Strange Part: Airborne Enemies Killed";
const STR_GIB_KILLS: &str = "Strange Part: Gib Kills";
const STR_KILLS_UNDER_A_FULL_MOON: &str = "Strange Part: Full Moon Kills";
const STR_DOMINATIONS: &str = "Strange Part: Domination Kills";
const STR_REVENGES: &str = "Strange Part: Revenge Kills";
const STR_POSTHUMOUS_KILLS: &str = "Strange Part: Posthumous Kills";
const STR_TEAMMATES_EXTINGUISHED: &str = "Strange Part: Teammates Extinguished";
const STR_CRITICAL_KILLS: &str = "Strange Part: Critical Kills";
const STR_KILLS_WHILE_EXPLOSIVE_JUMPING: &str = "Strange Part: Kills While Explosive Jumping";
const STR_SAPPERS_REMOVED: &str = "Strange Part: Sappers Destroyed";
const STR_CLOAKED_SPIES_KILLED: &str = "Strange Part: Cloaked Spies Killed";
const STR_MEDICS_KILLED_THAT_HAVE_FULL_UBER_CHARGE: &str = "Strange Part: Medics Killed That Have Full ÜberCharge";
const STR_ROBOTS_DESTROYED: &str = "Strange Part: Robots Destroyed";
const STR_GIANT_ROBOTS_DESTROYED: &str = "Strange Part: Giant Robots Destroyed";
const STR_KILLS_WHILE_LOW_HEALTH: &str = "Strange Part: Low-Health Kills";
const STR_KILLS_DURING_HALLOWEEN: &str = "Strange Part: Halloween Kills";
const STR_ROBOTS_KILLED_DURING_HALLOWEEN: &str = "Strange Part: Robots Destroyed During Halloween";
const STR_DEFENDER_KILLS: &str = "Strange Part: Defender Kills";
const STR_SUBMERGED_ENEMY_KILLS: &str = "Strange Part: Underwater Kills";
const STR_KILLS_WHILE_INVULN_UBER_CHARGED: &str = "Strange Part: Kills While Übercharged";
const STR_TANKS_DESTROYED: &str = "Strange Part: Tanks Destroyed";
const STR_LONG_DISTANCE_KILLS: &str = "Strange Part: Long-Distance Kills";
const STR_KILLS_DURING_VICTORY_TIME: &str = "Strange Part: Kills During Victory Time";
const STR_ROBOT_SCOUTS_DESTROYED: &str = "Strange Part: Robot Scouts Destroyed";
const STR_ROBOT_SPIES_DESTROYED: &str = "Strange Part: Robot Spies Destroyed";
const STR_TAUNT_KILLS: &str = "Strange Part: Kills with a Taunt Attack";
const STR_UNUSUAL_WEARING_PLAYER_KILLS: &str = "Strange Part: Unusual-Wearing Player Kills";
const STR_BURNING_PLAYER_KILLS: &str = "Strange Part: Burning Enemy Kills";
const STR_KILLSTREAKS_ENDED: &str = "Strange Part: Killstreaks Ended";
const STR_FREEZECAM_TAUNT_APPEARANCES: &str = "Strange Cosmetic Part: Freezecam Taunt Appearances";
const STR_DAMAGE_DEALT: &str = "Strange Part: Damage Dealt";
const STR_FIRES_SURVIVED: &str = "Strange Cosmetic Part: Fires Survived";
const STR_ALLIED_HEALING_DONE: &str = "Strange Part: Allied Healing Done";
const STR_POINT_BLANK_KILLS: &str = "Strange Part: Point-Blank Kills";
const STR_KILLS: &str = "Strange Cosmetic Part: Kills";
const STR_FULL_HEALTH_KILLS: &str = "Strange Part: Full Health Kills";
const STR_TAUNTING_PLAYER_KILLS: &str = "Strange Part: Taunting Player Kills";
const STR_NOT_CRIT_NOR_MINI_CRIT_KILLS: &str = "Strange Part: Not Crit nor MiniCrit Kills";
const STR_PLAYER_HITS: &str = "Strange Part: Player Hits";
const STR_ASSISTS: &str = "Strange Cosmetic Part: Assists";

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
#[allow(missing_docs)]
pub enum StrangePart {
    #[strum(serialize = "Scouts Killed")]
    ScoutsKilled = 10,
    #[strum(serialize = "Snipers Killed")]
    SnipersKilled = 11,
    #[strum(serialize = "Soldiers Killed")]
    SoldiersKilled = 12,
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
    #[strum(serialize = "Giant Robots Destroyed")]
    GiantRobotsDestroyed = 40,
    #[strum(serialize = "Kills While Low Health")]
    KillsWhileLowHealth = 44,
    #[strum(serialize = "Kills During Halloween")]
    KillsDuringHalloween = 45,
    #[strum(serialize = "Robots Killed During Halloween")]
    RobotsKilledDuringHalloween = 46,
    #[strum(serialize = "Defender Kills")]
    #[serde(alias = "Defenders Killed")]
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
    #[strum(serialize = "Robot Scouts Destroyed")]
    RobotScoutsDestroyed = 68,
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
    #[strum(serialize = "Kills")]
    Kills = 87,
    #[strum(serialize = "Full Health Kills")]
    FullHealthKills = 88,
    #[strum(serialize = "Taunting Player Kills")]
    TauntingPlayerKills = 89,
    #[strum(serialize = "Not Crit nor MiniCrit Kills")]
    NotCritNorMiniCritKills = 93,
    #[strum(serialize = "Player Hits")]
    #[serde(alias = "Players Hit")]
    PlayerHits = 94,
    #[strum(serialize = "Assists")]
    Assists = 95,
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
            Self::ScoutsKilled => STR_SCOUTS_KILLED,
            Self::SnipersKilled => STR_SNIPERS_KILLED,
            Self::SoldiersKilled => STR_SOLDIERS_KILLED,
            Self::DemomenKilled => STR_DEMOMEN_KILLED,
            Self::HeaviesKilled => STR_HEAVIES_KILLED,
            Self::PyrosKilled => STR_PYROS_KILLED,
            Self::SpiesKilled => STR_SPIES_KILLED,
            Self::EngineersKilled => STR_ENGINEERS_KILLED,
            Self::MedicsKilled => STR_MEDICS_KILLED,
            Self::BuildingsDestroyed => STR_BUILDINGS_DESTROYED,
            Self::ProjectilesReflected => STR_PROJECTILES_REFLECTED,
            Self::HeadshotKills => STR_HEADSHOT_KILLS,
            Self::AirborneEnemyKills => STR_AIRBORNE_ENEMY_KILLS,
            Self::GibKills => STR_GIB_KILLS,
            Self::KillsUnderAFullMoon => STR_KILLS_UNDER_A_FULL_MOON,
            Self::Dominations => STR_DOMINATIONS,
            Self::Revenges => STR_REVENGES,
            Self::PosthumousKills => STR_POSTHUMOUS_KILLS,
            Self::TeammatesExtinguished => STR_TEAMMATES_EXTINGUISHED,
            Self::CriticalKills => STR_CRITICAL_KILLS,
            Self::KillsWhileExplosiveJumping => STR_KILLS_WHILE_EXPLOSIVE_JUMPING,
            Self::SappersRemoved => STR_SAPPERS_REMOVED,
            Self::CloakedSpiesKilled => STR_CLOAKED_SPIES_KILLED,
            Self::MedicsKilledThatHaveFullUberCharge => STR_MEDICS_KILLED_THAT_HAVE_FULL_UBER_CHARGE,
            Self::RobotsDestroyed => STR_ROBOTS_DESTROYED,
            Self::GiantRobotsDestroyed => STR_GIANT_ROBOTS_DESTROYED,
            Self::KillsWhileLowHealth => STR_KILLS_WHILE_LOW_HEALTH,
            Self::KillsDuringHalloween => STR_KILLS_DURING_HALLOWEEN,
            Self::RobotsKilledDuringHalloween => STR_ROBOTS_KILLED_DURING_HALLOWEEN,
            Self::DefenderKills => STR_DEFENDER_KILLS,
            Self::SubmergedEnemyKills => STR_SUBMERGED_ENEMY_KILLS,
            Self::KillsWhileInvulnUberCharged => STR_KILLS_WHILE_INVULN_UBER_CHARGED,
            Self::TanksDestroyed => STR_TANKS_DESTROYED,
            Self::LongDistanceKills => STR_LONG_DISTANCE_KILLS,
            Self::KillsDuringVictoryTime => STR_KILLS_DURING_VICTORY_TIME,
            Self::RobotScoutsDestroyed => STR_ROBOT_SCOUTS_DESTROYED,
            Self::RobotSpiesDestroyed => STR_ROBOT_SPIES_DESTROYED,
            Self::TauntKills => STR_TAUNT_KILLS,
            Self::UnusualWearingPlayerKills => STR_UNUSUAL_WEARING_PLAYER_KILLS,
            Self::BurningPlayerKills => STR_BURNING_PLAYER_KILLS,
            Self::KillstreaksEnded => STR_KILLSTREAKS_ENDED,
            Self::FreezecamTauntAppearances => STR_FREEZECAM_TAUNT_APPEARANCES,
            Self::DamageDealt => STR_DAMAGE_DEALT,
            Self::FiresSurvived => STR_FIRES_SURVIVED,
            Self::AlliedHealingDone => STR_ALLIED_HEALING_DONE,
            Self::PointBlankKills => STR_POINT_BLANK_KILLS,
            Self::Kills => STR_KILLS,
            Self::FullHealthKills => STR_FULL_HEALTH_KILLS,
            Self::TauntingPlayerKills => STR_TAUNTING_PLAYER_KILLS,
            Self::NotCritNorMiniCritKills => STR_NOT_CRIT_NOR_MINI_CRIT_KILLS,
            Self::PlayerHits => STR_PLAYER_HITS,
            Self::Assists => STR_ASSISTS,
        }
    }
    
    /// Gets the related [`StrangePart`] by its strange part name, if it exists.
    pub fn from_strange_part_name(name: &str) -> Option<StrangePart> {
        match name {
            STR_SCOUTS_KILLED => Some(Self::ScoutsKilled),
            STR_SNIPERS_KILLED => Some(Self::SnipersKilled),
            STR_SOLDIERS_KILLED => Some(Self::SoldiersKilled),
            STR_DEMOMEN_KILLED => Some(Self::DemomenKilled),
            STR_HEAVIES_KILLED => Some(Self::HeaviesKilled),
            STR_PYROS_KILLED => Some(Self::PyrosKilled),
            STR_SPIES_KILLED => Some(Self::SpiesKilled),
            STR_ENGINEERS_KILLED => Some(Self::EngineersKilled),
            STR_MEDICS_KILLED => Some(Self::MedicsKilled),
            STR_BUILDINGS_DESTROYED => Some(Self::BuildingsDestroyed),
            STR_PROJECTILES_REFLECTED => Some(Self::ProjectilesReflected),
            STR_HEADSHOT_KILLS => Some(Self::HeadshotKills),
            STR_AIRBORNE_ENEMY_KILLS => Some(Self::AirborneEnemyKills),
            STR_GIB_KILLS => Some(Self::GibKills),
            STR_KILLS_UNDER_A_FULL_MOON => Some(Self::KillsUnderAFullMoon),
            STR_DOMINATIONS => Some(Self::Dominations),
            STR_REVENGES => Some(Self::Revenges),
            STR_POSTHUMOUS_KILLS => Some(Self::PosthumousKills),
            STR_TEAMMATES_EXTINGUISHED => Some(Self::TeammatesExtinguished),
            STR_CRITICAL_KILLS => Some(Self::CriticalKills),
            STR_KILLS_WHILE_EXPLOSIVE_JUMPING => Some(Self::KillsWhileExplosiveJumping),
            STR_SAPPERS_REMOVED => Some(Self::SappersRemoved),
            STR_CLOAKED_SPIES_KILLED => Some(Self::CloakedSpiesKilled),
            STR_MEDICS_KILLED_THAT_HAVE_FULL_UBER_CHARGE => Some(Self::MedicsKilledThatHaveFullUberCharge),
            STR_ROBOTS_DESTROYED => Some(Self::RobotsDestroyed),
            STR_GIANT_ROBOTS_DESTROYED => Some(Self::GiantRobotsDestroyed),
            STR_KILLS_WHILE_LOW_HEALTH => Some(Self::KillsWhileLowHealth),
            STR_KILLS_DURING_HALLOWEEN => Some(Self::KillsDuringHalloween),
            STR_ROBOTS_KILLED_DURING_HALLOWEEN => Some(Self::RobotsKilledDuringHalloween),
            STR_DEFENDER_KILLS => Some(Self::DefenderKills),
            STR_SUBMERGED_ENEMY_KILLS => Some(Self::SubmergedEnemyKills),
            STR_KILLS_WHILE_INVULN_UBER_CHARGED => Some(Self::KillsWhileInvulnUberCharged),
            STR_TANKS_DESTROYED => Some(Self::TanksDestroyed),
            STR_LONG_DISTANCE_KILLS => Some(Self::LongDistanceKills),
            STR_KILLS_DURING_VICTORY_TIME => Some(Self::KillsDuringVictoryTime),
            STR_ROBOT_SCOUTS_DESTROYED => Some(Self::RobotScoutsDestroyed),
            STR_ROBOT_SPIES_DESTROYED => Some(Self::RobotSpiesDestroyed),
            STR_TAUNT_KILLS => Some(Self::TauntKills),
            STR_UNUSUAL_WEARING_PLAYER_KILLS => Some(Self::UnusualWearingPlayerKills),
            STR_BURNING_PLAYER_KILLS => Some(Self::BurningPlayerKills),
            STR_KILLSTREAKS_ENDED => Some(Self::KillstreaksEnded),
            STR_FREEZECAM_TAUNT_APPEARANCES => Some(Self::FreezecamTauntAppearances),
            STR_DAMAGE_DEALT => Some(Self::DamageDealt),
            STR_FIRES_SURVIVED => Some(Self::FiresSurvived),
            STR_ALLIED_HEALING_DONE => Some(Self::AlliedHealingDone),
            STR_POINT_BLANK_KILLS => Some(Self::PointBlankKills),
            STR_KILLS => Some(Self::Kills),
            STR_FULL_HEALTH_KILLS => Some(Self::FullHealthKills),
            STR_TAUNTING_PLAYER_KILLS => Some(Self::TauntingPlayerKills),
            STR_NOT_CRIT_NOR_MINI_CRIT_KILLS => Some(Self::NotCritNorMiniCritKills),
            STR_PLAYER_HITS => Some(Self::PlayerHits),
            STR_ASSISTS => Some(Self::Assists),
            _ => None,
        }
    }
    
    /// Is this strange part for cosmetics?
    pub fn is_cosmetic_part(&self) -> bool {
        matches!(self, Self::Kills | Self::FullHealthKills | Self::Assists | Self::FreezecamTauntAppearances)
    }
    
    /// Converts this [`StrangePart`] into its related [`KillEaterScoreType`], if it exists.
    pub fn kill_eater_score_type(&self) -> Option<KillEaterScoreType> {
        KillEaterScoreType::try_from(*self).ok()
    }
}

/// Represents the "kill_eater_user_score_type_1", "kill_eater_user_score_type_2", and
/// "kill_eater_user_score_type_3", attributes.
impl Attributes for StrangePart {
    const DEFINDEX: &[u32] = &[
        380,
        382,
        384,
    ];
    const ATTRIBUTES: &'static [AttributeDef] = &[
        AttributeDef {
            defindex: 380,
            name: "kill eater user score type 1",
            attribute_class: Some("kill_eater_user_score_type_1"),
            description_string: None,
            description_format: None,
            effect_type: EffectType::Positive,
            hidden: true,
            stored_as_integer: false,
        },
        AttributeDef {
            defindex: 382,
            name: "kill eater user score type 2",
            attribute_class: Some("kill_eater_user_score_type_2"),
            description_string: None,
            description_format: None,
            effect_type: EffectType::Positive,
            hidden: true,
            stored_as_integer: false,
        },
        AttributeDef {
            defindex: 384,
            name: "kill eater user score type 3",
            attribute_class: Some("kill_eater_user_score_type_3"),
            description_string: None,
            description_format: None,
            effect_type: EffectType::Positive,
            hidden: true,
            stored_as_integer: false,
        },
    ];
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        Some((*self as u32) as f64)
    }
}

impl TryFromAttributeValueU32 for StrangePart {}

impl HasItemDefindex for StrangePart {
    /// Gets the `defindex` for the [`StrangePart`].
    fn defindex(&self) -> u32 {
        match self {
            Self::ScoutsKilled => 6003,
            Self::SnipersKilled => 6005,
            Self::SoldiersKilled => 6002,
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
            Self::GiantRobotsDestroyed => 6028,
            Self::KillsWhileLowHealth => 6032,
            Self::KillsDuringHalloween => 6033,
            Self::RobotsKilledDuringHalloween => 6034,
            Self::DefenderKills => 6035,
            Self::SubmergedEnemyKills => 6036,
            Self::KillsWhileInvulnUberCharged => 6037,
            Self::TanksDestroyed => 6038,
            Self::LongDistanceKills => 6039,
            Self::KillsDuringVictoryTime => 6041,
            Self::RobotScoutsDestroyed => 6042,
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
            Self::Kills => 6060,
            Self::FullHealthKills => 6061,
            Self::TauntingPlayerKills => 6062,
            Self::NotCritNorMiniCritKills => 6063,
            Self::PlayerHits => 6064,
            Self::Assists => 6065,
        }
    }
    
    /// Converts a `defindex` into its related [`StrangePart`], if it exists.
    fn from_defindex(defindex: u32) -> Option<Self> {
        match defindex {
            6003 => Some(Self::ScoutsKilled),
            6005 => Some(Self::SnipersKilled),
            6002 => Some(Self::SoldiersKilled),
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
            6028 => Some(Self::GiantRobotsDestroyed),
            6032 => Some(Self::KillsWhileLowHealth),
            6033 => Some(Self::KillsDuringHalloween),
            6034 => Some(Self::RobotsKilledDuringHalloween),
            6035 => Some(Self::DefenderKills),
            6036 => Some(Self::SubmergedEnemyKills),
            6037 => Some(Self::KillsWhileInvulnUberCharged),
            6038 => Some(Self::TanksDestroyed),
            6039 => Some(Self::LongDistanceKills),
            6041 => Some(Self::KillsDuringVictoryTime),
            6042 => Some(Self::RobotScoutsDestroyed),
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
            6060 => Some(Self::Kills),
            6061 => Some(Self::FullHealthKills),
            6062 => Some(Self::TauntingPlayerKills),
            6063 => Some(Self::NotCritNorMiniCritKills),
            6064 => Some(Self::PlayerHits),
            6065 => Some(Self::Assists),
            _ => None,
        }
    }
}

impl TryFrom<KillEaterScoreType> for StrangePart {
    type Error = TryFromPrimitiveError<Self>;
    
    fn try_from(part: KillEaterScoreType) -> Result<Self, Self::Error> {
        StrangePart::try_from(part as u32)
    }
}

impl TryFrom<&KillEaterScoreType> for StrangePart {
    type Error = TryFromPrimitiveError<Self>;
    
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
    
    #[test]
    fn attribute_slices_are_equal_length() {
        assert_eq!(StrangePart::DEFINDEX.len(), StrangePart::ATTRIBUTES.len());
    }
}
