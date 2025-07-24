# tf2-enum

Provides enumerated types for models related to the Team Fortress 2 item schema.

## Usage

```rust
use tf2_enum::{Quality, Spell};
use std::str::FromStr;

assert_eq!("Unusual".parse::<Quality>().unwrap(), Quality::Unusual);
assert_eq!(Quality::Unusual as u32, 5);
assert_eq!(Spell::HalloweenFire.to_string(), "Halloween Fire");
```

## License

[MIT](https://github.com/juliarose/tf2-enum/tree/main/LICENSE)