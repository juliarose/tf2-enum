//! Serialization utilities.

use crate::AttributeValue;
use crate::serialize::option_float_as_integers_when_whole;
use serde::{Serialize, Deserialize};

/// Container type for item attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAttribute {
    /// The defindex of this attribute.
    pub defindex: u32,
    /// The attribute value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
    /// The attribute float value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "option_float_as_integers_when_whole")]
    pub float_value: Option<f64>,
}
