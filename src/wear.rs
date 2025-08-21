use crate::{
    Attribute,
    AttributeDef,
    EffectType,
    ItemAttribute,
    TryFromIntAttributeValue,
};
use crate::error::TryFromPrimitiveError;
use strum::{Display, EnumCount, EnumIter, EnumString};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Wear.
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
    Serialize_repr,
    Deserialize_repr,
    EnumString,
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
)]
#[repr(u32)]
#[strum(serialize_all = "title_case")]
#[allow(missing_docs)]
pub enum Wear {
    FactoryNew = 1,
    MinimalWear = 2,
    #[strum(serialize = "Field-Tested")]
    FieldTested = 3,
    #[strum(serialize = "Well-Worn")]
    WellWorn = 4,
    BattleScarred = 5,
}

impl Wear {
    /// Converts the wear to a float value.
    #[inline]
    pub fn as_float(&self) -> f32 {
        match self {
            Self::FactoryNew => 0.2,
            Self::MinimalWear => 0.4,
            Self::FieldTested => 0.6,
            Self::WellWorn => 0.8,
            Self::BattleScarred => 1.0,
        }
    }
}

impl Attribute for Wear {
    const DEFINDEX: u32 = 725;
    const USES_FLOAT_VALUE: bool = true;
    /// Represents the "set_item_texture_wear" attribute.
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: Self::DEFINDEX,
        name: "set_item_texture_wear",
        attribute_class: Some("set_item_texture_wear"),
        description_string: None,
        description_format: None,
        effect_type: EffectType::Positive,
        hidden: true,
        stored_as_integer: false,
    };
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f32> {
        // This could be done using arithmetic but this is a little more explicit.
        Some(self.as_float())
    }
}

impl TryFromIntAttributeValue for Wear {
    fn try_from_attribute_float_value(v: f32) -> Option<Self> {
        Self::try_from(v).ok()
    }
}

impl TryFrom<f64> for Wear {
    type Error = TryFromPrimitiveError<Self>;
    
    fn try_from(float_value: f64) -> Result<Wear, Self::Error> {
        Wear::try_from((float_value * 5.0).round() as u32)
    }
}

impl TryFrom<&f64> for Wear {
    type Error = TryFromPrimitiveError<Self>;
    
    fn try_from(float_value: &f64) -> Result<Wear, Self::Error> {
        Wear::try_from(*float_value)
    }
}

impl TryFrom<f32> for Wear {
    type Error = TryFromPrimitiveError<Self>;
    
    fn try_from(float_value: f32) -> Result<Wear, Self::Error> {
        Wear::try_from((float_value * 5.0).round() as u32)
    }
}

impl TryFrom<&f32> for Wear {
    type Error = TryFromPrimitiveError<Self>;
    
    fn try_from(float_value: &f32) -> Result<Wear, Self::Error> {
        Wear::try_from(*float_value)
    }
}

impl From<Wear> for ItemAttribute {
    fn from(val: Wear) -> Self {
        ItemAttribute {
            defindex: Wear::DEFINDEX,
            value: val.attribute_value(),
            float_value: val.attribute_float_value(),
        }
    }
}
