#![cfg(feature = "serde")]

use std::{
    collections::BTreeMap,
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    slice,
    str::FromStr,
};

use serde::de::{
    value::BorrowedStrDeserializer, Deserialize, DeserializeSeed, Deserializer, Error as DeError,
    IgnoredAny, MapAccess, SeqAccess, Unexpected, Visitor,
};

use crate::{
    generated_mods::{GameMod, UnknownMod},
    order::GameModOrder,
    Acronym, GameModIntermode, GameMode, GameMods, GameModsIntermode,
};

pub(crate) const BITFLAGS_U32: &str = "bitflags must be a u32";
const EXPECTED_ACRONYM_FIRST: &str = "expected `acronym` as first field";

const MODES: [GameMode; 4] = [
    GameMode::Osu,
    GameMode::Taiko,
    GameMode::Catch,
    GameMode::Mania,
];

pub(crate) struct GameModSettingsSeed<'a> {
    pub(crate) acronym: &'a str,
    pub(crate) mode: GameMode,
}

impl<'de> DeserializeSeed<'de> for GameModSettingsSeed<'_> {
    type Value = <Self as Visitor<'de>>::Value;

    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_map(self)
    }
}

/// Struct to pass parameters into a [`GameMod`] deserialization.
///
/// This deserializes an integer as bitflags, a string as an acronym, or a map
/// as a [`GameMod`] struct.
///
/// # Examples
///
/// If the [`GameMode`] is available, we can use the `Mode` variant.
///
/// ```
/// use rosu_mods::{GameMod, GameMode, serde::GameModSeed, generated_mods::DifficultyAdjustMania};
/// use serde::de;
///
/// // Note that `GameMod` does not implement `Deserialize` so we must
/// // implement it manually.
/// #[derive(serde::Deserialize)]
/// struct MyStruct(
///     #[serde(deserialize_with = "custom_deser")]
///     GameMod
/// );
///
/// // If this is our JSON...
/// const JSON: &str = r#"{
///     "mode": 3,
///     "mod": {
///         "acronym": "DA",
///         "settings": {
///             "drain_rate": 0.73
///         }
///     }
/// }"#;
///
/// // ... our deserialization could look like this
/// fn custom_deser<'de, D: de::Deserializer<'de>>(d: D) -> Result<GameMod, D::Error> {
///     struct MyVisitor;
///
///     impl<'de> de::Visitor<'de> for MyVisitor {
///         type Value = GameMod;
///
///         fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///             f.write_str("MyStruct")
///         }
///
///         fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
///         where
///             A: de::MapAccess<'de>
///         {
///             let Some("mode") = map.next_key()? else {
///                 return Err(de::Error::custom("expected `mode` as first field"));
///             };
///             let mode: GameMode = map.next_value()?;
///             let Some("mod") = map.next_key()? else {
///                 return Err(de::Error::custom("expected `mod` as second field"));
///             };
///
///             let seed = GameModSeed::Mode(mode); // <-
///
///             map.next_value_seed(seed)
///         }
///     }
///
///     d.deserialize_map(MyVisitor)
/// }
///
/// let MyStruct(gamemod) = serde_json::from_str(JSON).unwrap();
///
/// assert_eq!(
///     gamemod,
///     GameMod::DifficultyAdjustMania(DifficultyAdjustMania {
///         drain_rate: Some(0.73),
///         ..Default::default()
///     })
/// );
/// ```
///
/// If the mode is not available, we can use the `GuessMode` variant.
///
/// ```
/// use rosu_mods::{GameMod, serde::GameModSeed, generated_mods::DifficultyAdjustTaiko};
/// use serde::de;
///
/// #[derive(serde::Deserialize)]
/// struct MyStruct(
///     #[serde(deserialize_with = "custom_deser")]
///     GameMod,
/// );
///
/// // No mode in here so we can only guess
/// const JSON: &str = r#"{
///     "acronym": "DA",
///     "settings": {
///         "scroll_speed": 0.95
///     }
/// }"#;
///
/// // For the above JSON, the deserialization will try to use each mode's
/// // `GameMod` variant for the `DA` acronym.
/// // First it tries `DifficultyAdjustOsu` but sees that it has no
/// // `scroll_speed` field so it continues on and succeeds for the taiko
/// // variant.
///
/// fn custom_deser<'de, D: de::Deserializer<'de>>(d: D) -> Result<GameMod, D::Error> {
///     d.deserialize_map(GameModSeed::GuessMode)
/// }
///
/// let MyStruct(gamemod) = serde_json::from_str(JSON).unwrap();
///
/// assert_eq!(
///     gamemod,
///     GameMod::DifficultyAdjustTaiko(DifficultyAdjustTaiko {
///         scroll_speed: Some(0.95),
///         ..Default::default()
///     })
/// );
/// ```
#[derive(Copy, Clone)]
pub enum GameModSeed {
    /// Use a specified [`GameMode`] for deserialization.
    Mode(GameMode),
    /// Try to deserialize for each [`GameMode`] and pick the first one that
    /// doesn't fail.
    GuessMode,
}

