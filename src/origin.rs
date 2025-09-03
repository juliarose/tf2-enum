use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::{Display, EnumCount, EnumIter, EnumString};

/// Origin.
#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Display,
    Serialize_repr,
    Deserialize_repr,
    EnumString,
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
)]
#[repr(u32)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Origin {
    #[strum(serialize = "Timed Drop")]
    TimedDrop = 0,
    #[strum(serialize = "Achievement")]
    Achievement = 1,
    #[strum(serialize = "Purchased")]
    Purchased = 2,
    #[strum(serialize = "Traded")]
    Traded = 3,
    #[strum(serialize = "Crafted")]
    Crafted = 4,
    #[strum(serialize = "Store Promotion")]
    StorePromotion = 5,
    #[strum(serialize = "Gifted")]
    Gifted = 6,
    #[strum(serialize = "Support Granted")]
    SupportGranted = 7,
    #[strum(serialize = "Found in Crate")]
    FoundInCrate = 8,
    #[strum(serialize = "Earned")]
    Earned = 9,
    #[strum(serialize = "Third-Party Promotion")]
    ThirdPartyPromotion = 10,
    #[strum(serialize = "Wrapped Gift")]
    WrappedGift = 11,
    #[strum(serialize = "Halloween Drop")]
    HalloweenDrop = 12,
    #[strum(serialize = "Steam Purchase")]
    SteamPurchase = 13,
    #[strum(serialize = "Foreign Item")]
    ForeignItem = 14,
    #[strum(serialize = "CD Key")]
    CDKey = 15,
    #[strum(serialize = "Collection Reward")]
    CollectionReward = 16,
    #[strum(serialize = "Preview Item")]
    PreviewItem = 17,
    #[strum(serialize = "Steam Workshop Contribution")]
    SteamWorkshopContribution = 18,
    #[strum(serialize = "Periodic score reward")]
    PeriodicScoreReward = 19,
    #[strum(serialize = "MvM Badge completion reward")]
    MvMBadgeCompletionReward = 20,
    #[strum(serialize = "MvM Squad surplus reward")]
    MvMSquadSurplusReward = 21,
    #[strum(serialize = "Recipe output")]
    RecipeOutput = 22,
    #[strum(serialize = "Quest Drop")]
    QuestDrop = 23,
    #[strum(serialize = "Quest Loaner Item")]
    QuestLoanerItem = 24,
    #[strum(serialize = "Trade-Up")]
    TradeUp = 25,
    #[strum(serialize = "Viral Competitive Beta Pass Spread")]
    ViralCompetitiveBetaPassSpread = 26,
    #[strum(serialize = "CYOA Blood Money Purchase")]
    CYOABloodMoneyPurchase = 27,
    #[strum(serialize = "War Paint")]
    WarPaint = 28,
    #[strum(serialize = "Untradable Free Contract Reward")]
    UntradableFreeContractReward = 29,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(Origin::from_str("Purchased").unwrap(), Origin::Purchased);
    }

    #[test]
    fn test_try_from_primitive() {
        assert_eq!(Origin::try_from(2u32).unwrap(), Origin::Purchased);
    }
}
