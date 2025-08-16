use serde::{Deserialize, Serialize};

/// A value for an attribute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttributeValue {
    /// Represents an integer attribute value.
    Integer(u64),
    /// Represents a float attribute value.
    Float(f64),
    /// Represents a string attribute value.
    String(String),
}

impl From<u64> for AttributeValue {
    fn from(value: u64) -> Self {
        AttributeValue::Integer(value)
    }
}

impl From<u32> for AttributeValue {
    fn from(value: u32) -> Self {
        AttributeValue::Integer(value as u64)
    }
}

impl From<f64> for AttributeValue {
    fn from(value: f64) -> Self {
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
}