impl GameModSeed {
    fn convert_acronym(self, acronym: &str) -> GameMod {
        match self {
            GameModSeed::Mode(mode) => GameMod::new(acronym, mode),
            GameModSeed::GuessMode => {
                // False positive from clippy
                #[allow(clippy::needless_match)]
                let unknown = match GameMod::new(acronym, GameMode::Osu) {
                    gamemod @ GameMod::UnknownOsu(_) => gamemod,
                    gamemod => return gamemod,
                };

                match GameMod::new(acronym, GameMode::Taiko) {
                    GameMod::UnknownTaiko(_) => {}
                    gamemod => return gamemod,
                }

                match GameMod::new(acronym, GameMode::Catch) {
                    GameMod::UnknownCatch(_) => {}
                    gamemod => return gamemod,
                }

                match GameMod::new(acronym, GameMode::Mania) {
                    GameMod::UnknownMania(_) => {}
                    gamemod => return gamemod,
                }

                unknown
            }
        }
    }
}

impl<'de> DeserializeSeed<'de> for GameModSeed {
    type Value = GameMod;

    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for GameModSeed {
    type Value = GameMod;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("GameMod")
    }

    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        Ok(self.convert_acronym(v))
    }

    fn visit_string<E: DeError>(self, v: String) -> Result<Self::Value, E> {
        self.visit_str(&v)
    }

    fn visit_i64<E: DeError>(self, v: i64) -> Result<Self::Value, E> {
        let Ok(bits) = u32::try_from(v) else {
            return Err(DeError::custom(BITFLAGS_U32));
        };

        self.visit_u32(bits)
    }

    fn visit_u32<E: DeError>(self, v: u32) -> Result<Self::Value, E> {
        let acronym = GameModIntermode::try_from_bits(v)
            .ok_or_else(|| DeError::custom(format!("invalid bits value `{v}`")))?
            .acronym();

        Ok(self.convert_acronym(acronym.as_str()))
    }

    fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
        let Ok(bits) = u32::try_from(v) else {
            return Err(DeError::custom(BITFLAGS_U32));
        };

        self.visit_u32(bits)
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let Some(GameModField::Acronym) = map.next_key()? else {
            return Err(DeError::custom(EXPECTED_ACRONYM_FIRST));
        };

        let acronym_raw: MaybeOwnedStr<'de> = map.next_value()?;
        let acronym = acronym_raw.as_str();
        let mut gamemod = None;

        while let Some(field) = map.next_key::<GameModField>()? {
            if field == GameModField::Settings {
                match self {
                    GameModSeed::Mode(mode) => {
                        let seed = GameModSettingsSeed { acronym, mode };
                        gamemod = Some(map.next_value_seed(seed)?);
                    }
                    GameModSeed::GuessMode => {
                        let settings: GameModSettings<'de> = map.next_value()?;

                        gamemod = match settings.try_deserialize(acronym) {
                            gamemod @ Some(_) => gamemod,
                            None => Some(GameMod::UnknownOsu(UnknownMod {
                                acronym: <Acronym as FromStr>::from_str(acronym)
                                    .unwrap_or(UnknownMod::UNKNOWN_ACRONYM),
                            })),
                        };
                    }
                }
            } else {
                let _: IgnoredAny = map.next_value()?;
            }
        }

        Ok(gamemod.unwrap_or_else(|| self.convert_acronym(acronym)))
    }
}

pub(crate) struct GameModSettings<'a> {
    fields: Vec<GameModSettingField<'a>>,
}

impl<'de> Deserialize<'de> for GameModSettings<'de> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct FieldsVisitor;

        impl<'de> Visitor<'de> for FieldsVisitor {
            type Value = Vec<GameModSettingField<'de>>;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("GameModSettings")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut fields = Vec::with_capacity(map.size_hint().unwrap_or(0));

                while let Some((name, value)) = map.next_entry()? {
                    fields.push(GameModSettingField { name, value });
                }

                Ok(fields)
            }
        }

        Ok(Self {
            fields: d.deserialize_map(FieldsVisitor)?,
        })
    }
}

#[derive(Debug)]
pub(crate) struct GameModDeserializeError {
    msg: Box<str>,
}

impl Display for GameModDeserializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.msg)
    }
}

impl StdError for GameModDeserializeError {}

impl DeError for GameModDeserializeError {
    fn custom<T: Display>(msg: T) -> Self {
        Self {
            msg: msg.to_string().into_boxed_str(),
        }
    }
}

