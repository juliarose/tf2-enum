use crate::{
    Attributes,
    AttributeDef,
    EffectType,
    ItemLevel,
    StrangePart,
    TryFromIntAttributeValue,
};
use crate::error::TryFromPrimitiveError;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::{Display, EnumCount, EnumIter};

/// Kill eater score type. Conversion from strings is not supported due to multiple variants
/// having the same string representation. They can still be formatted into strings.
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
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
    Clone,
    Copy,
)]
#[repr(u32)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum KillEaterScoreType {
    /// "Kills" used for most Strange weapons.
    #[strum(serialize = "Kills")]
    Kills = 0,
    // Allow conversion from both versions, but serialize as "Übers".
    #[strum(serialize = "Ubers", serialize = "Übers")]
    Ubers = 1,
    #[strum(serialize = "Kill Assists")]
    KillAssists = 2,
    #[strum(serialize = "Sentry Kills")]
    SentryKills = 3,
    #[strum(serialize = "Sodden Victims")]
    SoddenVictims = 4,
    #[strum(serialize = "Spies Shocked")]
    SpiesShocked = 5,
    #[strum(serialize = "Heads Taken")]
    HeadsTaken = 6,
    #[strum(serialize = "Humiliations")]
    Humiliations = 7,
    #[strum(serialize = "Gifts Given")]
    GiftsGiven = 8,
    #[strum(serialize = "Deaths Feigned")]
    DeathsFeigned = 9,
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
    #[strum(serialize = "Buildings Sapped")]
    BuildingsSapped = 24,
    #[strum(serialize = "Tickle Fights Won")]
    TickleFightsWon = 25,
    #[strum(serialize = "Opponents Flattened")]
    OpponentsFlattened = 26,
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
    DefenderKills = 47,
    #[strum(serialize = "Submerged Enemy Kills")]
    SubmergedEnemyKills = 48,
    #[strum(serialize = "Kills While Invuln ÜberCharged")]
    KillsWhileInvulnUberCharged = 49,
    #[strum(serialize = "Food Items Eaten")]
    FoodItemsEaten = 50,
    #[strum(serialize = "Banners Deployed")]
    BannersDeployed = 51,
    #[strum(serialize = "Seconds Cloaked")]
    SecondsCloaked = 58,
    #[strum(serialize = "Health Dispensed to Teammates")]
    HealthDispensedToTeammates = 59,
    #[strum(serialize = "Teammates Teleported")]
    TeammatesTeleported = 60,
    #[strum(serialize = "Tanks Destroyed")]
    TanksDestroyed = 61,
    #[strum(serialize = "Long-Distance Kills")]
    LongDistanceKills = 62,
    #[strum(serialize = "KillEaterEvent_UniquePlayerKills")]
    KillEaterEventUniquePlayerKills = 63,
    #[strum(serialize = "Points Scored")]
    PointsScored = 64,
    #[strum(serialize = "Double Donks")]
    DoubleDonks = 65,
    #[strum(serialize = "Teammates Whipped")]
    TeammatesWhipped = 66,
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
    #[strum(serialize = "Wrangled Sentry Kills")]
    WrangledSentryKills = 86,
    /// "Kills" used for cosmetic items.
    #[strum(serialize = "Kills")]
    CosmeticKills = 87,
    #[strum(serialize = "Full Health Kills")]
    FullHealthKills = 88,
    #[strum(serialize = "Taunting Player Kills")]
    TauntingPlayerKills = 89,
    #[strum(serialize = "Carnival Kills")]
    CarnivalKills = 90,
    #[strum(serialize = "Carnival Underworld Kills")]
    CarnivalUnderworldKills = 91,
    #[strum(serialize = "Carnival Games Won")]
    CarnivalGamesWon = 92,
    #[strum(serialize = "Not Crit nor MiniCrit Kills")]
    NotCritNorMiniCritKills = 93,
    #[strum(serialize = "Player Hits")]
    PlayerHits = 94,
    #[strum(serialize = "Assists")]
    Assists = 95,
    #[strum(serialize = "Contracts Completed")]
    ContractsCompleted = 96,
    /// "Kills" used for contracts.
    #[strum(serialize = "Kills")]
    ContractKills = 97,
    #[strum(serialize = "Contract Points")]
    ContractPoints = 98,
    #[strum(serialize = "Contract Bonus Points")]
    ContractBonusPoints = 99,
    #[strum(serialize = "Times Performed")]
    TimesPerformed = 100,
    #[strum(serialize = "Kills and Assists during Invasion Event")]
    KillsAndAssistsDuringInvasionEvent = 101,
    #[strum(serialize = "Kills and Assists on 2Fort Invasion")]
    KillsAndAssistsOn2FortInvasion = 102,
    #[strum(serialize = "Kills and Assists on Probed")]
    KillsAndAssistsOnProbed = 103,
    #[strum(serialize = "Kills and Assists on Byre")]
    KillsAndAssistsOnByre = 104,
    #[strum(serialize = "Kills and Assists on Watergate")]
    KillsAndAssistsOnWatergate = 105,
    #[strum(serialize = "Souls Collected")]
    SoulsCollected = 106,
    #[strum(serialize = "Merasmissions Completed")]
    MerasmissionsCompleted = 107,
    #[strum(serialize = "Halloween Transmutes Performed")]
    HalloweenTransmutesPerformed = 108,
    #[strum(serialize = "Power Up Canteens Used")]
    PowerUpCanteensUsed = 109,
    #[strum(serialize = "Contract Points Earned")]
    ContractPointsEarned = 110,
    #[strum(serialize = "Contract Points Contributed To Friends")]
    ContractPointsContributedToFriends = 111,
}

