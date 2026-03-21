use std::{
    collections::HashMap,
    fmt::{Debug, Formatter, Result as FmtResult},
};

use crate::{Acronym, GameModIntermode};

/// A simplified version of [`GameMod`].
///
/// [`GameMod`]: crate::GameMod
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[derive(Clone, Debug, PartialEq)]
pub struct GameModSimple {
    pub acronym: Acronym,
    #[cfg_attr(feature = "serde", serde(default))]
    pub settings: HashMap<Box<str>, SettingSimple>,
}

impl GameModSimple {
    /// Convert a [`GameModSimple`] to a [`GameModIntermode`].
    pub fn as_intermode(&self) -> GameModIntermode {
        GameModIntermode::from_acronym(self.acronym)
    }

    /// Convert a [`GameModSimple`] into a [`GameMod`].
    ///
    /// The `seed` controls which [`GameMode`] to target and whether unknown
    /// fields are rejected:
    ///
    /// - [`GameModSeed::Mode`] targets a specific mode.
    /// - [`GameModSeed::GuessMode`] tries each mode in turn and picks the
    ///   first one whose settings all match.
    ///
    /// Returns `Ok(GameMod::Unknown*(..))` if the acronym is not valid for the
    /// resolved mode — that is a legitimate, expected outcome rather than an
    /// error.
    ///
    /// Returns `Err` only when the settings themselves are malformed: a value
    /// has the wrong type for its field, or — when `deny_unknown_fields` is
    /// `true` in the seed — a key is not recognised by the target mod.
    #[cfg(feature = "serde")]
    #[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "serde")))]
    pub fn try_as_mod(
        self,
        seed: crate::serde::GameModSeed,
    ) -> Result<crate::GameMod, GameModSimpleConversionError> {
        use serde::de::DeserializeSeed;

        use crate::serde::GameModSettings;

        let settings = GameModSettings::from_simple_settings(&self.settings);

        // Drive GameModSeed::visit_map by presenting a two-entry map:
        //   { "acronym": <str>, "settings": <fields> }
        let d = simple_deserializer::SimpleMapDeserializer::new(self.acronym.as_str(), &settings);

        seed.deserialize(d)
            .map_err(|e| GameModSimpleConversionError {
                msg: e.to_string().into_boxed_str(),
            })
    }
}

/// Error returned by [`GameModSimple::try_as_mod`].
///
/// This is produced when the settings stored in a [`GameModSimple`] are
/// incompatible with the target [`GameMod`] variant — for example when a
/// value has the wrong type for its field, or when an unrecognised field key
/// is encountered and `deny_unknown_fields` is `true`.
///
/// An *unknown acronym* is **not** an error; [`GameModSimple::try_as_mod`]
/// returns `Ok(GameMod::Unknown*(..))` in that case.
#[cfg(feature = "serde")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "serde")))]
#[derive(Debug)]
pub struct GameModSimpleConversionError {
    msg: Box<str>,
}

/// A setting value for [`GameModSimple`].
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[derive(Clone, PartialEq)]
pub enum SettingSimple {
    Bool(bool),
    Number(f64),
    String(String),
}

impl Debug for SettingSimple {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            SettingSimple::Bool(value) => Debug::fmt(value, f),
            SettingSimple::Number(value) => Debug::fmt(value, f),
            SettingSimple::String(value) => Debug::fmt(value, f),
        }
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "serde")))]
const _: () = {
    use std::{error::Error, fmt::Display};

    use serde::de::{Deserialize, Deserializer};

    use crate::serde::Value;

    impl<'de> Deserialize<'de> for SettingSimple {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            match Value::deserialize(d)? {
                Value::Bool(value) => Ok(Self::Bool(value)),
                Value::Str(value) => Ok(Self::String(value.into_owned())),
                Value::Number(value) => Ok(Self::Number(value)),
            }
        }
    }

    impl Display for GameModSimpleConversionError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str(&self.msg)
        }
    }

    impl Error for GameModSimpleConversionError {}
};

#[cfg(feature = "serde")]
mod simple_deserializer {
    use serde::{
        de::{value::BorrowedStrDeserializer, DeserializeSeed, Error, MapAccess, Visitor},
        Deserializer,
    };

    use crate::serde::{GameModDeserializeError, GameModSettings};

    // -------------------------------------------------------------------------
    // SimpleMapDeserializer
    //
    // Presents a two-entry map  { "acronym": <str>, "settings": <fields> }
    // to GameModSeed::visit_map, which expects exactly that shape. This lets
    // us fully reuse GameModSeed's dispatch logic — including the GuessMode
    // path that tries every mode — without duplicating any of it.
    // -------------------------------------------------------------------------

