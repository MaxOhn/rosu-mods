use std::collections::{BTreeMap, HashMap};

use itoa::Buffer;

pub use self::{
    error::GenResult,
    model::{Acronym, RulesetMods},
    writer::Writer,
};

mod error;
mod model;
mod writer;

pub fn specify_preamble(writer: &mut Writer, url: &str) -> GenResult {
    writer.write(
        "//! Each individual [`GameMod`] as defined by osu!lazer.\n\
        //!\n\
        //! See <",
    )?;
    writer.write(url)?;

    writer.write(
        ">\n\
        //!\n\
        //! This file was generated automatically.\n\n\
        #![allow(clippy::all, clippy::pedantic)]\n\n\
        use std::{\
            borrow::Borrow,\
            cmp::Ordering,\
            num::NonZeroU8,\
            fmt::{Display, Formatter, Result as FmtResult},\
        };",
    )?;

    writer.write("\n\nuse crate::{Acronym, GameMode};\n\n")?;

    Ok(())
}

pub fn define_gamemod_structs(
    rulesets: &[RulesetMods],
    writer: &mut Writer,
    itoa_buf: &mut Buffer,
) -> GenResult {
    let mut archives = String::new();
    let mut resolvers = String::new();

    let mut archived = |name| {
        archives.push_str("Archived");
        archives.push_str(name);
        archives.push(',');
    };
    let mut resolver = |name| {
        resolvers.push_str(name);
        resolvers.push_str("Resolver");
        resolvers.push(',');
    };

    writer.write("mod all_structs {")?;

    for ruleset in rulesets.iter() {
        for gamemod in ruleset.mods.iter() {
            gamemod.define_struct(writer, &mut archived, &mut resolver)?;
        }
    }

    define_unknown_mod_struct(writer)?;

    writer.write_raw(
        b"}\
        pub use all_structs::{",
    )?;

    for ruleset in rulesets.iter() {
        for gamemod in ruleset.mods.iter() {
            writer.write(&gamemod.name)?;
            writer.write_raw(b",")?;
        }
    }

    writer.write_raw(
        b"\
            UnknownMod\
        };\
        pub use gamemod::GameMod;\
        pub use intermode::GameModIntermode;\
        use crate::GameModKind;\
        /// Types for (de)serialization through `rkyv`.\n\
        #[cfg(feature = \"rkyv\")]\
        #[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = \"rkyv\")))]\
        #[doc(hidden)]\
        pub mod rkyv {\
            pub use super::gamemod::{ArchivedGameMod, GameModResolver};\
            pub use super::intermode::GameModIntermodeResolver;\
            pub use crate::kind::GameModKindResolver;\
            pub use super::all_structs::{",
    )?;
    writer.write(&*archives)?;
    writer.write(&*resolvers)?;
    writer.write_raw(b"UnknownModResolver};}")?;

    for ruleset in rulesets.iter() {
        for gamemod in ruleset.mods.iter() {
            gamemod.define_fns(writer, itoa_buf)?;
        }
    }

    define_unknown_mod_impl(writer)?;

    Ok(())
}

pub fn check_gamemod_kind(rulesets: &[RulesetMods]) {
    let expected = [
        "DifficultyReduction",
        "DifficultyIncrease",
        "Conversion",
        "Automation",
        "Fun",
        "System",
    ];

    for ruleset in rulesets.iter() {
        for gamemod in ruleset.mods.iter() {
            if !expected.contains(&&*gamemod.kind) {
                panic!("unexpected GameModKind `{}`", gamemod.kind);
            }
        }
    }
}

