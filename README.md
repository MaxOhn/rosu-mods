[![crates.io](https://img.shields.io/crates/v/rosu-mods.svg)](https://crates.io/crates/rosu-mods) [![docs](https://docs.rs/rosu-mods/badge.svg)](https://docs.rs/rosu-mods)

# rosu-mods

<!-- cargo-rdme start -->

A library to provide types for all [osu!] gamemods as defined in the official [mods.json] file.

Individual gamemod types are generated automatically through the [`generate-mods`] binary.

### Types

In total there are three different collections for mods.

#### `GameModsLegacy`

[`GameModsLegacy`] is a lightweight type that is essentially just bitflags for the [legacy mods].

```rust
use rosu_mods::GameModsLegacy;

let hdnc = GameModsLegacy::Nightcore | GameModsLegacy::Hidden;
assert_eq!(hdnc.to_string(), "HDNC");
assert!(hdnc.contains(GameModsLegacy::DoubleTime));
```

#### `GameMods`

[`GameMods`] is a collection of the [`GameMod`] enum. [`GameMod`] distinguishes between each
mode so if a mod is valid for multiple modes, each of those modes will have a variant for that
mod.

```rust
use rosu_mods::{GameMod, GameMods, generated_mods::AccuracyChallengeMania};

// The `mods!` macro is only available if the `macros` feature is enabled
let mut mods: GameMods = rosu_mods::mods!(Taiko: AC HR);

// In addition to the two taiko mods, let's add a mania mod too
mods.insert(GameMod::AccuracyChallengeMania(AccuracyChallengeMania {
    restart: Some(true),
    ..Default::default()
}));

assert_eq!(mods.to_string(), "HRACAC");

let mut iter = mods.into_iter();
assert_eq!(iter.next(), Some(GameMod::HardRockTaiko(Default::default())));
assert_eq!(iter.next(), Some(GameMod::AccuracyChallengeTaiko(Default::default())));
assert_eq!(iter.next(), Some(GameMod::AccuracyChallengeMania(AccuracyChallengeMania {
    restart: Some(true),
    minimum_accuracy: None,
    accuracy_judge_mode: None,
})));
assert_eq!(iter.next(), None);
```

#### `GameModsIntermode`

[`GameModsIntermode`] is a collection of the [`GameModIntermode`] enum. Unlike [`GameMod`],
this enum does not distinguish between modes. As such, variants do not carry further data
because a mod may have different settings depending on the mode.

Since [`GameModsIntermode`] does not carry additional data and also consists of fewer variants,
it is generally easier to deal with than [`GameMods`].

```rust
use rosu_mods::{GameModIntermode, GameModsIntermode};

// The `mods!` macro is only available if the `macros` feature is enabled
let mut mods: GameModsIntermode = rosu_mods::mods!(WG FI);

// Let's add some more mods
mods.extend([GameModIntermode::Easy, GameModIntermode::HardRock]);

assert_eq!(mods.to_string(), "EZHRFIWG");

let mut iter = mods.into_iter();
assert_eq!(iter.next(), Some(GameModIntermode::Easy));
assert_eq!(iter.next(), Some(GameModIntermode::HardRock));
assert_eq!(iter.next(), Some(GameModIntermode::FadeIn));
assert_eq!(iter.next(), Some(GameModIntermode::Wiggle));
assert_eq!(iter.next(), None);
```

### Features

| Flag      | Description                                                                           | Dependencies
| --------- | ------------------------------------------------------------------------------------- | ------------
| `default` | No features enabled                                                                   |
| `macros`  | Enables the `mods!` macro                                                             | [`paste`]
| `serde`   | Implements `Deserialize` and `Serialize` for all types and enables the `serde` module | [`serde`]

[osu!]: https://osu.ppy.sh/home
[mods.json]: https://github.com/ppy/osu-web/blob/master/database/mods.json
[`generate-mods`]: https://github.com/MaxOhn/rosu-mods/tree/main/generate-mods
[legacy mods]: https://github.com/ppy/osu-api/wiki#reference
[`paste`]: https://docs.rs/paste
[`serde`]: https://docs.rs/serde

<!-- cargo-rdme end -->
