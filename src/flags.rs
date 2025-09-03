use serde::{Deserialize, Serialize};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::{Display, EnumCount, EnumIter};

/// Flags.
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
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
    Clone,
    Copy,
)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum Flags {
    CannotTrade = 1 << 0,
    CannotBeUsedInCrafting = 1 << 1,
    CanBeTradedByFreeAccounts = 1 << 2,
    NonEconomy = 1 << 3,
    PurchasedAfterStoreCraftabilityChanges2012 = 1 << 4,
    // The following are client-only flags:
    ClientForceBlueTeam = 1 << 5,
    ClientStoreItem = 1 << 6,
    ClientPreview = 1 << 7,
    // All flags (except client).
    CheckFlagsAllGCFlags = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn all_gc_flags_correct() {
        let all_gc_flags = Flags::CheckFlagsAllGCFlags;
        let result = Flags::CannotTrade as u32 |
            Flags::CannotBeUsedInCrafting as u32 |
            Flags::CanBeTradedByFreeAccounts as u32 |
            Flags::NonEconomy as u32 |
            Flags::PurchasedAfterStoreCraftabilityChanges2012 as u32;

        assert_eq!(all_gc_flags as u32, result);
    }
}