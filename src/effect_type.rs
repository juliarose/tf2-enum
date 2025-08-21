use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString};

/// Effect type.
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
