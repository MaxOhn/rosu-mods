use std::fmt;

/// Available game modes
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum GameMode {
    /// osu!standard
    #[default]
    Osu = 0,
    /// osu!taiko
    Taiko = 1,
    /// osu!catch
    Catch = 2,
    /// osu!mania
    Mania = 3,
}

impl GameMode {
    /// Returns `osu`, `taiko`, `fruits`, or `mania`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Osu => "osu",
            Self::Taiko => "taiko",
            Self::Catch => "fruits",
            Self::Mania => "mania",
        }
    }
}

impl From<u8> for GameMode {
    fn from(mode: u8) -> Self {
        match mode {
            0 => GameMode::Osu,
            1 => GameMode::Taiko,
            2 => GameMode::Catch,
            3 => GameMode::Mania,
            _ => GameMode::Osu,
        }
    }
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
const _: () = {
    use serde::{
        de::{Deserialize, Deserializer, Error, Unexpected, Visitor},
        ser::{Serialize, Serializer},
    };

    struct ModeVisitor;

    impl<'de> Visitor<'de> for ModeVisitor {
        type Value = GameMode;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a gamemode")
        }

        fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
            let mode = match v {
                "0" | "osu" | "osu!" => GameMode::Osu,
                "1" | "taiko" | "tko" => GameMode::Taiko,
                "2" | "ctb" | "fruits" => GameMode::Catch,
                "3" | "mania" | "mna" => GameMode::Mania,
                _ => return Err(Error::invalid_value(Unexpected::Str(v), &Self)),
            };

            Ok(mode)
        }

        fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
            match v {
                0 => Ok(GameMode::Osu),
                1 => Ok(GameMode::Taiko),
                2 => Ok(GameMode::Catch),
                3 => Ok(GameMode::Mania),
                _ => Err(Error::invalid_value(Unexpected::Unsigned(v), &Self)),
            }
        }
    }

    impl<'de> Deserialize<'de> for GameMode {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            d.deserialize_any(ModeVisitor)
        }
    }

    impl Serialize for GameMode {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_u8(*self as u8)
        }
    }
};

#[cfg(feature = "rkyv")]
#[cfg_attr(docsrs, doc(cfg(feature = "rkyv")))]
const _: () = {
    use rkyv::{bytecheck::EnumCheckError, Archive, CheckBytes, Deserialize, Fallible, Serialize};

    #[repr(u8)]
    enum ArchivedTag {
        Osu,
        Taiko,
        Catch,
        Mania,
    }

    impl Archive for GameMode {
        type Archived = Self;
        type Resolver = ();

        unsafe fn resolve(&self, _: usize, (): Self::Resolver, out: *mut Self) {
            let tag = match self {
                Self::Osu => ArchivedTag::Osu,
                Self::Taiko => ArchivedTag::Taiko,
                Self::Catch => ArchivedTag::Catch,
                Self::Mania => ArchivedTag::Mania,
            };

            out.cast::<ArchivedTag>().write(tag);
        }
    }

    impl<S: Fallible + ?Sized> Serialize<S> for GameMode {
        fn serialize(&self, _: &mut S) -> Result<(), S::Error> {
            Ok(())
        }
    }

    impl<D: Fallible + ?Sized> Deserialize<Self, D> for GameMode {
        fn deserialize(&self, _: &mut D) -> Result<Self, D::Error> {
            Ok(*self)
        }
    }

    struct Discriminant;

    #[allow(non_upper_case_globals)]
    impl Discriminant {
        const Osu: u8 = ArchivedTag::Osu as u8;
        const Taiko: u8 = ArchivedTag::Taiko as u8;
        const Catch: u8 = ArchivedTag::Catch as u8;
        const Mania: u8 = ArchivedTag::Mania as u8;
    }

    impl<C: ?Sized> CheckBytes<C> for GameMode {
        type Error = EnumCheckError<u8>;

        unsafe fn check_bytes<'a>(
            value: *const Self,
            _: &mut C,
        ) -> Result<&'a Self, EnumCheckError<u8>> {
            let tag = *value.cast::<u8>();

            match tag {
                Discriminant::Osu
                | Discriminant::Taiko
                | Discriminant::Catch
                | Discriminant::Mania => Ok(&*value),
                _ => Err(EnumCheckError::InvalidTag(tag)),
            }
        }
    }
};
