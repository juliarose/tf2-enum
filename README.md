# tf2-enum

Provides enumerated types for models related to the Team Fortress 2 item schema.

## Usage

```rust
use tf2_enum::{Quality, Spell, ItemLevel, KillstreakTier};
use std::str::FromStr;

assert_eq!("Unusual".parse::<Quality>().unwrap(), Quality::Unusual);
assert_eq!(Quality::Unusual as u32, 5);
assert_eq!(Spell::HalloweenFire.to_string(), "Halloween Fire");

let level = ItemLevel::KillEaterRank.score_level(9000);
let killstreak_tier = KillstreakTier::Professional;
let full_name = format!("{level} {killstreak_tier} Pomson 6000");

assert_eq!(full_name, "Hale's Own Professional Killstreak Pomson 6000");
```

## License

[MIT](https://github.com/juliarose/tf2-enum/tree/main/LICENSE)