impl<'de> Deserializer<'de> for &'de GameModSettings<'_> {
    type Error = GameModDeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_bool<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        _: usize,
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut d = GameModSettingsMap {
            fields: self.fields.iter(),
            value: None,
        };

        visitor.visit_map(&mut d)
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct GameModSettingsMap<'de> {
    fields: slice::Iter<'de, GameModSettingField<'de>>,
    value: Option<&'de Value<'de>>,
}

impl<'de> MapAccess<'de> for GameModSettingsMap<'de> {
    type Error = GameModDeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.fields.next() {
            Some(field) => {
                self.value = Some(&field.value);
                let key_de = BorrowedStrDeserializer::new(field.name.as_str());

                seed.deserialize(key_de).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(self.value.take().unwrap())
    }

    fn next_entry_seed<K, V>(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> Result<Option<(K::Value, V::Value)>, Self::Error>
    where
        K: DeserializeSeed<'de>,
        V: DeserializeSeed<'de>,
    {
        match self.fields.next() {
            Some(field) => {
                self.value.take();

                let key_de = BorrowedStrDeserializer::new(field.name.as_str());
                let key = kseed.deserialize(key_de)?;
                let value = vseed.deserialize(&field.value)?;

                Ok(Some((key, value)))
            }
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        let (lower, _) = self.fields.size_hint();

        Some(lower)
    }
}

struct GameModSettingField<'a> {
    name: MaybeOwnedStr<'a>,
    value: Value<'a>,
}

enum Value<'de> {
    Bool(bool),
    Str(MaybeOwnedStr<'de>),
    Number(f64),
}

impl<'de> Deserialize<'de> for Value<'de> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value<'de>;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("a bool, string, or number")
            }

            fn visit_bool<E: DeError>(self, v: bool) -> Result<Self::Value, E> {
                Ok(Value::Bool(v))
            }

            fn visit_f32<E: DeError>(self, v: f32) -> Result<Self::Value, E> {
                self.visit_f64(f64::from(v))
            }

            fn visit_f64<E: DeError>(self, v: f64) -> Result<Self::Value, E> {
                Ok(Value::Number(v))
            }

            fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
                self.visit_f64(v as f64)
            }

            fn visit_i64<E: DeError>(self, v: i64) -> Result<Self::Value, E> {
                self.visit_f64(v as f64)
            }

            fn visit_borrowed_str<E: DeError>(self, v: &'de str) -> Result<Self::Value, E> {
                Ok(Value::Str(MaybeOwnedStr::Borrowed(v)))
            }

            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                self.visit_string(v.to_owned())
            }

            fn visit_string<E: DeError>(self, v: String) -> Result<Self::Value, E> {
                Ok(Value::Str(MaybeOwnedStr::Owned(v)))
            }
        }

        d.deserialize_any(ValueVisitor)
    }
}

impl<'de> Deserializer<'de> for &'de Value<'_> {
    type Error = GameModDeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Bool(v) => visitor.visit_bool(*v),
            Value::Number(v) => visitor.visit_f64(*v),
            Value::Str(v) => visitor.visit_borrowed_str(v.as_str()),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Bool(v) => visitor.visit_bool(*v),
            Value::Number(v) => Err(DeError::invalid_type(Unexpected::Float(*v), &visitor)),
            Value::Str(v) => Err(DeError::invalid_type(Unexpected::Str(v.as_str()), &visitor)),
        }
    }

    fn deserialize_i8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Bool(v) => Err(DeError::invalid_type(Unexpected::Bool(*v), &visitor)),
            Value::Number(v) => visitor.visit_f64(*v),
            Value::Str(v) => Err(DeError::invalid_type(Unexpected::Str(v.as_str()), &visitor)),
        }
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Bool(v) => Err(DeError::invalid_type(Unexpected::Bool(*v), &visitor)),
            Value::Number(v) => Err(DeError::invalid_type(Unexpected::Float(*v), &visitor)),
            Value::Str(v) => visitor.visit_borrowed_str(v.as_str()),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        _: usize,
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

