[![crates.io](https://img.shields.io/crates/v/rosu-mods.svg)](https://crates.io/crates/rosu-mods) [![docs](https://docs.rs/rosu-mods/badge.svg)](https://docs.rs/rosu-mods)

# rosu-mods

<!-- cargo-rdme start -->

A library to provide types for all [osu!] gamemods as defined in the official [mods.json] file.

Individual gamemod types are generated automatically through the [`generate-mods`] binary.

### Types

In total, there are four different ways of handling mods.

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

assert_eq!(mods.to_string(), "ACHRAC");

let mut iter = mods.into_iter();
assert_eq!(iter.next(), Some(GameMod::AccuracyChallengeTaiko(Default::default())));
assert_eq!(iter.next(), Some(GameMod::HardRockTaiko(Default::default())));
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

assert_eq!(mods.to_string(), "EZFIHRWG");

let mut iter = mods.into_iter();
assert_eq!(iter.next(), Some(GameModIntermode::Easy));
assert_eq!(iter.next(), Some(GameModIntermode::FadeIn));
assert_eq!(iter.next(), Some(GameModIntermode::HardRock));
assert_eq!(iter.next(), Some(GameModIntermode::Wiggle));
assert_eq!(iter.next(), None);
```

#### `GameModSimple`

Unlike the other three, [`GameModSimple`] is not a collection but just a
single mod. Instead of providing types for each mod, it keeps things simple
and stores all settings into a plain `HashMap`.

```rust
use rosu_mods::{GameMod, GameModSimple, SettingSimple, generated_mods::AccuracyChallengeMania};

let gamemod = GameMod::AccuracyChallengeMania(AccuracyChallengeMania {
    restart: Some(true),
    ..Default::default()
});
let simple = gamemod.into_simple();
assert_eq!(simple.settings.get("restart"), Some(&SettingSimple::Bool(true)));
```

### Features

| Flag      | Description                                                                                      | Dependencies
| --------- | ------------------------------------------------------------------------------------------------ | ------------
| `default` | No features enabled                                                                              |
| `macros`  | Enables the `mods!` macro                                                                        | [`pastey`]
| `serde`   | Implements `serde::{Deserialize, Serialize}` for all types and enables the `serde` module        | [`serde`]
| `rkyv`    | Implements `rkyv::{Archive, Serialize, Deserialize}` for all types and enables the `rkyv` module | [`rkyv`]

[osu!]: https://osu.ppy.sh/home
[mods.json]: https://github.com/ppy/osu-web/blob/master/database/mods.json
[`generate-mods`]: https://github.com/MaxOhn/rosu-mods/tree/main/generate-mods
[legacy mods]: https://github.com/ppy/osu-api/wiki#reference
[`pastey`]: https://docs.rs/pastey
[`serde`]: https://docs.rs/serde
[`rkyv`]: https://docs.rs/rkyv
[`GameModsLegacy`]: https://docs.rs/rosu-mods/latest/rosu_mods/legacy/struct.GameModsLegacy.html
[`GameMods`]: https://docs.rs/rosu-mods/latest/rosu_mods/mods/struct.GameMods.html
[`GameMod`]: https://docs.rs/rosu-mods/latest/rosu_mods/generated_mods/gamemod/enum.GameMod.html
[`GameModsIntermode`]: https://docs.rs/rosu-mods/latest/rosu_mods/intermode/struct.GameModsIntermode.html
[`GameModIntermode`]: https://docs.rs/rosu-mods/latest/rosu_mods/generated_mods/intermode/enum.GameModIntermode.html

<!-- cargo-rdme end -->
