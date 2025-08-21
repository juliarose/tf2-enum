use crate::{EffectType, DescriptionFormat};

/// Represents the definition of an attribute in the schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttributeDef {
    /// The unique identifier for the attribute.
    pub defindex: u32,
    /// The name of the attribute.
    pub name: &'static str,
    /// The attribute class of the attribute.
    pub attribute_class: Option<&'static str>,
    /// The description string of the attribute.
    pub description_string: Option<&'static str>,
    /// The description format of the attribute.
    pub description_format: Option<DescriptionFormat>,
    /// The effect type of the description.
    pub effect_type: EffectType,
    /// Indicates whether the description is hidden from display.
    pub hidden: bool,
    /// Indicates whether the attribute's value is stored as an integer.
    pub stored_as_integer: bool,
}

impl AttributeDef {
    /// Returns the description of the attribute with the supplied value.
    pub fn description<F>(&self, value: Option<F>) -> Option<String>
    where
        F: std::fmt::Display,
    {
        let description_string = self.description_string.as_ref()?;
        
        if let Some(value) = value {
            return description_string
                .replace("%s1", &value.to_string())
                .into();
        }
            
        Some(description_string.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Attribute, Sheen};
    
    #[test]
    fn formats_description() {
        let sheen = Sheen::TeamShine;
        let formatted = Sheen::ATTRIBUTE.description(Some(&sheen));
        
        assert_eq!(formatted, Some("Sheen: Team Shine".into()));
    }
}
