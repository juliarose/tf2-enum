# Changelog

## 0.12.0 (2025-08-20)

### Added
- Added `PartialOrd` for `AttributeValue`.

### Fixed
- Some minor doc issues.
- `rust-version` not using the correct MSRV.
- Lifetime annotations for static string constants causing issues with older Rust versions.

### Changed
- For deserializing into `SpellSet` and `AttributeSet`, if multiple attributes have the same defindex, only the first is used.

## 0.11.0 (2025-08-20)

### Added
- `ItemAttribute` container struct for attributes.
- `PartialOrd` and `Ord` traits to `StrangePartSet` and `SpellSet`.
- `NONE` const for `SpellSet` and `StrangePartSet`.
- `as_slice`, `as_slice_mut`, `iter_mut`, `iter_attributes`, `is_full`, `try_insert`, `insert_or_replace_last`, `replace`, `first`, `last`, `capacity` methods to `SpellSet` and `StrangePartSet` through `AttributeSet`.
- `econ_defindex` method for `StockWeapon`.
- `DescriptionFormat` and `EffectType` enums for traits `Attribute` and `Attributes`.
- `Colored` trait for definitions associated with a color.
- `TryFromAttributeValueU32` trait for definitions associated with an attribute value.
- `HasItemDefindex` trait for definitions associated with an item defindex.
- `IntoEnumIterator`, `EnumCount`, `TryFromPrimitive`, `IntoPrimitive` re-exports.
- Missing `From` and `TryFrom` conversions for spell enums.
- `is_paint_spell` and `is_footprints_spell` methods to `Spell`.
- `AttributeValue` enum for attribute values.
- `AttributeDef` struct for defintitions associated with multiple attributes.
- Attributes related to economy items. These can be found in `econ_attributes`.
- `TryFromPrimitive<u32>` impl for `StockWeapon`.
- `Capability::CanUnusualify`.
- `Serialize` and `Deserialize` derives for `Capability`.
- The base kill eater score type attributes for `KillEaterScoreType`.
- repr(u32) for `Class`.

### Removed
- `defindex_is_strange_part` for `StrangePart`.
- `defindex_is_paint` for `Paint`.
- `defindex_is_stock_weapon` for `StockWeapon`.
- `strum`, `num_enum`, `strum_macros` re-exports. Only the derived traits are re-exported.
- `strum_macro` as a dependency, it's now included using `derive` feature for `strum`.

### Changed
- Order of `StrangePart` variants from lowest to highest `kill_eater_score_type` defindex.
- `SpellSet` and `StrangePartSet` now implement `AttributeSet`.
- Moved extra fields for `Attribute` into `ATTRIBUTE` through `AttributeDef`.
- `attribute_value` methods now return `AttributeValue` instead of `Option<AttributeValue>`.
- Attribute integers and floats from 64 bit to 32 bit.
- `attribute_value` to `attribute_id` for `Spell` so it does not conflict with the trait method.

## 0.10.0 (2025-08-14)

### Added
- `KillEaterScoreType` enum.
- `ItemLevel` enum.
- `Capability` enum.
- `Level` struct associated with `ItemLevel`.
- `SpellSet` for holding sets of spells.
- `StrangePartSet` for holding sets of strange parts.
- `is_cosmetic_part` method to `StrangePart`.
- `strange_part_name` method to `StrangePart`.
- `from_strange_part_name` method to `StrangePart`.
- `kill_eater_score_type` method to `StrangePart`.
- Missing `EnumCount` for some definitions.
- `defindex_is_strange_part` for `StrangePart`.
- `defindex_is_paint` for `Paint`.
- More fields for `Attribute`.
- More fields for `Attributes`.

### Changed
- `StrangePart::DefendersKilled` to `StrangePart::DefenderKills` to match the schema.
- `StrangePart::PlayersHit` to `StrangePart::PlayerHits` to match the schema.
- Related strings for `StrangePart::PlayerHits` and `StrangePart::DefenderKills`.
- Collisions for `StrangePartSet` and `SpellSet` now prioritize keeping the first values rather than the last.
- `Spell::VoicesFromBelow` now serializes as `Voices from Below` instead of `Voices From Below`.

### Removed
- `IntoEnumIterator` and `EnumCount` re-exports. These can be pulled in from the `strum` re-export. 

## 0.9.4 (2024-05-18)

