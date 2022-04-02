use strum_macros::{Display, EnumString, EnumIter};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::Attribute;

#[derive(Serialize_repr, Deserialize_repr, Debug, Hash, Eq, PartialEq, Display, EnumString, EnumIter, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum Wear {
    #[strum(serialize = "Factory New")]
    FactoryNew = 1,
    #[strum(serialize = "Minimal Wear")]
    MinimalWear = 2,
    #[strum(serialize = "Field-Tested")]
    FieldTested = 3,
    #[strum(serialize = "Well-Worn")]
    WellWorn = 4,
    #[strum(serialize = "Battle Scarred")]
    BattleScarred = 5
}

impl Attribute for Wear {
    const DEFINDEX: u32 = 725;
}

impl TryFrom<f64> for Wear {
    type Error = TryFromPrimitiveError<Wear>;
    
    fn try_from(float_value: f64) -> Result<Wear, Self::Error> {
        Wear::try_from((float_value * 5.0).round() as u8)
    }
}

impl TryFrom<f32> for Wear {
    type Error = TryFromPrimitiveError<Wear>;
    
    fn try_from(float_value: f32) -> Result<Wear, Self::Error> {
        Wear::try_from((float_value * 5.0).round() as u8)
    }
}