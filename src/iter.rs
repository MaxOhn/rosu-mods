use std::{
    collections::{
        btree_map::{IntoValues, Values, ValuesMut},
        btree_set::{IntoIter, Iter},
    },
    fmt::{Debug, Formatter, Result as FmtResult},
    iter::{Copied, FusedIterator},
};

use crate::legacy_mods::GameModsLegacy;

use super::{generated_mods::GameModOrder, GameMod, GameModIntermode};

macro_rules! mods_iter {
    (
        $( #[$meta:meta] )*
        $name:ident $( < $outer_lifetime:lifetime > )? :
        $inner:ident < $( $inner_generic:tt ),+ > =>
        $item:ty
    ) => {
        $( #[ $meta ] )*
        pub struct $name $( < $outer_lifetime > )? {
            inner: $inner < $( $inner_generic ),* >,
        }

        impl $( < $outer_lifetime > )? $name $( < $outer_lifetime > )? {
            pub(super) const fn new(inner: $inner < $( $inner_generic ),* >) -> Self {
                Self { inner }
            }
        }

        impl $( < $outer_lifetime > )? Debug for $name $( < $outer_lifetime > )? {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                Debug::fmt(&self.inner, f)
            }
        }

        impl $( < $outer_lifetime > )? Iterator for $name $( < $outer_lifetime > )? {
            type Item = $item;

            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next()
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.inner.size_hint()
            }

            fn last(mut self) -> Option<Self::Item> {
                self.inner.next_back()
            }
        }

        impl $( < $outer_lifetime > )? DoubleEndedIterator for $name $( < $outer_lifetime > )? {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.inner.next_back()
            }
        }

        impl $( < $outer_lifetime > )? ExactSizeIterator for $name $( < $outer_lifetime > )? {
            fn len(&self) -> usize {
                self.inner.len()
            }
        }

        impl $( < $outer_lifetime > )? FusedIterator for $name $( < $outer_lifetime > )? {}
    };
}

mods_iter! {
    #[derive(Clone)]
    #[doc = "Iterates over [`GameMod`] references"]
    GameModsIter<'m>: Values<'m, GameModOrder, GameMod> => &'m GameMod
}
mods_iter! {
    #[doc = "Iterates over mutable [`GameMod`] references"]
    GameModsIterMut<'m>: ValuesMut<'m, GameModOrder, GameMod> => &'m mut GameMod
}
mods_iter! {
    #[doc = "Iterates over [`GameMod`]"]
    IntoGameModsIter: IntoValues<GameModOrder, GameMod> => GameMod
}

type GameModsIntermodeIterInner<'m> = Copied<Iter<'m, GameModIntermode>>;

mods_iter! {
    #[derive(Clone)]
    #[doc = "Iterates over [`GameModIntermode`]"]
    GameModsIntermodeIter<'m>: GameModsIntermodeIterInner<'m> => GameModIntermode
}
mods_iter! {
    #[doc = "Iterates over [`GameModIntermode`]"]
    IntoGameModsIntermodeIter: IntoIter<GameModIntermode> => GameModIntermode
}

/// Iterates over [`GameModsLegacy`]
pub struct GameModsLegacyIter {
    mods: GameModsLegacy,
    shift: usize,
}

impl GameModsLegacyIter {
    /// Creates a new [`GameModsLegacyIter`]
    pub const fn new(mods: GameModsLegacy) -> Self {
        Self { mods, shift: 0 }
    }
}

impl Iterator for GameModsLegacyIter {
    type Item = GameModsLegacy;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.mods.is_empty() {
            loop {
                if self.shift == 32 {
                    return None;
                }

                let mut bit = 1 << self.shift;
                self.shift += 1;

                if (GameModsLegacy::from_bits_retain(bit) == GameModsLegacy::SuddenDeath
                    && self.mods.contains(GameModsLegacy::Perfect))
                    || (GameModsLegacy::from_bits_retain(bit) == GameModsLegacy::DoubleTime
                        && self.mods.contains(GameModsLegacy::Nightcore))
                {
                    continue;
                }

                if bit == (GameModsLegacy::Nightcore - GameModsLegacy::DoubleTime).bits() {
                    bit += GameModsLegacy::DoubleTime.bits();
                } else if bit == (GameModsLegacy::Perfect - GameModsLegacy::SuddenDeath).bits() {
                    bit += GameModsLegacy::SuddenDeath.bits();
                }

                if self.mods.bits() & bit == bit {
                    let mods = GameModsLegacy::from_bits(bit);
                    self.mods.remove(mods);

                    return Some(mods);
                }
            }
        } else if self.shift == 0 {
            self.shift = 32;

            Some(GameModsLegacy::NoMod)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();

        (len, Some(len))
    }
}

// TODO: impl DoubleEndedIterator for GameModsLegacyIter

impl ExactSizeIterator for GameModsLegacyIter {
    fn len(&self) -> usize {
        self.mods.len() + usize::from(self.mods.is_empty() && self.shift == 0)
    }
}

impl FusedIterator for GameModsLegacyIter {}