    pub(super) struct SimpleMapDeserializer<'a> {
        acronym: &'a str,
        settings: &'a GameModSettings<'a>,
    }

    impl<'a> SimpleMapDeserializer<'a> {
        pub(super) const fn new(acronym: &'a str, settings: &'a GameModSettings<'a>) -> Self {
            Self { acronym, settings }
        }
    }

    impl<'de, 'a: 'de> Deserializer<'de> for SimpleMapDeserializer<'a> {
        type Error = GameModDeserializeError;

        fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
            self.deserialize_map(visitor)
        }

        fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
            visitor.visit_map(SimpleMapAccess::new(self.acronym, self.settings))
        }

        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
            byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct
            struct enum identifier ignored_any
        }
    }

    // State machine driving the two-entry map:
    //   key1 → value1 → key2 → value2 → done.
    enum MapState {
        AcronymKey,
        AcronymValue,
        SettingsKey,
        SettingsValue,
        Done,
    }

    struct SimpleMapAccess<'a> {
        acronym: &'a str,
        settings: &'a GameModSettings<'a>,
        state: MapState,
    }

    impl<'a> SimpleMapAccess<'a> {
        const fn new(acronym: &'a str, settings: &'a GameModSettings<'a>) -> Self {
            Self {
                acronym,
                settings,
                state: MapState::AcronymKey,
            }
        }
    }

    impl<'de, 'a: 'de> MapAccess<'de> for SimpleMapAccess<'a> {
        type Error = GameModDeserializeError;

        fn next_key_seed<K: DeserializeSeed<'de>>(
            &mut self,
            seed: K,
        ) -> Result<Option<K::Value>, Self::Error> {
            match self.state {
                MapState::AcronymKey => {
                    self.state = MapState::AcronymValue;
                    let d = BorrowedStrDeserializer::new("acronym");

                    seed.deserialize(d).map(Some)
                }
                MapState::SettingsKey => {
                    self.state = MapState::SettingsValue;
                    let d = BorrowedStrDeserializer::new("settings");

                    seed.deserialize(d).map(Some)
                }
                _ => Ok(None),
            }
        }

        fn next_value_seed<V: DeserializeSeed<'de>>(
            &mut self,
            seed: V,
        ) -> Result<V::Value, Self::Error> {
            match self.state {
                MapState::AcronymValue => {
                    self.state = MapState::SettingsKey;
                    let d = BorrowedStrDeserializer::<GameModDeserializeError>::new(self.acronym);

                    seed.deserialize(d)
                }
                MapState::SettingsValue => {
                    self.state = MapState::Done;

                    // GameModSettings<'a> implements Deserializer, so
                    // feeding it to the seed drives the per-mod field visitor.
                    seed.deserialize(self.settings)
                }
                _ => Err(GameModDeserializeError::custom(
                    "next_value called out of sequence",
                )),
            }
        }

        fn size_hint(&self) -> Option<usize> {
            Some(2)
        }
    }
}

#[cfg(test)]
mod tests {
    mod common {
        #![allow(unused, reason = "depends on enabled features")]

        pub(super) use crate::{GameMod, GameMode};

        pub(super) use super::super::*;