/// Struct to pass parameters into a [`GameMods`] deserialization.
///
/// This deserializes an integer as bitflags, a string as acronyms, or a
/// sequence of [`GameMod`] data.
///
/// # Example
///
/// Let's define deserialization so that each contained [`GameMod`] should
/// belong to the same (guessed) [`GameMode`].
///
/// ```
/// use rosu_mods::{
///     generated_mods::{NoScopeCatch, UnknownMod},
///     serde::GameModsSeed,
///     Acronym, GameMod, GameMods,
/// };
/// use serde::de;
/// # use std::str::FromStr;
///
/// #[derive(serde::Deserialize)]
/// struct MyStruct(#[serde(deserialize_with = "custom_deser")] GameMods);
///
/// // A JSON sequence containing bitflags of mania's `FadeIn` mod, the
/// // `FloatingFruits` acronym, and `NoScope` data.
/// const JSON: &str = r#"[
///     1048576,
///     "FF",
///     {
///         "acronym": "NS",
///         "settings": {
///             "hidden_combo_count": 5
///         }
///     }
/// ]"#;
///
/// fn custom_deser<'de, D: de::Deserializer<'de>>(d: D) -> Result<GameMods, D::Error> {
///     // Here, we're defining that all deserialized mods should belong to the
///     // same mode.
///     d.deserialize_any(GameModsSeed::SameModeForEachMod)
/// }
///
/// // Although `FI` is not a `Catch` mod, the mode still has the most
/// // successfully deserialized mods (2) so that's the one that will be
/// // chosen.
///
/// let MyStruct(mods) = serde_json::from_str(JSON).unwrap();
/// let mut iter = mods.into_iter();
///
/// // The following order of mods is irrelevant here. The iterator just shows
/// // which mods are contained.
///
/// // `FF` was only specified as acronym so it only contains default values.
/// assert_eq!(
///     iter.next(),
///     Some(GameMod::FloatingFruitsCatch(Default::default()))
/// );
///
/// // For `NS` we did deserialize a field
/// assert_eq!(
///     iter.next(),
///     Some(GameMod::NoScopeCatch(NoScopeCatch {
///         hidden_combo_count: Some(5.0)
///     }))
/// );
///
/// // `FI` is not a `Catch` mod so it's deserialized as `UnknownCatch`
/// assert_eq!(
///     iter.next(),
///     Some(GameMod::UnknownCatch(UnknownMod {
///         acronym: Acronym::from_str("FI").unwrap()
///     }))
/// );
///
/// assert_eq!(iter.next(), None);
/// ```
///
/// Next, let's define deserialization so that each contained [`GameMod`]
/// picks the first [`GameMode`] that deserializes successfully.
///
/// ```
/// use rosu_mods::{serde::GameModsSeed, GameMod, GameMods};
/// use serde::de;
///
/// #[derive(serde::Deserialize)]
/// struct MyStruct(#[serde(deserialize_with = "custom_deser")] GameMods);
///
/// // The bitflags for `DT` and `FI`
/// const JSON: &str = "1048640";
///
/// fn custom_deser<'de, D: de::Deserializer<'de>>(d: D) -> Result<GameMods, D::Error> {
///     d.deserialize_any(GameModsSeed::AllowMultipleModes)
/// }
///
/// let MyStruct(mods) = serde_json::from_str(JSON).unwrap();
/// let mut iter = mods.into_iter();
///
/// // `Osu` is the first mode that successfully deserializes `DT`.
/// assert_eq!(
///     iter.next(),
///     Some(GameMod::DoubleTimeOsu(Default::default()))
/// );
///
/// // `FI` is only deserialized properly for `Mania`
/// assert_eq!(
///     iter.next(),
///     Some(GameMod::FadeInMania(Default::default()))
/// );
///
/// assert_eq!(iter.next(), None);
/// ```
#[derive(Copy, Clone)]
pub enum GameModsSeed {
    /// Use a specified [`GameMode`] for deserialization.
    Mode(GameMode),
    /// For each contained [`GameMod`], try to deserialize it for each
    /// [`GameMode`] and pick the first one that doesn't fail.
    AllowMultipleModes,
    /// For each [`GameMode`], deserialize each [`GameMod`] for that mode and
    /// pick the first one for which each [`GameMod`] succeeds deserialization
    /// or alternatively the mode with the least amount of unknown mods.
    SameModeForEachMod,
}

impl GameModsSeed {
    fn convert_intermode(self, intermode: &GameModsIntermode) -> GameMods {
        match self {
            GameModsSeed::Mode(mode) => intermode.with_mode(mode),
            GameModsSeed::SameModeForEachMod => {
                for mode in MODES {
                    if let Some(mods) = intermode.try_with_mode(mode) {
                        return mods;
                    }
                }

                intermode.with_mode(GameMode::Osu)
            }
            GameModsSeed::AllowMultipleModes => intermode
                .iter()
                .map(|gamemod| {
                    let [osu, modes @ ..] = MODES;
                    let osu = GameMod::new(gamemod.acronym().as_str(), osu);

                    if !matches!(osu, GameMod::UnknownOsu(_)) {
                        return osu;
                    }

                    for mode in modes {
                        match GameMod::new(gamemod.acronym().as_str(), mode) {
                            GameMod::UnknownTaiko(_)
                            | GameMod::UnknownCatch(_)
                            | GameMod::UnknownMania(_) => {}
                            gamemod => return gamemod,
                        }
                    }

                    osu
                })
                .collect(),
        }
    }
}

