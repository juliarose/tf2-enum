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
- `PaintSpell` enum.
- `FootprintsSpell` enum.

### Changed
- `Spell` is now divided by spell type.