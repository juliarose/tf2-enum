//! Includes commonly-used non-enumerated attributes related to economy items. Included for 
//! convenience.
//! 
//! While the attribute values don't change much, there is no guarantee that Valve won't update
//! them. The defindex values, however, will always remain the same.

use crate::{
    Attribute,
    Attributes,
    AttributeDef,
    AttributeValue,
    DescriptionFormat,
    EffectType,
    ItemAttribute,
    TryFromIntAttributeValue,
};
use std::borrow::Borrow;
use std::ops::Deref;

macro_rules! impl_from_u32 {
    ($t:ty) => {
        impl TryFromIntAttributeValue for $t {}
        
        impl From<u32> for $t {
            fn from(val: u32) -> Self {
                Self(val)
            }
        }
        
        impl From<&u32> for $t {
            fn from(val: &u32) -> Self {
                Self(*val)
            }
        }
        
        impl std::ops::Deref for $t {
            type Target = u32;
            
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<u32> for $t {
            fn as_ref(&self) -> &u32 {
                &self.0
            }
        }

        impl std::borrow::Borrow<u32> for $t {
            fn borrow(&self) -> &u32 {
                &self.0
            }
        }
    };
}
    
macro_rules! impl_attr {
    (
        unit,
        $t:ty,
        $defindex:expr,
        $name:expr,
        $attribute_class:expr,
        $description_string:expr,
        $description_format:expr,
        $effect_type:expr,
        $hidden:expr,
        $stored_as_integer:expr
    ) => {
        impl Attribute for $t {
            const DEFINDEX: u32 = $defindex;
            const USES_FLOAT_VALUE: bool = true;
            const ATTRIBUTE: AttributeDef = AttributeDef {
                defindex: $defindex,
                name: $name,
                attribute_class: $attribute_class,
                description_string: $description_string,
                description_format: $description_format,
                effect_type: $effect_type,
                hidden: $hidden,
                stored_as_integer: $stored_as_integer,
            };
            
            // These attributes are booleans but our structs are unit types because we assume if an
            // item has attribute that this is set to true.
            fn attribute_float_value(&self) -> Option<f32> {
                Some(1.0)
            }
        }
    };
    (
        u32,
        $t:ty,
        $defindex:expr,
        $name:expr,
        $attribute_class:expr,
        $description_string:expr,
        $description_format:expr,
        $effect_type:expr,
        $hidden:expr,
        $stored_as_integer:expr
    ) => {
        impl Attribute for $t {
            const DEFINDEX: u32 = $defindex;
            const USES_FLOAT_VALUE: bool = false;
            const ATTRIBUTE: AttributeDef = AttributeDef {
                defindex: $defindex,
                name: $name,
                attribute_class: $attribute_class,
                description_string: $description_string,
                description_format: $description_format,
                effect_type: $effect_type,
                hidden: $hidden,
                stored_as_integer: $stored_as_integer,
            };
            
            fn attribute_value(&self) -> AttributeValue {
                self.0.into()
            }
            
            fn attribute_float_value(&self) -> Option<f32> {
                None
            }
        }
        
        impl From<$t> for ItemAttribute {
            fn from(val: $t) -> Self {
                ItemAttribute {
                    defindex: <$t as Attribute>::DEFINDEX,
                    value: val.attribute_value(),
                    float_value: val.attribute_float_value(),
                }
            }
        }
        
        impl_from_u32!($t);
    };
    (
        u32_float,
        $t:ty,
        $defindex:expr,
        $name:expr,
        $attribute_class:expr,
        $description_string:expr,
        $description_format:expr,
        $effect_type:expr,
        $hidden:expr,
        $stored_as_integer:expr
    ) => {
        impl Attribute for $t {
            const DEFINDEX: u32 = $defindex;
            const USES_FLOAT_VALUE: bool = true;
            const ATTRIBUTE: AttributeDef = AttributeDef {
                defindex: $defindex,
                name: $name,
                attribute_class: $attribute_class,
                description_string: $description_string,
                description_format: $description_format,
                effect_type: $effect_type,
                hidden: $hidden,
                stored_as_integer: $stored_as_integer,
            };
            
            fn attribute_float_value(&self) -> Option<f32> {
                Some(self.0 as f32)
            }
        }
        
        impl_from_u32!($t);
    };
    (
        string,
        $t:ty,
        $defindex:expr,
        $name:expr,
        $attribute_class:expr,
        $description_string:expr,
        $description_format:expr,
        $effect_type:expr,
        $hidden:expr,
        $stored_as_integer:expr
    ) => {
        impl Attribute for $t {
            const DEFINDEX: u32 = $defindex;
            const USES_FLOAT_VALUE: bool = false;
            const ATTRIBUTE: AttributeDef = AttributeDef {
                defindex: $defindex,
                name: $name,
                attribute_class: $attribute_class,
                description_string: $description_string,
                description_format: $description_format,
                effect_type: $effect_type,
                hidden: $hidden,
                stored_as_integer: $stored_as_integer,
            };
            
            fn attribute_value(&self) -> AttributeValue {
                self.0.clone().into()
            }

            fn attribute_float_value(&self) -> Option<f32> {
                None
            }
        }
        
        impl From<$t> for ItemAttribute {
            fn from(val: $t) -> Self {
                ItemAttribute {
                    defindex: <$t as Attribute>::DEFINDEX,
                    value: val.attribute_value(),
                    float_value: val.attribute_float_value(),
                }
            }
        }
        
        impl From<String> for $t {
            fn from(val: String) -> Self {
                Self(val)
            }
        }
        
        impl From<&String> for $t {
            fn from(val: &String) -> Self {
                Self(val.to_owned())
            }
        }
        
        impl From<&str> for $t {
            fn from(val: &str) -> Self {
                Self(val.to_owned())
            }
        }
        
        impl Deref for $t {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<str> for $t {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl Borrow<str> for $t {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl From<$t> for String {
            fn from(val: $t) -> Self {
                val.0
            }
        }
    };
}

/// Represents the "is_festivized" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IsFestivized;

impl_attr!(
    unit,
    IsFestivized,
    2053,
    "is_festivized",
    Some("is_festivized"),
    Some("Festivized"),
    Some(DescriptionFormat::ValueIsAdditive),
    EffectType::Unusual,
    false,
    false
);

/// Represents the "is_australium_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IsAustralium;

impl_attr!(
    unit,
    IsAustralium,
    2027,
    "is australium item",
    Some("is_australium_item"),
    None,
    Some(DescriptionFormat::ValueIsFromLookupTable),
    EffectType::Unusual,
    true,
    true
);

/// Represents the "kill eater" attribute.
/// 
/// This attribute is included with items that have a strange counter, regardless of quality which
/// allows strangified non-strange items to be identified.
/// 
/// The value refers to the number of kills, or score count.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KillEater(pub u32);

impl_attr!(
    u32,
    KillEater,
    214,
    "kill eater",
    Some("kill_eater"),
    None,
    None,
    EffectType::Positive,
    true,
    true
);

/// Represents the "taunt attach particle index" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TauntAttachParticleIndex(pub u32);

impl_attr!(
    u32,
    TauntAttachParticleIndex,
    2041,
    "taunt attach particle index",
    None,
    Some("★ Unusual Effect: %s1"),
    Some(DescriptionFormat::ValueIsParticleIndex),
    EffectType::Unusual,
    false,
    true
);

/// Represents the "set_attached_particle" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetAttachedParticle(pub u32);

impl_attr!(
    u32_float,
    SetAttachedParticle,
    134,
    "attach particle effect",
    Some("set_attached_particle"),
    Some("★ Unusual Effect: %s1"),
    Some(DescriptionFormat::ValueIsParticleIndex),
    EffectType::Unusual,
    false,
    false
);

/// Represents the "paintkit_proto_def_index" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PaintkitProtoDefIndex(pub u32);

impl_attr!(
    u32,
    PaintkitProtoDefIndex,
    834,
    "paintkit_proto_def_index",
    Some("paintkit_proto_def_index"),
    None,
    Some(DescriptionFormat::ValueIsAdditive),
    EffectType::Neutral,
    false,
    true
);

/// Represents the "tool_target_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ToolTargetItem(pub u32);

impl_attr!(
    u32_float,
    ToolTargetItem,
    2012,
    "tool target item",
    Some("tool_target_item"),
    None,
    None,
    EffectType::ValueIsFromLookupTable,
    true,
    false
);

/// Represents the "supply_crate_series" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SupplyCrateSeries(pub u32);

impl_attr!(
    u32_float,
    SupplyCrateSeries,
    187,
    "set supply crate series",
    Some("supply_crate_series"),
    Some("Crate Series #%s1"),
    Some(DescriptionFormat::ValueIsAdditive),
    EffectType::Positive,
    false,
    false
);

/// Represents the "series_number" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeriesNumber(pub u32);

impl_attr!(
    u32_float,
    SeriesNumber,
    2031,
    "series number",
    Some("series_number"),
    None,
    None,
    EffectType::ValueIsFromLookupTable,
    true,
    false
);

/// Represents the "custom_name_attr" attribute.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct CustomNameAttr(pub String);

impl_attr!(
    string,
    CustomNameAttr,
    500,
    "custom name attr",
    Some("custom_name_attr"),
    None,
    None,
    EffectType::Positive,
    true,
    false
);

/// Represents the "custom_desc_attr" attribute.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct CustomDescAttr(pub String);

impl_attr!(
    string,
    CustomDescAttr,
    501,
    "custom desc attr",
    Some("custom_desc_attr"),
    None,
    None,
    EffectType::Positive,
    true,
    false
);

/// Represents the "unique_craft_index" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UniqueCraftIndex(pub u32);

impl_attr!(
    u32,
    UniqueCraftIndex,
    229,
    "unique craft index",
    Some("unique_craft_index"),
    None,
    None,
    EffectType::Positive,
    true,
    true
);

/// Represents the "makers_mark_id" attribute. The integer refers to the account's 32-bit SteamID
/// of the crafter.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MakersMarkId(pub u32);

impl_attr!(
    u32,
    MakersMarkId,
    228,
    "makers mark id",
    Some("makers_mark_id"),
    Some("Crafted by %s1"),
    Some(DescriptionFormat::ValueIsAccountId),
    EffectType::Positive,
    true,
    true
);

/// Represents the "gifter_account_id" attribute. The integer refers to the account's 32-bit
/// SteamID.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GifterAccountId(pub u32);

impl_attr!(
    u32,
    GifterAccountId,
    186,
    "gifter account id",
    Some("gifter_account_id"),
    Some("\nGift from: %s1"),
    Some(DescriptionFormat::ValueIsAccountId),
    EffectType::Positive,
    true,
    true
);

/// Represents the "event_date" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventDate(pub u32);

impl_attr!(
    u32,
    EventDate,
    185,
    "event date",
    Some("event_date"),
    Some("Date Received: %s1"),
    Some(DescriptionFormat::ValueIsDate),
    EffectType::Neutral,
    true,
    true
);

/// Represents the "is_australium_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TradableAfterDate(pub u32);

impl_attr!(
    u32,
    TradableAfterDate,
    211,
    "tradable after date",
    Some("tradable_after_date"),
    Some("\nTradable After: %s1"),
    Some(DescriptionFormat::ValueIsDate),
    EffectType::Negative,
    true,
    true
);

/// Represents the "custom_texture_lo" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomTextureLo(pub u32);

impl_attr!(
    u32,
    CustomTextureLo,
    152,
    "custom texture lo",
    Some("custom_texture_lo"),
    None,
    None,
    EffectType::Positive,
    true,
    true
);

/// Represents the "custom_texture_hi" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomTextureHi(pub u32);

impl_attr!(
    u32,
    CustomTextureHi,
    227,
    "custom texture hi",
    Some("custom_texture_hi"),
    None,
    None,
    EffectType::Positive,
    true,
    true
);

/// Represents the "halloween_voice_modulation" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HalloweenVoiceModulation;

impl_attr!(
    unit,
    HalloweenVoiceModulation,
    1006,
    "SPELL: Halloween voice modulation",
    Some("halloween_voice_modulation"),
    Some("Voices from Below"),
    Some(DescriptionFormat::ValueIsAdditive),
    EffectType::Positive,
    false,
    false
);

/// Represents the "halloween_pumpkin_explosions" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HalloweenPumpkinExplosions;

impl_attr!(
    unit,
    HalloweenPumpkinExplosions,
    1007,
    "SPELL: Halloween pumpkin explosions",
    Some("halloween_pumpkin_explosions"),
    Some("Pumpkin Bombs"),
    Some(DescriptionFormat::ValueIsAdditive),
    EffectType::Positive,
    false,
    false
);

/// Represents the "halloween_green_flames" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HalloweenGreenFlames;

impl_attr!(
    unit,
    HalloweenGreenFlames,
    1008,
    "SPELL: Halloween green flames",
    Some("halloween_green_flames"),
    Some("Halloween Fire"),
    Some(DescriptionFormat::ValueIsAdditive),
    EffectType::Positive,
    false,
    false
);

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HalloweenDeathGhosts;

impl_attr!(
    unit,
    HalloweenDeathGhosts,
    1009,
    "SPELL: Halloween death ghosts",
    Some("halloween_death_ghosts"),
    Some("Exorcism"),
    Some(DescriptionFormat::ValueIsAdditive),
    EffectType::Positive,
    false,
    false
);

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicRecipeComponentDefinedItem1;

impl_attr!(
    unit,
    DynamicRecipeComponentDefinedItem1,
    2000,
    "recipe component defined item 1",
    Some("dynamic_recipe_component_defined_item"),
    None,
    Some(DescriptionFormat::ValueIsFromLookupTable),
    EffectType::Neutral,
    false,
    false
);

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicRecipeComponentDefinedItem2;

impl_attr!(
    unit,
    DynamicRecipeComponentDefinedItem2,
    2001,
    "recipe component defined item 2",
    Some("dynamic_recipe_component_defined_item"),
    None,
    Some(DescriptionFormat::ValueIsFromLookupTable),
    EffectType::Neutral,
    false,
    false
);

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicRecipeComponentDefinedItem3;

impl_attr!(
    unit,
    DynamicRecipeComponentDefinedItem3,
    2002,
    "recipe component defined item 3",
    Some("dynamic_recipe_component_defined_item"),
    None,
    Some(DescriptionFormat::ValueIsFromLookupTable),
    EffectType::Neutral,
    false,
    false
);

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicRecipeComponentDefinedItem4;

impl_attr!(
    unit,
    DynamicRecipeComponentDefinedItem4,
    2003,
    "recipe component defined item 4",
    Some("dynamic_recipe_component_defined_item"),
    None,
    Some(DescriptionFormat::ValueIsFromLookupTable),
    EffectType::Neutral,
    false,
    false
);

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicRecipeComponentDefinedItem5;

impl_attr!(
    unit,
    DynamicRecipeComponentDefinedItem5,
    2004,
    "recipe component defined item 5",
    Some("dynamic_recipe_component_defined_item"),
    None,
    Some(DescriptionFormat::ValueIsFromLookupTable),
    EffectType::Neutral,
    false,
    false
);

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicRecipeComponentDefinedItem6;

impl_attr!(
    unit,
    DynamicRecipeComponentDefinedItem6,
    2005,
    "recipe component defined item 6",
    Some("dynamic_recipe_component_defined_item"),
    None,
    Some(DescriptionFormat::ValueIsFromLookupTable),
    EffectType::Neutral,
    false,
    false
);

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicRecipeComponentDefinedItem7;

impl_attr!(
    unit,
    DynamicRecipeComponentDefinedItem7,
    2006,
    "recipe component defined item 7",
    Some("dynamic_recipe_component_defined_item"),
    None,
    Some(DescriptionFormat::ValueIsFromLookupTable),
    EffectType::Neutral,
    false,
    false
);

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicRecipeComponentDefinedItem8;

impl_attr!(
    unit,
    DynamicRecipeComponentDefinedItem8,
    2007,
    "recipe component defined item 8",
    Some("dynamic_recipe_component_defined_item"),
    None,
    Some(DescriptionFormat::ValueIsFromLookupTable),
    EffectType::Neutral,
    false,
    false
);

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicRecipeComponentDefinedItem9;

impl_attr!(
    unit,
    DynamicRecipeComponentDefinedItem9,
    2008,
    "recipe component defined item 9",
    Some("dynamic_recipe_component_defined_item"),
    None,
    Some(DescriptionFormat::ValueIsFromLookupTable),
    EffectType::Neutral,
    false,
    false
);

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicRecipeComponentDefinedItem10;

impl_attr!(
    unit,
    DynamicRecipeComponentDefinedItem10,
    2009,
    "recipe component defined item 10",
    Some("dynamic_recipe_component_defined_item"),
    None,
    Some(DescriptionFormat::ValueIsFromLookupTable),
    EffectType::Neutral,
    false,
    false
);

/// Represents the "dynamic_recipe_component_defined_item" attributes.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicRecipeComponentDefinedItem;

impl Attributes for DynamicRecipeComponentDefinedItem {
    const DEFINDEX: &'static [u32] = &[
        2000,
        2001,
        2002,
        2003,
        2004,
        2005,
        2006,
        2007,
        2008,
        2009,
    ];
    const USES_FLOAT_VALUE: bool = true;
    const ATTRIBUTES: &'static [AttributeDef] = &[
        DynamicRecipeComponentDefinedItem1::ATTRIBUTE,
        DynamicRecipeComponentDefinedItem2::ATTRIBUTE,
        DynamicRecipeComponentDefinedItem3::ATTRIBUTE,
        DynamicRecipeComponentDefinedItem4::ATTRIBUTE,
        DynamicRecipeComponentDefinedItem5::ATTRIBUTE,
        DynamicRecipeComponentDefinedItem6::ATTRIBUTE,
        DynamicRecipeComponentDefinedItem7::ATTRIBUTE,
        DynamicRecipeComponentDefinedItem8::ATTRIBUTE,
        DynamicRecipeComponentDefinedItem9::ATTRIBUTE,
        DynamicRecipeComponentDefinedItem10::ATTRIBUTE,
    ];
}

/// Represents the "kill_eater", "kill_eater_2", and "kill_eater_3" attributes. The integer
/// refers to the score count.
pub struct KillEaterScore(pub u32);

impl Attributes for KillEaterScore {
    const DEFINDEX: &'static [u32] = &[
        214,
        294,
        494,
    ];
    const USES_FLOAT_VALUE: bool = false;
    const ATTRIBUTES: &'static [AttributeDef] = &[
        AttributeDef {
            defindex: 214,
            name: "kill eater",
            attribute_class: Some("kill_eater"),
            description_string: None,
            description_format: None,
            effect_type: EffectType::Positive,
            hidden: true,
            stored_as_integer: true,
        },
        AttributeDef {
            defindex: 294,
            name: "kill eater 2",
            attribute_class: Some("kill_eater_2"),
            description_string: None,
            description_format: None,
            effect_type: EffectType::Positive,
            hidden: true,
            stored_as_integer: true,
        },
        AttributeDef {
            defindex: 494,
            name: "kill eater 3",
            attribute_class: Some("kill_eater_3"),
            description_string: None,
            description_format: None,
            effect_type: EffectType::Positive,
            hidden: true,
            stored_as_integer: true,
        },
    ];
}

