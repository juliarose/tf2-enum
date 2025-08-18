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
    /// Not part of the schema - this is a marker to specify which attribute field is meaningful to
    /// us in obtaining the attribute's value.
    /// 
    /// # Kill eaters example
    /// ```json
    /// {
    ///     "defindex": 214,
    ///     "value": 918,
    ///     "float_value": 1.28639199025018207e-42
    /// },
    /// ```
    /// This is the "kill_eater" attribute. "918" refers to the number of kills. The `float_value`
    /// field is the same number as a 32-bit float.
    /// 
    /// You can perform the conversions yourself with the following code:
    /// ```
    /// let value = 918u32;
    /// let float_value = 1.28639199025018207e-42f32;
    /// 
    /// assert_eq!(f32::from_bits(value), float_value);
    /// assert_eq!(float_value.to_bits(), value);
    /// ```
    /// 
    /// # Sheens example
    /// ```json
    /// {
    ///     "defindex": 2014,
    ///     "value": 1086324736,
    ///     "float_value": 6
    /// }
    /// ```
    /// This is the "killstreak_idleeffect" attribute. "6" refers to the associated sheen (
    /// [`crate::Sheen::VillainousViolet`]), but is stored in the `float_value` field, unlike
    /// "kill_eater". The `value` field is the same number as a 32-bit float.
    /// 
    /// While both values refer to the same value, and internally the attribute's value is its
    /// `float_value`, there are many cases where the `float_value` doesn't mean anything to us
    /// unless converted to a 32-bit integer from its bits, and if the `float_value` does mean
    /// something to us we don't want to convert it to an integer. This can be a little confusing,
    /// but it's just how the API is.
    /// 
    /// By marking each attribute with a `uses_float_value` flag, we can indicate whether the
    /// `float_value` field is meaningful to use for that attribute.
    pub uses_float_value: bool,
}

impl AttributeDef {
    /// Returns the description of the attribute with the supplied value.
    pub fn description<F>(&self, value: Option<F>) -> Option<String>
    where
        F: std::fmt::Display,
    {
        if let Some(description_string) = self.description_string {
            if let Some(value) = value {
                return description_string
                    .replace("%s1", &value.to_string())
                    .into();
            } else {
                return Some(description_string.into());
            }   
        }
        
        None
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