pub fn define_gamemod_intermode(
    rulesets: &[RulesetMods],
    writer: &mut Writer,
    itoa_buf: &mut Buffer,
) -> GenResult {
    let mut mods = rulesets
        .iter()
        .flat_map(|ruleset| {
            let suffix_len = ruleset.name.as_capitalized_str().len();

            ruleset.mods.iter().map(move |gamemod| {
                let name = &gamemod.name[..gamemod.name.len() - suffix_len];
                let bits = gamemod.bits();
                let kind = gamemod.kind.as_ref();

                (name, (bits, gamemod.acronym, kind))
            })
        })
        .collect::<HashMap<_, _>>()
        .into_iter()
        .collect::<Vec<_>>();

    mods.sort_unstable_by(|(a, ..), (b, ..)| a.cmp(b));

    writer.write(
        "pub(crate) mod intermode {\
            /// A single game mod when the mode is ignored\n\
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]\
            #[cfg_attr(\
                feature = \"rkyv\",\
                derive(\
                    rkyv::Archive,\
                    rkyv::Serialize,\
                    rkyv::Deserialize,\
                    rkyv::Portable,\
                    rkyv::bytecheck::CheckBytes,\
                ),\
                rkyv(as = Self),\
                bytecheck(crate = rkyv::bytecheck),\
                repr(u8),\
            )]\
            #[non_exhaustive]\
            pub enum GameModIntermode {",
    )?;

    for (name, _) in mods.iter() {
        writer.write(*name)?;
        writer.write(b',')?;
    }

    writer.write("Unknown(super::UnknownMod),")?;
    writer.write("}}")?;

    writer.write(
        "impl GameModIntermode {\
            /// The [`Acronym`] of this [`GameModIntermode`]\n\
            pub const fn acronym(&self) -> Acronym {\
                unsafe { match self {",
    )?;

    for (name, (_, acronym, _)) in mods.iter() {
        writer.write("Self::")?;
        writer.write(*name)?;
        writer.write(" => ")?;
        acronym.write(writer)?;
        writer.write(b',')?;
    }

    writer.write(
        "\
                        Self::Unknown(m) => m.acronym(),\
                    }\
                }\
            }\
            /// Bit value of the [`GameModIntermode`]\n\
            ///\n\
            /// See <https://github.com/ppy/osu-api/wiki#mods>\n\
            pub const fn bits(self) -> Option<u32> {\
                match self {",
    )?;

    for (name, (bits, ..)) in mods.iter() {
        writer.write("Self::")?;
        writer.write(*name)?;
        writer.write(" => ")?;

        match bits {
            Some(bits) => {
                writer.write("Some(")?;
                writer.write(itoa_buf.format(*bits))?;
                writer.write(b')')?;
            }
            None => writer.write("None")?,
        }

        writer.write(b',')?;
    }

    writer.write(
        "\
                    Self::Unknown(_) => None,\
                }\
            }\
            /// The [`GameModKind`] of this [`GameModIntermode`]\n\
            pub const fn kind(&self) -> GameModKind {\
                match self {",
    )?;

    for (name, (.., kind)) in mods.iter() {
        writer.write("Self::")?;
        writer.write(*name)?;
        writer.write(" => GameModKind::")?;
        writer.write(*kind)?;
        writer.write(b',')?;
    }

    writer.write(
        "\
                    Self::Unknown(_) => GameModKind::System,\
                }\
            }\
            /// Parse an [`Acronym`] into a [`GameModIntermode`]\n\
            pub fn from_acronym(acronym: Acronym) -> Self {\
                match acronym.as_str() {",
    )?;

    for (name, (_, acronym, _)) in mods.iter() {
        writer.write(b'"')?;
        writer.write(acronym.as_str())?;
        writer.write("\" => Self::")?;
        writer.write(*name)?;
        writer.write(b',')?;
    }

    writer.write(
        "\
                    _ => Self::Unknown(UnknownMod { acronym }),\
                }\
            }\
            /// Try to convert bitvalues into a [`GameModIntermode`]\n\
            pub const fn try_from_bits(bits: u32) -> Option<Self> {\
                match bits {",
    )?;

    for (name, (bits, ..)) in mods.iter() {
        let Some(bits) = bits else { continue };

        writer.write(itoa_buf.format(*bits))?;
        writer.write(" => Some(Self::")?;
        writer.write(*name)?;
        writer.write("),")?;
    }

    writer.write(
        "\
                    _ => None,\
                }\
            }\
        }",
    )?;

    writer.write(
        "impl PartialOrd for GameModIntermode {\
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {\
                Some(self.cmp(other))\
            }\
        }\
        impl Ord for GameModIntermode {\
            fn cmp(&self, other: &Self) -> Ordering {\
                match (self.bits(), other.bits()) {\
                    (Some(self_bits), Some(other_bits)) => self_bits.cmp(&other_bits),\
                    (Some(_), None) => Ordering::Less,\
                    (None, Some(_)) => Ordering::Greater,\
                    (None, None) => self.acronym().as_str().cmp(other.acronym().as_str()),\
                }\
            }\
        }\
        impl Display for GameModIntermode {\
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {\
                f.write_str(self.acronym().as_str())\
            }\
        }\
        impl From<&GameModIntermode> for GameModIntermode {\
            fn from(gamemod: &GameModIntermode) -> Self {\
                *gamemod\
            }\
        }\
        impl From<GameMod> for GameModIntermode {\
            fn from(gamemod: GameMod) -> Self {\
                gamemod.intermode()\
            }\
        }",
    )
}