impl<'de> DeserializeSeed<'de> for GameModsSeed {
    type Value = GameMods;

    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for GameModsSeed {
    type Value = GameMods;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("GameMods")
    }

    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        let intermode = v.parse::<GameModsIntermode>().map_err(DeError::custom)?;

        Ok(self.convert_intermode(&intermode))
    }

    fn visit_string<E: DeError>(self, v: String) -> Result<Self::Value, E> {
        self.visit_str(&v)
    }

    fn visit_i64<E: DeError>(self, v: i64) -> Result<Self::Value, E> {
        let Ok(bits) = u32::try_from(v) else {
            return Err(DeError::custom(BITFLAGS_U32));
        };

        self.visit_u32(bits)
    }

    fn visit_u32<E: DeError>(self, v: u32) -> Result<Self::Value, E> {
        let intermode = GameModsIntermode::from_bits(v);

        Ok(self.convert_intermode(&intermode))
    }

    fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
        let Ok(bits) = u32::try_from(v) else {
            return Err(DeError::custom(BITFLAGS_U32));
        };

        self.visit_u32(bits)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut inner = BTreeMap::new();

        let seed = match self {
            GameModsSeed::Mode(mode) => GameModSeed::Mode(mode),
            GameModsSeed::AllowMultipleModes => GameModSeed::GuessMode,
            GameModsSeed::SameModeForEachMod => {
                let mut mods_raw = Vec::new();

                while let Some(gamemod) = seq.next_element::<GameModRaw<'de>>()? {
                    mods_raw.push(gamemod);
                }

                return GameModRaw::convert_slice::<A::Error>(&mods_raw);
            }
        };

        while let Some(gamemod) = seq.next_element_seed(seed)? {
            inner.insert(GameModOrder::from(&gamemod), gamemod);
        }

        Ok(GameMods { inner })
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        let seed = match self {
            GameModsSeed::Mode(mode) => GameModSeed::Mode(mode),
            GameModsSeed::AllowMultipleModes | GameModsSeed::SameModeForEachMod => {
                GameModSeed::GuessMode
            }
        };

        let gamemod = seed.visit_map(map)?;

        let mut mods = GameMods::new();
        mods.insert(gamemod);

        Ok(mods)
    }
}

pub(crate) enum GameModRaw<'a> {
    Bits(u32),
    Acronym(MaybeOwnedStr<'a>),
    Full {
        acronym: MaybeOwnedStr<'a>,
        settings: GameModSettings<'a>,
    },
}

impl GameModRaw<'_> {
    fn convert_slice<E: DeError>(mods_raw: &[Self]) -> Result<GameMods, E> {
        // Collect raw mods for each mode and see which one has the most known
        // mods.
        let mut inner = BTreeMap::new();
        let mut best_known_count = 0;

        for mode in MODES {
            let mut mods = BTreeMap::new();

            for mod_raw in mods_raw.iter() {
                let gamemod = mod_raw.try_convert(mode)?;
                mods.insert(GameModOrder::from(&gamemod), gamemod);
            }

            let known_count = mods
                .values()
                .filter(|gamemod| {
                    !matches!(
                        gamemod,
                        GameMod::UnknownOsu(_)
                            | GameMod::UnknownTaiko(_)
                            | GameMod::UnknownCatch(_)
                            | GameMod::UnknownMania(_)
                    )
                })
                .count();

            if known_count == mods_raw.len() {
                return Ok(GameMods { inner: mods });
            } else if known_count > best_known_count {
                best_known_count = known_count;
                inner = mods;
            }
        }

        Ok(GameMods { inner })
    }

    fn try_convert<E: DeError>(&self, mode: GameMode) -> Result<GameMod, E> {
        match self {
            GameModRaw::Bits(bits) => GameModIntermode::try_from_bits(*bits)
                .ok_or_else(|| DeError::custom(format!("invalid bits value `{bits}`")))
                .map(|intermode| GameMod::new(intermode.acronym().as_str(), mode)),
            GameModRaw::Acronym(acronym) => Ok(GameMod::new(acronym.as_str(), mode)),
            GameModRaw::Full { acronym, settings } => GameModSettingsSeed {
                acronym: acronym.as_str(),
                mode,
            }
            .deserialize(settings)
            .map_err(DeError::custom),
        }
    }
}

