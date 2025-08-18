//! Serialization utilities.

use crate::AttributeSet;
use serde::ser::SerializeSeq;
use serde::Serializer;

/// Serializes a float into an integer when the float is a whole number.
pub fn option_float_as_integers_when_whole<S: Serializer>(
    value: &Option<f64>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(v) if v.fract() == 0.0 && (*v as u64 as f64 == *v) => s.serialize_u64(*v as u64),
        Some(v) => s.serialize_f64(*v),
        None => s.serialize_none(),
    }
}

pub fn serialize_attribute_set<S, T>(set: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AttributeSet,
{
    let mut seq = serializer.serialize_seq(Some(set.len()))?;
    
    for attr in set.iter_attributes() {
        seq.serialize_element(&attr)?;
    }
    
    seq.end()
}