impl KillEaterScoreType {
    /// The defindex for the kill eater score type.
    pub const DEFINDEX_KILL_EATER_SCORE_TYPE: u32 = 292;
    /// The defindex for the kill eater score type 2.
    pub const DEFINDEX_KILL_EATER_SCORE_TYPE_2: u32 = 293;
    /// The defindex for the kill eater score type 3.
    pub const DEFINDEX_KILL_EATER_SCORE_TYPE_3: u32 = 495;
    /// The defindex for the kill eater user score type 1.
    pub const DEFINDEX_KILL_EATER_USER_SCORE_TYPE_1: u32 = 380;
    /// The defindex for the kill eater user score type 2.
    pub const DEFINDEX_KILL_EATER_USER_SCORE_TYPE_2: u32 = 382;
    /// The defindex for the kill eater user score type 3.
    pub const DEFINDEX_KILL_EATER_USER_SCORE_TYPE_3: u32 = 384;
    /// The defindex values for the kill eater score types.
    pub const DEFINDEX_SCORE_TYPES: &'static [u32] = &[
        292,
        293,
        495,
    ];
    /// The defindex values for the kill eater user score types.
    pub const DEFINDEX_USER_SCORE_TYPES: &'static [u32] = &[
        380,
        382,
        384,
    ];
    /// Converts this [`KillEaterScoreType`] into its related [`StrangePart`], if it exists.
    pub fn strange_part(&self) -> Option<StrangePart> {
        StrangePart::try_from(self).ok()
    }
    
    /// Converts this [`KillEaterScoreType`] into its related [`ItemLevel`].
    pub fn item_level(&self) -> ItemLevel {
        self.into()
    }
}

impl Attributes for KillEaterScoreType {
    const DEFINDEX: &'static [u32] = &[
        292,
        293,
        495,
        380,
        382,
        384,
    ];
    const USES_FLOAT_VALUE: bool = true;
    /// Represents the "kill_eater_score_type", "kill_eater_score_type_2",
    /// "kill_eater_score_type_3", "kill_eater_user_score_type_1", "kill_eater_user_score_type_2",
    /// and "kill_eater_user_score_type_3", attributes.
    const ATTRIBUTES: &'static [AttributeDef] = &[
        AttributeDef {
            defindex: 292,
            name: "kill eater score type",
            attribute_class: Some("kill_eater_score_type"),
            description_string: None,
            description_format: None,
            effect_type: EffectType::Positive,
            hidden: true,
            stored_as_integer: false,
        },
        AttributeDef {
            defindex: 293,
            name: "kill eater score type 2",
            attribute_class: Some("kill_eater_score_type_2"),
            description_string: None,
            description_format: None,
            effect_type: EffectType::Positive,
            hidden: true,
            stored_as_integer: false,
        },
        AttributeDef {
            defindex: 495,
            name: "kill eater score type 3",
            attribute_class: Some("kill_eater_score_type_3"),
            description_string: None,
            description_format: None,
            effect_type: EffectType::Positive,
            hidden: true,
            stored_as_integer: false,
        },
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
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f32> {
        Some((*self as u32) as f32)
    }
}

impl TryFromIntAttributeValue for KillEaterScoreType {}

impl TryFrom<StrangePart> for KillEaterScoreType {
    type Error = TryFromPrimitiveError<Self>;
    
    fn try_from(part: StrangePart) -> Result<Self, Self::Error> {
        KillEaterScoreType::try_from(part as u32)
    }
}

impl TryFrom<&StrangePart> for KillEaterScoreType {
    type Error = TryFromPrimitiveError<Self>;
    
    fn try_from(part: &StrangePart) -> Result<Self, Self::Error> {
        KillEaterScoreType::try_from(*part as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn converts_to_string() {
        let score_type = KillEaterScoreType::Kills;
        assert_eq!(score_type.to_string(), "Kills");
    }
    
    #[test]
    fn gets_the_item_level() {
        let score_type = KillEaterScoreType::Kills;
        assert_eq!(ItemLevel::from(score_type), ItemLevel::KillEaterRank);
    }
    
    #[test]
    fn attribute_slices_are_equal_length() {
        assert_eq!(KillEaterScoreType::DEFINDEX.len(), KillEaterScoreType::ATTRIBUTES.len());
    }
}
