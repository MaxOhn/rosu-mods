#![cfg(feature = "macros")]

/// Short-hand macro to easily create [`GameMods`]
/// or [`GameModsIntermode`].
///
/// To create [`GameModsIntermode`],
/// specify a space-separated list of acronyms.
///
/// To create [`GameMods`],
/// specify `Osu`, `Taiko`, `Catch`, or `Mania`, followed
/// by a colon (`:`), followed by a space-separated list of acronyms.
/// Note that creating [`GameMods`] requires the `macros` feature flag.
///
/// # Example
///
/// ```rust
/// # use rosu_mods::{mods, GameMods, GameModsIntermode};
/// let mods: GameMods = mods!(Taiko: NC HR);
/// assert_eq!(mods.to_string(), "HRNC");
///
/// let mods: GameModsIntermode = mods!(DT HR TC);
/// assert_eq!(mods.to_string(), "HRDTTC");
/// ```
///
/// [`GameMods`]: crate::GameMods
/// [`GameModsIntermode`]: crate::GameModsIntermode
#[macro_export(local_inner_macros)]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! mods {
    ( Osu $( : $( $acronym:tt )* )? ) => {
        mods_inner!(@ Osu: $( $( $acronym )* )?)
    };
    ( Taiko $( : $( $acronym:tt )* )? ) => {
        mods_inner!(@ Taiko: $( $( $acronym )* )?)
    };
    ( Catch $( : $( $acronym:tt )* )? ) => {
        mods_inner!(@ Catch: $( $( $acronym )* )?)
    };
    ( Mania $( : $( $acronym:tt )* )? ) => {
        mods_inner!(@ Mania: $( $( $acronym )* )?)
    };
    ( $( $acronym:tt )* ) => {
        mods_inner!(@ $( $acronym )*)
    };
}

pub use paste::paste;

#[cfg(test)]
mod tests {
    #[test]
    fn empty_intermode() {
        let mods = mods!();
        assert!(mods.is_empty());
    }

    #[test]
    fn single_intermode() {
        let mods = mods!(WG);
        assert_eq!(mods.len(), 1);
    }

    #[test]
    fn full_intermode() {
        let mods = mods!(HD DT DT HR TC);
        assert_eq!(mods.to_string(), "HDHRDTTC");
    }

    #[test]
    fn empty_catch() {
        let mods = mods!(Catch);
        assert!(mods.is_empty());
    }

    #[cfg(feature = "macros")]
    #[test]
    fn full_taiko() {
        let mods = mods!(Taiko: HR PF);
        assert_eq!(mods.len(), 2);
    }
}