### Added
- `Sized` to `Attribute` and `Attributes` traits.
- `From<FootprintsSpell>` impl for `Spell`.
- `From<PaintSpell>` impl for `Spell`.

## 0.9.3 (2024-05-18)

### Added
- `strum` and `strum_macros` re-export.

## 0.9.2 (2024-05-06)

### Changed
- Updated readme and docs.
- Updated dependencies.

## 0.9.1 (2023-10-07)

### Changed
- `Spell::VoicesFromBelow` can be deserialized from "Voices from Below" in addition to "Voices From Below".

## 0.9.0 (2023-09-26)

### Changed
- `Spell` now includes individual variants for paint and footprints spells (reduces byte count from 8 to 1).
- Renamed `Rarity` to `Grade`.

### Added
- `color` method to `Grade`.
- `from_color` method to `Grade`.
- `from_color_str` method to `Grade`.

## 0.8.3 (2023-09-06)

### Added
- `Into<Spell>` impl for `FootprintsSpell`.
- `Into<Spell>` impl for `PaintSpell`.

## 0.8.2 (2023-09-06)

### Added
- `Ord` impl for enums.
- `PartialOrd` impl for enums.

## 0.8.1 (2023-02-27)

### Added
- `Serialize` impl for `Spell`.
- `Deserialize` impl for `Spell`.

## 0.8.0 (2023-02-23)

### Added
- Re-export `num_enum`.

### Removed
- Export `num_enum::TryFromPrimitiveError`.

## 0.7.0 (2023-02-23)

### Added
- Some missing inconsistencies.
- Export `num_enum::TryFromPrimitiveError`.
- `strum::EnumCount` on enums.

## 0.6.1 (2023-01-05)

### Added
- `Spell::from_str` now also includes "Voices from Below" for `Spell::VoicesFromBelow`.

## 0.6.0 (2022-10-15)

### Changed
- `Basic` in `KillstreakTier` is now `Killstreak`.

## 0.5.0 (2022-04-29)

### Added
- `Serialize` implementation for `CraftClass`, `ItemSlot`, and `Class`.
- `Deserialize` implementation for `CraftClass`, `ItemSlot`, and `Class`.
- `CraftMaterialType` enum.
- `Origin` enum.
- `IntoEnumIterator` re-export from `strum` for iterating over enums.

### Remove
- `strum` re-export.

## 0.4.0 (2022-04-23)

### Added
- `CraftClass` enum.

### Changed
- All enum reprs to use `u32` for consistency.

## 0.4.0 (2022-04-02)

### Added
- `Attribute` trait.
- `Attributes` trait.
- `Attribute` implementation for `Paint`.
- `Attribute` implementation for `Killstreaker`.
- `Attribute` implementation for `Sheen`.
- `Attribute` implementation for `KillstreakTier`.
- `Attribute` implementation for `Wear`.
- `Attributes` implementation for `StrangePart`.
- `Attributes` implementation for `Spell`.
- `TryFrom<f64>` implementation for `Wear`.
- `TryFrom<f32>` implementation for `Wear`.
- `attribute_defindex` method for `Spell`.
- `DEFINDEX_PAINT` constant for `Spell`.
- `DEFINDEX_FOOTPRINTS` constant for `Spell`.
- `DEFINDEX_VOICES_FROM_BELOW` constant for `Spell`.
- `DEFINDEX_PUMPKIN_BOMBS` constant for `Spell`.
- `DEFINDEX_HALLOWEEN_FIRE` constant for `Spell`.
- `DEFINDEX_EXORCISM` constant for `Spell`.
- `PaintSpell` enum.
- `FootprintsSpell` enum.

### Changed
- `Spell` is now divided by spell type.
- `from_color_str` methods now accept strings prefixed by `#`.

## 0.3.1 (2022-03-17)

### Added
- `EnumIter` derive to all enums.

### Fixed
- `Paint` enum to parse string variants using lowercase and blu color variations.

## 0.3.0 (2022-03-14)

### Added
- `StrangePart` enum.
- `Paint` enum.
- Missing `Rarity2` and `Rarity3` variants for `Quality` enum.
- `color` method for `Quality`.
- `from_color` method for `Quality`.
- `from_color_str` method for `Quality`.

## 0.2.0 (2022-02-23)

### Added
- `Sheen` enum.
- `Killstreaker` enum.
- `Spell` enum.

## 0.1.1 (2022-02-20)

### Added
- `Hash` derive to all enums. 
- `Eq` derive to all enums. 
