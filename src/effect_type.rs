use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumIter, EnumCount};

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
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EffectType {
    Neutral,
    Positive,
    Negative,
    Unusual,
    ValueIsFromLookupTable,
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
