//! Includes commonly-used non-enumerated attributes related to economy items. Included for 
//! convenience.

use crate::{Attribute, AttributeValue, EffectType, DescriptionFormat};

/// Represents the "is_festivized" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IsFestivized;

impl Attribute for IsFestivized {
    const DEFINDEX: u32 = 2053;
    const NAME: &str = "is_festivized";
    const ATTRIBUTE_CLASS: &str = "is_festivized";
    const DESCRIPTION_STRING: Option<&str> = Some("Festivized");
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = Some(DescriptionFormat::ValueIsAdditive);
    const EFFECT_TYPE: EffectType = EffectType::Unusual;
    const HIDDEN: bool = false;
    const STORED_AS_INTEGER: bool = false;
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "is_australium_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IsAustralium;

impl Attribute for IsAustralium {
    const DEFINDEX: u32 = 2027;
    const NAME: &str = "is australium item";
    const ATTRIBUTE_CLASS: &str = "is_australium_item";
    const DESCRIPTION_STRING: Option<&str> = None;
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = Some(DescriptionFormat::ValueIsFromLookupTable);
    const EFFECT_TYPE: EffectType = EffectType::Unusual;
    const HIDDEN: bool = true;
    const STORED_AS_INTEGER: bool = true;
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "kill eater" attribute. This attribute is included with items that have a strange
/// counter, regardless of quality which allows strangified non-strange items to be identified.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct KillEater;

impl Attribute for KillEater {
    const DEFINDEX: u32 = 214;
    const NAME: &str = "kill eater";
    const ATTRIBUTE_CLASS: &str = "kill_eater";
    const DESCRIPTION_STRING: Option<&str> = None;
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = None;
    const EFFECT_TYPE: EffectType = EffectType::Positive;
    const HIDDEN: bool = true;
    const STORED_AS_INTEGER: bool = true;
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "is_australium_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TradableAfterDate(pub u64);

impl Attribute for TradableAfterDate {
    const DEFINDEX: u32 = 211;
    const NAME: &str = "tradable after date";
    const ATTRIBUTE_CLASS: &str = "tradable_after_date";
    const DESCRIPTION_STRING: Option<&str> = Some("\nTradable After: %s1");
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = Some(DescriptionFormat::ValueIsDate);
    const EFFECT_TYPE: EffectType = EffectType::Negative;
    const HIDDEN: bool = true;
    const STORED_AS_INTEGER: bool = true;
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.into())
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "unique_craft_index" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UniqueCraftIndex(pub u64);

impl Attribute for UniqueCraftIndex {
    const DEFINDEX: u32 = 229;
    const NAME: &str = "unique craft index";
    const ATTRIBUTE_CLASS: &str = "unique_craft_index";
    const DESCRIPTION_STRING: Option<&str> = None;
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = None;
    const EFFECT_TYPE: EffectType = EffectType::Positive;
    const HIDDEN: bool = true;
    const STORED_AS_INTEGER: bool = true;
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.into())
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "supply_crate_series" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SupplyCrateSeries(pub u64);

impl Attribute for SupplyCrateSeries {
    const DEFINDEX: u32 = 187;
    const NAME: &str = "set supply crate series";
    const ATTRIBUTE_CLASS: &str = "supply_crate_series";
    const DESCRIPTION_STRING: Option<&str> = Some("Crate Series #%s1");
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = Some(DescriptionFormat::ValueIsAdditive);
    const EFFECT_TYPE: EffectType = EffectType::Positive;
    const HIDDEN: bool = false;
    const STORED_AS_INTEGER: bool = false;
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        Some(self.0 as f64)
    }
}

/// Represents the "custom_name_attr" attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomNameAttr(pub String);

impl Attribute for CustomNameAttr {
    const DEFINDEX: u32 = 500;
    const NAME: &str = "custom name attr";
    const ATTRIBUTE_CLASS: &str = "custom_name_attr";
    const DESCRIPTION_STRING: Option<&str> = None;
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = None;
    const EFFECT_TYPE: EffectType = EffectType::Positive;
    const HIDDEN: bool = true;
    const STORED_AS_INTEGER: bool = false;
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.clone().into())
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "custom_desc_attr" attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomDescAttr(pub String);

impl Attribute for CustomDescAttr {
    const DEFINDEX: u32 = 501;
    const NAME: &str = "custom desc attr";
    const ATTRIBUTE_CLASS: &str = "custom_desc_attr";
    const DESCRIPTION_STRING: Option<&str> = None;
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = None;
    const EFFECT_TYPE: EffectType = EffectType::Positive;
    const HIDDEN: bool = true;
    const STORED_AS_INTEGER: bool = false;

    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.clone().into())
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "gifter_account_id" attribute. The integer refers to the account's 32-bit
/// SteamID.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GifterAccountId(pub u64);

impl Attribute for GifterAccountId {
    const DEFINDEX: u32 = 186;
    const NAME: &str = "gifter account id";
    const ATTRIBUTE_CLASS: &str = "gifter_account_id";
    const DESCRIPTION_STRING: Option<&str> = Some("\nGift from: %s1");
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = Some(DescriptionFormat::ValueIsAccountId);
    const EFFECT_TYPE: EffectType = EffectType::Positive;
    const HIDDEN: bool = true;
    const STORED_AS_INTEGER: bool = true;

    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.into())
    }

    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "event_date" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EventDate(pub u64);

impl Attribute for EventDate {
    const DEFINDEX: u32 = 185;
    const NAME: &str = "event date";
    const ATTRIBUTE_CLASS: &str = "event_date";
    const DESCRIPTION_STRING: Option<&str> = Some("Date Received: %s1");
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = Some(DescriptionFormat::ValueIsDate);
    const EFFECT_TYPE: EffectType = EffectType::Neutral;
    const HIDDEN: bool = true;
    const STORED_AS_INTEGER: bool = true;

    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.into())
    }

    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "custom_texture_lo" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CustomTextureLo(pub u64);

