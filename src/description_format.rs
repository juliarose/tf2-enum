use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, EnumIter, EnumCount};
 
/// Description format.
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
 pub enum DescriptionFormat {
    #[strum(serialize = "value_is_additive")]
    #[serde(rename = "value_is_additive")]
    ValueIsAdditive,
    #[strum(serialize = "value_is_percentage")]
    #[serde(rename = "value_is_percentage")]
    ValueIsPercentage,
    #[strum(serialize = "value_is_inverted_percentage")]
    #[serde(rename = "value_is_inverted_percentage")]
    ValueIsInvertedPercentage,
    #[strum(serialize = "value_is_additive_percentage")]
    #[serde(rename = "value_is_additive_percentage")]
    ValueIsAdditivePercentage,
    #[strum(serialize = "value_is_or")]
    #[serde(rename = "value_is_or")]
    ValueIsOr,
    #[strum(serialize = "value_is_date")]
    #[serde(rename = "value_is_date")]
    ValueIsDate,
    #[strum(serialize = "value_is_particle_index")]
    #[serde(rename = "value_is_particle_index")]
    ValueIsParticleIndex,
    #[strum(serialize = "value_is_account_id")]
    #[serde(rename = "value_is_account_id")]
    ValueIsAccountId,
    #[strum(serialize = "value_is_item_def")]
    #[serde(rename = "value_is_item_def")]
    ValueIsItemDef,
    #[strum(serialize = "value_is_from_lookup_table")]
    #[serde(rename = "value_is_from_lookup_table")]
    ValueIsFromLookupTable,
    #[strum(serialize = "visuals_mvm_boss")]
    #[serde(rename = "visuals_mvm_boss")]
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
    }
    
    #[test]
    fn deserializes() {
        let json = r#""value_is_additive""#;
        let variant: DescriptionFormat = serde_json::from_str(json).unwrap();
        assert_eq!(variant, DescriptionFormat::ValueIsAdditive);
    }
 }
 