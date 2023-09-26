# tf2-enum

Provides enumerated types for models related to the Team Fortress 2 item schema.

## Usage

```rs
use tf2_enum::{Quality, KillstreakTier};

assert_eq!(Quality::from_str("Unusual").unwrap(), Quality::Unusual);
assert_eq!(Quality::Unusual as u32, 5);
assert_eq!(KillstreakTier::Professional.to_string(), "Professional Killstreak");
```

## License

[MIT](https://github.com/juliarose/tf2-enum/tree/main/LICENSE)