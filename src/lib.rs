//! TODO: docs

#![cfg_attr(docsrs, feature(doc_cfg))]
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
    clippy::cast_possible_wrap
)]

#[cfg(feature = "macros")]
extern crate paste;

#[macro_use]
#[doc(hidden)]
pub mod macros;

mod acronym;
mod intermode;
mod legacy_mods;
mod mod_manual;
mod mode;
mod mode_as_seed;
mod mods;
mod util;

/// Error types
pub mod error;

pub mod generated_mods;

/// Types to calculate intersecting mods.
pub mod intersection;

/// Iterator types for mods.
pub mod iter;

pub use self::{mode::GameMode, mods::GameMods};

#[doc(inline)]
pub use self::{
    acronym::Acronym,
    generated_mods::{GameMod, GameModIntermode, GameModKind},
    intermode::GameModsIntermode,
    legacy_mods::GameModsLegacy,
};

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
/// Types for (de)serialization.
pub mod serde {
    pub use super::mode_as_seed::ModeAsSeed;
}