pub fn define_gamemod_order(
    rulesets: &[RulesetMods],
    writer: &mut Writer,
    itoa_buf: &mut Buffer,
) -> GenResult {
    writer.write(
        "#[derive(Copy, Clone, PartialEq, Eq)]\
        pub(crate) struct GameModOrder {\
            mode: GameMode,\
            index: Option<NonZeroU8>,\
            intermode: GameModIntermode,\
        }\
        impl From<&GameMod> for GameModOrder {\
            fn from(gamemod: &GameMod) -> Self {\
                const fn inner(gamemod: &GameMod) -> GameModOrder {\
                    macro_rules! arm {\
                        ($mode:ident, $gamemod:ident, Some($discriminant:literal), $intermode:ident) => {\
                            arm!(\
                                $mode,\
                                $gamemod,\
                                Some(unsafe { NonZeroU8::new_unchecked($discriminant) }),\
                                $intermode,\
                            )\
                        };\
                        ($mode:ident, $gamemod:ident, $index:expr, $intermode:ident $(,)?) => {\
                            GameModOrder {\
                                mode: GameMode::$mode,\
                                index: $index,\
                                intermode: GameModIntermode::$intermode,\
                            }\
                        };\
                    }\
                    match gamemod {",
    )?;

    for ruleset in rulesets {
        let ruleset_str = ruleset.name.as_capitalized_str();

        for gamemod in ruleset.mods.iter() {
            writer.write("GameMod::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) => arm!(")?;
            writer.write(ruleset_str)?;
            writer.write(b',')?;
            writer.write(&gamemod.name)?;
            writer.write(b',')?;

            match gamemod.discriminant() {
                Some(discriminant) => {
                    writer.write("Some(")?;
                    writer.write(itoa_buf.format(discriminant))?;
                    writer.write(b')')?;
                }
                None => {
                    writer.write("None")?;
                }
            }

            let intermode = &gamemod.name[..gamemod.name.len() - ruleset_str.len()];
            writer.write(b',')?;
            writer.write(intermode)?;
            writer.write("),")?;
        }

        writer.write("GameMod::Unknown")?;
        writer.write(ruleset_str)?;
        writer.write(
            "(m) => GameModOrder {\
                mode: GameMode::",
        )?;
        writer.write(ruleset_str)?;
        writer.write(
            ",\
                index: None,\
                intermode: GameModIntermode::Unknown(*m),\
            },",
        )?;
    }

    writer.write(
        "\
                    }\
                }\
                inner(gamemod)\
            }\
        }\
        impl PartialOrd for GameModOrder {\
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {\
                Some(self.cmp(other))\
            }\
        }\
        impl Ord for GameModOrder {\
            fn cmp(&self, other: &Self) -> Ordering {\
                self.mode.cmp(&other.mode)\
                    .then_with(|| match (self.index, other.index) {\
                        (Some(self_idx), Some(other_idx)) => self_idx.cmp(&other_idx),\
                        (Some(_), None) => Ordering::Less,\
                        (None, Some(_)) => Ordering::Greater,\
                        (None, None) => self\
                            .intermode\
                            .acronym()\
                            .as_str()\
                            .cmp(other.intermode.acronym().as_str()),\
                    })\
            }\
        }\
        impl PartialEq<GameModIntermode> for GameModOrder {\
            fn eq(&self, other: &GameModIntermode) -> bool {\
                self.intermode.eq(other)\
            }\
        }\
        impl Borrow<GameModIntermode> for GameModOrder {\
            fn borrow(&self) -> &GameModIntermode {\
                &self.intermode\
            }\
        }",
    )
}