impl Attribute for CustomTextureLo {
    const DEFINDEX: u32 = 152;
    const NAME: &str = "custom texture lo";
    const ATTRIBUTE_CLASS: &str = "custom_texture_lo";
    const DESCRIPTION_STRING: Option<&str> = None;
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = None;
    const EFFECT_TYPE: EffectType = EffectType::Positive;
    const HIDDEN: bool = true;
    const STORED_AS_INTEGER: bool = true;
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.into())
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "custom_texture_hi" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CustomTextureHi(pub u64);

impl Attribute for CustomTextureHi {
    const DEFINDEX: u32 = 227;
    const NAME: &str = "custom texture hi";
    const ATTRIBUTE_CLASS: &str = "custom_texture_hi";
    const DESCRIPTION_STRING: Option<&str> = None;
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = None;
    const EFFECT_TYPE: EffectType = EffectType::Positive;
    const HIDDEN: bool = true;
    const STORED_AS_INTEGER: bool = true;

    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.into())
    }

    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}
 
/// Represents the "makers_mark_id" attribute. The integer refers to the account's 32-bit SteamID
/// of the crafter.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MakersMarkId(pub u64);

impl Attribute for MakersMarkId {
    const DEFINDEX: u32 = 228;
    const NAME: &str = "makers mark id";
    const ATTRIBUTE_CLASS: &str = "makers_mark_id";
    const DESCRIPTION_STRING: Option<&str> = Some("Crafted by %s1");
    const DESCRIPTION_FORMAT: Option<DescriptionFormat> = Some(DescriptionFormat::ValueIsAccountId);
    const EFFECT_TYPE: EffectType = EffectType::Positive;
    const HIDDEN: bool = true;
    const STORED_AS_INTEGER: bool = true;

    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.into())
    }

    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}
