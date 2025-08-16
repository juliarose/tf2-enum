//! Includes commonly-used non-enumerated attributes related to economy items. Included for 
//! convenience.

use crate::{Attribute, AttributeValue, AttributeDef, EffectType, DescriptionFormat};

/// Represents the "is_festivized" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IsFestivized;

impl Attribute for IsFestivized {
    const DEFINDEX: u32 = 2053;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2053,
        name: "is_festivized",
        attribute_class: Some("is_festivized"),
        description_string: Some("Festivized"),
        description_format: Some(DescriptionFormat::ValueIsAdditive),
        effect_type: EffectType::Unusual,
        hidden: false,
        stored_as_integer: false,
    };
    
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
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2027,
        name: "is australium item",
        attribute_class: Some("is_australium_item"),
        description_string: None,
        description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
        effect_type: EffectType::Unusual,
        hidden: true,
        stored_as_integer: true,
    };
    
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
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 214,
        name: "kill eater",
        attribute_class: Some("kill_eater"),
        description_string: None,
        description_format: None,
        effect_type: EffectType::Positive,
        hidden: true,
        stored_as_integer: true,
    };
    
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
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 211,
        name: "tradable after date",
        attribute_class: Some("tradable_after_date"),
        description_string: Some("\nTradable After: %s1"),
        description_format: Some(DescriptionFormat::ValueIsDate),
        effect_type: EffectType::Negative,
        hidden: true,
        stored_as_integer: true,
    };
    
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
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 229,
        name: "unique craft index",
        attribute_class: Some("unique_craft_index"),
        description_string: None,
        description_format: None,
        effect_type: EffectType::Positive,
        hidden: true,
        stored_as_integer: true,
    };
    
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
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 187,
        name: "set supply crate series",
        attribute_class: Some("supply_crate_series"),
        description_string: Some("Crate Series #%s1"),
        description_format: Some(DescriptionFormat::ValueIsAdditive),
        effect_type: EffectType::Positive,
        hidden: false,
        stored_as_integer: false,
    };
    
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
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 500,
        name: "custom name attr",
        attribute_class: Some("custom_name_attr"),
        description_string: None,
        description_format: None,
        effect_type: EffectType::Positive,
        hidden: true,
        stored_as_integer: false,
    };
    
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
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 501,
        name: "custom desc attr",
        attribute_class: Some("custom_desc_attr"),
        description_string: None,
        description_format: None,
        effect_type: EffectType::Positive,
        hidden: true,
        stored_as_integer: false,
    };

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
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 186,
        name: "gifter account id",
        attribute_class: Some("gifter_account_id"),
        description_string: Some("\nGift from: %s1"),
        description_format: Some(DescriptionFormat::ValueIsAccountId),
        effect_type: EffectType::Positive,
        hidden: true,
        stored_as_integer: true,
    };

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
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 185,
        name: "event date",
        attribute_class: Some("event_date"),
        description_string: Some("Date Received: %s1"),
        description_format: Some(DescriptionFormat::ValueIsDate),
        effect_type: EffectType::Neutral,
        hidden: true,
        stored_as_integer: true,
    };

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
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 152,
        name: "custom texture lo",
        attribute_class: Some("custom_texture_lo"),
        description_string: None,
        description_format: None,
        effect_type: EffectType::Positive,
        hidden: true,
        stored_as_integer: true,
    };
    
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
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 227,
        name: "custom texture hi",
        attribute_class: Some("custom_texture_hi"),
        description_string: None,
        description_format: None,
        effect_type: EffectType::Positive,
        hidden: true,
        stored_as_integer: true,
    };

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
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 228,
        name: "makers mark id",
        attribute_class: Some("makers_mark_id"),
        description_string: Some("Crafted by %s1"),
        description_format: Some(DescriptionFormat::ValueIsAccountId),
        effect_type: EffectType::Positive,
        hidden: true,
        stored_as_integer: true,
    };
    
    /// Gets the attribute value.
    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.into())
    }
    
    /// Gets the attribute float value.
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "halloween_voice_modulation" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HalloweenVoiceModulation;