pub fn define_gamemod_enum(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "pub(crate) mod gamemod {\
            use super::*;\
            /// A single game mod\n\
            #[derive(Clone, Debug, PartialEq)]\
            #[cfg_attr(\
                feature = \"rkyv\",\
                derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize),\
            )]\
            #[non_exhaustive]\
            pub enum GameMod {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            writer.write(&gamemod.name)?;
            writer.write(b'(')?;
            writer.write(&gamemod.name)?;
            writer.write("),")?;
        }

        writer.write("Unknown")?;
        writer.write(ruleset.name.as_capitalized_str())?;
        writer.write("(UnknownMod),")?;
    }

    writer.write("}}")
}

pub fn define_gamemod_fns(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write("impl GameMod {")?;

    define_gamemod_fn_new(rulesets, writer)?;
    define_gamemod_fn_acronym(rulesets, writer)?;
    define_gamemod_fn_incompatible_mods(rulesets, writer)?;
    define_gamemod_fn_description(rulesets, writer)?;
    define_gamemod_fn_kind(rulesets, writer)?;
    define_gamemod_fn_bits(rulesets, writer)?;
    define_gamemod_fn_mode(rulesets, writer)?;
    define_gamemod_fn_intermode(rulesets, writer)?;

    writer.write(b'}')
}

fn define_unknown_mod_struct(writer: &mut Writer) -> GenResult {
    writer.write(
        "/// Any unknown mod.\n\
        #[derive(Copy, Eq, Clone, Debug, PartialEq, PartialOrd, Ord, Hash)]\
        #[cfg_attr(\
            feature = \"rkyv\",\
            derive(\
                rkyv::Archive,\
                rkyv::Serialize,\
                rkyv::Deserialize,\
                rkyv::Portable,\
                rkyv::bytecheck::CheckBytes,\
            ),\
            bytecheck(crate = rkyv::bytecheck),\
            rkyv(as = Self),\
            repr(transparent),\
        )]\
        pub struct UnknownMod {\
            pub acronym: crate::Acronym,\
        }",
    )
}

fn define_unknown_mod_impl(writer: &mut Writer) -> GenResult {
    writer.write(
        "impl UnknownMod {\
            /// The default [`Acronym`] for an unknown mod without specific\n\
            /// acronym.\n\
            pub const UNKNOWN_ACRONYM: Acronym = unsafe { Acronym::from_str_unchecked(\"??\") };\n\
            /// A custom [`Acronym`] for any unknown mod\n\
            pub const fn acronym(self) -> Acronym {\
                self.acronym\
            }\
            /// Returns an empty iterator\n\
            pub const fn incompatible_mods() -> std::iter::Empty<Acronym> {\
                std::iter::empty()\
            }\
            /// A custom description for any unknown mod\n\
            pub const fn description() -> &'static str {\
                \"Some unknown mod\"\
            }\
            /// A manually assigned [`GameModKind`] for any unknown mod\n\
            pub const fn kind() -> GameModKind {\
                GameModKind::System\
            }\
        }\
        impl Default for UnknownMod {\
            fn default() -> Self {\
                Self {\
                    acronym: Self::UNKNOWN_ACRONYM,\
                }\
            }\
        }",
    )
}

