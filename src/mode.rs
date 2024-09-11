use std::fmt;

/// Available game modes
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "rkyv",
    repr(u8),
    derive(rkyv::Portable, rkyv::bytecheck::CheckBytes),
    bytecheck(crate = rkyv::bytecheck),
)]
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
    use rkyv::{rancor::Fallible, traits::NoUndef, Archive, Deserialize, Place, Serialize};

    unsafe impl NoUndef for GameMode {}

    impl Archive for GameMode {
        type Archived = Self;
        type Resolver = ();

        fn resolve(&self, (): Self::Resolver, out: Place<Self::Archived>) {
            out.write(*self);
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
};
