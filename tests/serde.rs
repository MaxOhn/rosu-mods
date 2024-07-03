#![cfg(feature = "serde")]

use std::fmt::Debug;

use rosu_mods::{Acronym, GameModIntermode, GameMode, GameModsIntermode, GameModsLegacy};
use serde::{Deserialize, Serialize};

fn roundtrip<T>(orig: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq + Debug,
{
    let bytes = serde_json::to_string(&orig).unwrap();
    let deserialized = serde_json::from_str::<T>(&bytes).unwrap();

    assert_eq!(orig, &deserialized);
}

#[test]
fn acronym() {
    let acronym = "ABC".parse::<Acronym>().unwrap();
    roundtrip(&acronym);
}

#[test]
fn mode() {
    roundtrip(&GameMode::Taiko);
}

#[test]
fn legacy() {
    let mods = GameModsLegacy::Hidden | GameModsLegacy::Nightcore;
    roundtrip(&mods);
}

#[test]
fn intermode() {
    let mut mods = GameModsIntermode::new();
    mods.insert(GameModIntermode::AccuracyChallenge);
    mods.insert(GameModIntermode::AccuracyChallenge);
    mods.insert(GameModIntermode::FadeIn);
    mods.insert(GameModIntermode::Wiggle);
    mods.insert(GameModIntermode::Nightcore);
    mods.insert(GameModIntermode::Daycore);

    roundtrip(&mods);
}