impl<'de> Deserialize<'de> for GameModRaw<'de> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct GameModRawVisitor;

        impl<'de> Visitor<'de> for GameModRawVisitor {
            type Value = GameModRaw<'de>;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("GameMod")
            }

            fn visit_i64<E: DeError>(self, v: i64) -> Result<Self::Value, E> {
                let bits = u32::try_from(v).map_err(|_| DeError::custom(BITFLAGS_U32))?;

                self.visit_u32(bits)
            }

            fn visit_u32<E: DeError>(self, v: u32) -> Result<Self::Value, E> {
                Ok(GameModRaw::Bits(v))
            }

            fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
                let bits = u32::try_from(v).map_err(|_| DeError::custom(BITFLAGS_U32))?;

                self.visit_u32(bits)
            }

            fn visit_borrowed_str<E: DeError>(self, v: &'de str) -> Result<Self::Value, E> {
                Ok(GameModRaw::Acronym(MaybeOwnedStr::Borrowed(v)))
            }

            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                self.visit_string(v.to_owned())
            }

            fn visit_string<E: DeError>(self, v: String) -> Result<Self::Value, E> {
                Ok(GameModRaw::Acronym(MaybeOwnedStr::Owned(v)))
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let Some(GameModField::Acronym) = map.next_key()? else {
                    return Err(DeError::custom(EXPECTED_ACRONYM_FIRST));
                };

                let acronym: MaybeOwnedStr<'de> = map.next_value()?;

                loop {
                    match map.next_key::<GameModField>()? {
                        Some(GameModField::Settings) => {
                            let settings: GameModSettings<'de> = map.next_value()?;

                            while map.next_entry::<GameModField, IgnoredAny>()?.is_some() {}

                            return Ok(GameModRaw::Full { acronym, settings });
                        }
                        Some(_) => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                        None => return Ok(GameModRaw::Acronym(acronym)),
                    }
                }
            }
        }

        d.deserialize_any(GameModRawVisitor)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum GameModField {
    Acronym,
    Settings,
    Other,
}

impl<'de> Deserialize<'de> for GameModField {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct GameModFieldVisitor;

        impl Visitor<'_> for GameModFieldVisitor {
            type Value = GameModField;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("identifier")
            }

            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                let field = match v {
                    "acronym" => GameModField::Acronym,
                    "settings" => GameModField::Settings,
                    _ => GameModField::Other,
                };

                Ok(field)
            }

            fn visit_string<E: DeError>(self, v: String) -> Result<Self::Value, E> {
                self.visit_str(&v)
            }
        }

        d.deserialize_identifier(GameModFieldVisitor)
    }
}

pub(crate) enum MaybeOwnedStr<'a> {
    Borrowed(&'a str),
    Owned(String),
}

impl MaybeOwnedStr<'_> {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            MaybeOwnedStr::Borrowed(a) => a,
            MaybeOwnedStr::Owned(a) => a,
        }
    }
}