        pub(super) const JSON: &str = r#"[
            {
                "acronym":"DA",
                "settings":{
                    "scroll_speed":2
                }
            },
            {
                "acronym":"CS"
            }
        ]"#;
    }

    #[allow(unused, reason = "depends on enabled features")]
    use common::*;

    #[test]
    #[cfg(feature = "serde")]
    fn roundtrip_serde() {
        let mods: Vec<GameModSimple> = serde_json::from_str(JSON).unwrap();

        let expected = vec![
            GameModSimple {
                acronym: "DA".parse().unwrap(),
                settings: vec![("scroll_speed".into(), SettingSimple::Number(2.0))]
                    .into_iter()
                    .collect(),
            },
            GameModSimple {
                acronym: "CS".parse().unwrap(),
                settings: HashMap::new(),
            },
        ];

        assert_eq!(mods, expected);

        let serialized = serde_json::to_string(&mods).unwrap();
        let deserialized: Vec<GameModSimple> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(mods, deserialized);
    }

    /// Converting a mod whose acronym is valid for the given mode, with a
    /// correctly typed setting that exists on that variant.
    #[test]
    #[cfg(feature = "serde")]
    fn try_as_mod_known_with_setting() {
        use crate::{generated_mods::DifficultyAdjustTaiko, serde::GameModSeed};

        let simple = GameModSimple {
            acronym: "DA".parse().unwrap(),
            settings: [("scroll_speed".into(), SettingSimple::Number(2.0))]
                .into_iter()
                .collect(),
        };

        assert_eq!(
            simple
                .try_as_mod(GameModSeed::Mode {
                    mode: GameMode::Taiko,
                    deny_unknown_fields: true
                })
                .unwrap(),
            GameMod::DifficultyAdjustTaiko(DifficultyAdjustTaiko {
                scroll_speed: Some(2.0),
                ..Default::default()
            })
        );
    }

    /// Multiple settings are all forwarded correctly.
    #[test]
    #[cfg(feature = "serde")]
    fn try_as_mod_multiple_settings() {
        use crate::serde::GameModSeed;

        let simple = GameModSimple {
            acronym: "DA".parse().unwrap(),
            settings: [
                ("approach_rate".into(), SettingSimple::Number(9.5)),
                ("circle_size".into(), SettingSimple::Number(4.0)),
            ]
            .into_iter()
            .collect(),
        };

        let GameMod::DifficultyAdjustOsu(da) = simple
            .try_as_mod(GameModSeed::Mode {
                mode: GameMode::Osu,
                deny_unknown_fields: true,
            })
            .unwrap()
        else {
            panic!("expected DifficultyAdjustOsu");
        };

        assert_eq!(da.approach_rate, Some(9.5));
        assert_eq!(da.circle_size, Some(4.0));
    }

    /// A mod with an empty settings map converts to its default variant.
    #[test]
    #[cfg(feature = "serde")]
    fn try_as_mod_no_settings() {
        use crate::serde::GameModSeed;

        let simple = GameModSimple {
            acronym: "CS".parse().unwrap(),
            settings: HashMap::new(),
        };

        assert_eq!(
            simple
                .try_as_mod(GameModSeed::Mode {
                    mode: GameMode::Taiko,
                    deny_unknown_fields: true
                })
                .unwrap(),
            GameMod::ConstantSpeedTaiko(Default::default())
        );
    }

    /// An acronym that does not exist for the given mode produces the
    /// appropriate `Unknown*` variant — this is `Ok`, not `Err`.
    #[test]
    #[cfg(feature = "serde")]
    fn try_as_mod_unknown_acronym_is_ok() {
        use crate::{generated_mods::UnknownMod, serde::GameModSeed};

        let simple = GameModSimple {
            acronym: "XX".parse().unwrap(),
            settings: HashMap::new(),
        };

        assert_eq!(
            simple
                .try_as_mod(GameModSeed::Mode {
                    mode: GameMode::Osu,
                    deny_unknown_fields: true
                })
                .unwrap(),
            GameMod::UnknownOsu(UnknownMod {
                acronym: "XX".parse().unwrap()
            })
        );
    }

    /// A mode-specific acronym produces `Unknown*` when a different mode is
    /// requested — also `Ok`.
    #[test]
    #[cfg(feature = "serde")]
    fn try_as_mod_wrong_mode_is_ok_unknown() {
        use crate::{generated_mods::UnknownMod, serde::GameModSeed};

        // "FI" (FadeIn) only exists for Mania.
        let simple = GameModSimple {
            acronym: "FI".parse().unwrap(),
            settings: HashMap::new(),
        };

        assert_eq!(
            simple
                .try_as_mod(GameModSeed::Mode {
                    mode: GameMode::Osu,
                    deny_unknown_fields: true
                })
                .unwrap(),
            GameMod::UnknownOsu(UnknownMod {
                acronym: "FI".parse().unwrap()
            })
        );
    }

    /// GuessMode picks the correct mode-specific variant automatically.
    #[test]
    #[cfg(feature = "serde")]
    fn try_as_mod_guess_mode_picks_correct_variant() {
        use crate::{generated_mods::FadeInMania, serde::GameModSeed};

        // "FI" only exists for Mania; GuessMode should find it.
        let simple = GameModSimple {
            acronym: "FI".parse().unwrap(),
            settings: HashMap::new(),
        };

        assert_eq!(
            simple
                .try_as_mod(GameModSeed::GuessMode {
                    deny_unknown_fields: true
                })
                .unwrap(),
            GameMod::FadeInMania(FadeInMania::default())
        );
    }

    /// GuessMode with a setting that only matches one mode's variant selects
    /// that mode even when the acronym exists across multiple modes.
    #[test]
    #[cfg(feature = "serde")]
    fn try_as_mod_guess_mode_uses_settings_to_disambiguate() {
        use crate::{generated_mods::DifficultyAdjustTaiko, serde::GameModSeed};

        // "DA" exists for every mode, but `scroll_speed` is only a field on
        // the Taiko variant — GuessMode with deny_unknown_fields should pick it.
        let simple = GameModSimple {
            acronym: "DA".parse().unwrap(),
            settings: [("scroll_speed".into(), SettingSimple::Number(1.5))]
                .into_iter()
                .collect(),
        };

        assert_eq!(
            simple
                .try_as_mod(GameModSeed::GuessMode {
                    deny_unknown_fields: true
                })
                .unwrap(),
            GameMod::DifficultyAdjustTaiko(DifficultyAdjustTaiko {
                scroll_speed: Some(1.5),
                ..Default::default()
            })
        );
    }

    /// An unrecognised field key with `deny_unknown_fields: true` is an error.
    #[test]
    #[cfg(feature = "serde")]
    fn try_as_mod_unknown_field_denied_is_err() {
        use crate::serde::GameModSeed;

        let simple = GameModSimple {
            acronym: "CS".parse().unwrap(),
            settings: [("not_a_real_field".into(), SettingSimple::Bool(true))]
                .into_iter()
                .collect(),
        };

        assert!(simple
            .try_as_mod(GameModSeed::Mode {
                mode: GameMode::Taiko,
                deny_unknown_fields: true
            })
            .is_err());
    }

    /// The same unrecognised field is silently ignored with
    /// `deny_unknown_fields: false`, producing the default variant.
    #[test]
    #[cfg(feature = "serde")]
    fn try_as_mod_unknown_field_allowed_is_ok() {
        use crate::serde::GameModSeed;

        let simple = GameModSimple {
            acronym: "CS".parse().unwrap(),
            settings: [("not_a_real_field".into(), SettingSimple::Bool(true))]
                .into_iter()
                .collect(),
        };

        assert_eq!(
            simple
                .try_as_mod(GameModSeed::Mode {
                    mode: GameMode::Taiko,
                    deny_unknown_fields: false
                })
                .unwrap(),
            GameMod::ConstantSpeedTaiko(Default::default())
        );
    }

    /// A setting value with the wrong type for its field (a bool where a
    /// number is expected) is an error regardless of `deny_unknown_fields`.
    #[test]
    #[cfg(feature = "serde")]
    fn try_as_mod_wrong_value_type_is_err() {
        use crate::serde::GameModSeed;

        // `scroll_speed` on DifficultyAdjustTaiko expects an f64, not a bool.
        let simple = GameModSimple {
            acronym: "DA".parse().unwrap(),
            settings: [("scroll_speed".into(), SettingSimple::Bool(true))]
                .into_iter()
                .collect(),
        };

        assert!(simple
            .clone()
            .try_as_mod(GameModSeed::Mode {
                mode: GameMode::Taiko,
                deny_unknown_fields: false
            })
            .is_err());
        assert!(simple
            .try_as_mod(GameModSeed::Mode {
                mode: GameMode::Taiko,
                deny_unknown_fields: true
            })
            .is_err());
    }

    /// String settings are forwarded correctly.
    #[test]
    #[cfg(feature = "serde")]
    fn try_as_mod_string_setting() {
        use crate::serde::GameModSeed;

        let simple = GameModSimple {
            acronym: "AC".parse().unwrap(),
            settings: [(
                "accuracy_judge_mode".into(),
                SettingSimple::String("standard_all".into()),
            )]
            .into_iter()
            .collect(),
        };

        let GameMod::AccuracyChallengeOsu(ac) = simple
            .try_as_mod(GameModSeed::Mode {
                mode: GameMode::Osu,
                deny_unknown_fields: true,
            })
            .unwrap()
        else {
            panic!("expected AccuracyChallengeOsu");
        };

        assert_eq!(ac.accuracy_judge_mode.as_deref(), Some("standard_all"));
    }

    /// Bool settings are forwarded correctly.
    #[test]
    #[cfg(feature = "serde")]
    fn try_as_mod_bool_setting() {
        use crate::{generated_mods::SuddenDeathOsu, serde::GameModSeed};

        let simple = GameModSimple {
            acronym: "SD".parse().unwrap(),
            settings: [("restart".into(), SettingSimple::Bool(true))]
                .into_iter()
                .collect(),
        };

        assert_eq!(
            simple
                .try_as_mod(GameModSeed::Mode {
                    mode: GameMode::Osu,
                    deny_unknown_fields: true
                })
                .unwrap(),
            GameMod::SuddenDeathOsu(SuddenDeathOsu {
                restart: Some(true),
                ..Default::default()
            })
        );
    }

    #[test]
    #[cfg(feature = "rkyv")]
    fn roundtrip_rkyv() {
        use rkyv::{
            rancor::{BoxedError as Err, Strategy},
            Archived, Deserialize,
        };

        let mods: Vec<GameModSimple> = serde_json::from_str(JSON).unwrap();

        let bytes = rkyv::to_bytes::<Err>(&mods).unwrap();
        let archived = rkyv::access::<Archived<Vec<GameModSimple>>, Err>(&bytes).unwrap();
        let deserialized: Vec<GameModSimple> = archived
            .deserialize(Strategy::<_, Err>::wrap(&mut ()))
            .unwrap();

        assert_eq!(mods, deserialized);
    }
}
