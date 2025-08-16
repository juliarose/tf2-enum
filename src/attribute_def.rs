use crate::{EffectType, DescriptionFormat};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttributeDef {
    pub defindex: u32,
    pub name: &'static str,
    pub attribute_class: Option<&'static str>,
    pub description_string: Option<&'static str>,
    pub description_format: Option<DescriptionFormat>,
    pub effect_type: EffectType,
    pub hidden: bool,
    pub stored_as_integer: bool,
}

impl AttributeDef {
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