impl Attribute for HalloweenVoiceModulation {
    const DEFINDEX: u32 = 1006;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 1006,
        name: "SPELL: Halloween voice modulation",
        attribute_class: Some("halloween_voice_modulation"),
        description_string: Some("Voices from Below"),
        description_format: Some(DescriptionFormat::ValueIsAdditive),
        effect_type: EffectType::Positive,
        hidden: false,
        stored_as_integer: false,
    };

    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }

    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "halloween_pumpkin_explosions" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HalloweenPumpkinExplosions;

impl Attribute for HalloweenPumpkinExplosions {
    const DEFINDEX: u32 = 1007;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 1007,
        name: "SPELL: Halloween pumpkin explosions",
        attribute_class: Some("halloween_pumpkin_explosions"),
        description_string: Some("Pumpkin Bombs"),
        description_format: Some(DescriptionFormat::ValueIsAdditive),
        effect_type: EffectType::Positive,
        hidden: false,
        stored_as_integer: false,
    };

    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }

    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "halloween_green_flames" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HalloweenGreenFlames;

impl Attribute for HalloweenGreenFlames {
    const DEFINDEX: u32 = 1008;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 1008,
        name: "SPELL: Halloween green flames",
        attribute_class: Some("halloween_green_flames"),
        description_string: Some("Halloween Fire"),
        description_format: Some(DescriptionFormat::ValueIsAdditive),
        effect_type: EffectType::Positive,
        hidden: false,
        stored_as_integer: false,
    };

    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HalloweenDeathGhosts;

impl Attribute for HalloweenDeathGhosts {
    const DEFINDEX: u32 = 1009;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 1009,
        name: "SPELL: Halloween death ghosts",
        attribute_class: Some("halloween_death_ghosts"),
        description_string: Some("Exorcism"),
        description_format: Some(DescriptionFormat::ValueIsAdditive),
        effect_type: EffectType::Positive,
        hidden: false,
        stored_as_integer: false,
    };
    
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem1;

impl Attribute for DynamicRecipeComponentDefinedItem1 {
    const DEFINDEX: u32 = 2000;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2000,
        name: "recipe component defined item 1",
        attribute_class: Some("dynamic_recipe_component_defined_item"),
        description_string: None,
        description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
        effect_type: EffectType::Neutral,
        hidden: false,
        stored_as_integer: false,
    };
    
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem2;

impl Attribute for DynamicRecipeComponentDefinedItem2 {
    const DEFINDEX: u32 = 2001;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2001,
        name: "recipe component defined item 2",
        attribute_class: Some("dynamic_recipe_component_defined_item"),
        description_string: None,
        description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
        effect_type: EffectType::Neutral,
        hidden: false,
        stored_as_integer: false,
    };
    
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem3;

impl Attribute for DynamicRecipeComponentDefinedItem3 {
    const DEFINDEX: u32 = 2002;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2002,
        name: "recipe component defined item 3",
        attribute_class: Some("dynamic_recipe_component_defined_item"),
        description_string: None,
        description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
        effect_type: EffectType::Neutral,
        hidden: false,
        stored_as_integer: false,
    };

    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem4;

impl Attribute for DynamicRecipeComponentDefinedItem4 {
    const DEFINDEX: u32 = 2003;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2003,
        name: "recipe component defined item 4",
        attribute_class: Some("dynamic_recipe_component_defined_item"),
        description_string: None,
        description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
        effect_type: EffectType::Neutral,
        hidden: false,
        stored_as_integer: false,
    };
    
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem5;

impl Attribute for DynamicRecipeComponentDefinedItem5 {
    const DEFINDEX: u32 = 2004;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2004,
        name: "recipe component defined item 5",
        attribute_class: Some("dynamic_recipe_component_defined_item"),
        description_string: None,
        description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
        effect_type: EffectType::Neutral,
        hidden: false,
        stored_as_integer: false,
    };
    
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem6;

impl Attribute for DynamicRecipeComponentDefinedItem6 {
    const DEFINDEX: u32 = 2005;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2005,
        name: "recipe component defined item 6",
        attribute_class: Some("dynamic_recipe_component_defined_item"),
        description_string: None,
        description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
        effect_type: EffectType::Neutral,
        hidden: false,
        stored_as_integer: false,
    };
    
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem7;

