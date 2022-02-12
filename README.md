# tf2-enum

Enum values related to the Team Fortress 2 item schema.

```rs
use tf2_enum::Quality;

assert_eq!(Quality::from_str("Decorated Weapon").unwrap(), Quality::DecoratedWeapon);
```

## License

MIT