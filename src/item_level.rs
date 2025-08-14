use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, EnumIter, EnumCount};

use crate::KillEaterScoreType;

/// Item level.
#[derive(
    Debug,
    Deserialize,
    Serialize,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Display,
    EnumString,
    EnumIter,
    EnumCount,
    Clone,
    Copy,
)]
pub enum ItemLevel {
    #[strum(serialize = "KillEaterRank")]
    KillEaterRank,
    #[strum(serialize = "SpiritOfGivingRank")]
    SpiritOfGivingRank,
    #[strum(serialize = "KillEater_HolidayPunchRank")]
    KillEaterHolidayPunchRank,
    #[strum(serialize = "KillEater_ManTreadsRank")]
    KillEaterManTreadsRank,
    #[strum(serialize = "KillEater_SapperRank")]
    KillEaterSapperRank,
    #[strum(serialize = "KillEater_RobotsKilledRank")]
    KillEaterRobotsKilledRank,
    #[strum(serialize = "KillEater_TimeCloakedRank")]
    KillEaterTimeCloakedRank,
    #[strum(serialize = "KillEater_HealthGivenRank")]
    KillEaterHealthGivenRank,
    #[strum(serialize = "KillEater_PointsScored")]
    KillEaterPointsScored,
    #[strum(serialize = "Journal_DuckBadge")]
    JournalDuckBadge,
    #[strum(serialize = "KillEater_OperationContractRank")]
    KillEaterOperationContractRank,
    #[strum(serialize = "KillEater_HalloweenSoulsRank")]
    KillEaterHalloweenSoulsRank,
    #[strum(serialize = "KillEater_ContractPointsEarnedRank")]
    KillEaterContractPointsEarnedRank,
    #[strum(serialize = "KillEater_BackstabsAbsorbed")]
    KillEaterBackstabsAbsorbed,
}

impl ItemLevel {
    /// Gets the level for a given `score`.
    pub fn score_level(&self, score: u32) -> &'static Level {
        let levels = self.levels();
        let mut prev_level = &levels[levels.len() - 1];
        
        for i in (0..levels.len()).rev() {
            if score >= levels[i].required_score {
                return prev_level;
            } else {
                prev_level = &levels[i];
            }
        }
        
