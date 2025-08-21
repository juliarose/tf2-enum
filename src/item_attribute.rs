//! Serialization utilities.

use crate::AttributeValue;
use crate::serialize::option_float_as_integers_when_whole;
use serde::{Deserialize, Serialize};

/// Container type for item attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAttribute {
    /// The defindex of this attribute.
    pub defindex: u32,
    /// The attribute value.
    #[serde(default)]
    pub value: AttributeValue,
    /// The attribute float value. This is usually only `None` when value is a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "option_float_as_integers_when_whole")]
    pub float_value: Option<f32>,
}