impl<'de> Deserialize<'de> for MaybeOwnedStr<'de> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AcronymRawVisitor;

        impl<'de> Visitor<'de> for AcronymRawVisitor {
            type Value = MaybeOwnedStr<'de>;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("string")
            }

            fn visit_borrowed_str<E: DeError>(self, v: &'de str) -> Result<Self::Value, E> {
                Ok(MaybeOwnedStr::Borrowed(v))
            }

            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                self.visit_string(v.to_owned())
            }

            fn visit_string<E: DeError>(self, v: String) -> Result<Self::Value, E> {
                Ok(MaybeOwnedStr::Owned(v))
            }
        }

        d.deserialize_str(AcronymRawVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Deserializer;

    use crate::generated_mods::{AccuracyChallengeOsu, AccuracyChallengeTaiko};

    use super::*;

    #[test]
    fn deser_mod_bits() {
        let json = "64";

        let mut d = Deserializer::from_str(json);
        let osu_dt = GameModSeed::GuessMode.deserialize(&mut d).unwrap();
        assert_eq!(osu_dt, GameMod::DoubleTimeOsu(Default::default()));

        let mut d = Deserializer::from_str(json);
        let taiko_dt = GameModSeed::Mode(GameMode::Taiko)
            .deserialize(&mut d)
            .unwrap();
        assert_eq!(taiko_dt, GameMod::DoubleTimeTaiko(Default::default()));
    }

    #[test]
    fn deser_mod_bits_fail() {
        let json = "2147483648";

        let mut d = Deserializer::from_str(json);
        let err = GameModSeed::GuessMode.deserialize(&mut d);
        assert!(err.is_err());

        let mut d = Deserializer::from_str(json);
        let err = GameModSeed::Mode(GameMode::Mania).deserialize(&mut d);
        assert!(err.is_err());
    }

    #[test]
    fn deser_mod_acronym() {
        let json = r#""AS""#;

        let mut d = Deserializer::from_str(json);
        let osu_as = GameModSeed::GuessMode.deserialize(&mut d).unwrap();
        assert_eq!(osu_as, GameMod::AdaptiveSpeedOsu(Default::default()));

        let mut d = Deserializer::from_str(json);
        let taiko_as = GameModSeed::Mode(GameMode::Taiko)
            .deserialize(&mut d)
            .unwrap();
        assert_eq!(taiko_as, GameMod::AdaptiveSpeedTaiko(Default::default()));

        let mut d = Deserializer::from_str(json);
        let catch_unknown = GameModSeed::Mode(GameMode::Catch)
            .deserialize(&mut d)
            .unwrap();
        assert_eq!(
            catch_unknown,
            GameMod::UnknownCatch(UnknownMod {
                acronym: "AS".parse().unwrap()
            })
        );
    }

    #[test]
    fn deser_mod_data() {
        let json = r#"{
            "acronym": "AC",
            "settings": {
                "minimum_accuracy": 12.34,
                "accuracy_judge_mode": "my string",
                "restart": false
            }        
        }"#;

        let mut d = Deserializer::from_str(json);
        let osu_ac = GameModSeed::GuessMode.deserialize(&mut d).unwrap();
        assert_eq!(
            osu_ac,
            GameMod::AccuracyChallengeOsu(AccuracyChallengeOsu {
                minimum_accuracy: Some(12.34),
                accuracy_judge_mode: Some(String::from("my string")),
                restart: Some(false),
            })
        );

        let mut d = Deserializer::from_str(json);
        let taiko_ac = GameModSeed::Mode(GameMode::Taiko)
            .deserialize(&mut d)
            .unwrap();
        assert_eq!(
            taiko_ac,
            GameMod::AccuracyChallengeTaiko(AccuracyChallengeTaiko {
                minimum_accuracy: Some(12.34),
                accuracy_judge_mode: Some(String::from("my string")),
                restart: Some(false),
            })
        );
    }

    #[test]
    fn deser_mod_data_unknown_field() {
        let json = r#"{
            "acronym": "HD",
            "settings": {
                "unknown_field": true,
            }        
        }"#;

        let mut d = Deserializer::from_str(json);
        let err = GameModSeed::GuessMode.deserialize(&mut d);
        assert!(err.is_err());

        let mut d = Deserializer::from_str(json);
        let err = GameModSeed::Mode(GameMode::Catch).deserialize(&mut d);
        assert!(err.is_err());
    }

    #[test]
    fn deser_mod_data_invalid_type() {
        let json = r#"{
            "acronym": "DT",
            "settings": {
                "speed_change": "should be number; not string",
            }        
        }"#;

        let mut d = Deserializer::from_str(json);
        let err = GameModSeed::GuessMode.deserialize(&mut d);
        assert!(err.is_err());

        let mut d = Deserializer::from_str(json);
        let err = GameModSeed::Mode(GameMode::Catch).deserialize(&mut d);
        assert!(err.is_err());
    }

    #[test]
    fn deser_mods_bits() {
        let json = "1048584";

        let mut d = Deserializer::from_str(json);
        let mania_hdfi = GameModsSeed::SameModeForEachMod
            .deserialize(&mut d)
            .unwrap();
        let mut expected = GameMods::new();
        expected.insert(GameMod::HiddenMania(Default::default()));
        expected.insert(GameMod::FadeInMania(Default::default()));
        assert_eq!(mania_hdfi, expected);

        let mut d = Deserializer::from_str(json);
        let osu_hd_mania_fi = GameModsSeed::AllowMultipleModes
            .deserialize(&mut d)
            .unwrap();
        let mut expected = GameMods::new();
        expected.insert(GameMod::HiddenOsu(Default::default()));
        expected.insert(GameMod::FadeInMania(Default::default()));
        assert_eq!(osu_hd_mania_fi, expected);

        let mut d = Deserializer::from_str(json);
        let osu_hdfi = GameModsSeed::Mode(GameMode::Osu)
            .deserialize(&mut d)
            .unwrap();
        let mut expected = GameMods::new();
        expected.insert(GameMod::HiddenOsu(Default::default()));
        expected.insert(GameMod::UnknownOsu(UnknownMod {
            acronym: "FI".parse().unwrap(),
        }));
        assert_eq!(osu_hdfi, expected);
    }

    #[test]
    fn deser_mods_unknown_bits() {
        let json = "2147483648";

        let mut d = Deserializer::from_str(json);
        let mods = GameModsSeed::SameModeForEachMod
            .deserialize(&mut d)
            .unwrap();
        assert_eq!(mods.len(), 0);

        let mut d = Deserializer::from_str(json);
        let mods = GameModsSeed::AllowMultipleModes
            .deserialize(&mut d)
            .unwrap();
        assert_eq!(mods.len(), 0);

        let mut d = Deserializer::from_str(json);
        let mods = GameModsSeed::Mode(GameMode::Mania)
            .deserialize(&mut d)
            .unwrap();
        assert_eq!(mods.len(), 0);
    }

    #[test]
    fn deser_mods_acronyms() {
        let json = r#""HDFI""#;

        let mut d = Deserializer::from_str(json);
        let mania_hdfi = GameModsSeed::SameModeForEachMod
            .deserialize(&mut d)
            .unwrap();
        let mut expected = GameMods::new();
        expected.insert(GameMod::HiddenMania(Default::default()));
        expected.insert(GameMod::FadeInMania(Default::default()));
        assert_eq!(mania_hdfi, expected);

        let mut d = Deserializer::from_str(json);
        let osu_hd_mania_fi = GameModsSeed::AllowMultipleModes
            .deserialize(&mut d)
            .unwrap();
        let mut expected = GameMods::new();
        expected.insert(GameMod::HiddenOsu(Default::default()));
        expected.insert(GameMod::FadeInMania(Default::default()));
        assert_eq!(osu_hd_mania_fi, expected);

        let mut d = Deserializer::from_str(json);
        let osu_hdfi = GameModsSeed::Mode(GameMode::Osu)
            .deserialize(&mut d)
            .unwrap();
        let mut expected = GameMods::new();
        expected.insert(GameMod::HiddenOsu(Default::default()));
        expected.insert(GameMod::UnknownOsu(UnknownMod {
            acronym: "FI".parse().unwrap(),
        }));
        assert_eq!(osu_hdfi, expected);
    }

    #[test]
    fn deser_mods_data() {
        let json = r#"[
            8,
            "FF",
            {
                "acronym": "WG"
            },
            {
                "acronym": "AC",
                "settings": {
                    "minimum_accuracy": 12.34,
                    "accuracy_judge_mode": "my string",
                    "restart": false
                }        
            }
        ]"#;

        let mut d = Deserializer::from_str(json);
        let mods = GameModsSeed::SameModeForEachMod
            .deserialize(&mut d)
            .unwrap();
        let mut expected = GameMods::new();
        expected.insert(GameMod::HiddenOsu(Default::default()));
        expected.insert(GameMod::UnknownOsu(UnknownMod {
            acronym: "FF".parse().unwrap(),
        }));
        expected.insert(GameMod::WiggleOsu(Default::default()));
        expected.insert(GameMod::AccuracyChallengeOsu(AccuracyChallengeOsu {
            minimum_accuracy: Some(12.34),
            accuracy_judge_mode: Some(String::from("my string")),
            restart: Some(false),
        }));
        assert_eq!(mods, expected);

        let mut d = Deserializer::from_str(json);
        let mods = GameModsSeed::AllowMultipleModes
            .deserialize(&mut d)
            .unwrap();
        let mut expected = GameMods::new();
        expected.insert(GameMod::HiddenOsu(Default::default()));
        expected.insert(GameMod::FloatingFruitsCatch(Default::default()));
        expected.insert(GameMod::WiggleOsu(Default::default()));
        expected.insert(GameMod::AccuracyChallengeOsu(AccuracyChallengeOsu {
            minimum_accuracy: Some(12.34),
            accuracy_judge_mode: Some(String::from("my string")),
            restart: Some(false),
        }));
        assert_eq!(mods, expected);

        let mut d = Deserializer::from_str(json);
        let mods = GameModsSeed::Mode(GameMode::Taiko)
            .deserialize(&mut d)
            .unwrap();
        let mut expected = GameMods::new();
        expected.insert(GameMod::HiddenTaiko(Default::default()));
        expected.insert(GameMod::UnknownTaiko(UnknownMod {
            acronym: "FF".parse().unwrap(),
        }));
        expected.insert(GameMod::UnknownTaiko(UnknownMod {
            acronym: "WG".parse().unwrap(),
        }));
        expected.insert(GameMod::AccuracyChallengeTaiko(AccuracyChallengeTaiko {
            minimum_accuracy: Some(12.34),
            accuracy_judge_mode: Some(String::from("my string")),
            restart: Some(false),
        }));
        assert_eq!(mods, expected);
    }

    #[test]
    fn deser_mods_single_data() {
        let json = r#"{
            "acronym": "FI"
        }"#;

        let mut d = Deserializer::from_str(json);
        let mods = GameModsSeed::AllowMultipleModes
            .deserialize(&mut d)
            .unwrap();
        let mut expected = GameMods::new();
        expected.insert(GameMod::FadeInMania(Default::default()));
        assert_eq!(mods, expected);

        let mut d = Deserializer::from_str(json);
        let mods = GameModsSeed::Mode(GameMode::Taiko)
            .deserialize(&mut d)
            .unwrap();
        let mut expected = GameMods::new();
        expected.insert(GameMod::UnknownTaiko(UnknownMod {
            acronym: "FI".parse().unwrap(),
        }));
        assert_eq!(mods, expected);
    }
}