fn define_gamemod_fn_new(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// Create a new [`GameMod`]\n\
        pub fn new(acronym: &str, mode: GameMode) -> Self {\
            match (acronym, mode) {",
    )?;

    for ruleset in rulesets {
        let ruleset_str = ruleset.name.as_capitalized_str();

        for gamemod in ruleset.mods.iter() {
            writer.write("(\"")?;
            writer.write(gamemod.acronym.as_str())?;
            writer.write("\", GameMode::")?;
            writer.write(ruleset_str)?;
            writer.write(") => Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(Default::default()),")?;
        }
    }

    writer.write(
        "_ => {\
            let acronym = <Acronym as std::str::FromStr>::from_str(acronym)\
                .unwrap_or(UnknownMod::UNKNOWN_ACRONYM);\
            let unknown = UnknownMod { acronym };\
            match mode {",
    )?;

    for ruleset in rulesets {
        let ruleset_str = ruleset.name.as_capitalized_str();
        writer.write("GameMode::")?;
        writer.write(ruleset_str)?;
        writer.write(" => GameMod::Unknown")?;
        writer.write(ruleset_str)?;
        writer.write("(unknown),")?;
    }

    writer.write(
        "\
                    }\
                }\
            }\
        }",
    )
}

fn define_gamemod_fn_acronym(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// The acronym of this [`GameMod`]\n\
        pub const fn acronym(&self) -> Acronym {\
            match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) => ")?;
            writer.write(&gamemod.name)?;
            writer.write("::acronym(),")?;
        }
    }

    let mut rulesets_iter = rulesets.iter();

    if let Some(ruleset) = rulesets_iter.next() {
        writer.write("Self::Unknown")?;
        writer.write(ruleset.name.as_capitalized_str())?;
        writer.write("(m)")?;

        for ruleset in rulesets_iter {
            writer.write(" | Self::Unknown")?;
            writer.write(ruleset.name.as_capitalized_str())?;
            writer.write("(m)")?;
        }

        writer.write(" => m.acronym(),")?;
    }

    writer.write(
        "\
            }\
        }",
    )
}

fn define_gamemod_fn_incompatible_mods(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// List of [`Acronym`] for mods that are incompatible with this [`GameMod`]\n\
        pub fn incompatible_mods(&self) -> Box<[Acronym]> {\
        match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) => ")?;
            writer.write(&gamemod.name)?;
            writer.write("::incompatible_mods().collect(),")?;
        }
    }

    writer.write("_ => UnknownMod::incompatible_mods().collect(),")?;

    writer.write("}}")
}

fn define_gamemod_fn_description(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// The description of this [`GameMod`]\n\
        pub const fn description(&self) -> &'static str {\
        match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) => ")?;
            writer.write(&gamemod.name)?;
            writer.write("::description(),")?;
        }
    }

    writer.write("_ => UnknownMod::description(),")?;

    writer.write("}}")
}

fn define_gamemod_fn_kind(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// The [`GameModKind`] of this [`GameMod`]\n\
        pub const fn kind(&self) -> GameModKind {\
            match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) => ")?;
            writer.write(&gamemod.name)?;
            writer.write("::kind(),")?;
        }
    }

    writer.write("_ => UnknownMod::kind(),")?;

    writer.write(
        "\
            }\
        }",
    )
}

fn define_gamemod_fn_bits(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// Optional bit value of this [`GameMod`]\n\
        ///\n\
        /// See <https://github.com/ppy/osu-api/wiki#mods>\n\
        pub const fn bits(&self) -> Option<u32> {\
            match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            if gamemod.bits().is_some() {
                writer.write("Self::")?;
                writer.write(&gamemod.name)?;
                writer.write("(_) => Some(")?;
                writer.write(&gamemod.name)?;
                writer.write("::bits()),")?;
            }
        }
    }

    writer.write(
        "\
                _ => None,\
            }\
        }",
    )
}

