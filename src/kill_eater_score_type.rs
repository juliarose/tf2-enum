use crate::{Attributes, AttributeValue, EffectType, DescriptionFormat, StrangePart, ItemLevel};
use crate::error::TryFromPrimitiveError;
use strum::{Display, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

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
pub enum KillEaterScoreType {
    /// "Kills" used for most Strange weapons.
    #[strum(serialize = "Kills")]
    Kills = 0,
    #[strum(serialize = "Ubers")]
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
    /// Converts this [`KillEaterScoreType`] into its related [`StrangePart`], if it exists.
    pub fn strange_part(&self) -> Option<StrangePart> {
        StrangePart::try_from(self).ok()
    }
    
    /// Converts this [`KillEaterScoreType`] into its related [`ItemLevel`].
    pub fn item_level(&self) -> ItemLevel {
        self.into()
    }
}

/// kill_eater_user_score_type_1
/// kill_eater_user_score_type_2
/// kill_eater_user_score_type_3
impl Attributes for KillEaterScoreType {
    const DEFINDEX: &[u32] = &[
        380,
        382,
        384,
    ];
    const NAME: &[&str] = &[
        "kill eater user score type 1",
        "kill eater user score type 2",
        "kill eater user score type 3",
    ];
    const ATTRIBUTE_CLASS: &[&str] = &[
        "kill_eater_user_score_type_1",
        "kill_eater_user_score_type_2",
        "kill_eater_user_score_type_3",
    ];
    const DESCRIPTION_STRING: &[Option<&str>] = &[
        None,
        None,
        None,
    ];
    const DESCRIPTION_FORMAT: &[Option<DescriptionFormat>] = &[
        None,
        None,
        None,
    ];
    const EFFECT_TYPE: &[EffectType] = &[
        EffectType::Positive,
        EffectType::Positive,
        EffectType::Positive,
    ];
    const HIDDEN: &[bool] = &[
        true,
        true,
        true,
    ];
    const STORED_AS_INTEGER: &[bool] = &[
        false,
        false,
        false,
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
}
