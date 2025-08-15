use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, EnumIter, EnumCount};

/// Effect type.
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
pub enum EffectType {
    #[strum(serialize = "neutral")]
    #[serde(rename = "neutral")]
    Neutral,
    #[strum(serialize = "positive")]
    #[serde(rename = "positive")]
    Positive,
    #[strum(serialize = "negative")]
    #[serde(rename = "negative")]
    Negative,
    #[strum(serialize = "unusual")]
    #[serde(rename = "unusual")]
    Unusual,
    #[strum(serialize = "value_is_from_lookup_table")]
    #[serde(rename = "value_is_from_lookup_table")]
    ValueIsFromLookupTable,
    #[strum(serialize = "strange")]
    #[serde(rename = "strange")]
    Strange,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn serializes() {
        let json = serde_json::to_string(&EffectType::Neutral).unwrap();
        assert_eq!(json, r#""neutral""#);
    }

    #[test]
    fn deserializes() {
        let json = r#""neutral""#;
        let variant: EffectType = serde_json::from_str(json).unwrap();
        assert_eq!(variant, EffectType::Neutral);
    }
}