fn define_gamemod_fn_intermode(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// The kind of a [`GameMod`] when ignoring the mode\n\
        pub const fn intermode(&self) -> GameModIntermode {\
            match self {",
    )?;

    for ruleset in rulesets {
        let ruleset_str = ruleset.name.as_capitalized_str();

        for gamemod in ruleset.mods.iter() {
            let intermode = &gamemod.name[..gamemod.name.len() - ruleset_str.len()];

            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) => GameModIntermode::")?;
            writer.write(intermode)?;
            writer.write(b',')?;
        }
    }

    let mut ruleset_iter = rulesets.iter();

    if let Some(ruleset) = ruleset_iter.next() {
        writer.write("Self::Unknown")?;
        writer.write(ruleset.name.as_capitalized_str())?;
        writer.write("(m)")?;

        for ruleset in ruleset_iter {
            writer.write(" | Self::Unknown")?;
            writer.write(ruleset.name.as_capitalized_str())?;
            writer.write("(m)")?;
        }

        writer.write(" => GameModIntermode::Unknown(*m),")?;
    }

    writer.write(
        "\
            }\
        }",
    )
}

fn define_gamemod_fn_mode(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// The [`GameMode`] of a [`GameMod`]\n\
        pub const fn mode(&self) -> GameMode {\
            match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) | ")?;
        }

        let ruleset_str = ruleset.name.as_capitalized_str();
        writer.write("Self::Unknown")?;
        writer.write(ruleset_str)?;
        writer.write("(_) => GameMode::")?;
        writer.write(ruleset_str)?;
        writer.write(b',')?;
    }

    writer.write("}}")
}

pub fn impl_gamemod_traits(writer: &mut Writer) -> GenResult {
    writer.write(
        "impl PartialOrd for GameMod {\
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {\
                self\
                    .bits()\
                    .zip(other.bits())\
                    .map(|(self_bits, other_bits)| self_bits.cmp(&other_bits))\
            }\
        }",
    )
}

