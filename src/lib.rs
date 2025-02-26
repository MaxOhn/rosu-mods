//! A library to provide types for all [osu!] gamemods as defined in the official [mods.json] file.
//!
//! Individual gamemod types are generated automatically through the [`generate-mods`] binary.
//!
//! ## Types
//!
//! In total there are three different collections for mods.
//!
//! ### `GameModsLegacy`
//!
//! [`GameModsLegacy`] is a lightweight type that is essentially just bitflags for the [legacy mods].
//!
//! ```
//! use rosu_mods::GameModsLegacy;
//!
//! let hdnc = GameModsLegacy::Nightcore | GameModsLegacy::Hidden;
//! assert_eq!(hdnc.to_string(), "HDNC");
//! assert!(hdnc.contains(GameModsLegacy::DoubleTime));
//! ```
//!
//! ### `GameMods`
//!
//! [`GameMods`] is a collection of the [`GameMod`] enum. [`GameMod`] distinguishes between each
//! mode so if a mod is valid for multiple modes, each of those modes will have a variant for that
//! mod.
//!
//! ```
//! use rosu_mods::{GameMod, GameMods, generated_mods::AccuracyChallengeMania};
//!
//! # let mut mods = GameMods::new();
//! # mods.insert(GameMod::HardRockTaiko(Default::default()));
//! # mods.insert(GameMod::AccuracyChallengeTaiko(Default::default()));
//! # /*
//! // The `mods!` macro is only available if the `macros` feature is enabled
//! let mut mods: GameMods = rosu_mods::mods!(Taiko: AC HR);
//! # */
#![cfg_attr(
    feature = "macros",
    doc = "# assert_eq!(mods, rosu_mods::mods!(Taiko: AC HR));"
)]
//!
//! // In addition to the two taiko mods, let's add a mania mod too
//! mods.insert(GameMod::AccuracyChallengeMania(AccuracyChallengeMania {
//!     restart: Some(true),
//!     ..Default::default()
//! }));
//!
//! assert_eq!(mods.to_string(), "ACHRAC");
//!
//! let mut iter = mods.into_iter();
//! assert_eq!(iter.next(), Some(GameMod::AccuracyChallengeTaiko(Default::default())));
//! assert_eq!(iter.next(), Some(GameMod::HardRockTaiko(Default::default())));
//! assert_eq!(iter.next(), Some(GameMod::AccuracyChallengeMania(AccuracyChallengeMania {
//!     restart: Some(true),
//!     minimum_accuracy: None,
//!     accuracy_judge_mode: None,
//! })));
//! assert_eq!(iter.next(), None);
//! ```
//!
//! ### `GameModsIntermode`
//!
//! [`GameModsIntermode`] is a collection of the [`GameModIntermode`] enum. Unlike [`GameMod`],
//! this enum does not distinguish between modes. As such, variants do not carry further data
//! because a mod may have different settings depending on the mode.
//!
//! Since [`GameModsIntermode`] does not carry additional data and also consists of fewer variants,
//! it is generally easier to deal with than [`GameMods`].
//!
//! ```
//! use rosu_mods::{GameModIntermode, GameModsIntermode};
//!
//! # let mut mods = GameModsIntermode::new();
//! # mods.insert(GameModIntermode::Wiggle);
//! # mods.insert(GameModIntermode::FadeIn);
//! # /*
//! // The `mods!` macro is only available if the `macros` feature is enabled
//! let mut mods: GameModsIntermode = rosu_mods::mods!(WG FI);
//! # */
#![cfg_attr(
    feature = "macros",
    doc = "# assert_eq!(mods, rosu_mods::mods!(WG FI));"
)]
//!
//! // Let's add some more mods
//! mods.extend([GameModIntermode::Easy, GameModIntermode::HardRock]);
//!
//! assert_eq!(mods.to_string(), "EZFIHRWG");
//!
//! let mut iter = mods.into_iter();
//! assert_eq!(iter.next(), Some(GameModIntermode::Easy));
//! assert_eq!(iter.next(), Some(GameModIntermode::FadeIn));
//! assert_eq!(iter.next(), Some(GameModIntermode::HardRock));
//! assert_eq!(iter.next(), Some(GameModIntermode::Wiggle));
//! assert_eq!(iter.next(), None);
//! ```
//!
//! ## Features
//!
//! | Flag      | Description                                                                                      | Dependencies
//! | --------- | ------------------------------------------------------------------------------------------------ | ------------
//! | `default` | No features enabled                                                                              |
//! | `macros`  | Enables the `mods!` macro                                                                        | [`paste`]
//! | `serde`   | Implements `serde::{Deserialize, Serialize}` for all types and enables the `serde` module        | [`serde`]
//! | `rkyv`    | Implements `rkyv::{Archive, Serialize, Deserialize}` for all types and enables the `rkyv` module | [`rkyv`]
//!
//! [osu!]: https://osu.ppy.sh/home
//! [mods.json]: https://github.com/ppy/osu-web/blob/master/database/mods.json
//! [`generate-mods`]: https://github.com/MaxOhn/rosu-mods/tree/main/generate-mods
//! [legacy mods]: https://github.com/ppy/osu-api/wiki#reference
//! [`paste`]: https://docs.rs/paste
//! [`serde`]: https://docs.rs/serde
//! [`rkyv`]: https://docs.rs/rkyv
//! [`GameModsLegacy`]: crate::legacy::GameModsLegacy
//! [`GameMods`]: crate::mods::GameMods
//! [`GameMod`]: crate::generated_mods::gamemod::GameMod
//! [`GameModsIntermode`]: crate::intermode::GameModsIntermode
//! [`GameModIntermode`]: crate::generated_mods::intermode::GameModIntermode

#![cfg_attr(all(docsrs, not(doctest)), feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links, rustdoc::missing_crate_level_docs)]
#![warn(clippy::missing_const_for_fn, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::struct_excessive_bools,
    clippy::match_same_arms,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::explicit_iter_loop,
    clippy::similar_names,
    clippy::cast_possible_wrap,
    clippy::default_trait_access
)]

#[cfg(feature = "macros")]
extern crate paste;

#[macro_use]
#[doc(hidden)]
pub mod macros;

mod acronym;
mod intermode;
mod kind;
mod legacy;
mod mod_manual;
mod mode;
mod mods;
mod order;
mod util;

/// Error types
pub mod error;

pub mod generated_mods;

/// Types to calculate intersecting mods.
pub mod intersection;

/// Iterator types for mods.
pub mod iter;

#[cfg(feature = "rkyv")]
#[doc(inline)]
pub use generated_mods::rkyv;

/// Types for (de)serialization through `serde`.
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "serde")))]
pub mod serde;

pub use self::{mode::GameMode, mods::GameMods};

#[doc(inline)]
pub use self::{
    acronym::Acronym,
    generated_mods::{GameMod, GameModIntermode},
    intermode::GameModsIntermode,
    kind::GameModKind,
    legacy::GameModsLegacy,
};