/// Represents the "kill_eater_user_1", "kill_eater_user_2", and "kill_eater_user_3" attributes.
pub struct KillEaterUserScore(pub u32);

impl Attributes for KillEaterUserScore {
    const DEFINDEX: &'static [u32] = &[
        379,
        381,
        383,
    ];
    const USES_FLOAT_VALUE: bool = false;
    const ATTRIBUTES: &'static [AttributeDef] = &[
        AttributeDef {
            defindex: 379,
            name: "kill eater user 1",
            attribute_class: Some("kill_eater_user_1"),
            description_string: None,
            description_format: None,
            effect_type: EffectType::Positive,
            hidden: true,
            stored_as_integer: true,
        },
        AttributeDef {
            defindex: 381,
            name: "kill eater user 2",
            attribute_class: Some("kill_eater_user_2"),
            description_string: None,
            description_format: None,
            effect_type: EffectType::Positive,
            hidden: true,
            stored_as_integer: true,
        },
        AttributeDef {
            defindex: 383,
            name: "kill eater user 3",
            attribute_class: Some("kill_eater_user_3"),
            description_string: None,
            description_format: None,
            effect_type: EffectType::Positive,
            hidden: true,
            stored_as_integer: true,
        },
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn uses_correct_value() {
        assert!(SeriesNumber::from(1u32).attribute_float_value().is_some());
        assert_eq!(SeriesNumber::from(1u32).attribute_float_value().unwrap(), 1.0);
        assert!(SupplyCrateSeries::from(42u32).attribute_float_value().is_some());
        assert_eq!(SupplyCrateSeries::from(42u32).attribute_float_value().unwrap(), 42.0);
        assert_eq!(KillEater::from(123u32).attribute_value(), AttributeValue::from(123u32));
        assert_eq!(CustomNameAttr::from("TestName").attribute_value(), AttributeValue::from("TestName"));
    }
    
    #[test]
    fn equals() {
        assert_eq!(SupplyCrateSeries::from(1u32), SupplyCrateSeries::from(1u32));
        assert_eq!(*SupplyCrateSeries::from(1u32).deref(), 1u32);
    }
    
    #[test]
    fn gets_attribute_values() {
        let series_number = SeriesNumber::from(2u32);
        let series_number_value_float = series_number.attribute_float_value().unwrap();
        let series_number_value = series_number.attribute_value();
        
        assert_eq!(series_number_value_float, 2.0);
        assert_eq!(series_number_value, AttributeValue::from(series_number_value_float.to_bits()));
    }
}