pub fn impl_serde(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "#[cfg(feature = \"serde\")]\
        #[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = \"serde\")))]\
        const _: () = {",
    )?;

    writer.write(
        "use serde::{\
            Deserialize,\
            de::{\
                value::MapAccessDeserializer, Deserializer, Error as DeError, IgnoredAny, \
                MapAccess, Visitor, DeserializeSeed,\
            },\
            ser::{Serialize, Serializer, SerializeMap},\
        };\n\n\
        use crate::serde::{\
            GameModSettings, GameModSettingsSeed, GameModRaw, MaybeOwnedStr,\
            DeserializedGameMod, GameModVisitor, GameModRawSeed,\
        };\n\n",
    )?;

    for ruleset in rulesets.iter() {
        for gamemod in ruleset.mods.iter() {
            gamemod.impl_serde(writer)?;
        }
    }

    writer.write(
        "impl<'de> Deserialize<'de> for UnknownMod {\
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {\
                struct UnknownModVisitor;\
                impl<'de> Visitor<'de> for UnknownModVisitor {\
                    type Value = UnknownMod;\
                    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {\
                        f.write_str(\"any unknown mod\")\
                    }\
                    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {\
                        while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}\
                        Ok(UnknownMod { acronym: UnknownMod::UNKNOWN_ACRONYM })\
                    }\
                }\
                d.deserialize_map(UnknownModVisitor)\
            }\
        }\
        impl Serialize for UnknownMod {\
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {\
                s.serialize_map(Some(0)).and_then(SerializeMap::end)\
            }\
        }"
    )?;

    writer.write(
        "impl<'a, 'de> Visitor<'de> for GameModSettingsSeed<'a> {\
            type Value = GameMod;\
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {\
                f.write_str(\"GameMod settings\")\
            }\
            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {\
                let d = MapAccessDeserializer::new(map);\
                let res = match (self.acronym, self.mode) {",
    )?;

    for ruleset in rulesets {
        let ruleset_str = ruleset.name.as_capitalized_str();

        for gamemod in ruleset.mods.iter() {
            writer.write("(\"")?;
            writer.write(gamemod.acronym.as_str())?;
            writer.write("\", GameMode::")?;
            writer.write(ruleset_str)?;
            writer.write(") => GameMod::")?;
            writer.write(&gamemod.name)?;
            writer.write(
                "(DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?),",
            )?;
        }
    }

    writer.write(
        "_ => {\
            let acronym = <Acronym as std::str::FromStr>::from_str(self.acronym).map_err(DeError::custom)?;\n\
            // All fields are specified already but we still want to clear\n\
            // out content from the deserializer.\n\
            #[allow(clippy::needless_update)]\
            let unknown = UnknownMod {\
                acronym,\
                ..Deserialize::deserialize(d)?\
            };\
            match self.mode {",
    )?;

    for ruleset in rulesets {
        let ruleset_str = ruleset.name.as_capitalized_str();
        writer.write("GameMode::")?;
        writer.write(ruleset_str)?;
        writer.write(" => GameMod::Unknown")?;
        writer.write(ruleset_str)?;
        writer.write("(unknown),")?;
    }

    writer.write(
        "\
                        }\
                    }\
                };\
                Ok(res)\
            }\
        }",
    )?;

    writer.write(
        "impl Serialize for GameMod {\
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {\
                let mut s = s.serialize_map(None)?;\
                s.serialize_entry(\"acronym\", self.acronym().as_str())?;\
                match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            if gamemod.settings.is_empty() {
                continue;
            }

            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write(
                "\
                    (m) => {\
                        let has_some = ",
            )?;

            let mut settings = gamemod.settings.iter();

            if let Some(setting) = settings.next() {
                writer.write("m.")?;
                writer.write(&setting.name)?;
                writer.write(".is_some()")?;

                for setting in settings {
                    writer.write(" || m.")?;
                    writer.write(&setting.name)?;
                    writer.write(".is_some()")?;
                }
            }

            writer.write(
                "\
                        ;\
                        if has_some {\
                            s.serialize_entry(\"settings\", m)?;\
                        }\
                    },",
            )?;
        }
    }

    writer.write(
        "\
                        _ => {},\
                    }\
                    s.end()\
                }\
            }\
            impl<'de> Deserialize<'de> for GameModIntermode {\
                fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {\
                    fn try_acronym_to_gamemod<E: DeError>(\
                        acronym: &MaybeOwnedStr<'_>,\
                    ) -> Result<GameModIntermode, E> {\
                        acronym\
                            .as_str()\
                            .parse()\
                            .map(GameModIntermode::from_acronym)\
                            .map_err(DeError::custom)\
                    }\
                    let raw_seed = GameModRawSeed { deny_unknown_fields: true };\
                    match raw_seed.deserialize(d)? {\
                        GameModRaw::Bits(bits) => GameModIntermode::try_from_bits(bits)\
                            .ok_or_else(|| DeError::custom(\"invalid bitflags\")),\
                        GameModRaw::Acronym(acronym) => try_acronym_to_gamemod(&acronym),\
                        GameModRaw::Full { acronym, .. } => try_acronym_to_gamemod(&acronym),\
                    }\
                }\
            }\
            impl serde::Serialize for GameModIntermode {\
                fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {\
                    s.serialize_str(self.acronym().as_str())\
                }\
            }",
    )?;

    writer.write(
        "\
    impl GameModSettings<'_> {\
        pub(crate) fn try_deserialize(&self, acronym: &str, deny_unknown_fields: bool) -> Option<GameMod> {",
    )?;

    writer.write_raw(
        b"\
            macro_rules! try_deser {
                ( $osu_mod:ident, $taiko_mod:ident, $catch_mod:ident, $mania_mod:ident, ) => {{
                    try_deser!(@ $osu_mod Osu);
                    try_deser!(@ $taiko_mod Taiko);
                    try_deser!(@ $catch_mod Catch);
                    try_deser!(@ $mania_mod Mania);
                }};
                ( @ Skip_ $mode:ident ) => {};
                ( @ $name:ident $mode:ident ) => {
                    if let Ok(m) = DeserializedGameMod::try_deserialize_mod(self, deny_unknown_fields) {
                        return Some(GameMod::$name(m));
                    }
                };
            }",
    )?;
    writer.write("match acronym {")?;

    let mut acronyms_map = BTreeMap::<Acronym, [Option<&str>; 4]>::new();

    for ruleset in rulesets.iter() {
        for gamemod in ruleset.mods.iter() {
            let entry = acronyms_map.entry(gamemod.acronym).or_default();
            entry[ruleset.name as usize] = Some(&gamemod.name);
        }
    }

    for (acronym, mods) in acronyms_map {
        writer.write_raw(b"\"")?;
        writer.write(acronym.as_str())?;
        writer.write("\" => try_deser!(")?;

        for gamemod in mods {
            match gamemod {
                Some(name) => {
                    writer.write(name)?;
                    writer.write_raw(b",")?;
                }
                None => writer.write_raw(b"Skip_,")?,
            }
        }

        writer.write_raw(b"),")?;
    }

    writer.write(
        "\
                _ => {}\
            }\
            None\
        }\
    }",
    )?;

    writer.write_raw(b"};")
}