        &levels[0]
    }
    
    /// Gets the levels for this [`ItemLevel`].
    pub fn levels(&self) -> &'static [Level] {
        match self {
            Self::KillEaterRank => &[
                Level { level: 0, required_score: 10, name: "Strange" },
                Level { level: 1, required_score: 25, name: "Unremarkable" },
                Level { level: 2, required_score: 45, name: "Scarcely Lethal" },
                Level { level: 3, required_score: 70, name: "Mildly Menacing" },
                Level { level: 4, required_score: 100, name: "Somewhat Threatening" },
                Level { level: 5, required_score: 135, name: "Uncharitable" },
                Level { level: 6, required_score: 175, name: "Notably Dangerous" },
                Level { level: 7, required_score: 225, name: "Sufficiently Lethal" },
                Level { level: 8, required_score: 275, name: "Truly Feared" },
                Level { level: 9, required_score: 350, name: "Spectacularly Lethal" },
                Level { level: 10, required_score: 500, name: "Gore-Spattered" },
                Level { level: 11, required_score: 750, name: "Wicked Nasty" },
                Level { level: 12, required_score: 999, name: "Positively Inhumane" },
                Level { level: 13, required_score: 1000, name: "Totally Ordinary" },
                Level { level: 14, required_score: 1500, name: "Face-Melting" },
                Level { level: 15, required_score: 2500, name: "Rage-Inducing" },
                Level { level: 16, required_score: 5000, name: "Server-Clearing" },
                Level { level: 17, required_score: 7500, name: "Epic" },
                Level { level: 18, required_score: 7616, name: "Legendary" },
                Level { level: 19, required_score: 8500, name: "Australian" },
                Level { level: 20, required_score: 10000, name: "Hale's Own" },
            ],
            Self::SpiritOfGivingRank => &[
                Level { level: 0, required_score: 1, name: "The" },
                Level { level: 1, required_score: 3, name: "The Baseline Benefactor's" },
                Level { level: 2, required_score: 7, name: "The Competent Contributor's" },
                Level { level: 3, required_score: 12, name: "The Adequate Altruist's" },
                Level { level: 4, required_score: 20, name: "The Satisfactory Santa's" },
                Level { level: 5, required_score: 28, name: "The Sufficient Samaritan's" },
                Level { level: 6, required_score: 37, name: "The Distinguished Donator's" },
                Level { level: 7, required_score: 47, name: "The Dynamic Do-Gooder's" },
                Level { level: 8, required_score: 57, name: "The Consumate Contributor's" },
                Level { level: 9, required_score: 68, name: "The Baron of Bequeathment's" },
                Level { level: 10, required_score: 79, name: "The Lord of Largesse's" },
                Level { level: 11, required_score: 91, name: "The Chieftain of Charity's" },
                Level { level: 12, required_score: 104, name: "The Generalissimo of Generosity's" },
                Level { level: 13, required_score: 120, name: "The Bigshot Benefactor's" },
                Level { level: 14, required_score: 138, name: "The Caesar of Pleasers'" },
                Level { level: 15, required_score: 158, name: "The First-Class Philanthropist's" },
                Level { level: 16, required_score: 179, name: "The Humanitarian Hotshot's" },
                Level { level: 17, required_score: 210, name: "The Selfless Samaritan's" },
                Level { level: 18, required_score: 250, name: "The Uber-Altruist's" },
                Level { level: 19, required_score: 500, name: "Saxton's Own" },
            ],
            Self::KillEaterHolidayPunchRank => &[
                Level { level: 0, required_score: 10, name: "Strange" },
                Level { level: 1, required_score: 25, name: "Unremarkable" },
                Level { level: 2, required_score: 45, name: "Almost Amusing" },
                Level { level: 3, required_score: 70, name: "Mildly Mirthful" },
                Level { level: 4, required_score: 100, name: "Somewhat Droll" },
                Level { level: 5, required_score: 135, name: "Thigh-Slapping" },
                Level { level: 6, required_score: 175, name: "Notably Cheery" },
                Level { level: 7, required_score: 225, name: "Sufficiently Wry" },
                Level { level: 8, required_score: 275, name: "Truly Feared" },
                Level { level: 9, required_score: 350, name: "Spectacularly Jocular" },
                Level { level: 10, required_score: 500, name: "Riotous" },
                Level { level: 11, required_score: 749, name: "Wicked Funny" },
                Level { level: 12, required_score: 750, name: "Totally Unamusing" },
                Level { level: 13, required_score: 1000, name: "Positively Persiflagious" },
                Level { level: 14, required_score: 1500, name: "Frown-Annihilating" },
                Level { level: 15, required_score: 2500, name: "Grin-Inducing" },
                Level { level: 16, required_score: 5000, name: "Server-Clearing" },
                Level { level: 17, required_score: 7500, name: "Epic" },
                Level { level: 18, required_score: 7923, name: "Legendary" },
                Level { level: 19, required_score: 8500, name: "Australian" },
                Level { level: 20, required_score: 10000, name: "Mann Co. Select" },
            ],
            Self::KillEaterManTreadsRank => &[
                Level { level: 0, required_score: 1, name: "Strange" },
                Level { level: 1, required_score: 3, name: "Broken-In" },
                Level { level: 2, required_score: 5, name: "Scarcely Worn" },
                Level { level: 3, required_score: 7, name: "Mildly Minatory" },
                Level { level: 4, required_score: 10, name: "Crushingly Crushing" },
                Level { level: 5, required_score: 14, name: "Inauspicious" },
                Level { level: 6, required_score: 18, name: "Profoundly Penumbric" },
                Level { level: 7, required_score: 23, name: "Sufficiently Eclipsing" },
                Level { level: 8, required_score: 28, name: "Truly Tenebrific" },
                Level { level: 9, required_score: 35, name: "Spectacularly Fell" },
                Level { level: 10, required_score: 50, name: "Fashion-Splattered" },
                Level { level: 11, required_score: 75, name: "Wicked Stinky" },
                Level { level: 12, required_score: 99, name: "Positively Planar" },
                Level { level: 13, required_score: 100, name: "Totally Comfortable" },
                Level { level: 14, required_score: 150, name: "Face-Flattening" },
                Level { level: 15, required_score: 250, name: "Rage-Inducing" },
                Level { level: 16, required_score: 500, name: "Server-Clearing" },
                Level { level: 17, required_score: 750, name: "Epic" },
                Level { level: 18, required_score: 784, name: "Legendary" },
                Level { level: 19, required_score: 850, name: "Australian" },
                Level { level: 20, required_score: 1000, name: "Hale's Custom" },
            ],
            Self::KillEaterSapperRank => &[
                Level { level: 0, required_score: 3, name: "Strange" },
                Level { level: 1, required_score: 9, name: "Unremarkable" },
                Level { level: 2, required_score: 15, name: "Scarcely Shocking" },
                Level { level: 3, required_score: 21, name: "Mildly Magnetizing" },
                Level { level: 4, required_score: 30, name: "Somewhat Inducting" },
                Level { level: 5, required_score: 42, name: "Unfortunate" },
                Level { level: 6, required_score: 54, name: "Notably Deleterious" },
                Level { level: 7, required_score: 69, name: "Sufficiently Ruinous" },
                Level { level: 8, required_score: 84, name: "Truly Conducting" },
                Level { level: 9, required_score: 105, name: "Spectacularly Pseudoful" },
                Level { level: 10, required_score: 150, name: "Ion-Spattered" },
                Level { level: 11, required_score: 225, name: "Wickedly Dynamizing" },
                Level { level: 12, required_score: 299, name: "Positively Plasmatic" },
                Level { level: 13, required_score: 300, name: "Totally Ordinary" },
                Level { level: 14, required_score: 450, name: "Circuit-Melting" },
                Level { level: 15, required_score: 750, name: "Nullity-Inducing" },
                Level { level: 16, required_score: 1500, name: "Server-Clearing" },
                Level { level: 17, required_score: 2250, name: "Epic" },
                Level { level: 18, required_score: 2345, name: "Legendary" },
                Level { level: 19, required_score: 2550, name: "Australian" },
                Level { level: 20, required_score: 3000, name: "Mann Co. Select" },
            ],
            Self::KillEaterRobotsKilledRank => &[
                Level { level: 0, required_score: 100, name: "KillEater_RobotsKilledRank0" },
                Level { level: 1, required_score: 250, name: "KillEater_RobotsKilledRank1" },
                Level { level: 2, required_score: 450, name: "KillEater_RobotsKilledRank2" },
                Level { level: 3, required_score: 700, name: "KillEater_RobotsKilledRank3" },
                Level { level: 4, required_score: 1000, name: "KillEater_RobotsKilledRank4" },
                Level { level: 5, required_score: 1350, name: "KillEater_RobotsKilledRank5" },
                Level { level: 6, required_score: 1750, name: "KillEater_RobotsKilledRank6" },
                Level { level: 7, required_score: 2250, name: "KillEater_RobotsKilledRank7" },
                Level { level: 8, required_score: 2750, name: "KillEater_RobotsKilledRank8" },
                Level { level: 9, required_score: 3500, name: "KillEater_RobotsKilledRank9" },
                Level { level: 10, required_score: 5000, name: "KillEater_RobotsKilledRank10" },
                Level { level: 11, required_score: 7500, name: "KillEater_RobotsKilledRank11" },
                Level { level: 12, required_score: 9990, name: "KillEater_RobotsKilledRank12" },
                Level { level: 13, required_score: 10000, name: "KillEater_RobotsKilledRank13" },
                Level { level: 14, required_score: 15000, name: "KillEater_RobotsKilledRank14" },
                Level { level: 15, required_score: 25000, name: "KillEater_RobotsKilledRank15" },
                Level { level: 16, required_score: 50000, name: "KillEater_RobotsKilledRank16" },
                Level { level: 17, required_score: 75000, name: "KillEater_RobotsKilledRank17" },
                Level { level: 18, required_score: 76160, name: "KillEater_RobotsKilledRank18" },
                Level { level: 19, required_score: 85000, name: "KillEater_RobotsKilledRank19" },
                Level { level: 20, required_score: 100000, name: "KillEater_RobotsKilledRank20" },
            ],
            Self::KillEaterTimeCloakedRank => &[
                Level { level: 0, required_score: 200, name: "Strange" },
                Level { level: 1, required_score: 500, name: "Unremarkable" },
                Level { level: 2, required_score: 900, name: "Scarcely Shocking" },
                Level { level: 3, required_score: 1337, name: "Mildly Magnetizing" },
                Level { level: 4, required_score: 2000, name: "Somewhat Inducting" },
                Level { level: 5, required_score: 2700, name: "Unfortunate" },
                Level { level: 6, required_score: 3500, name: "Notably Deleterious" },
                Level { level: 7, required_score: 4500, name: "Sufficiently Ruinous" },
                Level { level: 8, required_score: 5500, name: "Truly Conducting" },
                Level { level: 9, required_score: 7000, name: "Spectacularly Pseudoful" },
                Level { level: 10, required_score: 9000, name: "Ion-Spattered" },
                Level { level: 11, required_score: 12000, name: "Wickedly Dynamizing" },
                Level { level: 12, required_score: 16000, name: "Positively Plasmatic" },
                Level { level: 13, required_score: 21337, name: "Totally Ordinary" },
                Level { level: 14, required_score: 35000, name: "Circuit-Melting" },
                Level { level: 15, required_score: 58007, name: "Nullity-Inducing" },
                Level { level: 16, required_score: 90000, name: "Server-Clearing" },
                Level { level: 17, required_score: 120000, name: "Epic" },
                Level { level: 18, required_score: 140000, name: "Legendary" },
                Level { level: 19, required_score: 160000, name: "Australian" },
                Level { level: 20, required_score: 200000, name: "Mann Co. Select" },
            ],
            Self::KillEaterHealthGivenRank => &[
                Level { level: 0, required_score: 2000, name: "Strange" },
                Level { level: 1, required_score: 5000, name: "Unremarkable" },
                Level { level: 2, required_score: 9000, name: "Scarcely Shocking" },
                Level { level: 3, required_score: 13370, name: "Mildly Magnetizing" },
                Level { level: 4, required_score: 20000, name: "Somewhat Inducting" },
                Level { level: 5, required_score: 27000, name: "Unfortunate" },
                Level { level: 6, required_score: 35000, name: "Notably Deleterious" },
                Level { level: 7, required_score: 45000, name: "Sufficiently Ruinous" },
                Level { level: 8, required_score: 55000, name: "Truly Conducting" },
                Level { level: 9, required_score: 70000, name: "Spectacularly Pseudoful" },
                Level { level: 10, required_score: 90000, name: "Ion-Spattered" },
                Level { level: 11, required_score: 120000, name: "Wickedly Dynamizing" },
                Level { level: 12, required_score: 160000, name: "Positively Plasmatic" },
                Level { level: 13, required_score: 213370, name: "Totally Ordinary" },
                Level { level: 14, required_score: 350000, name: "Circuit-Melting" },
                Level { level: 15, required_score: 518008, name: "Nullity-Inducing" },
                Level { level: 16, required_score: 900000, name: "Server-Clearing" },
                Level { level: 17, required_score: 1200000, name: "Epic" },
                Level { level: 18, required_score: 1400000, name: "Legendary" },
                Level { level: 19, required_score: 1600000, name: "Australian" },
                Level { level: 20, required_score: 2000000, name: "Mann Co. Select" },
            ],
            Self::KillEaterPointsScored => &[
                Level { level: 0, required_score: 15, name: "Strange" },
                Level { level: 1, required_score: 30, name: "Ragged" },
                Level { level: 2, required_score: 50, name: "Tacky" },
                Level { level: 3, required_score: 75, name: "Secondhand" },
                Level { level: 4, required_score: 100, name: "Odious" },
                Level { level: 5, required_score: 135, name: "Garish" },
                Level { level: 6, required_score: 175, name: "Comfortable" },
                Level { level: 7, required_score: 250, name: "Dapper" },
                Level { level: 8, required_score: 375, name: "Sharp" },
                Level { level: 9, required_score: 500, name: "Fancy" },
                Level { level: 10, required_score: 725, name: "Fancy Shmancy" },
                Level { level: 11, required_score: 1000, name: "Fashionable" },
                Level { level: 12, required_score: 1500, name: "Glamorous" },
                Level { level: 13, required_score: 2000, name: "Posh" },
                Level { level: 14, required_score: 2750, name: "Fabulous" },
                Level { level: 15, required_score: 4000, name: "Stunning" },
                Level { level: 16, required_score: 5500, name: "Epic" },
                Level { level: 17, required_score: 7500, name: "Legendary" },
                Level { level: 18, required_score: 10000, name: "Australian" },
                Level { level: 19, required_score: 15000, name: "Mann Co. Select" },
                Level { level: 20, required_score: 25000, name: "Mannceaux Signature Collection" },
            ],
            Self::JournalDuckBadge => &[
                Level { level: 0, required_score: 1, name: "" },
                Level { level: 1, required_score: 2, name: "Crumb Chasing " },
                Level { level: 2, required_score: 3, name: "Kinda Ducky " },
                Level { level: 3, required_score: 4, name: "Fairly Fowl " },
                Level { level: 4, required_score: 5, name: "Somewhat Quackworthy " },
                Level { level: 5, required_score: 6, name: "Quacknowledged " },
                Level { level: 6, required_score: 7, name: "Duckstinguished " },
                Level { level: 7, required_score: 8, name: "Pleasantly Paddling " },
                Level { level: 8, required_score: 9, name: "Perfectly Preened " },
                Level { level: 9, required_score: 10, name: "Duckalicious " },
                Level { level: 10, required_score: 11, name: "Quacktastic " },
                Level { level: 12, required_score: 13, name: "Feather-Ruffling " },
                Level { level: 6, required_score: 10000, name: "Cheerful" },
                Level { level: 7, required_score: 12000, name: "Joyous" },
                Level { level: 8, required_score: 14000, name: "Dignified" },
                Level { level: 9, required_score: 16000, name: "Proud" },
                Level { level: 10, required_score: 18000, name: "Noble" },
                Level { level: 11, required_score: 20000, name: "Honored" },
                Level { level: 12, required_score: 22000, name: "Exalted" },
                Level { level: 13, required_score: 24000, name: "Breathtaking" },
                Level { level: 14, required_score: 26000, name: "Outrageous" },
                Level { level: 15, required_score: 28000, name: "Spectacular" },
                Level { level: 16, required_score: 30000, name: "Majestic" },
                Level { level: 17, required_score: 32000, name: "Epic" },
                Level { level: 18, required_score: 34000, name: "Legendary" },
                Level { level: 19, required_score: 36000, name: "Australian" },
                Level { level: 20, required_score: 38000, name: "Merasmus's Own" },
            ],
            Self::KillEaterOperationContractRank => &[
                Level { level: 0, required_score: 2500, name: "Gravel" },
                Level { level: 1, required_score: 5000, name: "Bronze" },
                Level { level: 2, required_score: 6840, name: "Silver" },
                Level { level: 3, required_score: 6840, name: "Australium" },
            ],
            Self::KillEaterHalloweenSoulsRank => &[
                Level { level: 0, required_score: 666, name: "Sad" },
                Level { level: 1, required_score: 1337, name: "Silly" },
                Level { level: 2, required_score: 2000, name: "Sneering" },
                Level { level: 3, required_score: 4000, name: "Happy" },
                Level { level: 4, required_score: 6000, name: "Pleased" },
                Level { level: 5, required_score: 8000, name: "Delighted" },
                Level { level: 6, required_score: 10000, name: "Cheerful" },
                Level { level: 7, required_score: 12000, name: "Joyous" },
                Level { level: 8, required_score: 14000, name: "Dignified" },
                Level { level: 9, required_score: 16000, name: "Proud" },
                Level { level: 10, required_score: 18000, name: "Noble" },
                Level { level: 11, required_score: 20000, name: "Honored" },
                Level { level: 12, required_score: 22000, name: "Exalted" },
                Level { level: 13, required_score: 24000, name: "Breathtaking" },
                Level { level: 14, required_score: 26000, name: "Outrageous" },
                Level { level: 15, required_score: 28000, name: "Spectacular" },
                Level { level: 16, required_score: 30000, name: "Majestic" },
                Level { level: 17, required_score: 32000, name: "Epic" },
                Level { level: 18, required_score: 34000, name: "Legendary" },
                Level { level: 19, required_score: 36000, name: "Australian" },
                Level { level: 20, required_score: 38000, name: "Merasmus's Own" },
            ],
            Self::KillEaterContractPointsEarnedRank => &[
                Level { level: 0, required_score: 2500, name: "Gravel" },
                Level { level: 1, required_score: 5000, name: "Bronze" },
                Level { level: 2, required_score: 6840, name: "Silver" },
                Level { level: 3, required_score: 6840, name: "Australium" },
            ],
            Self::KillEaterBackstabsAbsorbed => &[
                Level { level: 0, required_score: 1, name: "Strange" },
                Level { level: 1, required_score: 3, name: "Unremarkable" },
                Level { level: 2, required_score: 5, name: "Scarcely Lethal" },
                Level { level: 3, required_score: 7, name: "Mildly Menacing" },
                Level { level: 4, required_score: 10, name: "Somewhat Threatening" },
                Level { level: 5, required_score: 14, name: "Uncharitable" },
                Level { level: 6, required_score: 18, name: "Notably Dangerous" },
                Level { level: 7, required_score: 23, name: "Sufficiently Lethal" },
                Level { level: 8, required_score: 28, name: "Truly Feared" },
                Level { level: 9, required_score: 35, name: "Spectacularly Lethal" },
                Level { level: 10, required_score: 50, name: "Gore-Spattered" },
                Level { level: 11, required_score: 75, name: "Wicked Nasty" },
                Level { level: 12, required_score: 99, name: "Positively Inhumane" },
                Level { level: 13, required_score: 100, name: "Totally Ordinary" },
                Level { level: 14, required_score: 150, name: "Face-Melting" },
                Level { level: 15, required_score: 250, name: "Rage-Inducing" },
                Level { level: 16, required_score: 500, name: "Server-Clearing" },
                Level { level: 17, required_score: 750, name: "Epic" },
                Level { level: 18, required_score: 784, name: "Legendary" },
                Level { level: 19, required_score: 850, name: "Australian" },
                Level { level: 20, required_score: 1000, name: "Hale's Own" },
            ],
        }
    }
}

