#![cfg(feature = "rkyv")]

use std::fmt::Debug;

use rkyv::{
    api::high::{HighSerializer, HighValidator},
    bytecheck::CheckBytes,
    rancor::{Panic, ResultExt, Strategy},
    ser::allocator::ArenaHandle,
    util::AlignedVec,
    Archive, Deserialize, Serialize,
};
use rosu_mods::{
    generated_mods::{BarrelRollOsu, DoubleTimeTaiko, NoFailCatch, NoScopeOsu, UnknownMod},
    Acronym, GameMod, GameModIntermode, GameMode, GameMods, GameModsIntermode, GameModsLegacy,
};

type Serializer<'a> = HighSerializer<AlignedVec, ArenaHandle<'a>, Panic>;

fn roundtrip<T>(orig: &T)
where
    T: Archive + for<'a> Serialize<Serializer<'a>> + PartialEq + Debug,
    T::Archived: Deserialize<T, Strategy<(), Panic>> + for<'a> CheckBytes<HighValidator<'a, Panic>>,
{
    let bytes = rkyv::to_bytes(orig).always_ok();
    let archived: &T::Archived = rkyv::access(&bytes).always_ok();
    let deserialized: T = rkyv::api::deserialize_using(archived, &mut ()).always_ok();

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

#[test]
fn mods() {
    let mut mods = GameMods::new();
    mods.insert(GameMod::NoFailCatch(NoFailCatch {}));
    mods.insert(GameMod::NoScopeOsu(NoScopeOsu {
        hidden_combo_count: Some(5.0),
    }));
    mods.insert(GameMod::BarrelRollOsu(BarrelRollOsu {
        spin_speed: None,
        direction: Some(String::from("somewhere")),
    }));
    mods.insert(GameMod::DoubleTimeTaiko(DoubleTimeTaiko {
        speed_change: Some(1.234567),
        adjust_pitch: Some(true),
    }));
    mods.insert(GameMod::UnknownMania(UnknownMod {
        acronym: "WG".parse().unwrap(),
    }));

    roundtrip(&mods);
}
