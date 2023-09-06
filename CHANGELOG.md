# Changelog

## 0.1.1 (2022-02-20)

### Added
- `Hash` derive to all enums. 
- `Eq` derive to all enums. 

## 0.2.0 (2022-02-23)

### Added
- `Sheen` enum.
- `Killstreaker` enum.
- `Spell` enum.

## 0.3.0 (2022-03-14)

### Added
- `StrangePart` enum.
- `Paint` enum.
- Missing `Rarity2` and `Rarity3` variants for `Quality` enum.
- `color` method for `Quality`.
- `from_color` method for `Quality`.
- `from_color_str` method for `Quality`.

## 0.3.1 (2022-03-17)

### Added
- `EnumIter` derive to all enums.

### Fixed
- `Paint` enum to parse string variants using lowercase and blu color variations.

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

## 0.4.0 (2022-04-23)

### Added
- `CraftClass` enum.

### Changed
- All enum reprs to use `u32` for consistency.

## 0.5.0 (2022-04-29)

### Added
- `Serialize` implementation for `CraftClass`, `ItemSlot`, and `Class`.
- `Deserialize` implementation for `CraftClass`, `ItemSlot`, and `Class`.
- `CraftMaterialType` enum.
- `Origin` enum.
- `IntoEnumIterator` re-export from `strum` for iterating over enums.

### Remove
- `strum` re-export.

## 0.6.0 (2022-10-15)

### Changed
- `Basic` in `KillstreakTier` is now `Killstreak`.

## 0.6.1 (2023-01-05)

### Added
- `Spell::from_str` now also includes "Voices from Below" for `Spell::VoicesFromBelow`.

## 0.7.0 (2023-02-23)

### Added
- Some missing inconsistencies.
- Export `num_enum::TryFromPrimitiveError`.
- `strum::EnumCount` on enums.

## 0.8.0 (2023-02-23)

### Added
- Re-export `num_enum`.

### Removed
- Export `num_enum::TryFromPrimitiveError`.

## 0.8.1 (2023-02-27)

### Added
- `Serialize` impl for `Spell`.
- `Deserialize` impl for `Spell`.

## 0.8.2 (2023-09-06)

### Added
- `Ord` impl for enums.
- `PartialOrd` impl for enums.

## 0.8.3 (2023-09-06)

### Added
- `Into<Spell>` impl for `FootprintsSpell`.
- `Into<Spell>` impl for `PaintSpell`.