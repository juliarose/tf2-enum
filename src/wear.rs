use crate::{Attribute, AttributeValue, AttributeDef, EffectType};
use crate::error::TryFromPrimitiveError;
use strum::{Display, EnumString, EnumIter, EnumCount};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Wear.
#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Display,
    EnumString,
    EnumIter,
    EnumCount,
    TryFromPrimitive,
    IntoPrimitive,
    Clone,
    Copy,
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

/// Represents the "set_item_texture_wear" attribute.
impl Attribute for Wear {
    const DEFINDEX: u32 = 725;
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
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        // This could be done using arithmetic but this is a little more explicit.
        Some(match self {
            Self::FactoryNew => 0.2,
            Self::MinimalWear => 0.4,
            Self::FieldTested => 0.6,
            Self::WellWorn => 0.8,
            Self::BattleScarred => 1.0,
        })
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