impl Attribute for DynamicRecipeComponentDefinedItem7 {
    const DEFINDEX: u32 = 2006;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2006,
        name: "recipe component defined item 7",
        attribute_class: Some("dynamic_recipe_component_defined_item"),
        description_string: None,
        description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
        effect_type: EffectType::Neutral,
        hidden: false,
        stored_as_integer: false,
    };

    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem8;

impl Attribute for DynamicRecipeComponentDefinedItem8 {
    const DEFINDEX: u32 = 2007;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2007,
        name: "recipe component defined item 8",
        attribute_class: Some("dynamic_recipe_component_defined_item"),
        description_string: None,
        description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
        effect_type: EffectType::Neutral,
        hidden: false,
        stored_as_integer: false,
    };
    
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem9;

impl Attribute for DynamicRecipeComponentDefinedItem9 {
    const DEFINDEX: u32 = 2008;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2008,
        name: "recipe component defined item 9",
        attribute_class: Some("dynamic_recipe_component_defined_item"),
        description_string: None,
        description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
        effect_type: EffectType::Neutral,
        hidden: false,
        stored_as_integer: false,
    };

    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "dynamic_recipe_component_defined_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DynamicRecipeComponentDefinedItem10;

impl Attribute for DynamicRecipeComponentDefinedItem10 {
    const DEFINDEX: u32 = 2009;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2009,
        name: "recipe component defined item 10",
        attribute_class: Some("dynamic_recipe_component_defined_item"),
        description_string: None,
        description_format: Some(DescriptionFormat::ValueIsFromLookupTable),
        effect_type: EffectType::Neutral,
        hidden: false,
        stored_as_integer: false,
    };
    
    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }
    
    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "tool_target_item" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ToolTargetItem(pub u64);

impl Attribute for ToolTargetItem {
    const DEFINDEX: u32 = 2012;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2012,
        name: "tool target item",
        attribute_class: Some("tool_target_item"),
        description_string: None,
        description_format: None,
        effect_type: EffectType::ValueIsFromLookupTable,
        hidden: true,
        stored_as_integer: false,
    };

    fn attribute_value(&self) -> Option<AttributeValue> {
        None
    }

    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "series_number" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SeriesNumber(pub u64);

impl Attribute for SeriesNumber {
    const DEFINDEX: u32 = 2031;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2031,
        name: "series number",
        attribute_class: Some("series_number"),
        description_string: None,
        description_format: None,
        effect_type: EffectType::ValueIsFromLookupTable,
        hidden: true,
        stored_as_integer: false,
    };

    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.into())
    }

    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "taunt attach particle index" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetAttachedParticleTaunt(pub u64);

impl Attribute for SetAttachedParticleTaunt {
    const DEFINDEX: u32 = 2041;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 2041,
        name: "taunt attach particle index",
        attribute_class: None,
        description_string: Some("★ Unusual Effect: %s1"),
        description_format: Some(DescriptionFormat::ValueIsParticleIndex),
        effect_type: EffectType::Unusual,
        hidden: false,
        stored_as_integer: true,
    };

    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.into())
    }

    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

/// Represents the "set_attached_particle" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetAttachedParticle(pub u64);

impl Attribute for SetAttachedParticle {
    const DEFINDEX: u32 = 134;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 134,
        name: "attach particle effect",
        attribute_class: Some("set_attached_particle"),
        description_string: Some("★ Unusual Effect: %s1"),
        description_format: Some(DescriptionFormat::ValueIsParticleIndex),
        effect_type: EffectType::Unusual,
        hidden: false,
        stored_as_integer: false,
    };

    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.into())
    }

    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}
  
/// Represents the "paintkit_proto_def_index" attribute.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PaintkitProtoDefIndex(pub u64);

impl Attribute for PaintkitProtoDefIndex {
    const DEFINDEX: u32 = 834;
    const ATTRIBUTE: AttributeDef = AttributeDef {
        defindex: 834,
        name: "paintkit_proto_def_index",
        attribute_class: Some("paintkit_proto_def_index"),
        description_string: None,
        description_format: Some(DescriptionFormat::ValueIsAdditive),
        effect_type: EffectType::Neutral,
        hidden: false,
        stored_as_integer: true,
    };

    fn attribute_value(&self) -> Option<AttributeValue> {
        Some(self.0.into())
    }

    fn attribute_float_value(&self) -> Option<f64> {
        None
    }
}