pub fn impl_macro(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    let mut intermodes = rulesets
        .iter()
        .flat_map(|ruleset| {
            let ruleset_str = ruleset.name.as_capitalized_str();

            ruleset.mods.iter().map(|gamemod| {
                let intermode = &gamemod.name[..gamemod.name.len() - ruleset_str.len()];

                (gamemod.acronym.as_str(), intermode)
            })
        })
        .collect::<HashMap<_, _>>()
        .into_iter()
        .collect::<Vec<_>>();

    intermodes.sort_unstable();

    writer.write_raw(
        b"#[macro_export(local_inner_macros)]
#[cfg(feature = \"macros\")]
#[doc(hidden)]
macro_rules! mods_inner {
    ( @ $mode:ident: $( $acronym:tt )* ) => {{
        // Making sure it's a valid GameMode
        let _ = $crate::GameMode::$mode;

        #[allow(unused_mut)]
        let mut mods = $crate::GameMods::new();
        $( mods.insert(mods_inner!(< ! $mode $acronym)(Default::default())); )*
        mods
    }};
    ( @ $( $acronym:tt )* ) => {{
        #[allow(unused_mut)]
        let mut mods = $crate::GameModsIntermode::new();
        $( mods.insert(mods_inner!(< $acronym)); )*
        mods
    }};

    // Translating acronym to variant name",
    )?;

    for (acronym, gamemod) in intermodes {
        writer.write_raw(
            b"
    ( < $( ! $mode:ident )? ",
        )?;
        writer.write(acronym)?;
        writer.write_raw(b" ) => { mods_inner!(> $( $mode )? ")?;
        writer.write(gamemod)?;
        writer.write_raw(b" ) };")?;
    }

    writer.write_raw(
        b"

    // Unknown acronym
    ( < ! $mode:ident $other:tt $( $rest:tt )* ) => { mods_inner!(<< $other) };
    ( < $other:tt $( $rest:tt )* ) => { mods_inner!(<< $other) };
    ( << $other:tt ) => {
        std::compile_error!(std::concat!(\"unknown mod acronym `\", std::stringify!($other), \"`\"))
    };

    // Prefixing variant name with the full type path
    ( > $mode:ident $name:ident ) => {
        $crate::macros::paste! { $crate::generated_mods::GameMod::[<$name $mode>] }
    };
    ( > $name:ident ) => {
        $crate::generated_mods::GameModIntermode::$name
    };
}",
    )
}
