use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString};
 
/// Description format.
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
    Deserialize,
    Serialize,
    EnumString,
    EnumIter,
    EnumCount,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
 pub enum DescriptionFormat {
    ValueIsAdditive,
    ValueIsPercentage,
    ValueIsInvertedPercentage,
    ValueIsAdditivePercentage,
    ValueIsOr,
    ValueIsDate,
    ValueIsParticleIndex,
    ValueIsAccountId,
    ValueIsItemDef,
    ValueIsFromLookupTable,
    VisualsMvmBoss,
    #[strum(serialize = "value_is_killstreakeffect_index")]
    #[serde(rename = "value_is_killstreakeffect_index")]
    ValueIsKillstreakEffectIndex,
    #[strum(serialize = "value_is_killstreak_idleeffect_index")]
    #[serde(rename = "value_is_killstreak_idleeffect_index")]
    ValueIsKillstreakIdleEffectIndex,
 }
 
 #[cfg(test)]
 mod tests {
    use super::*;
    
    #[test]
    fn serializes() {
        let json = serde_json::to_string(&DescriptionFormat::ValueIsAdditive).unwrap();
        assert_eq!(json, r#""value_is_additive""#);
        
        let json = serde_json::to_string(&DescriptionFormat::ValueIsKillstreakEffectIndex).unwrap();
        assert_eq!(json, r#""value_is_killstreakeffect_index""#);
    }
    
    #[test]
    fn deserializes() {
        let json = r#""value_is_additive""#;
        let variant: DescriptionFormat = serde_json::from_str(json).unwrap();
        assert_eq!(variant, DescriptionFormat::ValueIsAdditive);

        let json = r#""value_is_killstreakeffect_index""#;
        let variant: DescriptionFormat = serde_json::from_str(json).unwrap();
        assert_eq!(variant, DescriptionFormat::ValueIsKillstreakEffectIndex);
    }
 }
 