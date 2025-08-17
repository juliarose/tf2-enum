//! Serialization utilities.

use crate::AttributeValue;
use serde::{Serialize, Deserialize, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedAttribute {
    pub defindex: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "option_float_as_integers_when_whole")]
    pub float_value: Option<f64>,
}

/// Serializes a float into an integer when the float is a whole number.
fn option_float_as_integers_when_whole<S: Serializer>(
    value: &Option<f64>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(v) if v.fract() == 0.0 && (*v as u64 as f64 == *v) => s.serialize_u64(*v as u64),
        Some(v) => s.serialize_f64(*v),
        None => s.serialize_none(),
    }
}
