use std::{
    collections::HashMap,
    fmt::{Debug, Formatter, Result as FmtResult},
};

use crate::Acronym;

/// A simplified version of [`GameMod`].
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
};

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let json = r#"[
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

        let mods: Vec<GameModSimple> = serde_json::from_str(json).unwrap();

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
}
