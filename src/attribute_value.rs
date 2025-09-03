use serde::{Deserialize, Serialize};
use std::fmt;

/// A value for an attribute.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttributeValue {
    /// Represents an integer attribute value.
    Integer(u32),
    /// Represents a float attribute value.
    Float(f32),
    /// Represents a string attribute value.
    String(String),
    /// No value.
    #[default]
    None,
}

impl std::fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeValue::Integer(v) => write!(f, "{v}"),
            AttributeValue::Float(v) => {
                if v.fract() == 0.0 {
                    write!(f, "{:.0}", v)
                } else {
                    write!(f, "{}", v)
                }
            },
            AttributeValue::String(v) => write!(f, "{v}"),
            AttributeValue::None => write!(f, ""),
        }
    }
}

impl From<u32> for AttributeValue {
    fn from(value: u32) -> Self {
        AttributeValue::Integer(value)
    }
}

impl From<f32> for AttributeValue {
    fn from(value: f32) -> Self {
        AttributeValue::Float(value)
    }
}

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        AttributeValue::String(value)
    }
}

impl From<&String> for AttributeValue {
    fn from(value: &String) -> Self {
        AttributeValue::String(value.clone())
    }
}

impl From<&str> for AttributeValue {
    fn from(value: &str) -> Self {
        AttributeValue::String(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn serializes() {
        let int_value = AttributeValue::Integer(42);
        let float_value = AttributeValue::Float(3.14);
        let string_value = AttributeValue::String("Hello".into());

        let int_json = serde_json::to_string(&int_value).unwrap();
        let float_json = serde_json::to_string(&float_value).unwrap();
        let string_json = serde_json::to_string(&string_value).unwrap();
        
        assert_eq!(int_json, r#"42"#);
        assert_eq!(float_json, r#"3.14"#);
        assert_eq!(string_json, r#""Hello""#);
    }
    
    #[test]
    fn displays() {
        let int_value = AttributeValue::Integer(42);
        let float_value = AttributeValue::Float(3.14);
        let string_value = AttributeValue::String("Hello".into());
        let integer_float_value = AttributeValue::Float(42.0);
        
        assert_eq!(int_value.to_string(), "42");
        assert_eq!(float_value.to_string(), "3.14");
        assert_eq!(string_value.to_string(), "Hello");
        assert_eq!(integer_float_value.to_string(), "42");
    }
}
