# v0.3.0 (2025-03-10)

- __Breaking changes:__
  - Added the field `SuddenDeathOsu::fail_on_slider_tail`
  - All variants of both `GameModSeed` and `GameModsSeed` now have an additional
    `deny_unknown_fields: bool` field ([#6])
  - `Deserialize` impls for gamemod structs have been removed ([#6])
  - If the `rkyv` feature is enabled, archived `Option<{f64,bool}>` fields are now niched ([#7])

- __Additions:__
  - Added the enum variants `GameMod::BloomOsu` and `GameModIntermode::Bloom`
  - Added the gamemod struct `BloomOsu`

- __Adjustments:__
  - The bitflag value for mods is no longer considered for sorting ([#5])

## v0.2.1 (2024-11-28)

- `GameModsIntermode::legacy_clock_rate` now considers `GameModIntermode::Daycore` properly

## v0.2.0 (2024-11-18)

- [Breaking] All `f32` fields of inner `GameMod` types are now `f64` for added precision ([#3])
- [Breaking] `rkyv` has been bumped to 0.8 ([#2])

## v0.1.1 (2024-08-17)

- Updated `mods.json` which added the `NoReleaseMania` mod.
- Slightly improved deserialization error for mods without settings. ([#1])

## v0.1.0 (2024-07-04)

First release

[#1]: https://github.com/MaxOhn/rosu-mods/pull/1
[#2]: https://github.com/MaxOhn/rosu-mods/pull/2
[#3]: https://github.com/MaxOhn/rosu-mods/pull/3
[#5]: https://github.com/MaxOhn/rosu-mods/pull/5
[#6]: https://github.com/MaxOhn/rosu-mods/pull/6
[#7]: https://github.com/MaxOhn/rosu-mods/pull/7