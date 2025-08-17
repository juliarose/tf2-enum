//! Includes commonly-used non-enumerated attributes related to economy items. Included for 
//! convenience.
//! 
//! While the attribute values don't change much, there is no guarantee that Valve won't update
//! them. The defindex values, however, will always remain the same.

use crate::{Attribute, AttributeValue, AttributeDef, EffectType, DescriptionFormat};
use std::ops::Deref;
use std::borrow::Borrow;

macro_rules! impl_from_u64 {
    ($t:ty) => {
        impl From<u64> for $t {
            fn from(val: u64) -> Self {
                Self(val)
            }
        }
        
        impl From<u32> for $t {
            fn from(val: u32) -> Self {
                Self(val as u64)
            }
        }

        impl From<&u64> for $t {
            fn from(val: &u64) -> Self {
                Self(*val)
            }
        }

        impl From<&u32> for $t {
            fn from(val: &u32) -> Self {
                Self(*val as u64)
            }
        }

        impl std::ops::Deref for $t {
            type Target = u64;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<u64> for $t {
            fn as_ref(&self) -> &u64 {
                &self.0
            }
        }

        impl std::borrow::Borrow<u64> for $t {
            fn borrow(&self) -> &u64 {
                &self.0
            }
        }

        impl From<$t> for u64 {
            fn from(val: $t) -> Self {
                val.0
            }
        }
    };
}

/// Macro to implement Attribute for unit-like attribute structs.
macro_rules! impl_unit_attr {
    (
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
            
            fn attribute_value(&self) -> Option<AttributeValue> {
                None
            }
            
            fn attribute_float_value(&self) -> Option<f64> {
                None
            }
        }
    };
}

/// Macro to implement Attribute for newtypes wrapping u64.
/// If "float_value" is present at the end of the macro call, use the second match arm.
macro_rules! impl_u64_attr {
    (
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
            
            fn attribute_value(&self) -> Option<AttributeValue> {
                Some(self.0.into())
            }
            
            fn attribute_float_value(&self) -> Option<f64> {
                None
            }
        }
        
        impl_from_u64!($t);
    };
    (
        $t:ty,
        $defindex:expr,
        $name:expr,
        $attribute_class:expr,
        $description_string:expr,
        $description_format:expr,
        $effect_type:expr,
        $hidden:expr,
        $stored_as_integer:expr,
        float_value
    ) => {
        impl Attribute for $t {
            const DEFINDEX: u32 = $defindex;
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
            
            fn attribute_value(&self) -> Option<AttributeValue> {
                None
            }
            
            fn attribute_float_value(&self) -> Option<f64> {
                Some(self.0 as f64)
            }
        }
        
        impl_from_u64!($t);
    };
}

/// Macro to implement Attribute for newtypes wrapping String.
macro_rules! impl_string_attr {
    (
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
            
            fn attribute_value(&self) -> Option<AttributeValue> {
                Some(self.0.clone().into())
            }

            fn attribute_float_value(&self) -> Option<f64> {
                None
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IsFestivized;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IsAustralium;

impl_unit_attr!(
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

/// Represents the "kill eater" attribute. This attribute is included with items that have a strange
/// counter, regardless of quality which allows strangified non-strange items to be identified.
/// 
/// The value refers to the number of kills, or score count.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct KillEater(pub u64);

impl_u64_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TauntAttachParticleIndex(pub u64);

impl_u64_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetAttachedParticle(pub u64);

impl_u64_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PaintkitProtoDefIndex(pub u64);

impl_u64_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ToolTargetItem(pub u64);

impl_u64_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SupplyCrateSeries(pub u64);

impl_u64_attr!(
    SupplyCrateSeries,
    187,
    "set supply crate series",
    Some("supply_crate_series"),
    Some("Crate Series #%s1"),
    Some(DescriptionFormat::ValueIsAdditive),
    EffectType::Positive,
    false,
    false,
    float_value
);

/// Represents the "series_number" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SeriesNumber(pub u64);

impl_u64_attr!(
    SeriesNumber,
    2031,
    "series number",
    Some("series_number"),
    None,
    None,
    EffectType::ValueIsFromLookupTable,
    true,
    false,
    float_value
);

/// Represents the "custom_name_attr" attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomNameAttr(pub String);

impl_string_attr!(
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomDescAttr(pub String);

impl_string_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UniqueCraftIndex(pub u64);

impl_u64_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MakersMarkId(pub u64);

impl_u64_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GifterAccountId(pub u64);

impl_u64_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EventDate(pub u64);

impl_u64_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TradableAfterDate(pub u64);

impl_u64_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CustomTextureLo(pub u64);

impl_u64_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CustomTextureHi(pub u64);

impl_u64_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HalloweenVoiceModulation;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HalloweenPumpkinExplosions;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HalloweenGreenFlames;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HalloweenDeathGhosts;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem1;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem2;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem3;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem4;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem5;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem6;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem7;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem8;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem9;

impl_unit_attr!(
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem10;

impl_unit_attr!(
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn uses_correct_value() {
        assert!(SeriesNumber::from(1u64).attribute_float_value().is_some());
        assert_eq!(SeriesNumber::from(1u64).attribute_float_value().unwrap(), 1.0);
        assert!(SeriesNumber::from(1u64).attribute_value().is_none());
        assert!(SupplyCrateSeries::from(42u32).attribute_float_value().is_some());
        assert_eq!(SupplyCrateSeries::from(42u32).attribute_float_value().unwrap(), 42.0);
        
        assert!(KillEater::from(123u64).attribute_value().is_some());
        assert_eq!(KillEater::from(123u64).attribute_value().unwrap(), AttributeValue::from(123u64));
        
        assert!(CustomNameAttr::from("TestName").attribute_value().is_some());
        assert_eq!(CustomNameAttr::from("TestName").attribute_value().unwrap(), AttributeValue::from("TestName"));
        
        assert!(IsAustralium.attribute_value().is_none());
        assert!(IsAustralium.attribute_float_value().is_none());
        assert!(HalloweenVoiceModulation.attribute_value().is_none());
    }
}