impl From<KillEaterScoreType> for ItemLevel {
    fn from(score_type: KillEaterScoreType) -> Self {
        match score_type {
            KillEaterScoreType::Kills |
            KillEaterScoreType::Ubers |
            KillEaterScoreType::KillAssists |
            KillEaterScoreType::SentryKills |
            KillEaterScoreType::SoddenVictims |
            KillEaterScoreType::HeadsTaken |
            KillEaterScoreType::Humiliations |
            KillEaterScoreType::DeathsFeigned |
            KillEaterScoreType::ScoutsKilled |
            KillEaterScoreType::SnipersKilled |
            KillEaterScoreType::SoldiersKilled |
            KillEaterScoreType::DemomenKilled |
            KillEaterScoreType::HeaviesKilled |
            KillEaterScoreType::PyrosKilled |
            KillEaterScoreType::SpiesKilled |
            KillEaterScoreType::EngineersKilled |
            KillEaterScoreType::MedicsKilled |
            KillEaterScoreType::BuildingsDestroyed |
            KillEaterScoreType::ProjectilesReflected |
            KillEaterScoreType::HeadshotKills |
            KillEaterScoreType::AirborneEnemyKills |
            KillEaterScoreType::GibKills |
            KillEaterScoreType::KillsUnderAFullMoon |
            KillEaterScoreType::Dominations |
            KillEaterScoreType::Revenges |
            KillEaterScoreType::PosthumousKills |
            KillEaterScoreType::TeammatesExtinguished |
            KillEaterScoreType::CriticalKills |
            KillEaterScoreType::KillsWhileExplosiveJumping |
            KillEaterScoreType::SappersRemoved |
            KillEaterScoreType::CloakedSpiesKilled |
            KillEaterScoreType::MedicsKilledThatHaveFullUberCharge |
            KillEaterScoreType::KillsWhileLowHealth |
            KillEaterScoreType::KillsDuringHalloween |
            KillEaterScoreType::RobotsKilledDuringHalloween |
            KillEaterScoreType::DefenderKills |
            KillEaterScoreType::SubmergedEnemyKills |
            KillEaterScoreType::KillsWhileInvulnUberCharged |
            KillEaterScoreType::FoodItemsEaten |
            KillEaterScoreType::BannersDeployed |
            KillEaterScoreType::TeammatesTeleported |
            KillEaterScoreType::TanksDestroyed |
            KillEaterScoreType::LongDistanceKills |
            KillEaterScoreType::KillEaterEventUniquePlayerKills |
            KillEaterScoreType::DoubleDonks |
            KillEaterScoreType::TeammatesWhipped |
            KillEaterScoreType::KillsDuringVictoryTime |
            KillEaterScoreType::TauntKills |
            KillEaterScoreType::UnusualWearingPlayerKills |
            KillEaterScoreType::BurningPlayerKills |
            KillEaterScoreType::KillstreaksEnded |
            KillEaterScoreType::FreezecamTauntAppearances |
            KillEaterScoreType::DamageDealt |
            KillEaterScoreType::FiresSurvived |
            KillEaterScoreType::AlliedHealingDone |
            KillEaterScoreType::PointBlankKills |
            KillEaterScoreType::WrangledSentryKills |
            KillEaterScoreType::CosmeticKills |
            KillEaterScoreType::FullHealthKills |
            KillEaterScoreType::TauntingPlayerKills |
            KillEaterScoreType::CarnivalKills |
            KillEaterScoreType::CarnivalUnderworldKills |
            KillEaterScoreType::CarnivalGamesWon |
            KillEaterScoreType::NotCritNorMiniCritKills |
            KillEaterScoreType::PlayerHits |
            KillEaterScoreType::Assists |
            KillEaterScoreType::ContractsCompleted |
            KillEaterScoreType::ContractKills |
            KillEaterScoreType::ContractBonusPoints |
            KillEaterScoreType::TimesPerformed |
            KillEaterScoreType::KillsAndAssistsDuringInvasionEvent |
            KillEaterScoreType::KillsAndAssistsOn2FortInvasion |
            KillEaterScoreType::KillsAndAssistsOnProbed |
            KillEaterScoreType::KillsAndAssistsOnByre |
            KillEaterScoreType::KillsAndAssistsOnWatergate |
            KillEaterScoreType::MerasmissionsCompleted |
            KillEaterScoreType::HalloweenTransmutesPerformed |
            KillEaterScoreType::PowerUpCanteensUsed |
            KillEaterScoreType::ContractPointsContributedToFriends => Self::KillEaterRank,
            KillEaterScoreType::SpiesShocked => Self::KillEaterBackstabsAbsorbed,
            KillEaterScoreType::GiftsGiven => Self::SpiritOfGivingRank,
            KillEaterScoreType::BuildingsSapped => Self::KillEaterSapperRank,
            KillEaterScoreType::TickleFightsWon => Self::KillEaterHolidayPunchRank,
            KillEaterScoreType::OpponentsFlattened => Self::KillEaterManTreadsRank,
            KillEaterScoreType::RobotsDestroyed |
            KillEaterScoreType::GiantRobotsDestroyed |
            KillEaterScoreType::RobotScoutsDestroyed |
            KillEaterScoreType::RobotSpiesDestroyed => Self::KillEaterRobotsKilledRank,
            KillEaterScoreType::SecondsCloaked => Self::KillEaterTimeCloakedRank,
            KillEaterScoreType::HealthDispensedToTeammates => Self::KillEaterHealthGivenRank,
            KillEaterScoreType::PointsScored => Self::KillEaterPointsScored,
            KillEaterScoreType::ContractPoints => Self::KillEaterOperationContractRank,
            KillEaterScoreType::SoulsCollected => Self::KillEaterHalloweenSoulsRank,
            KillEaterScoreType::ContractPointsEarned => Self::KillEaterContractPointsEarnedRank,
        }
    }
}

impl From<&KillEaterScoreType> for ItemLevel {
    fn from(score_type: &KillEaterScoreType) -> Self {
        (*score_type).into()
    }
}

/// Level belonging to an [`ItemLevel`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Level {
  pub level: u32,
  pub required_score: u32,
  pub name: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn hales_own() {
        assert_eq!(ItemLevel::KillEaterRank.score_level(9000).name, "Hale's Own");
        assert_eq!(ItemLevel::KillEaterRank.score_level(11000).name, "Hale's Own");
    }
    
    #[test]
    fn strange_rank() {
        assert_eq!(ItemLevel::KillEaterRank.score_level(4).name, "Strange");
        assert_eq!(ItemLevel::KillEaterRank.score_level(15).name, "Unremarkable");
    }
}
